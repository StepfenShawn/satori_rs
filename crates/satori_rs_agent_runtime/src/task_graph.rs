//! Task Graph for managing task dependencies and concurrent execution

use satori_rs_agent_core::AgentError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    pub id: Uuid,
    pub name: String,
    pub deps: Vec<Uuid>,
    pub status: TaskStatus,
    pub result: Option<TaskResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub output: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct TaskGraph {
    nodes: Arc<RwLock<HashMap<Uuid, TaskNode>>>,
}

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, name: impl Into<String>, deps: Vec<Uuid>) -> Uuid {
        let id = Uuid::new_v4();
        let node = TaskNode {
            id,
            name: name.into(),
            deps,
            status: TaskStatus::Pending,
            result: None,
        };
        let mut nodes = self.nodes.write().await;
        nodes.insert(id, node);
        id
    }

    pub async fn get_node(&self, id: Uuid) -> Option<TaskNode> {
        let nodes = self.nodes.read().await;
        nodes.get(&id).cloned()
    }

    pub async fn set_status(&self, id: Uuid, status: TaskStatus) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&id) {
            node.status = status;
        }
    }

    pub async fn set_result(&self, id: Uuid, result: TaskResult) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(&id) {
            node.result = Some(result);
            node.status = TaskStatus::Completed;
        }
    }

    pub async fn get_ready_nodes(&self) -> Vec<Uuid> {
        let nodes = self.nodes.read().await;
        let mut ready = Vec::new();

        for (id, node) in nodes.iter() {
            if matches!(node.status, TaskStatus::Pending) {
                let all_deps_completed = node.deps.iter().all(|dep_id| {
                    nodes.get(dep_id)
                        .map(|n| matches!(n.status, TaskStatus::Completed))
                        .unwrap_or(false)
                });
                if all_deps_completed {
                    ready.push(*id);
                }
            }
        }
        ready
    }

    pub async fn has_pending(&self) -> bool {
        let nodes = self.nodes.read().await;
        nodes.values().any(|n| matches!(n.status, TaskStatus::Pending | TaskStatus::Running))
    }

    pub async fn execute<F, Fut>(&self, mut executor: F) -> Result<Vec<TaskResult>, AgentError>
    where
        F: FnMut(String, serde_json::Value) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<serde_json::Value, AgentError>> + Send,
    {
        let mut results = Vec::new();

        while self.has_pending().await {
            let ready_ids = self.get_ready_nodes().await;

            if ready_ids.is_empty() {
                break;
            }

            let handles: Vec<_> = ready_ids.into_iter().map(|id| {
                let nodes = Arc::clone(&self.nodes);
                let _exec = &executor;

                tokio::spawn(async move {
                    let name = {
                        let nodes = nodes.read().await;
                        nodes.get(&id).map(|n| n.name.clone()).unwrap_or_default()
                    };
                    (id, name)
                })
            }).collect();

            for handle in handles {
                if let Ok((id, name)) = handle.await {
                    self.set_status(id, TaskStatus::Running).await;
                    let result = executor(name, serde_json::json!({})).await;
                    let task_result = match result {
                        Ok(output) => TaskResult { output, error: None },
                        Err(e) => TaskResult {
                            output: serde_json::Value::Null,
                            error: Some(e.to_string()),
                        },
                    };
                    self.set_result(id, task_result.clone()).await;
                    results.push(task_result);
                }
            }
        }

        Ok(results)
    }

    pub async fn clear(&self) {
        let mut nodes = self.nodes.write().await;
        nodes.clear();
    }

    pub async fn node_count(&self) -> usize {
        let nodes = self.nodes.read().await;
        nodes.len()
    }

    pub async fn get_all_results(&self) -> Vec<TaskResult> {
        let nodes = self.nodes.read().await;
        nodes.values()
            .filter_map(|n| n.result.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_graph_basic() {
        let graph = TaskGraph::new();

        let node1_id = graph.add_node("task1", vec![]).await;
        let _node2_id = graph.add_node("task2", vec![node1_id]).await;

        assert_eq!(graph.node_count().await, 2);

        let ready = graph.get_ready_nodes().await;
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0], node1_id);
    }

    #[tokio::test]
    async fn test_task_graph_no_deps() {
        let graph = TaskGraph::new();

        let _node1 = graph.add_node("task1", vec![]).await;
        let _node2 = graph.add_node("task2", vec![]).await;

        let ready = graph.get_ready_nodes().await;
        assert_eq!(ready.len(), 2);
    }
}