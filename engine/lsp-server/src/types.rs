//! Shared types used by the LSP server.
//!
//! Graph types (`ArtifactGraph`, `ArtifactNode`, `ArtifactRef`) are re-exported
//! from `orqa_engine` — the engine is the single import point for all access layers.

pub use orqa_engine::graph::{ArtifactGraph, ArtifactNode, ArtifactRef};
