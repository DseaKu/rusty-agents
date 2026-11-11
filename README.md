# rusty_agents

A minimal Rust template for multi-agent LLM pipelines. Each agent is just a role text.
The example pipeline runs a planner, a developer, a code reviewer, and an output reviewer.

## Run

```sh
cargo run -- "write a function that adds two numbers"
cat src/main.rs | cargo run -- "review this code"
```

## Providers

Select the provider with `active` in `config.yaml` (`local`, `claude`, `openai`, ...),
or for one run with `AGENTS_PROVIDER=claude cargo run -- "..."`.

Supported types: `ollama` (local), `anthropic`, and `openai` (any OpenAI-compatible API).

Put API tokens in a `.env` file (Git ignores it):

```sh
ANTHROPIC_API_KEY=your-key-here
OPENAI_API_KEY=your-key-here
```

## Customize

Edit the role texts and the agent order in `src/pipeline.rs`.
