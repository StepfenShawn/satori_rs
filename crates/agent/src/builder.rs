//! Agent builder for convenient construction

use crate::planner::Planner;
use crate::{Agent, RuleBasedPlanner};
use agent_core::Context;
use agent_llm::LLM;
use agent_runtime::ToolRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AgentBuilder {
    name: String,
    tools: Vec<Arc<dyn agent_runtime::Tool>>,
    llm: Option<Arc<dyn LLM>>,
    planner: Option<Arc<dyn Planner>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            name: "agent".to_string(),
            tools: Vec::new(),
            llm: None,
            planner: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn tool<T: agent_runtime::Tool + 'static>(mut self, tool: T) -> Self {
        self.tools.push(Arc::new(tool));
        self
    }

    pub fn llm<L: LLM + 'static>(mut self, llm: L) -> Self {
        self.llm = Some(Arc::new(llm));
        self
    }

    pub fn planner<P: Planner + 'static>(mut self, planner: P) -> Self {
        self.planner = Some(Arc::new(planner));
        self
    }

    pub async fn build_async(self) -> Agent {
        let tool_registry = Arc::new(RwLock::new(ToolRegistry::new()));

        for tool in self.tools {
            let registry = tool_registry.write().await;
            let _ = registry.register_boxed(tool).await;
        }

        let planner = self.planner.unwrap_or_else(|| Arc::new(RuleBasedPlanner::new()));

        Agent {
            name: self.name,
            planner,
            tool_registry,
            llm: self.llm,
            context: Context::new("agent"),
        }
    }

    pub fn build(self) -> Agent {
        let runtime = tokio::runtime::Handle::current();

        runtime.block_on(async {
            self.build_async().await
        })
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
}