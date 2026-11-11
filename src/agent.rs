use crate::Res;
use crate::backend::Backend;

/// An LLM agent defined solely by its role (system prompt).
pub struct Agent<'a> {
    backend: &'a Backend,
    role: &'a str,
}

impl<'a> Agent<'a> {
    pub fn new(backend: &'a Backend, role: &'a str) -> Self {
        Self { backend, role }
    }

    pub async fn run(&self, task: &str, context: &str) -> Res<String> {
        self.backend
            .chat(self.role, &format!("{context}\n\nTask: {task}"))
            .await
    }
}
