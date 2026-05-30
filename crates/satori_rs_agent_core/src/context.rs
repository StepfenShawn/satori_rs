//! Context for actor execution

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Execution context passed to actors during message handling.
///
/// Provides access to shared state and actor references.
#[derive(Debug, Clone)]
pub struct Context {
    pub actor_ref: crate::ActorRef,
    pub state: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl Context {
    pub fn new(actor_id: impl Into<String>) -> Self {
        Self {
            actor_ref: crate::ActorRef::new(actor_id),
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_state(&self, key: &str) -> Option<serde_json::Value> {
        let state = self.state.read().await;
        state.get(key).cloned()
    }

    pub async fn set_state(&self, key: impl Into<String>, value: serde_json::Value) {
        let mut state = self.state.write().await;
        state.insert(key.into(), value);
    }

    pub async fn clear_state(&self) {
        let mut state = self.state.write().await;
        state.clear();
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new("default")
    }
}