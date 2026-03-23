//! Artifact graph — scanning, building, and querying `.orqa/` artifacts.
//!
//! This module re-exports the canonical graph types and construction logic from
//! `orqa-validation`, ensuring that the MCP server and the app use the same
//! implementation for graph health, statistics, and integrity checks.

use std::path::Path;

use crate::error::McpError;

// ---------------------------------------------------------------------------
// Re-export graph types from orqa-validation
// ---------------------------------------------------------------------------

pub use orqa_validation::{ArtifactGraph, ArtifactNode, ArtifactRef, GraphHealth, GraphStats};

// ---------------------------------------------------------------------------
// Re-export integrity types from orqa-validation
// ---------------------------------------------------------------------------

pub use orqa_validation::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

// ---------------------------------------------------------------------------
// Graph construction
// ---------------------------------------------------------------------------

/// Build an `ArtifactGraph` by scanning all `.md` files under the project's `.orqa/` directory.
///
/// Delegates to `orqa_validation::build_artifact_graph`, which:
/// - Applies the same two-pass algorithm (forward refs → backlinks)
/// - Filters invalid relationship types against core.json + plugin manifests
/// - Handles organisation mode (multi-project scanning with qualified IDs)
/// - Uses the correct body-ref regex that matches both numeric and hex-suffix IDs
pub fn build_artifact_graph(project_path: &Path) -> Result<ArtifactGraph, McpError> {
    orqa_validation::build_artifact_graph(project_path)
        .map_err(|e| McpError::FileSystem(e.to_string()))
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

/// Compute summary statistics for the graph.
///
/// Delegates to `orqa_validation::graph_stats`.
pub fn graph_stats(graph: &ArtifactGraph) -> GraphStats {
    orqa_validation::graph_stats(graph)
}

// ---------------------------------------------------------------------------
// Health metrics
// ---------------------------------------------------------------------------

/// Compute graph health metrics using the `orqa-validation` library.
///
/// Returns connected-component count, orphan rate, average degree, graph
/// density, largest-component ratio, pillar traceability, and
/// bidirectionality ratio — all computed from the same canonical algorithm
/// used by the Tauri app backend.
pub fn compute_health(graph: &ArtifactGraph) -> orqa_validation::GraphHealth {
    orqa_validation::compute_health(graph)
}


// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract YAML frontmatter and body from a markdown string.
///
/// Returns `(Some(frontmatter_text), body)` if the file starts with `---`,
/// or `(None, full_content)` if no frontmatter block is found.
///
/// This function is retained as a public utility used by other modules
/// in the MCP server (e.g. search and content extraction).
pub fn extract_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    let after_open = &trimmed[3..];
    let Some(close_pos) = after_open.find("\n---") else {
        return (None, content.to_string());
    };

    let fm_text = after_open[..close_pos].to_string();
    let body = after_open[close_pos + 4..]
        .trim_start_matches('\n')
        .to_string();
    (Some(fm_text), body)
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
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
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
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
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
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-001\n    type: enforced-by\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let node = graph.nodes.get("TASK-001").expect("node");
        assert_eq!(node.references_out.len(), 1);
        assert_eq!(node.references_out[0].target_id, "EPIC-001");
        assert_eq!(node.references_out[0].field, "relationships");
    }

    #[test]
    fn backlinks_are_computed_in_pass_two() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &epics_dir,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: My Epic\n---\n",
        );
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: My Task\nrelationships:\n  - target: EPIC-001\n    type: enforced-by\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let epic = graph.nodes.get("EPIC-001").expect("epic node");
        assert_eq!(epic.references_in.len(), 1);
        assert_eq!(epic.references_in[0].source_id, "TASK-001");
    }

    #[test]
    fn broken_refs_counted_in_stats() {
        let tmp = make_project();
        let tasks_dir = tmp.path().join(".orqa/delivery/tasks");
        write_artifact(
            &tasks_dir,
            "TASK-001.md",
            "---\nid: TASK-001\ntitle: Task\nrelationships:\n  - target: EPIC-MISSING\n    type: enforced-by\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);
        assert_eq!(stats.broken_ref_count, 1);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.edge_count, 1);
    }

    #[test]
    fn readme_files_are_skipped() {
        let tmp = make_project();
        let epics_dir = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &epics_dir,
            "README.md",
            "---\nid: SHOULD-SKIP\ntitle: Nav\n---\n",
        );
        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn extract_frontmatter_parses_correctly() {
        let content = "---\nid: EPIC-001\ntitle: Test\n---\n# Body content";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("id: EPIC-001"));
        assert!(body.contains("# Body content"));
    }

    #[test]
    fn extract_frontmatter_returns_none_without_delimiters() {
        let content = "# Just a markdown file\nNo frontmatter";
        let (fm, body) = extract_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    // -----------------------------------------------------------------------
    // Organisation mode tests
    // -----------------------------------------------------------------------

    /// Helper: write a minimal project.json with organisation mode enabled.
    fn write_org_project_json(dir: &Path, child_name: &str, child_path: &str) {
        let json = format!(
            r#"{{
  "name": "Test Org",
  "organisation": true,
  "projects": [
    {{ "name": "{child_name}", "path": "{child_path}" }}
  ],
  "artifacts": []
}}"#
        );
        let orqa = dir.join(".orqa");
        fs::create_dir_all(&orqa).expect("create .orqa");
        fs::write(orqa.join("project.json"), json).expect("write project.json");
    }

    #[test]
    fn organisation_mode_scans_child_project() {
        let tmp = make_project();
        // Root project with one child sub-project "app" at ./app
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Write a rule in the child project's .orqa/
        let rules_dir = child_dir.join(".orqa/process/rules");
        write_artifact(
            &rules_dir,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Test Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");

        // Should be resolvable by both qualified and bare ID.
        assert!(
            graph.nodes.contains_key("app::RULE-001"),
            "qualified key must exist"
        );
        assert!(
            graph.nodes.contains_key("RULE-001"),
            "bare-ID alias must exist for cross-project resolution"
        );
        let node = graph.nodes.get("RULE-001").expect("bare-ID lookup");
        assert_eq!(node.id, "RULE-001");
        assert_eq!(node.project.as_deref(), Some("app"));
    }

    #[test]
    fn cross_project_ref_from_root_resolves_without_broken_link() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Root epic references RULE-001 which lives in the child project.
        let root_epics = tmp.path().join(".orqa/delivery/epics");
        write_artifact(
            &root_epics,
            "EPIC-001.md",
            "---\nid: EPIC-001\ntitle: Root Epic\nrelationships:\n  - target: RULE-001\n    type: enforced-by\n---\n",
        );

        // RULE-001 is in the child project only.
        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        let stats = graph_stats(&graph);

        assert_eq!(
            stats.broken_ref_count, 0,
            "cross-project ref should not be a broken link"
        );
    }

    #[test]
    fn root_project_takes_priority_over_child_on_id_conflict() {
        let tmp = make_project();
        let child_dir = tmp.path().join("app");
        write_org_project_json(tmp.path(), "app", "app");

        // Same ID in both root and child — root wins.
        let root_rules = tmp.path().join(".orqa/process/rules");
        write_artifact(
            &root_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Root Rule\n---\n",
        );
        let child_rules = child_dir.join(".orqa/process/rules");
        write_artifact(
            &child_rules,
            "RULE-001.md",
            "---\nid: RULE-001\ntitle: Child Rule\n---\n",
        );

        let graph = build_artifact_graph(tmp.path()).expect("build");
        // Bare-ID key should resolve to the root node.
        let node = graph.nodes.get("RULE-001").expect("node");
        assert_eq!(node.title, "Root Rule", "root project node must win");
        // Qualified key still accessible for child.
        let child_node = graph.nodes.get("app::RULE-001").expect("child node");
        assert_eq!(child_node.title, "Child Rule");
    }

    #[test]
    fn child_without_orqa_dir_is_silently_skipped() {
        let tmp = make_project();
        write_org_project_json(tmp.path(), "no-orqa-project", "no-orqa");
        // "no-orqa/" directory exists but has no .orqa/ inside.
        fs::create_dir_all(tmp.path().join("no-orqa")).expect("create dir");

        let graph = build_artifact_graph(tmp.path()).expect("build");
        assert!(graph.nodes.is_empty());
    }
}
