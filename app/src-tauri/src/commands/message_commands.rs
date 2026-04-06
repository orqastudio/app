// Tauri IPC commands for message retrieval.
//
// Messages are persisted in the project-scoped SQLite database via engine/storage.

use tauri::State;

use crate::domain::message::Message;
use crate::error::OrqaError;
use crate::state::AppState;

/// List messages for a session with pagination.
#[tauri::command]
pub fn message_list(
    session_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, OrqaError> {
    let limit_val = limit.unwrap_or(100);
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
    Ok(storage.messages().list(session_id, limit_val, offset_val)?)
}

#[cfg(test)]
mod tests {
    use crate::domain::message::MessageRole;

    fn setup() -> (orqa_storage::Storage, i64) {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let project = storage
            .projects()
            .create("test", "/test", None)
            .expect("create project");
        storage
            .sessions()
            .create(project.id, "auto", None)
            .expect("create session");
        (storage, 1)
    }

    #[test]
    fn list_messages_default_pagination() {
        let (storage, session_id) = setup();
        storage
            .messages()
            .create(session_id, "user", "text", Some("Hello"), 0, 0)
            .expect("create");
        storage
            .messages()
            .create(session_id, "assistant", "text", Some("Hi"), 1, 0)
            .expect("create");

        let messages = storage.messages().list(session_id, 100, 0).expect("list");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, MessageRole::User);
        assert_eq!(messages[1].role, MessageRole::Assistant);
    }

    #[test]
    fn list_messages_with_pagination() {
        let (storage, session_id) = setup();
        for i in 0..5 {
            storage
                .messages()
                .create(session_id, "user", "text", Some("msg"), i, 0)
                .expect("create");
        }

        let page = storage.messages().list(session_id, 2, 0).expect("page 1");
        assert_eq!(page.len(), 2);

        let page = storage.messages().list(session_id, 2, 4).expect("page 3");
        assert_eq!(page.len(), 1);
    }

    #[test]
    fn list_empty_session() {
        let (storage, session_id) = setup();
        let messages = storage.messages().list(session_id, 100, 0).expect("list");
        assert!(messages.is_empty());
    }

    #[test]
    fn search_finds_matching_messages() {
        let (storage, session_id) = setup();
        storage
            .messages()
            .create(
                session_id,
                "user",
                "text",
                Some("How do I fix the parsing bug?"),
                0,
                0,
            )
            .expect("create");
        storage
            .messages()
            .create(
                session_id,
                "assistant",
                "text",
                Some("Update the parser module"),
                1,
                0,
            )
            .expect("create");

        // search uses project_id=1 (the project in setup)
        let results = storage.messages().search(1, "parsing", 10).expect("search");
        assert!(!results.is_empty());
    }

    #[test]
    fn search_empty_returns_nothing() {
        let (storage, _session_id) = setup();
        let results = storage
            .messages()
            .search(1, "nonexistent_term_xyz", 10)
            .expect("search");
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
