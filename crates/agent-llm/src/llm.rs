//! LLM trait definition

use async_trait::async_trait;
use agent_core::AgentError;

#[async_trait]
pub trait LLM: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError>;

    fn name(&self) -> &str;

    fn model(&self) -> &str;
}