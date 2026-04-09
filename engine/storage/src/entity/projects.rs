// SeaORM entity for the `projects` table.
//
// Projects are the top-level container for sessions and governance artifacts.
// Each project maps to a filesystem root and holds configuration, detected stack,
// and timestamps.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `projects` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// Display name of the project.
    pub name: String,
    /// Absolute filesystem path to the project root (UNIQUE).
    #[sea_orm(unique)]
    pub path: String,
    /// Optional description of the project.
    pub description: Option<String>,
    /// JSON-encoded `DetectedStack`, or `None` if not yet scanned.
    pub detected_stack: Option<String>,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

/// Relation enum for foreign keys originating from `projects`.
///
/// Projects are referenced by sessions, themes, overrides, violations,
/// health snapshots, and enforcement violations. Those entities own the FK
/// columns, so `projects` has no outbound relations — only inbound.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
