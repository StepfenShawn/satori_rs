//! Actor trait definition for the actor model

use async_trait::async_trait;
use crate::{AgentError, Context, Message};

/// Trait for actors in the agent system.
///
/// Actors process messages and return responses.
/// Each actor maintains its own state and can be mutated
/// through message handling.
#[async_trait]
pub trait Actor: Send + Sync {
    async fn handle(
        &mut self,
        ctx: &Context,
        msg: Message,
    ) -> Result<Message, AgentError>;

    fn name(&self) -> &str;
}