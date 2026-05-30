//! Error types for the agent framework

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Message handling failed: {0}")]
    HandleError(String),

    #[error("Tool execution failed: {0}")]
    ToolError(String),

    #[error("Actor not found: {0}")]
    ActorNotFound(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Planner error: {0}")]
    PlannerError(String),

    #[error("Task graph error: {0}")]
    TaskGraphError(String),

    #[error("Registry error: {0}")]
    RegistryError(String),
}

impl serde::Serialize for AgentError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}