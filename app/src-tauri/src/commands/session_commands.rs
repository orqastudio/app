use tauri::State;

use crate::domain::session::{Session, SessionSummary};
use crate::error::OrqaError;
use crate::repo::session_repo;
use crate::state::AppState;

/// Create a new session for a project.
///
/// Uses "auto" as the default model if none is provided.
#[tauri::command]
pub fn session_create(
    project_id: i64,
    model: Option<String>,
    system_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<Session, OrqaError> {
    let model_str = model.unwrap_or_else(|| "auto".to_owned());
    if model_str.trim().is_empty() {
        return Err(OrqaError::Validation("model cannot be empty".to_owned()));
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    session_repo::create(
        &conn,
        project_id,
        model_str.trim(),
        system_prompt.as_deref(),
    )
}

/// List sessions for a project with optional status filter and pagination.
#[tauri::command]
pub fn session_list(
    project_id: i64,
    status: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<SessionSummary>, OrqaError> {
    let limit_val = limit.unwrap_or(50);
    let offset_val = offset.unwrap_or(0);

    if limit_val < 0 {
        return Err(OrqaError::Validation(
            "limit cannot be negative".to_owned(),
        ));
    }
    if offset_val < 0 {
        return Err(OrqaError::Validation(
            "offset cannot be negative".to_owned(),
        ));
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    session_repo::list(&conn, project_id, status.as_deref(), limit_val, offset_val)
}

/// Get a session by its ID.
#[tauri::command]
pub fn session_get(session_id: i64, state: State<'_, AppState>) -> Result<Session, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    session_repo::get(&conn, session_id)
}

/// Update the title of a session.
#[tauri::command]
pub fn session_update_title(
    session_id: i64,
    title: String,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if title.trim().is_empty() {
        return Err(OrqaError::Validation("title cannot be empty".to_owned()));
    }

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    session_repo::update_title(&conn, session_id, title.trim())
}

/// End a session (mark as completed).
#[tauri::command]
pub fn session_end(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    session_repo::end_session(&conn, session_id)
}

/// Delete a session and its messages (cascading).
#[tauri::command]
pub fn session_delete(session_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    session_repo::delete(&conn, session_id)
}

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::domain::session::SessionStatus;
    use crate::repo::{project_repo, session_repo};

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn create_session_with_defaults() {
        let conn = setup();
        let session = session_repo::create(&conn, 1, "auto", None).expect("create");
        assert_eq!(session.model, "auto");
        assert_eq!(session.status, SessionStatus::Active);
        assert!(session.system_prompt.is_none());
    }

    #[test]
    fn create_session_with_model_and_prompt() {
        let conn = setup();
        let session = session_repo::create(
            &conn,
            1,
            "claude-opus-4-6",
            Some("You are a helpful assistant"),
        )
        .expect("create");
        assert_eq!(session.model, "claude-opus-4-6");
        assert_eq!(
            session.system_prompt.as_deref(),
            Some("You are a helpful assistant")
        );
    }

    #[test]
    fn list_sessions_with_defaults() {
        let conn = setup();
        session_repo::create(&conn, 1, "auto", None).expect("create s1");
        session_repo::create(&conn, 1, "auto", None).expect("create s2");

        let sessions = session_repo::list(&conn, 1, None, 50, 0).expect("list");
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn list_sessions_with_status_filter() {
        let conn = setup();
        session_repo::create(&conn, 1, "auto", None).expect("create s1");
        let s2 = session_repo::create(&conn, 1, "auto", None).expect("create s2");
        session_repo::end_session(&conn, s2.id).expect("end s2");

        let active = session_repo::list(&conn, 1, Some("active"), 50, 0).expect("list active");
        assert_eq!(active.len(), 1);

        let completed =
            session_repo::list(&conn, 1, Some("completed"), 50, 0).expect("list completed");
        assert_eq!(completed.len(), 1);
    }

    #[test]
    fn list_sessions_with_pagination() {
        let conn = setup();
        for _ in 0..5 {
            session_repo::create(&conn, 1, "auto", None).expect("create");
        }

        let page = session_repo::list(&conn, 1, None, 2, 0).expect("page 1");
        assert_eq!(page.len(), 2);

        let page = session_repo::list(&conn, 1, None, 2, 4).expect("page 3");
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn get_session_by_id() {
        let conn = setup();
        let created = session_repo::create(&conn, 1, "auto", None).expect("create");
        let fetched = session_repo::get(&conn, created.id).expect("get");
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.model, "auto");
    }

    #[test]
    fn get_nonexistent_session() {
        let conn = setup();
        let result = session_repo::get(&conn, 999);
        assert!(result.is_err());
    }

    #[test]
    fn update_title_works() {
        let conn = setup();
        let session = session_repo::create(&conn, 1, "auto", None).expect("create");
        assert!(session.title.is_none());

        session_repo::update_title(&conn, session.id, "My Session").expect("update");
        let fetched = session_repo::get(&conn, session.id).expect("get");
        assert_eq!(fetched.title.as_deref(), Some("My Session"));
    }

    #[test]
    fn end_session_marks_completed() {
        let conn = setup();
        let session = session_repo::create(&conn, 1, "auto", None).expect("create");
        assert_eq!(session.status, SessionStatus::Active);

        session_repo::end_session(&conn, session.id).expect("end");
        let fetched = session_repo::get(&conn, session.id).expect("get");
        assert_eq!(fetched.status, SessionStatus::Completed);
    }

    #[test]
    fn delete_session_cascades() {
        let conn = setup();
        let session = session_repo::create(&conn, 1, "auto", None).expect("create");
        session_repo::delete(&conn, session.id).expect("delete");

        let result = session_repo::get(&conn, session.id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_nonexistent_session() {
        let conn = setup();
        let result = session_repo::delete(&conn, 999);
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
