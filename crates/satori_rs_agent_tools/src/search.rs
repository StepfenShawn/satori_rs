//! Search tool implementation

use satori_rs_agent_core::AgentError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTool {
    pub query: String,
}

impl SearchTool {
    pub fn new() -> Self {
        Self {
            query: String::new(),
        }
    }
}

impl Default for SearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl satori_rs_agent_runtime::Tool for SearchTool {
    fn name(&self) -> &str {
        "search"
    }

    fn description(&self) -> &str {
        "Search for information on the web"
    }

    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, AgentError> {
        let query = input
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let result = format!("Mock Search Result for: {}", query);

        Ok(serde_json::json!({
            "result": result,
            "query": query
        }))
    }
}