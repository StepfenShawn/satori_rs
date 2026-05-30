# satori_rs

A high-performance AI Agent framework written in Rust.  

Unlike traditional agent frameworks that rely heavily on prompt chaining, satori_rs is built around:

- Actor-based architecture
- Workflow DAG execution
- Strongly-typed tool system
- Local-first LLM support
- Extensible memory and reasoning engines

## Architecture

```
User Input
    ↓
Agent
    ↓
Planner (RuleBased or LLM-based(future version))
    ↓
Tool Runtime (ToolRegistry)
    ↓
LLM Summarize (optional)
    ↓
Final Answer
```

## Features

- ⚡ Tokio-powered runtime
- 🦀 Pure Rust implementation
- 🔧 Typed tool calling
- 📈 Concurrent DAG execution
- 🤖 Ollama / OpenAI compatible
- 🧠 Extensible memory layer

## Crates

- `satori_rs_agent_core` - Core types: Message, Payload, Actor trait, Context
- `satori_rs_agent_runtime` - Tool trait, ToolRegistry, TaskGraph
- `satori_rs_agent_llm` - LLM trait, MockLLM, OllamaLLM
- `satori_rs_agent_tools` - Built-in tools: SearchTool, CalculatorTool
- `satori_rs_agent_macros` - Utility macros
- `satori_rs_agent` - Main crate combining all components

## Quick Start

```rust
use satori_rs_agent::AgentBuilder;
use satori_rs_agent_llm::MockLLM;
use satori_rs_agent_tools::{CalculatorTool, SearchTool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = AgentBuilder::new()
        .name("assistant")
        .tool(CalculatorTool::new())
        .tool(SearchTool::new())
        .llm(MockLLM::new("mock", "mock-model"))
        .build_async()
        .await;

    let result = agent.run("计算 10 * 20").await?;
    println!("Result: {}", result);

    let result = agent.run("搜索 Rust Tokio").await?;
    println!("Result: {}", result);

    Ok(())
}
```

## Built-in Tools

### CalculatorTool

Supports operations: `add`, `subtract`, `multiply`, `divide`

```json
{
  "operation": "multiply",
  "a": 10.0,
  "b": 20.0
}
```

### SearchTool

Returns mock search results for demonstration.

```json
{
  "query": "search term"
}
```

## LLM Support

### MockLLM

For testing and development without external dependencies.

### OllamaLLM

Connects to local Ollama server for real LLM inference.

```rust
use satori_rs_agent_llm::OllamaLLM;

let llm = OllamaLLM::new("http://localhost:11434", "qwen3");
```

## Extending

### Custom Tools

```rust
use async_trait::async_trait;
use satori_rs_agent_runtime::Tool;
use satori_rs_agent_core::AgentError;

struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "My custom tool"
    }

    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": { "type": "string" }
            }
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, AgentError> {
        // Your logic here
        Ok(serde_json::json!({ "result": "done" }))
    }
}
```

### Custom Planner

```rust
use async_trait::async_trait;
use satori_rs_agent::Planner;
use satori_rs_agent_core::AgentError;

struct MyPlanner;

#[async_trait]
impl Planner for MyPlanner {
    async fn plan(&self, input: &str) -> Result<Option<PlanningResult>, AgentError> {
        // Your planning logic
        Ok(None)
    }

    fn name(&self) -> &str {
        "my_planner"
    }
}
```

## Building

```bash
cargo build
cargo test
cargo run --example simple_agent
```

## Future Extensions

- Multi-Agent coordination
- Memory system
- MCP (Model Context Protocol) support
- Prolog integration
- Workflow engine

## License

MIT