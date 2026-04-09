// SeaORM entity for the `messages` table.
//
// Messages are content blocks within a session turn. Each block has a role,
// content type, and stream status. Tool-related blocks carry tool_call_id,
// tool_name, and tool_input. FTS5 search is handled separately via a virtual
// table — not modelled here.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Role of the agent that produced a message — mirrors the CHECK on `role`.
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum MessageRole {
    /// Message originated from the human user.
    #[sea_orm(string_value = "user")]
    User,
    /// Message originated from the LLM assistant.
    #[sea_orm(string_value = "assistant")]
    Assistant,
    /// Injected system context, not a turn in the conversation.
    #[sea_orm(string_value = "system")]
    System,
}

/// Discriminator for the content block type — mirrors the CHECK on `content_type`.
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum ContentType {
    /// Plain text content.
    #[sea_orm(string_value = "text")]
    Text,
    /// Tool invocation request from the assistant.
    #[sea_orm(string_value = "tool_use")]
    ToolUse,
    /// Tool invocation result from the user/executor.
    #[sea_orm(string_value = "tool_result")]
    ToolResult,
    /// Internal reasoning block (extended thinking).
    #[sea_orm(string_value = "thinking")]
    Thinking,
    /// Binary image content (base64 or URL).
    #[sea_orm(string_value = "image")]
    Image,
}

/// Streaming state for a message block — mirrors the CHECK on `stream_status`.
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum StreamStatus {
    /// Block is still accumulating streamed tokens.
    #[sea_orm(string_value = "pending")]
    Pending,
    /// Block is fully received.
    #[sea_orm(string_value = "complete")]
    Complete,
    /// Block terminated due to a streaming error.
    #[sea_orm(string_value = "error")]
    Error,
}

/// SeaORM entity model for a row in the `messages` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `sessions(id)` — ON DELETE CASCADE.
    pub session_id: i64,
    /// Which agent role produced this block.
    pub role: MessageRole,
    /// Content block discriminator.
    pub content_type: ContentType,
    /// Raw text content; `None` for non-text blocks.
    pub content: Option<String>,
    /// Tool call correlation ID linking request to result.
    pub tool_call_id: Option<String>,
    /// Name of the tool being called or responded to.
    pub tool_name: Option<String>,
    /// JSON-encoded tool input arguments.
    pub tool_input: Option<String>,
    /// Whether the tool result is an error (stored as 0/1).
    pub tool_is_error: i32,
    /// Turn index within the session (increments per user message).
    pub turn_index: i32,
    /// Block index within the turn (increments per content block).
    pub block_index: i32,
    /// Current streaming state for this block.
    pub stream_status: StreamStatus,
    /// Input tokens consumed for this block, if reported.
    pub input_tokens: Option<i32>,
    /// Output tokens produced for this block, if reported.
    pub output_tokens: Option<i32>,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
}

/// Relations from `messages` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each message belongs to exactly one session.
    #[sea_orm(
        belongs_to = "super::sessions::Entity",
        from = "Column::SessionId",
        to = "super::sessions::Column::Id",
        on_delete = "Cascade"
    )]
    Session,
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
