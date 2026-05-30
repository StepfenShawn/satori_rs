//! # Simple Agent Example
//!
//! Demonstrates basic usage of the agent framework.

use agent::AgentBuilder;
use agent_llm::MockLLM;
use agent_tools::{CalculatorTool, SearchTool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("=== Simple Agent Demo ===\n");

    let agent = AgentBuilder::new()
        .name("assistant")
        .tool(CalculatorTool::new())
        .tool(SearchTool::new())
        .llm(MockLLM::new("mock", "mock-model"))
        .build_async()
        .await;

    println!("Testing Calculator Tool:");
    let result = agent.run("计算 10 * 20").await?;
    println!("Input: 10 * 20");
    println!("Output: {}\n", result);

    println!("Testing Search Tool:");
    let result = agent.run("搜索 Tokio Runtime").await?;
    println!("Input: 搜索 Tokio Runtime");
    println!("Output: {}\n", result);

    println!("=== Demo Complete ===");
    Ok(())
}