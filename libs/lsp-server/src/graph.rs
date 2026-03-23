//! Artifact graph construction for the LSP server.
//!
//! Delegates entirely to `orqa_validation::build_artifact_graph`.
//! Re-exports the canonical graph types so the rest of the LSP crate
//! can import from a single location.

pub use orqa_validation::graph::{ArtifactGraph, ArtifactNode, ArtifactRef};

use std::path::Path;

use crate::error::LspError;

/// Build an `ArtifactGraph` by scanning all `.md` files under `.orqa/`.
///
/// Delegates to `orqa_validation::build_artifact_graph`, which handles:
/// - Two-pass algorithm (forward refs → backlinks)
/// - Organisation mode (multi-project scanning with qualified IDs)
/// - Plugin-contributed artifact types for ID-prefix inference
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, LspError> {
    orqa_validation::build_artifact_graph(project_path)
        .map_err(|e| LspError::Validation(e.to_string()))
}
