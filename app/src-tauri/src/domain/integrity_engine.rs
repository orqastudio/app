//! Schema-driven integrity engine — bridge to `orqa-validation`.
//!
//! This module forwards to the `orqa-validation` standalone crate. All check
//! logic now lives in that crate; this module provides the compatibility shim
//! so that call sites within the app continue to compile without modification.
//!
//! Previously this module used JSON round-trip conversion between the app's
//! duplicate `ArtifactGraph` type and the lib's type. Now that `artifact_graph.rs`
//! re-exports the canonical types directly from `orqa_validation`, no conversion
//! is needed — both sides use the same type.
//!
//! Public API re-exported from `orqa_validation`:
//! - [`RelationshipSchema`]
//! - [`RelationshipConstraints`]
//! - [`StatusRule`]
//! - [`ValidationContext`]
//! - [`build_validation_context`]
//! - [`run_schema_checks`]

use orqa_validation::ArtifactGraph;

use crate::domain::artifact_graph::IntegrityCheck;
// DeliveryConfig and ProjectRelationshipConfig are re-exported from orqa_validation::settings
// in project_settings.rs, so they are the SAME type as the validation lib's — no conversion needed.
use crate::domain::project_settings::{DeliveryConfig, ProjectRelationshipConfig};

// ---------------------------------------------------------------------------
// Re-exports from orqa-validation
// ---------------------------------------------------------------------------

/// Re-exported from `orqa_validation::types`.
pub use orqa_validation::RelationshipConstraints;
/// Re-exported from `orqa_validation::types`.
pub use orqa_validation::RelationshipSchema;
/// Re-exported from `orqa_validation::types`.
pub use orqa_validation::StatusRule;
/// Re-exported from `orqa_validation::types`.
pub use orqa_validation::ValidationContext;

// ---------------------------------------------------------------------------
// Context building
// ---------------------------------------------------------------------------

/// Build a `ValidationContext` by merging platform config, project relationships,
/// and plugin manifests.
///
/// Delegates to [`orqa_validation::build_validation_context`].
///
/// Build a `ValidationContext` by merging platform config, project relationships,
/// and plugin manifests.
///
/// Since `DeliveryConfig` and `ProjectRelationshipConfig` are now re-exported from
/// `orqa_validation::settings`, they are the same type — no conversion needed.
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
) -> ValidationContext {
    orqa_validation::build_validation_context(
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
/// Delegates to [`orqa_validation::validate`].
///
/// Since `ArtifactGraph` and `IntegrityCheck` are now the canonical types
/// from `orqa_validation`, no conversion is needed.
pub fn run_schema_checks(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    orqa_validation::validate(graph, ctx)
}

// ---------------------------------------------------------------------------
// Traceability
// ---------------------------------------------------------------------------

/// Re-exported traceability types from `orqa_validation`.
pub use orqa_validation::{AncestryChain, AncestryNode, TraceabilityResult, TracedArtifact};

/// Compute full traceability for a single artifact by ID.
///
/// Delegates to [`orqa_validation::compute_traceability`].
///
/// Since `ArtifactGraph` is now the canonical type from `orqa_validation`,
/// no round-trip conversion is needed.
pub fn compute_traceability_for(
    graph: &ArtifactGraph,
    artifact_id: &str,
) -> Result<TraceabilityResult, serde_json::Error> {
    Ok(orqa_validation::compute_traceability(graph, artifact_id))
}
