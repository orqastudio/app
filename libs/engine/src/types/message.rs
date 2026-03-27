// Message domain types for the OrqaStudio engine.
//
// Defines structs and enums for chat messages within a session, including roles,
// content types, stream status, and full-text search results over message history.
// These types flow from the database through the Tauri IPC boundary to the frontend.

use serde::{Deserialize, Serialize};

/// A single message block within a session, persisted to the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub session_id: i64,
    pub role: MessageRole,
    pub content_type: ContentType,
    pub content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<String>,
    pub tool_is_error: bool,
    pub turn_index: i32,
    pub block_index: i32,
    pub stream_status: StreamStatus,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub created_at: String,
}

/// The role of the message author in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// The content type of a message block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    ToolUse,
    ToolResult,
    Thinking,
    Image,
}

/// The streaming state of a message block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StreamStatus {
    Pending,
    Complete,
    Error,
}

/// Alias for the message primary key type, used in cross-module references.
pub type MessageId = i64;

/// A full-text search result over message history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub message_id: i64,
    pub session_id: i64,
    pub session_title: Option<String>,
    pub content: String,
    pub highlighted: String,
    pub rank: f64,
}
