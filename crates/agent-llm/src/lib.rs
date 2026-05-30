//! # Agent LLM
//!
//! LLM abstractions and implementations.

pub mod llm;
pub mod mock;
pub mod ollama;

pub use llm::LLM;
pub use mock::MockLLM;
pub use ollama::OllamaLLM;