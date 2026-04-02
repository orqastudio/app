//! Graph-theoretic metrics — re-exported from orqa_graph.
//!
//! All metric computation logic (health, traceability, connectivity) lives in
//! `orqa_graph::metrics`. This module re-exports the public API so that callers
//! within `orqa-validation` do not need to change their import paths.

// Re-export traceability and health types from engine/types.
pub use orqa_engine_types::{
    AncestryChain, AncestryNode, GraphHealth, OutlierAgeDistribution, TracedArtifact,
    TraceabilityResult,
};

// Re-export metric computation functions from orqa_graph.
pub use orqa_graph::{
    compute_health, compute_traceability, find_siblings, trace_descendants, trace_to_pillars,
};
