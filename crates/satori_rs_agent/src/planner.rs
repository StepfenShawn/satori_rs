//! Planner trait and rule-based implementation

use satori_rs_agent_core::AgentError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Planner: Send + Sync {
    async fn plan(&self, input: &str) -> Result<Option<PlanningResult>, AgentError>;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningResult {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub struct RuleBasedPlanner {
    name: String,
}

impl RuleBasedPlanner {
    pub fn new() -> Self {
        Self {
            name: "rule_based".to_string(),
        }
    }
}

impl Default for RuleBasedPlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Planner for RuleBasedPlanner {
    async fn plan(&self, input: &str) -> Result<Option<PlanningResult>, AgentError> {
        let input_lower = input.to_lowercase();

        if input_lower.contains("搜索") || input_lower.contains("search") || input_lower.contains("查询") {
            let query = input
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect::<String>()
                .trim()
                .to_string();

            return Ok(Some(PlanningResult {
                tool_name: "search".to_string(),
                arguments: serde_json::json!({ "query": query }),
                reasoning: "Detected search intent".to_string(),
            }));
        }

        let calc_keywords = ["计算", "calculate", "算", "乘", "加", "减", "除", "*", "+", "-", "/"];
        if calc_keywords.iter().any(|k| input_lower.contains(k)) {
            if let Some(result) = parse_calculation_input(input) {
                return Ok(Some(PlanningResult {
                    tool_name: "calculator".to_string(),
                    arguments: result,
                    reasoning: "Detected calculation intent".to_string(),
                }));
            }
        }

        Ok(None)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

fn parse_calculation_input(input: &str) -> Option<serde_json::Value> {
    let input_lower = input.to_lowercase();

    if input_lower.contains('*') || input_lower.contains("乘") {
        if let Some((a, b)) = extract_two_numbers(input) {
            return Some(serde_json::json!({
                "operation": "multiply",
                "a": a,
                "b": b
            }));
        }
    }

    if input_lower.contains('+') || input_lower.contains("加") {
        if let Some((a, b)) = extract_two_numbers(input) {
            return Some(serde_json::json!({
                "operation": "add",
                "a": a,
                "b": b
            }));
        }
    }

    if input_lower.contains('-') || input_lower.contains("减") {
        if let Some((a, b)) = extract_two_numbers(input) {
            return Some(serde_json::json!({
                "operation": "subtract",
                "a": a,
                "b": b
            }));
        }
    }

    if input_lower.contains('/') || input_lower.contains("除") {
        if let Some((a, b)) = extract_two_numbers(input) {
            return Some(serde_json::json!({
                "operation": "divide",
                "a": a,
                "b": b
            }));
        }
    }

    None
}

fn extract_two_numbers(input: &str) -> Option<(f64, f64)> {
    let chars_only: String = input
        .chars()
        .filter(|c| c.is_numeric() || c.is_whitespace() || *c == '.' || *c == '-' || *c == '+')
        .collect();

    let numbers: Vec<f64> = chars_only
        .split(|c: char| !c.is_numeric() && c != '.')
        .filter(|s| !s.is_empty() && s.trim() != "-")
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if numbers.len() >= 2 {
        Some((numbers[0], numbers[1]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_planner_search() {
        let planner = RuleBasedPlanner::new();
        let result = planner.plan("搜索 Rust Tokio").await.unwrap();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.tool_name, "search");
    }

    #[tokio::test]
    async fn test_planner_calculator() {
        let planner = RuleBasedPlanner::new();
        let result = planner.plan("计算 123 * 456").await.unwrap();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.tool_name, "calculator");
    }

    #[tokio::test]
    async fn test_planner_no_match() {
        let planner = RuleBasedPlanner::new();
        let result = planner.plan("Hello world").await.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_two_numbers("计算 10 * 20"), Some((10.0, 20.0)));
        assert_eq!(extract_two_numbers("10 + 20"), Some((10.0, 20.0)));
        assert_eq!(extract_two_numbers("100 - 30"), Some((100.0, 30.0)));
        assert_eq!(extract_two_numbers("50 / 5"), Some((50.0, 5.0)));
    }
}