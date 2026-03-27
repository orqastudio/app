// Metrics module for the orqa-engine crate.
//
// Re-exports graph-theoretic metric types and computation functions from
// orqa_validation so that consumers can import them through
// orqa_engine::metrics without a direct dependency on orqa_validation.

pub use orqa_validation::metrics::{
    compute_traceability, find_siblings, trace_descendants, trace_to_pillars, AncestryChain,
    AncestryNode, TraceabilityResult, TracedArtifact,
};
pub use orqa_validation::compute_health;
pub use orqa_validation::types::GraphHealth;
