//! Artifact graph construction and query functions for OrqaStudio.
//!
//! Re-exports the public graph API from orqa-validation. This crate establishes
//! the correct architectural boundary (core.md section 3.2) while the graph logic
//! lives in the validation library. As graph-specific features grow beyond what
//! validation needs, they will be added directly to this crate.

pub use orqa_validation::compute_traceability;
pub use orqa_validation::graph::{
    build_artifact_graph, graph_stats, ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats,
};
pub use orqa_validation::validate;
pub use orqa_validation::{auto_fix, compute_health, update_artifact_field};
