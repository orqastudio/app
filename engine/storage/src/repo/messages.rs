// Messages repository for orqa-storage.
//
// Provides CRUD and search operations over the `messages` table. Messages are
// content blocks within a session turn (text, tool_use, tool_result, thinking,
// image). FTS5 search is provided via the `messages_fts` virtual table. All
// SQL is ported directly from app/src-tauri/src/repo/message_repo.rs.

use rusqlite::params;

use orqa_engine_types::types::message::{
    ContentType, Message, MessageRole, SearchResult, StreamStatus,
};

use crate::Storage;
use crate::error::StorageError;

/// Parameters for creating a tool-related message (tool_use or tool_result).
pub struct NewToolMessage<'a> {
    /// The session this message belongs to.
    pub session_id: i64,
    /// Message role: "user", "assistant", or "system".
    pub role: &'a str,
    /// Content type discriminator (e.g., "tool_use", "tool_result").
    pub content_type: &'a str,
    /// Optional text content of the message.
    pub content: Option<&'a str>,
    /// Tool call identifier linking the request to its result.
    pub tool_call_id: &'a str,
    /// Name of the tool being called or responded to.
    pub tool_name: &'a str,
    /// JSON-encoded tool input arguments.
    pub tool_input: Option<&'a str>,
    /// Whether the tool result represents an error.
    pub tool_is_error: bool,
    /// Turn index within the session (increments per user message).
    pub turn_index: i32,
    /// Block index within the turn (increments per content block).
    pub block_index: i32,
}

/// Zero-cost repository handle for the `messages` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::messages()`.
pub struct MessageRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl MessageRepo<'_> {
    /// Create a standard (non-tool) message and return the full row.
    pub fn create(
        &self,
        session_id: i64,
        role: &str,
        content_type: &str,
        content: Option<&str>,
        turn_index: i32,
        block_index: i32,
    ) -> Result<Message, StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO messages \
             (session_id, role, content_type, content, turn_index, block_index) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![session_id, role, content_type, content, turn_index, block_index],
        )?;
        let id = conn.last_insert_rowid();
        get_conn(&conn, id)
    }

    /// Create a tool-related message (tool_use or tool_result).
    pub fn create_tool_message(
        &self,
        msg: &NewToolMessage<'_>,
    ) -> Result<Message, StorageError> {
        let conn = self.storage.conn()?;
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
        get_conn(&conn, id)
    }

    /// List messages for a session, ordered by turn and block index.
    pub fn list(
        &self,
        session_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                    tool_input, tool_is_error, turn_index, block_index, stream_status, \
                    input_tokens, output_tokens, created_at \
             FROM messages \
             WHERE session_id = ?1 \
             ORDER BY turn_index ASC, block_index ASC \
             LIMIT ?2 OFFSET ?3",
        )?;

        let result = stmt
            .query_map(params![session_id, limit, offset], map_message)?
            .map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect();
        result
    }

    /// Search messages across a project using FTS5 full-text search.
    pub fn search(
        &self,
        project_id: i64,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchResult>, StorageError> {
        let conn = self.storage.conn()?;
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

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Get the next turn index for a session (max existing + 1, or 0 if no messages).
    pub fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError> {
        let conn = self.storage.conn()?;
        let max: Option<i32> = conn.query_row(
            "SELECT MAX(turn_index) FROM messages WHERE session_id = ?1",
            params![session_id],
            |row| row.get(0),
        )?;
        Ok(max.map_or(0, |m| m + 1))
    }

    /// Update the content of a message (used during streaming accumulation).
    pub fn update_content(&self, id: i64, content: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE messages SET content = ?1 WHERE id = ?2",
            params![content, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("message {id}")));
        }
        Ok(())
    }

    /// Update the stream status of a message.
    pub fn update_stream_status(&self, id: i64, status: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE messages SET stream_status = ?1 WHERE id = ?2",
            params![status, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("message {id}")));
        }
        Ok(())
    }
}

/// Fetch a message by id from an existing open connection.
fn get_conn(conn: &rusqlite::Connection, id: i64) -> Result<Message, StorageError> {
    conn.query_row(
        "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                tool_input, tool_is_error, turn_index, block_index, stream_status, \
                input_tokens, output_tokens, created_at \
         FROM messages WHERE id = ?1",
        params![id],
        map_message,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => StorageError::NotFound(format!("message {id}")),
        other => StorageError::Database(other.to_string()),
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
    use crate::Storage;

    fn setup() -> Storage {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .expect("create project");
        storage
            .sessions()
            .create(1, "auto", None)
            .expect("create session");
        storage
    }

    #[test]
    fn create_and_list_messages() {
        let storage = setup();
        storage
            .messages()
            .create(1, "user", "text", Some("Hello"), 0, 0)
            .expect("create msg1");
        storage
            .messages()
            .create(1, "assistant", "text", Some("Hi there"), 1, 0)
            .expect("create msg2");

        let msgs = storage.messages().list(1, 100, 0).expect("list");
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, MessageRole::User);
        assert_eq!(msgs[1].role, MessageRole::Assistant);
    }

    #[test]
    fn next_turn_index_empty_session() {
        let storage = setup();
        let idx = storage.messages().next_turn_index(1).expect("next_turn");
        assert_eq!(idx, 0);
    }

    #[test]
    fn next_turn_index_after_messages() {
        let storage = setup();
        storage
            .messages()
            .create(1, "user", "text", Some("m1"), 0, 0)
            .expect("m1");
        storage
            .messages()
            .create(1, "assistant", "text", Some("m2"), 1, 0)
            .expect("m2");
        let idx = storage.messages().next_turn_index(1).expect("next_turn");
        assert_eq!(idx, 2);
    }

    #[test]
    fn update_content_works() {
        let storage = setup();
        let msg = storage
            .messages()
            .create(1, "assistant", "text", Some("partial"), 0, 0)
            .expect("create");
        storage
            .messages()
            .update_content(msg.id, "complete")
            .expect("update");
        // Verify by listing
        let msgs = storage.messages().list(1, 10, 0).expect("list");
        assert_eq!(msgs[0].content.as_deref(), Some("complete"));
    }

    #[test]
    fn fts_search_finds_messages() {
        let storage = setup();
        storage
            .messages()
            .create(1, "user", "text", Some("How do I fix the parsing bug?"), 0, 0)
            .expect("create");
        storage
            .messages()
            .create(1, "assistant", "text", Some("You need to update the parser"), 1, 0)
            .expect("create");

        let results = storage.messages().search(1, "parsing", 10).expect("search");
        assert!(!results.is_empty(), "FTS should find messages matching 'parsing'");
    }
}
