use std::path::Path;

use tauri::State;

use crate::domain::artifact_graph::{
    build_artifact_graph, update_artifact_field as domain_update_artifact_field,
};
use crate::domain::status_transitions::{evaluate_transitions, ProposedTransition};
use crate::error::OrqaError;
use crate::state::AppState;

use super::helpers::active_project_path;

/// Load `StatusDefinition` entries for a project path from `project.json`.
fn load_status_definitions(
    project_path: &str,
) -> Vec<crate::domain::project_settings::StatusDefinition> {
    crate::repo::project_settings_repo::read(project_path)
        .unwrap_or(None)
        .map(|s| s.statuses)
        .unwrap_or_default()
}

/// Retrieve the cached graph from state, or build it fresh if absent.
fn get_or_build_graph(
    state: &State<'_, AppState>,
) -> Result<crate::domain::artifact_graph::ArtifactGraph, OrqaError> {
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

/// Evaluate the artifact graph and return all proposed status transitions.
///
/// The engine reads the graph without modifying it. The caller (frontend or
/// another command) decides whether to apply any proposals.
#[tauri::command]
pub fn evaluate_status_transitions(
    state: State<'_, AppState>,
) -> Result<Vec<ProposedTransition>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let graph = get_or_build_graph(&state)?;
    let statuses = load_status_definitions(&project_path);
    Ok(evaluate_transitions(&graph, &statuses))
}

/// Apply a single proposed status transition by updating the `status` field
/// in the artifact's frontmatter on disk.
///
/// - `artifact_id` — the artifact identifier, e.g. `"EPIC-048"`
/// - `proposed_status` — the target status string to write
///
/// After applying the update the in-memory graph cache is refreshed so
/// subsequent calls reflect the new state.
#[tauri::command]
pub fn apply_status_transition(
    artifact_id: String,
    proposed_status: String,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if artifact_id.trim().is_empty() {
        return Err(OrqaError::Validation(
            "artifact_id cannot be empty".to_owned(),
        ));
    }
    if proposed_status.trim().is_empty() {
        return Err(OrqaError::Validation(
            "proposed_status cannot be empty".to_owned(),
        ));
    }

    let graph = get_or_build_graph(&state)?;

    let node = graph
        .nodes
        .get(&artifact_id)
        .ok_or_else(|| OrqaError::NotFound(format!("artifact not found: {artifact_id}")))?;

    if node.path.contains("..") {
        return Err(OrqaError::Validation(
            "artifact path contains path traversal".to_owned(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(node.path.replace('\\', "/"));

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact file not found on disk: {}",
            full_path.display()
        )));
    }

    domain_update_artifact_field(&full_path, "status", &proposed_status)?;

    // Refresh the graph cache so subsequent queries see the updated status.
    let new_graph = build_artifact_graph(Path::new(&project_path))?;
    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(new_graph);

    Ok(())
}
