//! Agent implementation

use crate::planner::Planner;
use satori_rs_agent_core::{AgentError, Context};
use satori_rs_agent_llm::LLM;
use satori_rs_agent_runtime::ToolRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Agent {
    pub name: String,
    pub(crate) planner: Arc<dyn Planner>,
    pub(crate) tool_registry: Arc<RwLock<ToolRegistry>>,
    pub(crate) llm: Option<Arc<dyn LLM>>,
    pub(crate) context: Context,
}

impl Agent {
    pub async fn run(&self, input: &str) -> Result<String, AgentError> {
        let planning_result = self.planner.plan(input).await?;

        let planning_result = match planning_result {
            Some(r) => r,
            None => {
                if let Some(llm) = &self.llm {
                    let summary = llm.complete(input).await?;
                    return Ok(summary);
                }
                return Ok(format!("I don't know how to handle: {}", input));
            }
        };

        let tool_result = {
            let registry = self.tool_registry.read().await;
            registry.call(&planning_result.tool_name, planning_result.arguments.clone()).await?
        };

        let result_str = tool_result
            .get("result")
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                _ => v.to_string(),
            })
            .unwrap_or_else(|| "No result".to_string());

        if let Some(llm) = &self.llm {
            let prompt = format!(
                "User asked: '{}'. Tool '{}' returned: {}. Provide a helpful response.",
                input,
                planning_result.tool_name,
                result_str
            );
            let summary = llm.complete(&prompt).await?;
            return Ok(summary);
        }

        Ok(result_str)
    }

    pub async fn run_with_tool_call(&self, tool_name: &str, arguments: serde_json::Value) -> Result<String, AgentError> {
        let tool_result = {
            let registry = self.tool_registry.read().await;
            registry.call(tool_name, arguments).await?
        };

        let result_str = tool_result
            .get("result")
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                _ => v.to_string(),
            })
            .unwrap_or_else(|| "No result".to_string());

        Ok(result_str)
    }

    pub async fn add_tool<T: satori_rs_agent_runtime::Tool + 'static>(&self, tool: T) -> Result<(), AgentError> {
        let registry = self.tool_registry.write().await;
        registry.register(tool).await
    }

    pub async fn list_tools(&self) -> Vec<String> {
        let registry = self.tool_registry.read().await;
        registry.list_tools().await
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::{builder::AgentBuilder, planner::RuleBasedPlanner};
    use satori_rs_agent_llm::MockLLM;
    use satori_rs_agent_tools::{CalculatorTool, SearchTool};

    #[tokio::test]
    async fn test_agent_calculator() {
        let planner = RuleBasedPlanner::new();
        let agent = AgentBuilder::new()
            .name("test_agent")
            .tool(CalculatorTool::new())
            .llm(MockLLM::new("test", "mock"))
            .planner(planner)
            .build_async()
            .await;

        let result: Result<String, _> = agent.run("计算 10 * 20").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_agent_search() {
        let planner = RuleBasedPlanner::new();
        let agent = AgentBuilder::new()
            .name("test_agent")
            .tool(SearchTool::new())
            .llm(MockLLM::new("test", "mock"))
            .planner(planner)
            .build_async()
            .await;

        let result: Result<String, _> = agent.run("搜索 Rust Tokio").await;
        assert!(result.is_ok());
    }
}