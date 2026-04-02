//! orqa-graph: Artifact graph construction and traversal for OrqaStudio.
//!
//! This crate is the canonical home of all graph logic. The engine/validation crate
//! depends on this crate for graph operations; the dependency flows one way only
//! (engine/graph → engine/types, engine/validation → engine/graph).
//!
//! Public modules:
//! - `build`: graph construction, scanning, node assembly, type inference
//! - `metrics`: graph health metrics and traceability queries
//! - `error`: `GraphError` type for I/O and YAML parse failures

pub mod build;
pub mod error;
pub mod metrics;

// Re-export graph data types and metric types from engine/types so consumers can access
// them via this crate without depending on engine/types directly.
pub use orqa_engine_types::{
    AncestryChain, AncestryNode, ArtifactGraph, ArtifactNode, ArtifactRef, GraphHealth,
    GraphStats, OutlierAgeDistribution, TracedArtifact, TraceabilityResult,
};

// Re-export the most commonly used public API at crate root.
pub use build::{
    build_artifact_graph, build_valid_relationship_types, extract_frontmatter, graph_stats,
    humanize_stem, infer_artifact_type, load_project_config, TypeRegistry,
};
pub use error::GraphError;
pub use metrics::{
    compute_health, compute_traceability, find_siblings, trace_descendants, trace_to_pillars,
};
