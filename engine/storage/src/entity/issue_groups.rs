// SeaORM entity for the `issue_groups` table.
//
// Issue groups represent deduplicated clusters of events sharing the same
// fingerprint (derived from source + level + message_template + stack_top).
// The sparkline_buckets and recent_event_ids columns are JSON arrays stored
// as TEXT; the repository layer handles serialization and ring-buffer rotation.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `issue_groups` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "issue_groups")]
pub struct Model {
    /// Stable hash derived from source + level + message_template + stack_top (TEXT PRIMARY KEY).
    #[sea_orm(primary_key, auto_increment = false)]
    pub fingerprint: String,
    /// Human-readable title derived from the message template.
    pub title: String,
    /// Source component that produced the events (e.g. "daemon", "mcp").
    pub component: String,
    /// Severity level string (e.g. "Error", "Warn").
    pub level: String,
    /// Unix millisecond timestamp of the first event with this fingerprint.
    pub first_seen: i64,
    /// Unix millisecond timestamp of the most recent event.
    pub last_seen: i64,
    /// Total number of events matching this fingerprint.
    pub count: i64,
    /// JSON-encoded 24-element i64 array of hourly occurrence counters.
    pub sparkline_buckets: String,
    /// JSON-encoded array of the most recent 50 event IDs (u64).
    pub recent_event_ids: String,
}

/// Relations from `issue_groups` to other tables.
///
/// Issue groups are a global deduplicated index and have no foreign-key relations.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
