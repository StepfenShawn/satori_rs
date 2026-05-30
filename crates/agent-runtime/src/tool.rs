//! Tool trait definition

use async_trait::async_trait;
use agent_core::AgentError;

/// Trait for tools that can be executed by the agent.
///
/// Tools provide specific capabilities like search, calculation, etc.
/// Each tool has a name and can execute with JSON input/output.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn input_schema(&self) -> serde_json::Value;

    async fn execute(
        &self,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, AgentError>;

    fn output_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "result": {
                    "type": "string"
                }
            }
        })
    }
}