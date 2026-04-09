// SeaORM entity for the `enforcement_violations` table.
//
// Violations are recorded whenever the enforcement engine blocks or warns on a
// tool call. Each row captures the rule that fired, the action taken (block or
// warn), the tool name, and optional detail text.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `enforcement_violations` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "enforcement_violations")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY AUTOINCREMENT).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `projects(id)` — no cascade (violations are retained for audit).
    pub project_id: i64,
    /// Name of the enforcement rule that fired.
    pub rule_name: String,
    /// Action taken: "block" or "warn".
    pub action: String,
    /// Name of the tool that triggered the violation.
    pub tool_name: String,
    /// Optional detail text explaining the violation.
    pub detail: Option<String>,
    /// ISO-8601 creation timestamp (uses `datetime('now')`, no fractional seconds).
    pub created_at: String,
}

/// Relations from `enforcement_violations` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each violation references a project (no cascade — audit retention).
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id"
    )]
    Project,
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
