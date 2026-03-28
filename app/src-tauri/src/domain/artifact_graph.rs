//! Artifact graph — thin Tauri app layer over the engine's graph API.
//!
//! All type definitions and graph-building logic live in `orqa_validation` and
//! are accessible through `orqa_engine::graph` and `orqa_engine::validation`.
//! This module re-exports the canonical types and provides app-specific
//! wrappers that bridge between `OrqaError` and `ValidationError`.
//!
//! ## What lives here
//! - Re-exports of all canonical types via `orqa_engine`
//! - `build_artifact_graph` — delegates to `orqa_engine::graph::build_artifact_graph`
//! - `graph_stats` — delegates to `orqa_engine::graph::graph_stats`
//! - `compute_graph_health` — delegates to `orqa_engine::graph::compute_health`
//! - `check_integrity` — calls `orqa_engine::graph::validate` with a validation context
//! - `apply_fixes` — delegates to `orqa_engine::graph::auto_fix`
//! - `update_artifact_field` — delegates to `orqa_engine::validation::update_artifact_field`
//!
//! ## What does NOT live here
//! No duplicate type definitions. No JSON round-trip conversions. No duplicate
//! graph-walking or metric computation.

use std::path::Path;

use crate::error::OrqaError;

// ---------------------------------------------------------------------------
// Re-exported canonical types
// ---------------------------------------------------------------------------

pub use orqa_engine::graph::{ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats};
pub use orqa_engine::validation::{
    AppliedFix, IntegrityCategory, IntegrityCheck, IntegritySeverity,
};

// GraphHealth is in orqa_engine::metrics — re-export from there.
pub use orqa_engine::metrics::GraphHealth;

// `graph_stats` and `build_artifact_graph` are wrapped below to convert error types.
// The remaining types are used directly from orqa_engine at call sites.

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

/// Build an `ArtifactGraph` by scanning all `.md` files under `project_path/.orqa/`.
///
/// Delegates to [`orqa_engine::graph::build_artifact_graph`].
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, OrqaError> {
    orqa_engine::graph::build_artifact_graph(project_path)
        .map_err(|e| OrqaError::Validation(e.to_string()))
}

// ---------------------------------------------------------------------------
// Graph statistics and health
// ---------------------------------------------------------------------------

/// Compute summary statistics (node count, edge count, orphan count, broken refs).
///
/// Delegates to [`orqa_engine::graph::graph_stats`].
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    orqa_engine::graph::graph_stats(graph)
}

/// Compute extended structural health metrics for the artifact graph.
///
/// Delegates to [`orqa_engine::graph::compute_health`].
pub fn compute_graph_health(graph: &ArtifactGraph) -> GraphHealth {
    orqa_engine::graph::compute_health(graph)
}

// ---------------------------------------------------------------------------
// Integrity checks
// ---------------------------------------------------------------------------

/// Run integrity checks on the artifact graph and return all findings.
///
/// Builds a `ValidationContext` from the provided parameters, then delegates
/// to [`orqa_engine::graph::validate`].
pub fn check_integrity(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    delivery: &crate::domain::project_settings::DeliveryConfig,
    project_relationships: &[crate::domain::project_settings::ProjectRelationshipConfig],
    plugin_relationships: &[orqa_engine::validation::RelationshipSchema],
) -> Vec<IntegrityCheck> {
    // Delegate via the integrity_engine shim which handles DeliveryConfig conversion.
    let ctx = crate::domain::integrity_engine::build_validation_context(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
    );
    orqa_engine::graph::validate(graph, &ctx)
}

// ---------------------------------------------------------------------------
// Auto-fix engine
// ---------------------------------------------------------------------------

/// Update a single scalar frontmatter field in an artifact file.
///
/// Delegates to [`orqa_engine::graph::update_artifact_field`].
pub fn update_artifact_field(full_path: &Path, field: &str, value: &str) -> Result<(), OrqaError> {
    orqa_engine::graph::update_artifact_field(full_path, field, value)
        .map_err(|e: orqa_engine::validation::ValidationError| OrqaError::Validation(e.to_string()))
}

/// Apply auto-fixable integrity checks by modifying artifact files on disk.
///
/// Delegates to [`orqa_engine::graph::auto_fix`].
pub fn apply_fixes(
    graph: &ArtifactGraph,
    checks: &[IntegrityCheck],
    project_path: &Path,
) -> Result<Vec<AppliedFix>, OrqaError> {
    orqa_engine::graph::auto_fix(graph, checks, project_path)
        .map_err(|e| OrqaError::Validation(e.to_string()))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write_artifact(dir: &Path, name: &str, content: &str) {
        fs::create_dir_all(dir).expect("create dir");
        fs::write(dir.join(name), content).expect("write file");
    }

    #[test]
    fn empty_orqa_dir_returns_empty_graph() {
        let tmp = make_project();
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
        assert!(graph.path_index.is_empty());
    }

    #[test]
    fn file_without_id_is_skipped() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/implementation/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\ntitle: No ID\n---\n# Body\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn file_with_id_creates_node() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/implementation/epics");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\nstatus: draft\n---\n# Body\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert_eq!(graph.nodes.len(), 1);
        let node = graph.nodes.get("EPIC-001").expect("node");
        assert_eq!(node.id, "EPIC-001");
        assert_eq!(node.title, "My Epic");
        assert_eq!(node.status.as_deref(), Some("draft"));
        assert_eq!(node.artifact_type, "epic");
    }

    #[test]
    fn relationship_creates_forward_ref() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/implementation/tasks");
        // Use `enforces` — a core.json relationship type that is always valid
        // even without a project.json in the test fixture.
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-001\n    type: enforces\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-001").expect("node");
        assert_eq!(node.references_out.len(), 1);
        assert_eq!(node.references_out[0].target_id, "EPIC-001");
        assert_eq!(node.references_out[0].field, "relationships");
        assert_eq!(node.references_out[0].source_id, "TASK-001");
        assert_eq!(
            node.references_out[0].relationship_type,
            Some("enforces".to_owned())
        );
    }
}
