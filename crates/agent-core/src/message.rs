//! Message types for agent communication

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a message in the agent system.
///
/// Messages are the primary means of communication between actors.
/// Each message has a unique ID, sender, target, and payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub target: String,
    pub payload: Payload,
}

impl Message {
    pub fn new(sender: impl Into<String>, target: impl Into<String>, payload: Payload) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender: sender.into(),
            target: target.into(),
            payload,
        }
    }

    pub fn user_input(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(sender, "agent", Payload::UserInput(content.into()))
    }

    pub fn tool_call(sender: impl Into<String>, tool_call: ToolCall) -> Self {
        Self::new(sender, "agent", Payload::ToolCall(tool_call))
    }

    pub fn tool_result(sender: impl Into<String>, result: impl Into<String>) -> Self {
        Self::new(sender, "agent", Payload::ToolResult(result.into()))
    }

    pub fn agent_response(sender: impl Into<String>, response: impl Into<String>) -> Self {
        Self::new(sender, "user", Payload::AgentResponse(response.into()))
    }
}

/// Payload types for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Payload {
    UserInput(String),
    ToolCall(ToolCall),
    ToolResult(String),
    AgentResponse(String),
}

/// Represents a tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

impl ToolCall {
    pub fn new(tool_name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            tool_name: tool_name.into(),
            arguments,
        }
    }
}