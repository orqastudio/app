// Session domain types for the OrqaStudio engine.
//
// Defines structs and enums representing agent sessions — the primary unit of
// interaction between a user and an LLM sidecar. Sessions contain messages,
// accumulate token counts, and transition through a defined lifecycle.

use serde::{Deserialize, Serialize};

/// A session between a user and an LLM sidecar, persisted to the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub project_id: i64,
    pub title: Option<String>,
    pub model: String,
    pub system_prompt: Option<String>,
    pub status: SessionStatus,
    pub summary: Option<String>,
    pub handoff_notes: Option<String>,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cost_usd: f64,
    pub provider_session_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    /// Whether the user explicitly set this title, preventing auto-naming from overwriting it.
    pub title_manually_set: bool,
}

/// A lightweight summary of a session for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: i64,
    pub title: Option<String>,
    pub status: SessionStatus,
    pub message_count: i64,
    pub preview: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// The lifecycle status of a session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Completed,
    Abandoned,
    Error,
}
