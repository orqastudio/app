//! Schema-driven integrity engine — bridge to `orqa_engine::validation` and `orqa_engine::graph`.
//!
//! This module forwards to the engine crate, which centralises all direct
//! dependencies on `orqa_validation`. The Tauri app binary never imports
//! `orqa_validation` directly; all paths go through `orqa_engine`.
//!
//! Public API re-exported from `orqa_engine`:
//! - [`RelationshipSchema`]
//! - [`RelationshipConstraints`]
//! - [`StatusRule`]
//! - [`ValidationContext`]
//! - [`build_validation_context`]
//! - [`run_schema_checks`]

use orqa_engine::graph::ArtifactGraph;

use crate::domain::artifact_graph::IntegrityCheck;
// DeliveryConfig and ProjectRelationshipConfig are re-exported from orqa_engine::validation
// in project_settings.rs, so they are the SAME type as the validation lib's — no conversion needed.
use crate::domain::project_settings::{DeliveryConfig, ProjectRelationshipConfig};

// ---------------------------------------------------------------------------
// Re-exports from orqa_engine::validation
// ---------------------------------------------------------------------------

/// Re-exported from `orqa_engine::validation`.
pub use orqa_engine::validation::RelationshipConstraints;
/// Re-exported from `orqa_engine::validation`.
pub use orqa_engine::validation::RelationshipSchema;
/// Re-exported from `orqa_engine::validation`.
pub use orqa_engine::validation::StatusRule;
/// Re-exported from `orqa_engine::validation`.
pub use orqa_engine::validation::ValidationContext;

// ---------------------------------------------------------------------------
// Context building
// ---------------------------------------------------------------------------

/// Build a `ValidationContext` by merging platform config, project relationships,
/// and plugin manifests.
///
/// Delegates to [`orqa_engine::validation::build_validation_context`].
///
/// Since `DeliveryConfig` and `ProjectRelationshipConfig` are the same types
/// re-exported from `orqa_engine::validation`, no conversion is needed.
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
) -> ValidationContext {
    orqa_engine::validation::build_validation_context(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
    )
}

// ---------------------------------------------------------------------------
// Checks
// ---------------------------------------------------------------------------

/// Run all schema-driven integrity checks on the artifact graph.
///
/// Delegates to [`orqa_engine::graph::validate`].
///
/// Since `ArtifactGraph` and `IntegrityCheck` are the canonical types
/// from `orqa_engine::graph`, no conversion is needed.
pub fn run_schema_checks(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    orqa_engine::graph::validate(graph, ctx)
}

// ---------------------------------------------------------------------------
// Traceability
// ---------------------------------------------------------------------------

/// Re-exported traceability types from `orqa_engine::metrics`.
pub use orqa_engine::metrics::{AncestryChain, AncestryNode, TraceabilityResult, TracedArtifact};

/// Compute full traceability for a single artifact by ID.
///
/// Delegates to [`orqa_engine::graph::compute_traceability`].
///
/// Since `ArtifactGraph` is now the canonical type from `orqa_engine::graph`,
/// no round-trip conversion is needed.
pub fn compute_traceability_for(
    graph: &ArtifactGraph,
    artifact_id: &str,
) -> Result<TraceabilityResult, serde_json::Error> {
    Ok(orqa_engine::graph::compute_traceability(graph, artifact_id))
}
