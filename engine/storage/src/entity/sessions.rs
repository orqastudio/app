// SeaORM entity for the `sessions` table.
//
// Sessions are the primary unit of user–agent interaction within a project.
// Each session holds token usage counters, a status enum, and an optional
// provider session ID for context continuity across restarts.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Status of a session — mirrors the CHECK constraint on the `status` column.
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum SessionStatus {
    /// Session is currently active.
    #[sea_orm(string_value = "active")]
    Active,
    /// Session ended normally.
    #[sea_orm(string_value = "completed")]
    Completed,
    /// Session was abandoned by the user.
    #[sea_orm(string_value = "abandoned")]
    Abandoned,
    /// Session terminated due to an error.
    #[sea_orm(string_value = "error")]
    Error,
}

/// SeaORM entity model for a row in the `sessions` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `projects(id)` — ON DELETE CASCADE.
    pub project_id: i64,
    /// Optional user identity string.
    pub user_id: Option<String>,
    /// Optional display title; auto-generated if not manually set.
    pub title: Option<String>,
    /// Model identifier (e.g. "claude-opus-4-6").
    pub model: String,
    /// Optional system prompt for this session.
    pub system_prompt: Option<String>,
    /// Session lifecycle status.
    pub status: SessionStatus,
    /// Optional summary generated at session end.
    pub summary: Option<String>,
    /// Optional handoff notes written at session end.
    pub handoff_notes: Option<String>,
    /// Cumulative input token count.
    pub total_input_tokens: i64,
    /// Cumulative output token count.
    pub total_output_tokens: i64,
    /// Cumulative cost in USD.
    pub total_cost_usd: f64,
    /// Provider-assigned session ID for context continuity.
    pub provider_session_id: Option<String>,
    /// Whether the title was manually set (1) or auto-generated (0).
    pub title_manually_set: i32,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

/// Relations from `sessions` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each session belongs to exactly one project.
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
    /// A session has many messages.
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
