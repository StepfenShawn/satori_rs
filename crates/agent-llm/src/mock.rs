//! Mock LLM implementation for testing

use crate::LLM;
use agent_core::AgentError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct MockLLM {
    name: String,
    model: String,
    response: Option<String>,
}

impl MockLLM {
    pub fn new(name: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            response: None,
        }
    }

    pub fn with_response(mut self, response: impl Into<String>) -> Self {
        self.response = Some(response.into());
        self
    }
}

impl Default for MockLLM {
    fn default() -> Self {
        Self::new("mock", "mock-model")
    }
}

#[async_trait]
impl LLM for MockLLM {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError> {
        if let Some(response) = &self.response {
            return Ok(response.clone());
        }

        let response = format!(
            "Mock response for prompt: '{}'. This is a simulated LLM response.",
            prompt
        );

        Ok(response)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn model(&self) -> &str {
        &self.model
    }
}