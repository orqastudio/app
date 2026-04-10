// HTTP client for `/messages` endpoints.
//
// Mirrors MessageRepository from engine/storage/src/traits.rs.
// The NewToolMessage struct is redefined here (it cannot be imported from
// engine/storage since libs/db does not depend on that crate).

use orqa_engine_types::types::message::{Message, MessageRole, SearchResult, StreamStatus};

use crate::{parse_empty_response, parse_response, DbError};

/// Parameters for creating a tool-related message, mirroring
/// `engine/storage/src/repo/messages.rs::NewToolMessage`.
///
/// Owned strings are used here because the HTTP boundary requires serialization.
pub struct NewToolMessage {
    /// The session this message belongs to.
    pub session_id: i64,
    /// Message role: "user", "assistant", or "system".
    pub role: String,
    /// Content type discriminator (e.g., "tool_use", "tool_result").
    pub content_type: String,
    /// Optional text content of the message.
    pub content: Option<String>,
    /// Tool call identifier linking the request to its result.
    pub tool_call_id: String,
    /// Name of the tool being called or responded to.
    pub tool_name: String,
    /// JSON-encoded tool input arguments.
    pub tool_input: Option<String>,
    /// Whether the tool result represents an error.
    pub tool_is_error: bool,
    /// Turn index within the session.
    pub turn_index: i32,
    /// Block index within the turn.
    pub block_index: i32,
}

/// Sub-client for `/messages` daemon endpoints.
///
/// Obtained via `DbClient::messages()`.
pub struct MessagesClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl MessagesClient<'_> {
    /// Create a standard (non-tool) message and return the full row.
    ///
    /// Calls POST /messages.
    pub async fn create(
        &self,
        session_id: i64,
        role: MessageRole,
        content: Option<&str>,
        turn_index: i32,
        block_index: i32,
    ) -> Result<Message, DbError> {
        let body = serde_json::json!({
            "session_id": session_id,
            "role": message_role_str(role),
            "content": content,
            "turn_index": turn_index,
            "block_index": block_index,
        });
        let resp = self
            .http
            .post(format!("{}/messages", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Create a tool-related message (tool_use or tool_result).
    ///
    /// Calls POST /messages/tool.
    pub async fn create_tool_message(&self, msg: &NewToolMessage) -> Result<Message, DbError> {
        let body = serde_json::json!({
            "session_id": msg.session_id,
            "role": msg.role,
            "content_type": msg.content_type,
            "content": msg.content,
            "tool_call_id": msg.tool_call_id,
            "tool_name": msg.tool_name,
            "tool_input": msg.tool_input,
            "tool_is_error": msg.tool_is_error,
            "turn_index": msg.turn_index,
            "block_index": msg.block_index,
        });
        let resp = self
            .http
            .post(format!("{}/messages/tool", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_response(resp).await
    }

    /// List messages for a session ordered by turn and block index.
    ///
    /// Calls GET /sessions/:id/messages?limit=...&offset=...
    pub async fn list(
        &self,
        session_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Message>, DbError> {
        let resp = self
            .http
            .get(format!("{}/sessions/{session_id}/messages", self.base_url))
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Search messages across a project using FTS5 full-text search.
    ///
    /// Calls POST /messages/search.
    pub async fn search(
        &self,
        project_id: i64,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchResult>, DbError> {
        let body = serde_json::json!({
            "project_id": project_id,
            "query": query,
            "limit": limit,
        });
        let resp = self
            .http
            .post(format!("{}/messages/search", self.base_url))
            .json(&body)
            .send()
            .await?;
        let val: serde_json::Value = parse_response(resp).await?;
        serde_json::from_value(val["results"].clone())
            .map_err(|e| DbError::Deserialization(e.to_string()))
    }

    /// Return the next turn index for a session.
    ///
    /// Calls GET /sessions/:id/next-turn-index (shared with sessions client).
    pub async fn next_turn_index(&self, session_id: i64) -> Result<i32, DbError> {
        let resp = self
            .http
            .get(format!(
                "{}/sessions/{session_id}/next-turn-index",
                self.base_url
            ))
            .send()
            .await?;
        let val: serde_json::Value = parse_response(resp).await?;
        val["next_turn_index"]
            .as_i64()
            .map(|n| n as i32)
            .ok_or_else(|| DbError::Deserialization("missing next_turn_index".to_owned()))
    }

    /// Update the content of a message (streaming accumulation).
    ///
    /// Calls PUT /messages/:id/content.
    pub async fn update_content(&self, id: i64, content: &str) -> Result<(), DbError> {
        let body = serde_json::json!({ "content": content });
        let resp = self
            .http
            .put(format!("{}/messages/{id}/content", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Update the stream status of a message.
    ///
    /// Calls PUT /messages/:id/stream-status.
    pub async fn update_stream_status(&self, id: i64, status: StreamStatus) -> Result<(), DbError> {
        let body = serde_json::json!({ "status": stream_status_str(status) });
        let resp = self
            .http
            .put(format!("{}/messages/{id}/stream-status", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }
}

/// Serialize a `MessageRole` to its wire string representation.
fn message_role_str(role: MessageRole) -> &'static str {
    match role {
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::System => "system",
    }
}

/// Serialize a `StreamStatus` to its wire string representation.
fn stream_status_str(status: StreamStatus) -> &'static str {
    match status {
        StreamStatus::Pending => "pending",
        StreamStatus::Complete => "complete",
        StreamStatus::Error => "error",
    }
}
