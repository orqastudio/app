use std::path::Path;

use tauri::State;

use crate::domain::artifact_graph::{
    build_artifact_graph, graph_stats, ArtifactNode, ArtifactRef, ArtifactGraph, GraphStats,
};
use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Look up the active project's filesystem path from the database.
fn active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
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
            .artifact_graph
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
        .artifact_graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(graph.clone());

    Ok(graph)
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Resolve an artifact by its ID (e.g. "EPIC-048").
///
/// Returns `None` when no artifact with the given ID exists in the graph.
#[tauri::command]
pub fn resolve_artifact(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<ArtifactNode>, OrqaError> {
    if id.trim().is_empty() {
        return Err(OrqaError::Validation("id cannot be empty".to_string()));
    }
    let graph = get_or_build_graph(&state)?;
    Ok(graph.nodes.get(&id).cloned())
}

/// Resolve an artifact by its relative file path.
///
/// Returns `None` when no artifact at the given path exists in the graph.
#[tauri::command]
pub fn resolve_artifact_path(
    path: String,
    state: State<'_, AppState>,
) -> Result<Option<ArtifactNode>, OrqaError> {
    if path.trim().is_empty() {
        return Err(OrqaError::Validation("path cannot be empty".to_string()));
    }
    if path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }
    let graph = get_or_build_graph(&state)?;
    let normalised = path.replace('\\', "/");
    let id = graph.path_index.get(&normalised).cloned();
    Ok(id.and_then(|i| graph.nodes.get(&i).cloned()))
}

/// Get all forward references (outgoing edges) from an artifact.
#[tauri::command]
pub fn get_references_from(
    id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactRef>, OrqaError> {
    if id.trim().is_empty() {
        return Err(OrqaError::Validation("id cannot be empty".to_string()));
    }
    let graph = get_or_build_graph(&state)?;
    Ok(graph
        .nodes
        .get(&id)
        .map(|n| n.references_out.clone())
        .unwrap_or_default())
}

/// Get all backlinks (incoming edges) to an artifact.
#[tauri::command]
pub fn get_references_to(
    id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactRef>, OrqaError> {
    if id.trim().is_empty() {
        return Err(OrqaError::Validation("id cannot be empty".to_string()));
    }
    let graph = get_or_build_graph(&state)?;
    Ok(graph
        .nodes
        .get(&id)
        .map(|n| n.references_in.clone())
        .unwrap_or_default())
}

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
        .artifact_graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(new_graph);
    Ok(())
}
