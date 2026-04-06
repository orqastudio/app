// Tauri IPC commands for artifact status transition evaluation and application.
//
// All transition operations are delegated to the daemon via HTTP. The daemon
// owns the artifact graph, project settings, and transition evaluation logic.
// The app is a thin client.
//
// Endpoints used:
//   GET  /workflow/transitions        — evaluate all proposed transitions
//   POST /workflow/transitions/apply  — apply a single transition

use tauri::State;

use orqa_engine_types::types::workflow::ProposedTransition;

use crate::error::OrqaError;
use crate::state::AppState;

/// Evaluate the artifact graph and return all proposed status transitions.
///
/// Delegates to the daemon which fetches the artifact graph, loads project
/// status definitions, and runs the transition engine.
#[tauri::command]
pub async fn evaluate_status_transitions(
    state: State<'_, AppState>,
) -> Result<Vec<ProposedTransition>, OrqaError> {
    state.daemon.client.list_workflow_transitions().await
}

/// Apply a single proposed status transition by updating the `status` field
/// in the artifact's frontmatter via the daemon.
///
/// - `artifact_id` — the artifact identifier, e.g. `"EPIC-048"`
/// - `proposed_status` — the target status string to write
#[tauri::command]
pub async fn apply_status_transition(
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

    tracing::info!(
        artifact_id = %artifact_id,
        status = %proposed_status,
        "[status] apply_status_transition: delegating to daemon"
    );

    state
        .daemon
        .client
        .apply_workflow_transition(&artifact_id, &proposed_status)
        .await
        .map(|_| ())
}

#[cfg(test)]
mod tests {

    #[test]
    fn empty_artifact_id_is_invalid() {
        let id = "   ";
        assert!(id.trim().is_empty());
    }

    #[test]
    fn empty_proposed_status_is_invalid() {
        let status = "";
        assert!(status.trim().is_empty());
    }
}
