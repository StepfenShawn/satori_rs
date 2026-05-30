//! # Agent Core
//!
//! Core types and traits for the agent framework.
//! Includes message system, actor model, and context.

use serde::{Deserialize, Serialize};

pub mod actor;
pub mod context;
pub mod error;
pub mod message;

pub use actor::Actor;
pub use context::Context;
pub use error::AgentError;
pub use message::{Message, Payload, ToolCall};

/// Actor reference for message passing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRef {
    pub id: String,
}

impl ActorRef {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}