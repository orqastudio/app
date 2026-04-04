//! Tauri IPC command wrappers for devtools session management via orqa-storage.
//!
//! Each command is a thin async wrapper that calls the corresponding
//! `DevtoolsRepo` method via `tokio::task::spawn_blocking` so the `!Send`
//! rusqlite connection never touches the async runtime directly.
//!
//! All commands receive `Storage` and `ActiveSession` from Tauri's managed state.

use std::sync::Arc;

use orqa_storage::repo::devtools::{
    DevtoolsEventQuery, DevtoolsEventQueryResponse, DevtoolsSessionInfo, DevtoolsSessionSummary,
};
use orqa_storage::Storage;
use tauri::State;

use crate::ActiveSession;

/// IPC command — list all devtools sessions ordered by start time descending.
///
/// Returns session summaries including the current session (flagged with
/// `is_current: true`) and all historical sessions. Used to populate the
/// session picker dropdown in the frontend.
#[tauri::command]
pub async fn list_sessions(
    storage: State<'_, Arc<Storage>>,
    active: State<'_, Arc<ActiveSession>>,
) -> Result<Vec<DevtoolsSessionSummary>, String> {
    let storage = Arc::clone(&storage);
    let session_id = active.0.clone();
    tokio::task::spawn_blocking(move || {
        storage
            .devtools()
            .list_sessions(&session_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("list_sessions panicked: {e}"))?
}

/// IPC command — query events for a specific session with optional filters and paging.
///
/// Supports filtering by `source`, `level`, `category`, and `search_text`
/// (substring match on message). Paginates via `offset`/`limit`. Returns both
/// the matching event page and the total count for the frontend to render a
/// pagination control.
#[tauri::command]
pub async fn query_session_events(
    storage: State<'_, Arc<Storage>>,
    params: DevtoolsEventQuery,
) -> Result<DevtoolsEventQueryResponse, String> {
    let storage = Arc::clone(&storage);
    tokio::task::spawn_blocking(move || {
        storage
            .devtools()
            .query_events(&params)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("query_session_events panicked: {e}"))?
}

/// IPC command — return metadata for the currently active session.
///
/// Used by the frontend status bar to display the current session label and
/// event count without fetching all sessions.
#[tauri::command]
pub async fn get_current_session(
    storage: State<'_, Arc<Storage>>,
    active: State<'_, Arc<ActiveSession>>,
) -> Result<DevtoolsSessionInfo, String> {
    let storage = Arc::clone(&storage);
    let session_id = active.0.clone();
    tokio::task::spawn_blocking(move || {
        storage
            .devtools()
            .get_session(&session_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("get_current_session panicked: {e}"))?
}

/// IPC command — set or update the user-editable label for a session.
///
/// The `session_id` may be any session (current or historical). An empty
/// `label` string effectively clears the label, causing the frontend to
/// fall back to the auto-generated "Session <date> <time>" display name.
#[tauri::command]
pub async fn rename_session(
    storage: State<'_, Arc<Storage>>,
    session_id: String,
    label: String,
) -> Result<(), String> {
    let storage = Arc::clone(&storage);
    tokio::task::spawn_blocking(move || {
        storage
            .devtools()
            .rename_session(&session_id, &label)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("rename_session panicked: {e}"))?
}

/// IPC command — delete a session and all its events (CASCADE).
///
/// The current session cannot be meaningfully deleted while the app is open,
/// but the guard is left to the frontend. Deleting a non-existent `session_id`
/// is silently ignored.
#[tauri::command]
pub async fn delete_session(
    storage: State<'_, Arc<Storage>>,
    session_id: String,
) -> Result<(), String> {
    let storage = Arc::clone(&storage);
    tokio::task::spawn_blocking(move || {
        storage
            .devtools()
            .delete_session(&session_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("delete_session panicked: {e}"))?
}
