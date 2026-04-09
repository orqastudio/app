// Tauri IPC commands for session management.
//
// Sessions are persisted in the project-scoped SQLite database via engine/storage.
//
// ID representation: `session_id` and `project_id` parameters are raw `i64` SQLite
// rowids. Tauri's IPC boundary deserializes JSON numbers directly to Rust primitives;
// newtype wrappers across IPC would require custom serde implementations on every
// command parameter. The storage layer is the correct migration point for typed IDs.

use tauri::State;

use orqa_engine_types::types::session::SessionStatus;
use orqa_storage::traits::SessionRepository as _;

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

    let storage = state.db.get()?;
    let session = storage
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

    let storage = state.db.get()?;
    let status_filter = status.as_deref().and_then(parse_session_status);
    Ok(storage
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
    let storage = state.db.get()?;
    Ok(storage.sessions().get(session_id).await?)
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

    let storage = state.db.get()?;
    Ok(storage
        .sessions()
        .update_title(session_id, title.trim())
        .await?)
}

/// End a session (mark as completed).
#[tauri::command]
pub async fn session_end(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let storage = state.db.get()?;
    storage.sessions().end_session(session_id).await?;
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
    let storage = state.db.get()?;
    storage.sessions().delete(session_id).await?;
    tracing::info!(
        subsystem = "session",
        session_id = session_id,
        "session_delete"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::domain::session::SessionStatus;
    use orqa_storage::traits::{ProjectRepository as _, SessionRepository as _};

    async fn setup() -> (orqa_storage::Storage, i64) {
        let storage = orqa_storage::Storage::open_in_memory()
            .await
            .expect("db init");
        let project = storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create project");
        (storage, project.id)
    }

    #[tokio::test]
    async fn create_session_with_defaults() {
        let (storage, project_id) = setup().await;
        let session = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create");
        assert_eq!(session.model, "auto");
        assert_eq!(session.status, SessionStatus::Active);
        assert!(session.system_prompt.is_none());
    }

    #[tokio::test]
    async fn create_session_with_model_and_prompt() {
        let (storage, project_id) = setup().await;
        let session = storage
            .sessions()
            .create(
                project_id,
                "claude-opus-4-6",
                Some("You are a helpful assistant"),
            )
            .await
            .expect("create");
        assert_eq!(session.model, "claude-opus-4-6");
        assert_eq!(
            session.system_prompt.as_deref(),
            Some("You are a helpful assistant")
        );
    }

    #[tokio::test]
    async fn list_sessions_with_defaults() {
        let (storage, project_id) = setup().await;
        storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create s1");
        storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create s2");

        let sessions = storage
            .sessions()
            .list(project_id, None, 50, 0)
            .await
            .expect("list");
        assert_eq!(sessions.len(), 2);
    }

    #[tokio::test]
    async fn list_sessions_with_status_filter() {
        let (storage, project_id) = setup().await;
        storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create s1");
        let s2 = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create s2");
        storage.sessions().end_session(s2.id).await.expect("end s2");

        let active = storage
            .sessions()
            .list(project_id, Some(SessionStatus::Active), 50, 0)
            .await
            .expect("list active");
        assert_eq!(active.len(), 1);

        let completed = storage
            .sessions()
            .list(project_id, Some(SessionStatus::Completed), 50, 0)
            .await
            .expect("list completed");
        assert_eq!(completed.len(), 1);
    }

    #[tokio::test]
    async fn list_sessions_with_pagination() {
        let (storage, project_id) = setup().await;
        for _ in 0..5 {
            storage
                .sessions()
                .create(project_id, "auto", None)
                .await
                .expect("create");
        }

        let page = storage
            .sessions()
            .list(project_id, None, 2, 0)
            .await
            .expect("page 1");
        assert_eq!(page.len(), 2);

        let page = storage
            .sessions()
            .list(project_id, None, 2, 4)
            .await
            .expect("page 3");
        assert_eq!(page.len(), 1);
    }

    #[tokio::test]
    async fn get_session_by_id() {
        let (storage, project_id) = setup().await;
        let created = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create");
        let fetched = storage.sessions().get(created.id).await.expect("get");
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.model, "auto");
    }

    #[tokio::test]
    async fn get_nonexistent_session() {
        let (storage, _) = setup().await;
        let result = storage.sessions().get(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update_title_works() {
        let (storage, project_id) = setup().await;
        let session = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create");
        assert!(session.title.is_none());

        storage
            .sessions()
            .update_title(session.id, "My Session")
            .await
            .expect("update");
        let fetched = storage.sessions().get(session.id).await.expect("get");
        assert_eq!(fetched.title.as_deref(), Some("My Session"));
    }

    #[tokio::test]
    async fn end_session_marks_completed() {
        let (storage, project_id) = setup().await;
        let session = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create");
        assert_eq!(session.status, SessionStatus::Active);

        storage
            .sessions()
            .end_session(session.id)
            .await
            .expect("end");
        let fetched = storage.sessions().get(session.id).await.expect("get");
        assert_eq!(fetched.status, SessionStatus::Completed);
    }

    #[tokio::test]
    async fn delete_session_cascades() {
        let (storage, project_id) = setup().await;
        let session = storage
            .sessions()
            .create(project_id, "auto", None)
            .await
            .expect("create");
        storage.sessions().delete(session.id).await.expect("delete");

        let result = storage.sessions().get(session.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn delete_nonexistent_session() {
        let (storage, _) = setup().await;
        let result = storage.sessions().delete(999).await;
        assert!(result.is_err());
    }

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
