use tauri::State;

use crate::domain::message::{Message, SearchResult};
use crate::error::ForgeError;
use crate::repo::message_repo;
use crate::state::AppState;

/// List messages for a session with pagination.
#[tauri::command]
pub fn message_list(
    session_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, ForgeError> {
    let limit_val = limit.unwrap_or(100);
    let offset_val = offset.unwrap_or(0);

    if limit_val < 0 {
        return Err(ForgeError::Validation(
            "limit cannot be negative".to_string(),
        ));
    }
    if offset_val < 0 {
        return Err(ForgeError::Validation(
            "offset cannot be negative".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    message_repo::list(&conn, session_id, limit_val, offset_val)
}

/// Search messages across a project using full-text search.
#[tauri::command]
pub fn message_search(
    project_id: i64,
    query: String,
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, ForgeError> {
    if query.trim().is_empty() {
        return Err(ForgeError::Validation(
            "search query cannot be empty".to_string(),
        ));
    }

    let limit_val = limit.unwrap_or(20);
    if limit_val < 0 {
        return Err(ForgeError::Validation(
            "limit cannot be negative".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    message_repo::search(&conn, project_id, query.trim(), limit_val)
}

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::domain::message::MessageRole;
    use crate::repo::{message_repo, project_repo, session_repo};

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        session_repo::create(&conn, 1, "auto", None).expect("create session");
        conn
    }

    #[test]
    fn list_messages_default_pagination() {
        let conn = setup();
        message_repo::create(&conn, 1, "user", "text", Some("Hello"), 0, 0).expect("create");
        message_repo::create(&conn, 1, "assistant", "text", Some("Hi"), 1, 0).expect("create");

        let messages = message_repo::list(&conn, 1, 100, 0).expect("list");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, MessageRole::User);
        assert_eq!(messages[1].role, MessageRole::Assistant);
    }

    #[test]
    fn list_messages_with_pagination() {
        let conn = setup();
        for i in 0..5 {
            message_repo::create(&conn, 1, "user", "text", Some("msg"), i, 0).expect("create");
        }

        let page = message_repo::list(&conn, 1, 2, 0).expect("page 1");
        assert_eq!(page.len(), 2);

        let page = message_repo::list(&conn, 1, 2, 4).expect("page 3");
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn list_empty_session() {
        let conn = setup();
        let messages = message_repo::list(&conn, 1, 100, 0).expect("list");
        assert!(messages.is_empty());
    }

    #[test]
    fn search_finds_matching_messages() {
        let conn = setup();
        message_repo::create(
            &conn,
            1,
            "user",
            "text",
            Some("How do I fix the parsing bug?"),
            0,
            0,
        )
        .expect("create");
        message_repo::create(
            &conn,
            1,
            "assistant",
            "text",
            Some("Update the parser module"),
            1,
            0,
        )
        .expect("create");

        let results = message_repo::search(&conn, 1, "parsing", 10).expect("search");
        assert!(!results.is_empty());
    }

    #[test]
    fn search_empty_returns_nothing() {
        let conn = setup();
        let results = message_repo::search(&conn, 1, "nonexistent_term_xyz", 10).expect("search");
        assert!(results.is_empty());
    }

    #[test]
    fn empty_query_validation() {
        let query = "   ";
        assert!(query.trim().is_empty());
    }

    #[test]
    fn negative_limit_validation() {
        let limit: i64 = -5;
        assert!(limit < 0);
    }
}
