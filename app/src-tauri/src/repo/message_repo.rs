use rusqlite::{params, Connection};

use crate::domain::message::{ContentType, Message, MessageRole, SearchResult, StreamStatus};
use crate::error::OrqaError;

/// Parameters for creating a tool-related message.
pub struct NewToolMessage<'a> {
    pub session_id: i64,
    pub role: &'a str,
    pub content_type: &'a str,
    pub content: Option<&'a str>,
    pub tool_call_id: &'a str,
    pub tool_name: &'a str,
    pub tool_input: Option<&'a str>,
    pub tool_is_error: bool,
    pub turn_index: i32,
    pub block_index: i32,
}

/// Create a standard (non-tool) message and return the full `Message` record.
pub fn create(
    conn: &Connection,
    session_id: i64,
    role: &str,
    content_type: &str,
    content: Option<&str>,
    turn_index: i32,
    block_index: i32,
) -> Result<Message, OrqaError> {
    conn.execute(
        "INSERT INTO messages (session_id, role, content_type, content, turn_index, block_index) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            session_id,
            role,
            content_type,
            content,
            turn_index,
            block_index
        ],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)
}

/// Create a tool-related message (tool_use or tool_result).
pub fn create_tool_message(
    conn: &Connection,
    msg: &NewToolMessage<'_>,
) -> Result<Message, OrqaError> {
    conn.execute(
        "INSERT INTO messages \
         (session_id, role, content_type, content, tool_call_id, tool_name, \
          tool_input, tool_is_error, turn_index, block_index) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            msg.session_id,
            msg.role,
            msg.content_type,
            msg.content,
            msg.tool_call_id,
            msg.tool_name,
            msg.tool_input,
            i32::from(msg.tool_is_error),
            msg.turn_index,
            msg.block_index,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)
}

/// List messages for a session, ordered by turn and block index.
pub fn list(
    conn: &Connection,
    session_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Message>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                tool_input, tool_is_error, turn_index, block_index, stream_status, \
                input_tokens, output_tokens, created_at \
         FROM messages \
         WHERE session_id = ?1 \
         ORDER BY turn_index ASC, block_index ASC \
         LIMIT ?2 OFFSET ?3",
    )?;

    let rows = stmt.query_map(params![session_id, limit, offset], map_message)?;

    let mut messages = Vec::new();
    for row in rows {
        messages.push(row?);
    }
    Ok(messages)
}

/// Search messages across a project using FTS5.
pub fn search(
    conn: &Connection,
    project_id: i64,
    query: &str,
    limit: i64,
) -> Result<Vec<SearchResult>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.session_id, s.title, \
                snippet(messages_fts, 0, '<mark>', '</mark>', '...', 32) AS highlighted, \
                m.content, \
                rank \
         FROM messages_fts \
         JOIN messages m ON m.id = messages_fts.rowid \
         JOIN sessions s ON s.id = m.session_id \
         WHERE s.project_id = ?1 AND messages_fts MATCH ?2 \
         ORDER BY rank \
         LIMIT ?3",
    )?;

    let rows = stmt.query_map(params![project_id, query, limit], |row| {
        Ok(SearchResult {
            message_id: row.get(0)?,
            session_id: row.get(1)?,
            session_title: row.get(2)?,
            highlighted: row.get(3)?,
            content: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            rank: row.get(5)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

/// Get the next turn index for a session (max turn_index + 1, or 0 if no messages).
pub fn next_turn_index(conn: &Connection, session_id: i64) -> Result<i32, OrqaError> {
    let max: Option<i32> = conn.query_row(
        "SELECT MAX(turn_index) FROM messages WHERE session_id = ?1",
        params![session_id],
        |row| row.get(0),
    )?;

    Ok(max.map_or(0, |m| m + 1))
}

/// Update the content of a message (used during streaming accumulation).
pub fn update_content(conn: &Connection, id: i64, content: &str) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE messages SET content = ?1 WHERE id = ?2",
        params![content, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("message {id}")));
    }
    Ok(())
}

/// Update the stream status of a message.
pub fn update_stream_status(conn: &Connection, id: i64, status: &str) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE messages SET stream_status = ?1 WHERE id = ?2",
        params![status, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("message {id}")));
    }
    Ok(())
}

fn get(conn: &Connection, id: i64) -> Result<Message, OrqaError> {
    conn.query_row(
        "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                tool_input, tool_is_error, turn_index, block_index, stream_status, \
                input_tokens, output_tokens, created_at \
         FROM messages WHERE id = ?1",
        params![id],
        map_message,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("message {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}

fn parse_role(s: &str) -> MessageRole {
    match s {
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        _ => MessageRole::System,
    }
}

fn parse_content_type(s: &str) -> ContentType {
    match s {
        "tool_use" => ContentType::ToolUse,
        "tool_result" => ContentType::ToolResult,
        "thinking" => ContentType::Thinking,
        "image" => ContentType::Image,
        _ => ContentType::Text,
    }
}

fn parse_stream_status(s: &str) -> StreamStatus {
    match s {
        "pending" => StreamStatus::Pending,
        "complete" => StreamStatus::Complete,
        _ => StreamStatus::Error,
    }
}

fn map_message(row: &rusqlite::Row<'_>) -> rusqlite::Result<Message> {
    let role_str: String = row.get(2)?;
    let ct_str: String = row.get(3)?;
    let ss_str: String = row.get(11)?;
    let tool_is_error: i32 = row.get(8)?;

    Ok(Message {
        id: row.get(0)?,
        session_id: row.get(1)?,
        role: parse_role(&role_str),
        content_type: parse_content_type(&ct_str),
        content: row.get(4)?,
        tool_call_id: row.get(5)?,
        tool_name: row.get(6)?,
        tool_input: row.get(7)?,
        tool_is_error: tool_is_error != 0,
        turn_index: row.get(9)?,
        block_index: row.get(10)?,
        stream_status: parse_stream_status(&ss_str),
        input_tokens: row.get(12)?,
        output_tokens: row.get(13)?,
        created_at: row.get(14)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::project_repo;
    use crate::repo::session_repo;

    fn setup() -> Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        session_repo::create(&conn, 1, "auto", None).expect("create session");
        conn
    }

    #[test]
    fn create_and_list_messages() {
        let conn = setup();
        create(&conn, 1, "user", "text", Some("Hello"), 0, 0).expect("create msg1");
        create(&conn, 1, "assistant", "text", Some("Hi there"), 1, 0).expect("create msg2");

        let messages = list(&conn, 1, 100, 0).expect("list");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, MessageRole::User);
        assert_eq!(messages[0].content.as_deref(), Some("Hello"));
        assert_eq!(messages[1].role, MessageRole::Assistant);
    }

    #[test]
    fn create_tool_message_works() {
        let conn = setup();
        let msg = create_tool_message(
            &conn,
            &NewToolMessage {
                session_id: 1,
                role: "assistant",
                content_type: "tool_use",
                content: None,
                tool_call_id: "call_123",
                tool_name: "read_file",
                tool_input: Some(r#"{"path": "/src"}"#),
                tool_is_error: false,
                turn_index: 0,
                block_index: 1,
            },
        )
        .expect("create tool message");

        assert_eq!(msg.content_type, ContentType::ToolUse);
        assert_eq!(msg.tool_call_id.as_deref(), Some("call_123"));
        assert_eq!(msg.tool_name.as_deref(), Some("read_file"));
        assert!(!msg.tool_is_error);
    }

    #[test]
    fn list_with_pagination() {
        let conn = setup();
        for i in 0..5 {
            create(&conn, 1, "user", "text", Some("msg"), i, 0).expect("create");
        }

        let page1 = list(&conn, 1, 2, 0).expect("page 1");
        assert_eq!(page1.len(), 2);

        let page3 = list(&conn, 1, 2, 4).expect("page 3");
        assert_eq!(page3.len(), 1);
    }

    #[test]
    fn fts5_search_works() {
        let conn = setup();
        create(
            &conn,
            1,
            "user",
            "text",
            Some("How do I fix the parsing bug?"),
            0,
            0,
        )
        .expect("create");
        create(
            &conn,
            1,
            "assistant",
            "text",
            Some("You need to update the parser"),
            1,
            0,
        )
        .expect("create");
        create(
            &conn,
            1,
            "user",
            "text",
            Some("What about the database migration?"),
            2,
            0,
        )
        .expect("create");

        let results = search(&conn, 1, "parsing", 10).expect("search");
        assert!(
            !results.is_empty(),
            "should find messages matching 'parsing'"
        );

        let results = search(&conn, 1, "database migration", 10).expect("search");
        assert!(
            !results.is_empty(),
            "should find messages matching 'database migration'"
        );
    }

    #[test]
    fn update_content_works() {
        let conn = setup();
        let msg = create(&conn, 1, "assistant", "text", Some("partial"), 0, 0).expect("create");

        update_content(&conn, msg.id, "complete response").expect("update");

        let fetched = get(&conn, msg.id).expect("get");
        assert_eq!(fetched.content.as_deref(), Some("complete response"));
    }

    #[test]
    fn update_stream_status_works() {
        let conn = setup();
        let msg = create(&conn, 1, "assistant", "text", Some("streaming"), 0, 0).expect("create");

        // Default is complete; set to pending
        update_stream_status(&conn, msg.id, "pending").expect("set pending");
        let fetched = get(&conn, msg.id).expect("get");
        assert_eq!(fetched.stream_status, StreamStatus::Pending);

        update_stream_status(&conn, msg.id, "complete").expect("set complete");
        let fetched = get(&conn, msg.id).expect("get");
        assert_eq!(fetched.stream_status, StreamStatus::Complete);
    }

    #[test]
    fn update_content_not_found() {
        let conn = setup();
        let result = update_content(&conn, 999, "text");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn cascade_delete_triggers_fts_cleanup() {
        let conn = setup();
        create(&conn, 1, "user", "text", Some("searchable content"), 0, 0).expect("create");

        // Delete the session (which cascades to messages)
        session_repo::delete(&conn, 1).expect("delete session");

        // FTS should no longer find the deleted message
        let results = search(&conn, 1, "searchable", 10).expect("search");
        assert!(results.is_empty(), "FTS should not find deleted messages");
    }

    #[test]
    fn message_fk_constraint() {
        let conn = init_memory_db().expect("db init");
        let result = create(&conn, 999, "user", "text", Some("hello"), 0, 0);
        assert!(result.is_err(), "should fail with FK violation");
    }

    #[test]
    fn next_turn_index_empty_session() {
        let conn = setup();
        let idx = next_turn_index(&conn, 1).expect("next turn index");
        assert_eq!(idx, 0, "empty session should start at turn 0");
    }

    #[test]
    fn next_turn_index_after_messages() {
        let conn = setup();
        create(&conn, 1, "user", "text", Some("msg1"), 0, 0).expect("create");
        create(&conn, 1, "assistant", "text", Some("msg2"), 1, 0).expect("create");

        let idx = next_turn_index(&conn, 1).expect("next turn index");
        assert_eq!(idx, 2, "should be max(1) + 1 = 2");
    }

    #[test]
    fn next_turn_index_with_gaps() {
        let conn = setup();
        create(&conn, 1, "user", "text", Some("msg1"), 0, 0).expect("create");
        create(&conn, 1, "assistant", "text", Some("msg2"), 5, 0).expect("create");

        let idx = next_turn_index(&conn, 1).expect("next turn index");
        assert_eq!(idx, 6, "should be max(5) + 1 = 6");
    }
}
