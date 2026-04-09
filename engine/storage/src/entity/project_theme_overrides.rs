// SeaORM entity for the `project_theme_overrides` table.
//
// Per-project token value overrides let users pin specific design tokens to
// custom values independent of the extracted theme. Both light and dark mode
// values are supported; dark is optional.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `project_theme_overrides` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project_theme_overrides")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `projects(id)` — ON DELETE CASCADE.
    pub project_id: i64,
    /// Design token name being overridden (e.g. "primary").
    pub token_name: String,
    /// Override value for light mode.
    pub value_light: String,
    /// Override value for dark mode, if set.
    pub value_dark: Option<String>,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

/// Relations from `project_theme_overrides` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each override belongs to exactly one project.
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_delete = "Cascade"
    )]
    Project,
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
