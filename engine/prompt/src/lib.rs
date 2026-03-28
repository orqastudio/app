//! orqa-prompt: Prompt generation pipeline for the OrqaStudio platform.
//!
//! Assembles system prompts from governance artifacts (P3: generated, not loaded).
//! Each submodule covers a distinct aspect of prompt generation:
//!   - `builder` — core system prompt assembly from rules, knowledge, and project config
//!   - `knowledge` — ONNX-based semantic knowledge artifact matching
//!   - `session_title` — LLM-driven session title generation via SidecarClient trait

/// Core system prompt assembly from rules, knowledge, and project config.
pub mod builder;
/// ONNX-based semantic knowledge artifact matching and embedding.
pub mod knowledge;
/// LLM-driven session title generation via SidecarClient trait.
pub mod session_title;

pub use builder::{
    build_system_prompt, collect_plugin_agent_definitions, list_knowledge_catalog,
    read_governance_file, read_rules, resolve_system_prompt,
};
