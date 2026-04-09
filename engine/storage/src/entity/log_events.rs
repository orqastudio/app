// SeaORM entity for the `log_events` table.
//
// Log events are structured daemon event bus records persisted for replay and
// audit. Previously stored in a separate events.db; now unified in orqa.db.
// The metadata column is a JSON string; the repository layer parses it on read.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `log_events` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "log_events")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Unix millisecond timestamp.
    pub timestamp: i64,
    /// Severity level string (e.g. "Info", "Error").
    pub level: String,
    /// Source subsystem string (e.g. "daemon", "mcp").
    pub source: String,
    /// Event category string for grouping.
    pub category: String,
    /// Human-readable event message.
    pub message: String,
    /// JSON-encoded metadata object.
    pub metadata: String,
    /// Optional session ID string linking this event to a session context.
    pub session_id: Option<String>,
    /// Optional fingerprint for deduplication (added in migration 4).
    pub fingerprint: Option<String>,
}

/// Relations from `log_events` to other tables.
///
/// Log events have no foreign-key relations — they are append-only audit rows.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
