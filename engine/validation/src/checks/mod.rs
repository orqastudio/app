//! All schema-driven integrity checks.
//!
//! Each sub-module handles one category of checks. The `run_all` function
//! coordinates them into a single result list.

pub mod cardinality;
pub mod conflicts;
pub mod cycles;
pub mod delivery;
pub mod enforcement;
pub mod file_level;
pub mod schema;
pub mod status;
pub mod structural;

use std::time::Instant;

use crate::graph::ArtifactGraph;
use crate::types::{IntegrityCheck, IntegritySeverity, ValidationContext};

/// Run all schema-driven integrity checks on the graph.
///
/// Returns a list of findings (errors and warnings). The checks are driven
/// entirely by the `ValidationContext` — no relationship keys or artifact types
/// are hardcoded.
pub fn run_all(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    let start = Instant::now();
    let mut checks = Vec::new();

    structural::check_broken_refs(graph, &mut checks);
    structural::check_relationship_type_constraints(graph, ctx, &mut checks);
    structural::check_required_relationships(graph, ctx, &mut checks);
    structural::check_missing_type_field(graph, &mut checks);
    structural::check_missing_status_field(graph, &ctx.artifact_types, &mut checks);
    structural::check_duplicate_relationships(graph, &mut checks);
    structural::check_filename_matches_id(graph, &mut checks);
    cardinality::check_cardinality(graph, ctx, &mut checks);
    cycles::check_circular_dependencies(graph, ctx, &mut checks);

    if !ctx.artifact_types.is_empty() {
        structural::check_type_prefix_mismatch(graph, &ctx.artifact_types, &mut checks);
    }

    if !ctx.artifact_types.is_empty() {
        schema::check_frontmatter_schemas_with_extensions(
            graph,
            &ctx.artifact_types,
            &ctx.schema_extensions,
            &mut checks,
        );
        structural::check_status_transitions(graph, &ctx.artifact_types, &mut checks);
    }

    if !ctx.enforcement_mechanisms.is_empty() {
        enforcement::check_enforcement_mechanisms(graph, &ctx.enforcement_mechanisms, &mut checks);
    }

    if !ctx.valid_statuses.is_empty() {
        status::check_valid_statuses(graph, ctx, &mut checks);
        status::check_parent_child_consistency(graph, ctx, &mut checks);
    }

    if !ctx.delivery.types.is_empty() {
        delivery::check_delivery_paths(graph, ctx, &mut checks);
    }

    let check_count = checks.len();
    let error_count = checks
        .iter()
        .filter(|c| c.severity == IntegritySeverity::Error)
        .count();

    tracing::info!(
        subsystem = "engine",
        elapsed_ms = start.elapsed().as_millis() as u64,
        check_count = check_count,
        error_count = error_count,
        "[engine] run_all integrity checks completed"
    );

    checks
}
