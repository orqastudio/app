//! Session domain types for the OrqaStudio engine.
//!
//! Defines structs and enums representing agent sessions — the primary unit of
//! interaction between a user and an LLM sidecar. Sessions contain messages,
//! accumulate token counts, and transition through a defined lifecycle.
//!
//! # ID representation
//!
//! `id` and `project_id` are raw `i64` SQLite rowids. Full newtype wrappers
//! (`SessionId(i64)`, `ProjectId(i64)`) would require changes across
//! `orqa-storage`, `orqa-engine-types`, and both Tauri backends simultaneously.
//! The current representation is kept as `i64` to preserve a single migration
//! boundary. The storage layer is the correct place to introduce typed IDs when
//! that refactor is scoped.

use serde::{Deserialize, Serialize};

/// A session between a user and an LLM sidecar, persisted to the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Database row ID.
    pub id: i64,
    /// ID of the project this session belongs to.
    pub project_id: i64,
    /// Display title for the session (auto-generated or manually set).
    pub title: Option<String>,
    /// LLM model identifier used for this session.
    pub model: String,
    /// Governance system prompt injected at the start of each turn.
    pub system_prompt: Option<String>,
    /// Current lifecycle state of the session.
    pub status: SessionStatus,
    /// Optional auto-generated summary of the conversation.
    pub summary: Option<String>,
    /// Notes left by the agent for handoff to the next session.
    pub handoff_notes: Option<String>,
    /// Cumulative input tokens consumed across all turns.
    pub total_input_tokens: i64,
    /// Cumulative output tokens generated across all turns.
    pub total_output_tokens: i64,
    /// Estimated total cost in USD based on model pricing.
    pub total_cost_usd: f64,
    /// Provider-assigned session identifier, if any.
    pub provider_session_id: Option<String>,
    /// ISO-8601 timestamp when this session was created.
    pub created_at: String,
    /// ISO-8601 timestamp of the last update to this session.
    pub updated_at: String,
    /// Whether the user explicitly set this title, preventing auto-naming from overwriting it.
    pub title_manually_set: bool,
}

/// A lightweight summary of a session for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    /// Database row ID.
    pub id: i64,
    /// Display title for the session.
    pub title: Option<String>,
    /// Current lifecycle state.
    pub status: SessionStatus,
    /// Number of message blocks in this session.
    pub message_count: i64,
    /// Short content preview (first few words of the last user message).
    pub preview: Option<String>,
    /// ISO-8601 timestamp when this session was created.
    pub created_at: String,
    /// ISO-8601 timestamp of the last update.
    pub updated_at: String,
}

/// The lifecycle status of a session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Session is in progress and accepting new messages.
    Active,
    /// Session has ended normally.
    Completed,
    /// Session was abandoned without a clean completion.
    Abandoned,
    /// Session terminated due to an error.
    Error,
}
