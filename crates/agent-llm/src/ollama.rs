//! Ollama LLM implementation

use crate::LLM;
use agent_core::AgentError;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct OllamaLLM {
    name: String,
    model: String,
    base_url: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaLLM {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            name: "ollama".to_string(),
            model: model.into(),
            base_url: base_url.into(),
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }
}

#[async_trait]
impl LLM for OllamaLLM {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError> {
        let url = format!("{}/api/generate", self.base_url);

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AgentError::NetworkError(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            return Err(AgentError::LlmError(format!(
                "Ollama API returned error: {}",
                response.status()
            )));
        }

        let ollama_resp: OllamaResponse = response
            .json()
            .await
            .map_err(|e| AgentError::LlmError(format!("Failed to parse response: {}", e)))?;

        Ok(ollama_resp.response)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn model(&self) -> &str {
        &self.model
    }
}