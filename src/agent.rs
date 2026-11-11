use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
};

use crate::Res;

const MODEL: &str = "qwen3:1.7b";

/// An LLM agent defined solely by its role (system prompt).
pub struct Agent<'a> {
    ollama: &'a Ollama,
    role: &'a str,
}

impl<'a> Agent<'a> {
    pub fn new(ollama: &'a Ollama, role: &'a str) -> Self {
        Self { ollama, role }
    }

    pub async fn run(&self, task: &str, context: &str) -> Res<String> {
        let request = ChatMessageRequest::new(
            MODEL.to_string(),
            vec![
                ChatMessage::system(self.role.to_string()),
                ChatMessage::user(format!("{context}\n\nTask: {task}")),
            ],
        );
        let response = self.ollama.send_chat_messages(request).await?;
        Ok(response.message.content)
    }
}
