use std::env;

use serde_json::{Value, json};

use crate::Res;
use crate::config::{Config, ProviderKind};

const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// The LLM provider that the agents talk to, built from the active config entry.
///
/// Ollama and OpenAI-compatible APIs share the chat completions protocol;
/// Anthropic uses the Messages API.
pub struct Backend {
    name: String,
    kind: ProviderKind,
    http: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    model: String,
    max_tokens: u32,
    effort: Option<String>,
}

impl Backend {
    /// Builds the backend for the active provider in the config.
    pub fn from_config(config: &Config) -> Res<Self> {
        let (name, p) = config.active_provider()?;

        let api_key = match p.api_key_env.as_deref().or(p.kind.default_api_key_env()) {
            Some(var) => Some(env::var(var).map_err(|_| {
                format!("provider '{name}' requires the {var} environment variable")
            })?),
            None => None,
        };

        Ok(Self {
            name: name.to_string(),
            kind: p.kind,
            http: reqwest::Client::new(),
            base_url: p
                .base_url
                .as_deref()
                .unwrap_or(p.kind.default_base_url())
                .trim_end_matches('/')
                .to_string(),
            api_key,
            model: p.model.clone(),
            max_tokens: p.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
            effort: p.effort.clone(),
        })
    }

    /// Sends one system + user message pair and returns the assistant's reply.
    pub async fn chat(&self, system: &str, user: &str) -> Res<String> {
        match self.kind {
            ProviderKind::Ollama | ProviderKind::OpenAi => self.chat_openai(system, user).await,
            ProviderKind::Anthropic => self.chat_anthropic(system, user).await,
        }
    }

    async fn chat_openai(&self, system: &str, user: &str) -> Res<String> {
        let mut body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": [
                { "role": "system", "content": system },
                { "role": "user", "content": user },
            ],
        });
        if let Some(effort) = &self.effort {
            body["reasoning_effort"] = json!(effort);
        }
        let mut request = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .json(&body);
        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }
        let payload = send(request).await?;

        payload["choices"][0]["message"]["content"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| format!("unexpected API response: {payload}").into())
    }

    async fn chat_anthropic(&self, system: &str, user: &str) -> Res<String> {
        let mut body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "system": system,
            "messages": [{ "role": "user", "content": user }],
        });
        if let Some(effort) = &self.effort {
            body["output_config"] = json!({ "effort": effort });
        }
        let mut request = self
            .http
            .post(format!("{}/messages", self.base_url))
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(&body);
        if let Some(key) = &self.api_key {
            request = request.header("x-api-key", key);
        }
        let payload = send(request).await?;

        // The reply is a list of content blocks; join the text blocks.
        let blocks = payload["content"]
            .as_array()
            .ok_or_else(|| format!("unexpected API response: {payload}"))?;
        Ok(blocks
            .iter()
            .filter(|b| b["type"] == "text")
            .filter_map(|b| b["text"].as_str())
            .collect::<Vec<_>>()
            .join(""))
    }

    /// A short label for the active provider and model, for logging.
    pub fn describe(&self) -> String {
        format!("{} ({} @ {})", self.name, self.model, self.base_url)
    }
}

/// Sends the request and returns the JSON body, or an error with the API's message.
async fn send(request: reqwest::RequestBuilder) -> Res<Value> {
    let response = request.send().await?;
    let status = response.status();
    let payload: Value = response.json().await?;
    if !status.is_success() {
        return Err(format!("API request failed ({status}): {payload}").into());
    }
    Ok(payload)
}
