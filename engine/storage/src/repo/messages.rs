// Messages repository for orqa-storage.
//
// Provides async CRUD and search operations over the `messages` table. Messages
// are content blocks within a session turn (text, tool_use, tool_result,
// thinking, image). FTS5 search is provided via the `messages_fts` virtual
// table and must be expressed as raw SQL — SeaORM cannot build FTS5 queries.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use orqa_engine_types::types::message::{
    ContentType, Message, MessageRole, SearchResult, StreamStatus,
};

use crate::error::StorageError;
use crate::traits::MessageRepository;

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

/// Async repository handle for the `messages` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::messages()`.
pub struct MessageRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to a `Message` domain value.
///
/// Column positions must match the SELECT order used in every message query.
#[allow(clippy::too_many_lines)]
fn map_message(row: &sea_orm::QueryResult) -> Result<Message, StorageError> {
    let role_str: String = row
        .try_get("", "role")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let ct_str: String = row
        .try_get("", "content_type")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let ss_str: String = row
        .try_get("", "stream_status")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let tool_is_error: i64 = row
        .try_get("", "tool_is_error")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(Message {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        session_id: row
            .try_get("", "session_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        role: parse_role(&role_str),
        content_type: parse_content_type(&ct_str),
        content: row
            .try_get("", "content")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tool_call_id: row
            .try_get("", "tool_call_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tool_name: row
            .try_get("", "tool_name")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tool_input: row
            .try_get("", "tool_input")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tool_is_error: tool_is_error != 0,
        turn_index: row
            .try_get("", "turn_index")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        block_index: row
            .try_get("", "block_index")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        stream_status: parse_stream_status(&ss_str),
        input_tokens: row
            .try_get("", "input_tokens")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        output_tokens: row
            .try_get("", "output_tokens")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

/// Parse a role string into the `MessageRole` enum.
fn parse_role(s: &str) -> MessageRole {
    match s {
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        _ => MessageRole::System,
    }
}

/// Serialize a `MessageRole` to its SQL string representation.
fn role_to_str(role: MessageRole) -> &'static str {
    match role {
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::System => "system",
    }
}

/// Parse a content_type string into the `ContentType` enum.
fn parse_content_type(s: &str) -> ContentType {
    match s {
        "tool_use" => ContentType::ToolUse,
        "tool_result" => ContentType::ToolResult,
        "thinking" => ContentType::Thinking,
        "image" => ContentType::Image,
        _ => ContentType::Text,
    }
}

/// Parse a stream_status string into the `StreamStatus` enum.
fn parse_stream_status(s: &str) -> StreamStatus {
    match s {
        "pending" => StreamStatus::Pending,
        "complete" => StreamStatus::Complete,
        _ => StreamStatus::Error,
    }
}

/// Serialize a `StreamStatus` to its SQL string representation.
fn stream_status_to_str(status: StreamStatus) -> &'static str {
    match status {
        StreamStatus::Pending => "pending",
        StreamStatus::Complete => "complete",
        StreamStatus::Error => "error",
    }
}

#[async_trait::async_trait]
impl MessageRepository for MessageRepo {
    /// Create a standard (non-tool) message and return the full row.
    async fn create(
        &self,
        session_id: i64,
        role: MessageRole,
        content: Option<&str>,
        turn_index: i32,
        block_index: i32,
    ) -> Result<Message, StorageError> {
        let role_str = role_to_str(role);
        let content_val: sea_orm::Value = match content {
            Some(c) => c.into(),
            None => sea_orm::Value::String(None),
        };
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO messages \
                 (session_id, role, content_type, content, turn_index, block_index) \
                 VALUES (?, ?, 'text', ?, ?, ?)",
                [
                    session_id.into(),
                    role_str.into(),
                    content_val,
                    turn_index.into(),
                    block_index.into(),
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                        tool_input, tool_is_error, turn_index, block_index, stream_status, \
                        input_tokens, output_tokens, created_at \
                 FROM messages ORDER BY id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| {
                StorageError::NotFound("messages table is empty after insert".to_owned())
            })?;
        map_message(&row)
    }

    /// Create a tool-related message (tool_use or tool_result).
    async fn create_tool_message(&self, msg: &NewToolMessage<'_>) -> Result<Message, StorageError> {
        let content_val: sea_orm::Value = match msg.content {
            Some(c) => c.into(),
            None => sea_orm::Value::String(None),
        };
        let tool_input_val: sea_orm::Value = match msg.tool_input {
            Some(t) => t.into(),
            None => sea_orm::Value::String(None),
        };
        let tool_is_error: i32 = i32::from(msg.tool_is_error);
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO messages \
                 (session_id, role, content_type, content, tool_call_id, tool_name, \
                  tool_input, tool_is_error, turn_index, block_index) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    msg.session_id.into(),
                    msg.role.into(),
                    msg.content_type.into(),
                    content_val,
                    msg.tool_call_id.into(),
                    msg.tool_name.into(),
                    tool_input_val,
                    tool_is_error.into(),
                    msg.turn_index.into(),
                    msg.block_index.into(),
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                        tool_input, tool_is_error, turn_index, block_index, stream_status, \
                        input_tokens, output_tokens, created_at \
                 FROM messages ORDER BY id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| {
                StorageError::NotFound("messages table is empty after insert".to_owned())
            })?;
        map_message(&row)
    }

    /// List messages for a session ordered by turn and block index.
    async fn list(
        &self,
        session_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, session_id, role, content_type, content, tool_call_id, tool_name, \
                        tool_input, tool_is_error, turn_index, block_index, stream_status, \
                        input_tokens, output_tokens, created_at \
                 FROM messages \
                 WHERE session_id = ? \
                 ORDER BY turn_index ASC, block_index ASC \
                 LIMIT ? OFFSET ?",
                [session_id.into(), limit.into(), offset.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_message).collect()
    }

    /// Search messages across a project using FTS5 full-text search.
    ///
    /// FTS5 queries cannot be expressed via SeaORM's query builder and are
    /// passed as raw SQL. The `MATCH` operator and `snippet()` function are
    /// SQLite FTS5 extensions.
    async fn search(
        &self,
        project_id: i64,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchResult>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT m.id, m.session_id, s.title, \
                        snippet(messages_fts, 0, '<mark>', '</mark>', '...', 32) AS highlighted, \
                        m.content, \
                        rank \
                 FROM messages_fts \
                 JOIN messages m ON m.id = messages_fts.rowid \
                 JOIN sessions s ON s.id = m.session_id \
                 WHERE s.project_id = ? AND messages_fts MATCH ? \
                 ORDER BY rank \
                 LIMIT ?",
                [project_id.into(), query.into(), limit.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter()
            .map(|row| {
                let content: Option<String> = row
                    .try_get("", "content")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                Ok(SearchResult {
                    message_id: row
                        .try_get("", "id")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    session_id: row
                        .try_get("", "session_id")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    session_title: row
                        .try_get("", "title")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    highlighted: row
                        .try_get("", "highlighted")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    content: content.unwrap_or_default(),
                    rank: row
                        .try_get("", "rank")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                })
            })
            .collect()
    }

    /// Return the next turn index for a session.
    async fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError> {
        let row = self.db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT COALESCE(MAX(turn_index), -1) AS max_turn FROM messages WHERE session_id = ?",
                [session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| StorageError::Database("next_turn_index query returned no row".to_owned()))?;
        let max: i64 = row
            .try_get("", "max_turn")
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok((max + 1) as i32)
    }

    /// Update the content of a message (streaming accumulation).
    async fn update_content(&self, id: i64, content: &str) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE messages SET content = ? WHERE id = ?",
                [content.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("message {id}")));
        }
        Ok(())
    }

    /// Update the stream status of a message.
    async fn update_stream_status(
        &self,
        id: i64,
        status: StreamStatus,
    ) -> Result<(), StorageError> {
        let status_str = stream_status_to_str(status);
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE messages SET stream_status = ? WHERE id = ?",
                [status_str.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("message {id}")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{MessageRepository, ProjectRepository, SessionRepository};
    use crate::Storage;

    async fn setup() -> Storage {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create project");
        storage
            .sessions()
            .create(1, "auto", None)
            .await
            .expect("create session");
        storage
    }

    #[tokio::test]
    async fn create_and_list_messages() {
        let storage = setup().await;
        storage
            .messages()
            .create(1, MessageRole::User, Some("Hello"), 0, 0)
            .await
            .expect("create msg1");
        storage
            .messages()
            .create(1, MessageRole::Assistant, Some("Hi there"), 1, 0)
            .await
            .expect("create msg2");

        let msgs = storage.messages().list(1, 100, 0).await.expect("list");
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, MessageRole::User);
        assert_eq!(msgs[1].role, MessageRole::Assistant);
    }

    #[tokio::test]
    async fn next_turn_index_empty_session() {
        let storage = setup().await;
        let idx = storage
            .messages()
            .next_turn_index(1)
            .await
            .expect("next_turn");
        assert_eq!(idx, 0);
    }

    #[tokio::test]
    async fn next_turn_index_after_messages() {
        let storage = setup().await;
        storage
            .messages()
            .create(1, MessageRole::User, Some("m1"), 0, 0)
            .await
            .expect("m1");
        storage
            .messages()
            .create(1, MessageRole::Assistant, Some("m2"), 1, 0)
            .await
            .expect("m2");
        let idx = storage
            .messages()
            .next_turn_index(1)
            .await
            .expect("next_turn");
        assert_eq!(idx, 2);
    }

    #[tokio::test]
    async fn update_content_works() {
        let storage = setup().await;
        let msg = storage
            .messages()
            .create(1, MessageRole::Assistant, Some("partial"), 0, 0)
            .await
            .expect("create");
        storage
            .messages()
            .update_content(msg.id, "complete")
            .await
            .expect("update");
        let msgs = storage.messages().list(1, 10, 0).await.expect("list");
        assert_eq!(msgs[0].content.as_deref(), Some("complete"));
    }

    #[tokio::test]
    async fn fts_search_finds_messages() {
        let storage = setup().await;
        storage
            .messages()
            .create(
                1,
                MessageRole::User,
                Some("How do I fix the parsing bug?"),
                0,
                0,
            )
            .await
            .expect("create");
        storage
            .messages()
            .create(
                1,
                MessageRole::Assistant,
                Some("You need to update the parser"),
                1,
                0,
            )
            .await
            .expect("create");

        let results = storage
            .messages()
            .search(1, "parsing", 10)
            .await
            .expect("search");
        assert!(
            !results.is_empty(),
            "FTS should find messages matching 'parsing'"
        );
    }
}
