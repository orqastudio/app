// SeaORM entity for the `devtools_events` table.
//
// Devtools events are copies of log events captured during a devtools session
// window. The `original_id` is the log_events.id from the daemon event bus;
// `rowid` is the autoincrement PK in this table. The fingerprint column was
// added in migration 4 for deduplication and is nullable.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `devtools_events` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "devtools_events")]
pub struct Model {
    /// Autoincrement row ID (INTEGER PRIMARY KEY AUTOINCREMENT).
    #[sea_orm(primary_key, column_name = "rowid")]
    pub rowid: i64,
    /// Original log_events.id from the daemon event bus.
    pub original_id: i64,
    /// FK to `devtools_sessions(id)` — ON DELETE CASCADE.
    pub session_id: String,
    /// Unix millisecond timestamp.
    pub timestamp: i64,
    /// Severity level string (e.g. "Info", "Error").
    pub level: String,
    /// Source subsystem string.
    pub source: String,
    /// Event tier string (e.g. "Runtime", "System").
    pub tier: String,
    /// Event category string for grouping.
    pub category: String,
    /// Human-readable event message.
    pub message: String,
    /// JSON-encoded metadata object.
    pub metadata: String,
    /// Original daemon event ID for cross-reference.
    pub daemon_event_id: Option<i64>,
    /// Optional fingerprint for deduplication (added in migration 4).
    pub fingerprint: Option<String>,
}

/// Relations from `devtools_events` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each event belongs to exactly one devtools session.
    #[sea_orm(
        belongs_to = "super::devtools_sessions::Entity",
        from = "Column::SessionId",
        to = "super::devtools_sessions::Column::Id",
        on_delete = "Cascade"
    )]
    DevtoolsSession,
}

impl Related<super::devtools_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DevtoolsSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
