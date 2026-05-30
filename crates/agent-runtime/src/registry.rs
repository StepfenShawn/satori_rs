//! Tool registry for managing and executing tools

use crate::Tool;
use agent_core::AgentError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}

impl Clone for ToolRegistry {
    fn clone(&self) -> Self {
        Self {
            tools: Arc::clone(&self.tools),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register<T: Tool + 'static>(&self, tool: T) -> Result<(), AgentError> {
        let name = tool.name().to_string();
        let mut tools = self.tools.write().await;
        tools.insert(name, Arc::new(tool));
        Ok(())
    }

    pub async fn register_boxed(&self, tool: Arc<dyn Tool>) -> Result<(), AgentError> {
        let name = tool.name().to_string();
        let mut tools = self.tools.write().await;
        tools.insert(name, tool);
        Ok(())
    }

    pub async fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        let tools = self.tools.read().await;
        tools.get(name).cloned()
    }

    pub async fn call(
        &self,
        name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, AgentError> {
        let tool = self.get(name).await
            .ok_or_else(|| AgentError::ToolError(format!("Tool not found: {}", name)))?;

        tool.execute(input).await
    }

    pub async fn list_tools(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.keys().cloned().collect()
    }

    pub async fn has_tool(&self, name: &str) -> bool {
        let tools = self.tools.read().await;
        tools.contains_key(name)
    }

    pub async fn unregister(&self, name: &str) -> bool {
        let mut tools = self.tools.write().await;
        tools.remove(name).is_some()
    }

    pub async fn clear(&self) {
        let mut tools = self.tools.write().await;
        tools.clear();
    }
}