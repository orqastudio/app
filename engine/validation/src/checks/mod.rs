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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ValidationContext, RelationshipSchema};
    use orqa_engine_types::{ArtifactGraph, ArtifactNode};
    use std::collections::HashMap;

    fn empty_ctx() -> ValidationContext {
        ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: Default::default(),
            dependency_keys: Default::default(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    fn make_node(id: &str, artifact_type: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({"id": id, "type": artifact_type, "status": "active"}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    #[test]
    fn run_all_on_empty_graph_returns_no_checks() {
        let graph = ArtifactGraph::default();
        let ctx = empty_ctx();
        let checks = run_all(&graph, &ctx);
        assert!(checks.is_empty());
    }

    #[test]
    fn run_all_detects_broken_ref() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-a1b2c3d4", "task");
        // Reference to a node that does not exist in the graph
        node.references_out.push(orqa_engine_types::ArtifactRef {
            target_id: "EPIC-nonexist".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-a1b2c3d4".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        graph.nodes.insert(node.id.clone(), node);

        let ctx = empty_ctx();
        let checks = run_all(&graph, &ctx);
        assert!(
            checks.iter().any(|c| c.message.contains("EPIC-nonexist")),
            "Expected broken ref check for EPIC-nonexist"
        );
    }

    #[test]
    fn run_all_skips_schema_checks_when_artifact_types_empty() {
        // With no artifact types, the schema validation and type-prefix checks
        // should be skipped entirely (no false positives).
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-a1b2c3d4", "task");
        graph.nodes.insert(node.id.clone(), node);

        let ctx = empty_ctx();
        let checks = run_all(&graph, &ctx);
        // No schema or type-prefix errors expected when artifact_types is empty
        let schema_errors: Vec<_> = checks
            .iter()
            .filter(|c| matches!(c.category, crate::types::IntegrityCategory::SchemaViolation))
            .collect();
        assert!(
            schema_errors.is_empty(),
            "No schema errors expected with empty artifact_types: {schema_errors:?}"
        );
    }

    #[test]
    fn run_all_respects_delivery_gate() {
        // Delivery checks only run when ctx.delivery.types is non-empty.
        // This verifies the gate prevents false positives on projects with no delivery config.
        let graph = ArtifactGraph::default();
        let ctx = ValidationContext {
            delivery: Default::default(), // empty types
            ..empty_ctx()
        };
        let checks = run_all(&graph, &ctx);
        // No delivery-path errors expected on an empty delivery config
        assert!(checks.is_empty());
    }

    #[test]
    fn run_all_returns_error_for_missing_type_field() {
        // A node without a 'type' field in frontmatter should produce a finding.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-a1b2c3d4", "task");
        // Remove the type field from frontmatter
        node.frontmatter
            .as_object_mut()
            .unwrap()
            .remove("type");
        graph.nodes.insert(node.id.clone(), node);

        let ctx = empty_ctx();
        let checks = run_all(&graph, &ctx);
        assert!(
            checks.iter().any(|c| c.artifact_id == "TASK-a1b2c3d4"
                && c.message.to_lowercase().contains("type")),
            "Expected a 'missing type field' check"
        );
    }
}
