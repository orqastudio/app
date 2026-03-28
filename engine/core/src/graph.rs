// Graph module for the orqa-engine crate.
//
// Re-exports the public graph API from orqa-graph so that consumers can
// import everything through orqa_engine::graph instead of depending on
// orqa-graph or orqa-validation directly.

pub use orqa_graph::*;
pub use orqa_validation::graph::{
    build_artifact_graph, graph_stats, ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats,
};
