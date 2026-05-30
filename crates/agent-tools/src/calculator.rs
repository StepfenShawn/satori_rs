//! Calculator tool implementation

use agent_core::AgentError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CalculatorTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl agent_runtime::Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Perform basic mathematical calculations"
    }

    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"],
                    "description": "The operation to perform"
                },
                "a": {
                    "type": "number",
                    "description": "First operand"
                },
                "b": {
                    "type": "number",
                    "description": "Second operand"
                }
            },
            "required": ["operation", "a", "b"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, AgentError> {
        let operation = input
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let a = input.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let b = input.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(AgentError::ToolError("Division by zero".to_string()));
                }
                a / b
            },
            _ => return Err(AgentError::ToolError(
                format!("Unknown operation: {}", operation)
            )),
        };

        Ok(serde_json::json!({
            "result": result,
            "operation": operation,
            "a": a,
            "b": b
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_runtime::Tool;

    #[tokio::test]
    async fn test_calculator_add() {
        let tool = CalculatorTool::new();
        let input = serde_json::json!({
            "operation": "add",
            "a": 10.0,
            "b": 20.0
        });

        let result = tool.execute(input).await.unwrap();
        assert_eq!(result["result"], 30.0);
    }

    #[tokio::test]
    async fn test_calculator_multiply() {
        let tool = CalculatorTool::new();
        let input = serde_json::json!({
            "operation": "multiply",
            "a": 123.0,
            "b": 456.0
        });

        let result = tool.execute(input).await.unwrap();
        assert_eq!(result["result"], 56088.0);
    }
}