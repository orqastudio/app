//! Artifact graph — scanning, building, and querying `.orqa/` artifacts.
//!
//! Graph construction logic lives in `orqa_graph`. This module re-exports the
//! public graph API from that crate so that callers within `orqa-validation`
//! do not need to change their import paths.

// Re-export graph types from engine/types (via orqa_graph re-exports).
pub use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats};

// Re-export graph construction and query functions from orqa_graph.
pub use orqa_graph::{
    build_artifact_graph, build_valid_relationship_types, extract_frontmatter, graph_stats,
    humanize_stem, infer_artifact_type, load_project_config, TypeRegistry,
};
