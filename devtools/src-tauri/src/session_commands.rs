//! Tauri IPC command wrappers for devtools session management via libs/db.
//!
//! All commands receive `DbClient` and `ActiveSession` from Tauri's managed state.

use std::sync::Arc;

use orqa_db::devtools::{
    DevtoolsEventQuery, DevtoolsEventQueryResponse, DevtoolsSessionInfo, DevtoolsSessionSummary,
};
use orqa_db::DbClient;
use tauri::State;

use crate::ActiveSession;

/// IPC command — list all devtools sessions ordered by start time descending.
///
/// Returns session summaries including the current session (flagged with
/// `is_current: true`) and all historical sessions. Used to populate the
/// session picker dropdown in the frontend.
#[tauri::command]
pub async fn list_sessions(
    db: State<'_, DbClient>,
    active: State<'_, Arc<ActiveSession>>,
) -> Result<Vec<DevtoolsSessionSummary>, String> {
    db.devtools()
        .list_sessions(&active.0)
        .await
        .map_err(|e| e.to_string())
}

/// IPC command — query events for a specific session with optional filters and paging.
///
/// Supports filtering by `source`, `level`, `category`, and `search_text`
/// (substring match on message). Paginates via `offset`/`limit`. Returns both
/// the matching event page and the total count for the frontend to render a
/// pagination control.
#[tauri::command]
pub async fn query_session_events(
    db: State<'_, DbClient>,
    params: DevtoolsEventQuery,
) -> Result<DevtoolsEventQueryResponse, String> {
    db.devtools()
        .query_events(&params)
        .await
        .map_err(|e| e.to_string())
}

/// IPC command — return metadata for the currently active session.
///
/// Used by the frontend status bar to display the current session label and
/// event count without fetching all sessions.
#[tauri::command]
pub async fn get_current_session(
    db: State<'_, DbClient>,
    active: State<'_, Arc<ActiveSession>>,
) -> Result<DevtoolsSessionInfo, String> {
    db.devtools()
        .get_session(&active.0)
        .await
        .map_err(|e| e.to_string())
}

/// IPC command — set or update the user-editable label for a session.
///
/// The `session_id` may be any session (current or historical). An empty
/// `label` string effectively clears the label, causing the frontend to
/// fall back to the auto-generated "Session <date> <time>" display name.
#[tauri::command]
pub async fn rename_session(
    db: State<'_, DbClient>,
    session_id: String,
    label: String,
) -> Result<(), String> {
    db.devtools()
        .rename_session(&session_id, &label)
        .await
        .map_err(|e| e.to_string())
}

/// IPC command — delete a session and all its events (CASCADE).
///
/// The current session cannot be meaningfully deleted while the app is open,
/// but the guard is left to the frontend. Deleting a non-existent `session_id`
/// is silently ignored.
#[tauri::command]
pub async fn delete_session(db: State<'_, DbClient>, session_id: String) -> Result<(), String> {
    db.devtools()
        .delete_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}
