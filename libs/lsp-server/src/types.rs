//! Shared types used by the LSP server.
//!
//! Graph types (`ArtifactGraph`, `ArtifactNode`, `ArtifactRef`) are re-exported
//! from `orqa_validation` — the validation library is the single source of truth.

pub use orqa_validation::graph::{ArtifactGraph, ArtifactNode, ArtifactRef};
