// Graph module for the orqa-engine crate.
//
// Re-exports the public graph API from orqa_validation so that consumers can
// import everything through orqa_engine::graph instead of depending on
// orqa_validation directly.

pub use orqa_validation::compute_traceability;
pub use orqa_validation::graph::{
    build_artifact_graph, graph_stats, ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats,
};
pub use orqa_validation::validate;
pub use orqa_validation::{auto_fix, compute_health};
