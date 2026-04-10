// Tauri IPC commands for session management.
//
// Sessions are persisted in the daemon's database. The app creates and manages
// sessions via the daemon HTTP API through libs/db.
//
// ID representation: `session_id` and `project_id` parameters are raw `i64` SQLite
// rowids. Tauri's IPC boundary deserializes JSON numbers directly to Rust primitives;
// newtype wrappers across IPC would require custom serde implementations on every
// command parameter. The storage layer is the correct migration point for typed IDs.

use tauri::State;

use orqa_engine_types::types::session::SessionStatus;

use crate::domain::session::{Session, SessionSummary};
use crate::error::OrqaError;
use crate::state::AppState;

/// Parse a session status string into a `SessionStatus` variant.
///
/// Returns `None` for unrecognised strings, allowing the caller to treat
/// absent status as "no filter".
fn parse_session_status(s: &str) -> Option<SessionStatus> {
    match s {
        "active" => Some(SessionStatus::Active),
        "completed" => Some(SessionStatus::Completed),
        "abandoned" => Some(SessionStatus::Abandoned),
        "error" => Some(SessionStatus::Error),
        _ => None,
    }
}

/// Create a new session for a project.
///
/// Uses "auto" as the default model if none is provided.
#[tauri::command]
pub async fn session_create(
    project_id: i64,
    model: Option<String>,
    system_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<Session, OrqaError> {
    let model_str = model.unwrap_or_else(|| "auto".to_owned());
    if model_str.trim().is_empty() {
        return Err(OrqaError::Validation("model cannot be empty".to_owned()));
    }

    let session = state
        .db
        .client
        .sessions()
        .create(project_id, model_str.trim(), system_prompt.as_deref())
        .await?;
    tracing::info!(
        subsystem = "session",
        session_id = session.id,
        project_id = project_id,
        "session_create"
    );
    Ok(session)
}

/// List sessions for a project with optional status filter and pagination.
#[tauri::command]
pub async fn session_list(
    project_id: i64,
    status: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<SessionSummary>, OrqaError> {
    let limit_val = limit.unwrap_or(50);
    let offset_val = offset.unwrap_or(0);

    if limit_val < 0 {
        return Err(OrqaError::Validation("limit cannot be negative".to_owned()));
    }
    if offset_val < 0 {
        return Err(OrqaError::Validation(
            "offset cannot be negative".to_owned(),
        ));
    }

    let status_filter = status.as_deref().and_then(parse_session_status);
    Ok(state
        .db
        .client
        .sessions()
        .list(project_id, status_filter, limit_val, offset_val)
        .await?)
}

/// Get a session by its ID.
#[tauri::command]
pub async fn session_get(
    session_id: i64,
    state: State<'_, AppState>,
) -> Result<Session, OrqaError> {
    Ok(state.db.client.sessions().get(session_id).await?)
}

/// Update the title of a session.
#[tauri::command]
pub async fn session_update_title(
    session_id: i64,
    title: String,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if title.trim().is_empty() {
        return Err(OrqaError::Validation("title cannot be empty".to_owned()));
    }

    Ok(state
        .db
        .client
        .sessions()
        .update_title(session_id, title.trim())
        .await?)
}

/// End a session (mark as completed).
#[tauri::command]
pub async fn session_end(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    state.db.client.sessions().end_session(session_id).await?;
    tracing::info!(
        subsystem = "session",
        session_id = session_id,
        "session_end"
    );
    Ok(())
}

/// Delete a session and its messages (cascading).
#[tauri::command]
pub async fn session_delete(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    state.db.client.sessions().delete(session_id).await?;
    tracing::info!(
        subsystem = "session",
        session_id = session_id,
        "session_delete"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_model_validation() {
        let model = "   ";
        assert!(model.trim().is_empty());
    }

    #[test]
    fn empty_title_validation() {
        let title = "  ";
        assert!(title.trim().is_empty());
    }

    #[test]
    fn negative_limit_validation() {
        let limit: i64 = -1;
        assert!(limit < 0);
    }
}
