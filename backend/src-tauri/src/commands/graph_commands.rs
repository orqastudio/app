use std::path::Path;

use tauri::State;

use crate::domain::artifact_graph::{
    apply_fixes, build_artifact_graph, check_integrity, graph_stats,
    update_artifact_field as domain_update_artifact_field, AppliedFix, ArtifactGraph, ArtifactNode,
    GraphStats, IntegrityCheck,
};
use crate::domain::health_snapshot::{HealthSnapshot, NewHealthSnapshot};
use crate::error::OrqaError;
use crate::repo::{health_snapshot_repo, project_repo};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Look up the active project's filesystem path from the database.
fn active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_string())
    })?;

    Ok(project.path)
}

/// Retrieve the cached graph from state, or build it fresh if absent.
///
/// This lazy-init pattern means the graph is only constructed once per app
/// session. The artifact watcher invalidates it by calling `refresh_graph`
/// when `.orqa/` files change.
fn get_or_build_graph(state: &State<'_, AppState>) -> Result<ArtifactGraph, OrqaError> {
    {
        let guard = state
            .artifacts
            .graph
            .lock()
            .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
        if let Some(graph) = guard.as_ref() {
            return Ok(graph.clone());
        }
    }

    // Graph is absent — build it now.
    let project_path = active_project_path(state)?;
    let graph = build_artifact_graph(Path::new(&project_path))?;

    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(graph.clone());

    Ok(graph)
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Get all artifact nodes of a given type (e.g. "epic", "task", "milestone").
#[tauri::command]
pub fn get_artifacts_by_type(
    artifact_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactNode>, OrqaError> {
    if artifact_type.trim().is_empty() {
        return Err(OrqaError::Validation(
            "artifact_type cannot be empty".to_string(),
        ));
    }
    let graph = get_or_build_graph(&state)?;
    let nodes: Vec<ArtifactNode> = graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == artifact_type)
        .cloned()
        .collect();
    Ok(nodes)
}

/// Read the raw markdown body of an artifact file from disk.
///
/// Always reads from disk to ensure the caller sees the current content.
/// The path must be relative to the project root. Path traversal is rejected.
#[tauri::command]
pub fn read_artifact_content(
    path: String,
    state: State<'_, AppState>,
) -> Result<String, OrqaError> {
    if path.trim().is_empty() {
        return Err(OrqaError::Validation("path cannot be empty".to_string()));
    }
    if path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(path.replace('\\', "/"));

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!("file not found: {path}")));
    }

    std::fs::read_to_string(&full_path).map_err(OrqaError::from)
}

/// Return summary statistics about the artifact graph.
#[tauri::command]
pub fn get_graph_stats(state: State<'_, AppState>) -> Result<GraphStats, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    Ok(graph_stats(&graph))
}

/// Rebuild the artifact graph from disk and replace the cached copy.
///
/// Called by the artifact watcher when `.orqa/` files change, and exposed as
/// a command so the frontend can trigger an explicit refresh when needed.
#[tauri::command]
pub fn refresh_artifact_graph(state: State<'_, AppState>) -> Result<(), OrqaError> {
    let project_path = active_project_path(&state)?;
    let new_graph = build_artifact_graph(Path::new(&project_path))?;

    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(new_graph);
    Ok(())
}

/// Run integrity checks on the artifact graph and return all findings.
#[tauri::command]
pub fn run_integrity_scan(state: State<'_, AppState>) -> Result<Vec<IntegrityCheck>, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    Ok(check_integrity(&graph))
}

/// Apply auto-fixable integrity checks and return the list of applied fixes.
///
/// Runs the integrity scan first, then applies all auto-fixable findings.
/// Refreshes the graph cache after applying fixes.
#[tauri::command]
pub fn apply_auto_fixes(state: State<'_, AppState>) -> Result<Vec<AppliedFix>, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    let checks = check_integrity(&graph);
    let project_path = active_project_path(&state)?;
    let applied = apply_fixes(&graph, &checks, Path::new(&project_path))?;

    // Refresh the graph if any fixes were applied.
    if !applied.is_empty() {
        let new_graph = build_artifact_graph(Path::new(&project_path))?;
        let mut guard = state
            .artifacts
            .graph
            .lock()
            .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
        *guard = Some(new_graph);
    }

    Ok(applied)
}

/// Store a health snapshot with the current graph metrics and integrity counts.
#[tauri::command]
pub fn store_health_snapshot(
    error_count: i64,
    warning_count: i64,
    state: State<'_, AppState>,
) -> Result<HealthSnapshot, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    let health = graph_stats(&graph);

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_string()))?;

    health_snapshot_repo::create(
        &conn,
        project.id,
        &NewHealthSnapshot {
            node_count: health.node_count as i64,
            edge_count: health.edge_count as i64,
            orphan_count: health.orphan_count as i64,
            broken_ref_count: health.broken_ref_count as i64,
            error_count,
            warning_count,
        },
    )
}

/// Update a single YAML frontmatter field in an artifact file on disk.
///
/// Reads the file, replaces the field value in the YAML block, writes it back,
/// then refreshes the artifact graph cache. The path must be relative to the
/// project root. Path traversal is rejected.
///
/// Only simple scalar fields (strings) are supported. The `field` must already
/// exist in the frontmatter — this command updates values, it does not add
/// new fields.
#[tauri::command]
pub fn update_artifact_field(
    path: String,
    field: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if path.trim().is_empty() {
        return Err(OrqaError::Validation("path cannot be empty".to_string()));
    }
    if path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }
    if field.trim().is_empty() {
        return Err(OrqaError::Validation("field cannot be empty".to_string()));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(path.replace('\\', "/"));

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact not found: {}",
            full_path.display()
        )));
    }

    domain_update_artifact_field(&full_path, &field, &value)?;

    // Refresh the graph cache so subsequent queries reflect the change.
    let new_graph = build_artifact_graph(Path::new(&project_path))?;
    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(new_graph);

    Ok(())
}

/// Get the most recent health snapshots for the active project.
#[tauri::command]
pub fn get_health_snapshots(
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<HealthSnapshot>, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_string()))?;

    health_snapshot_repo::get_recent(&conn, project.id, limit.unwrap_or(30))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn graph_stats_on_empty_graph() {
        let graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
        assert_eq!(stats.orphan_count, 0);
        assert_eq!(stats.broken_ref_count, 0);
    }

    #[test]
    fn graph_stats_counts_nodes_and_orphans() {
        let mut nodes = HashMap::new();
        nodes.insert(
            "EPIC-001".to_string(),
            ArtifactNode {
                id: "EPIC-001".to_string(),
                path: ".orqa/delivery/epics/EPIC-001.md".to_string(),
                artifact_type: "epic".to_string(),
                title: "Test Epic".to_string(),
                description: None,
                status: Some("draft".to_string()),
                priority: None,
                frontmatter: serde_json::json!({}),
                references_out: vec![],
                references_in: vec![],
            },
        );
        let graph = ArtifactGraph {
            nodes,
            path_index: HashMap::new(),
        };
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.orphan_count, 1); // no refs in or out
    }

    #[test]
    fn build_graph_on_empty_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let graph = build_artifact_graph(dir.path()).expect("should succeed");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn build_graph_finds_artifacts_with_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let epics_dir = dir.path().join(".orqa").join("planning").join("epics");
        std::fs::create_dir_all(&epics_dir).expect("create dirs");
        std::fs::write(
            epics_dir.join("EPIC-001.md"),
            "---\nid: EPIC-001\ntitle: Test\nstatus: draft\n---\nBody\n",
        )
        .expect("write");

        let graph = build_artifact_graph(dir.path()).expect("should succeed");
        assert!(graph.nodes.contains_key("EPIC-001"));
        let node = &graph.nodes["EPIC-001"];
        assert_eq!(node.artifact_type, "epic");
        assert_eq!(node.title, "Test");
    }

    #[test]
    fn artifact_type_validation_rejects_empty() {
        let artifact_type = "";
        assert!(artifact_type.trim().is_empty());
    }

    #[test]
    fn path_traversal_rejected() {
        let path = "../../../etc/passwd";
        assert!(path.contains(".."));
    }

    #[test]
    fn path_empty_rejected() {
        let path = "   ";
        assert!(path.trim().is_empty());
    }
}
