//! Schema-driven integrity engine — bridge to `orqa-validation`.
//!
//! This module forwards to the `orqa-validation` standalone crate. All check
//! logic now lives in that crate; this module provides the compatibility shim
//! so that call sites within the app continue to compile without modification.
//!
//! Public API re-exported from `orqa_validation`:
//! - [`RelationshipSchema`]
//! - [`RelationshipConstraints`]
//! - [`StatusRule`]
//! - [`ValidationContext`]
//! - [`build_validation_context`]
//! - [`run_schema_checks`]

use crate::domain::artifact_graph::{ArtifactGraph, IntegrityCheck};
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
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
) -> ValidationContext {
    // Convert app-side DeliveryConfig → lib DeliveryConfig via JSON round-trip.
    // Both types have identical serde representations.
    let lib_delivery = convert_delivery(delivery);

    // Convert app-side ProjectRelationshipConfig → lib type via JSON round-trip.
    let lib_project_rels = convert_project_relationships(project_relationships);

    orqa_validation::build_validation_context(
        valid_statuses,
        &lib_delivery,
        &lib_project_rels,
        plugin_relationships,
    )
}

// ---------------------------------------------------------------------------
// Checks
// ---------------------------------------------------------------------------

/// Run all schema-driven integrity checks on the artifact graph.
///
/// Delegates to [`orqa_validation::validate`]. The app's `ArtifactGraph` and
/// the lib's `ArtifactGraph` share the same JSON representation, so round-trip
/// conversion is used.
pub fn run_schema_checks(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    // Convert app ArtifactGraph → lib ArtifactGraph via JSON.
    let lib_graph = match convert_graph(graph) {
        Ok(g) => g,
        Err(e) => {
            tracing::warn!("integrity_engine: graph conversion failed — {e}");
            return Vec::new();
        }
    };

    let lib_checks = orqa_validation::validate(&lib_graph, ctx);

    // Convert lib IntegrityCheck → app IntegrityCheck via JSON.
    lib_checks.into_iter().filter_map(convert_check).collect()
}

// ---------------------------------------------------------------------------
// Traceability
// ---------------------------------------------------------------------------

/// Re-exported traceability types from `orqa_validation`.
pub use orqa_validation::{AncestryChain, AncestryNode, TraceabilityResult, TracedArtifact};

/// Compute full traceability for a single artifact by ID.
///
/// Delegates to [`orqa_validation::compute_traceability`] via JSON round-trip
/// conversion, consistent with the integrity-check bridge pattern.
pub fn compute_traceability_for(
    graph: &ArtifactGraph,
    artifact_id: &str,
) -> Result<TraceabilityResult, serde_json::Error> {
    let lib_graph = convert_graph(graph)?;
    let result = orqa_validation::compute_traceability(&lib_graph, artifact_id);
    Ok(result)
}

// ---------------------------------------------------------------------------
// Private conversion helpers
// ---------------------------------------------------------------------------

fn convert_graph(
    graph: &ArtifactGraph,
) -> Result<orqa_validation::ArtifactGraph, serde_json::Error> {
    let json = serde_json::to_value(graph)?;
    serde_json::from_value(json)
}

fn convert_check(check: orqa_validation::IntegrityCheck) -> Option<IntegrityCheck> {
    let json = serde_json::to_value(&check).ok()?;
    serde_json::from_value(json).ok()
}

fn convert_delivery(delivery: &DeliveryConfig) -> orqa_validation::settings::DeliveryConfig {
    let json = serde_json::to_value(delivery).unwrap_or_default();
    serde_json::from_value(json).unwrap_or_default()
}

fn convert_project_relationships(
    rels: &[ProjectRelationshipConfig],
) -> Vec<orqa_validation::settings::ProjectRelationshipConfig> {
    rels.iter()
        .filter_map(|r| {
            let json = serde_json::to_value(r).ok()?;
            serde_json::from_value(json).ok()
        })
        .collect()
}
