//! Knowledge injector — re-export from the orqa-engine prompt crate.
//!
//! The KnowledgeInjector implementation lives in
//! `engine/core/src/prompt/knowledge.rs`. This module re-exports the public API
//! so existing app code continues to use the `crate::domain::knowledge_injector`
//! path without changes.

pub use orqa_engine::prompt::knowledge::{KnowledgeInjector, KnowledgeInjectorError};
pub use orqa_engine::types::knowledge::KnowledgeMatch;
