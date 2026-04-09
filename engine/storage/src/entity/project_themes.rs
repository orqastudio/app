// SeaORM entity for the `project_themes` table.
//
// Project themes store design token maps extracted from source files such as
// tailwind.config.ts. Each row represents one extraction pass from a single
// source file. Light-mode tokens are required; dark-mode and unmapped tokens
// are optional JSON blobs.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `project_themes` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project_themes")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `projects(id)` — ON DELETE CASCADE.
    pub project_id: i64,
    /// Path to the source file this theme was extracted from.
    pub source_file: String,
    /// Hash of the source file at extraction time, for change detection.
    pub source_hash: String,
    /// ISO-8601 timestamp when the theme was extracted.
    pub extracted_at: String,
    /// JSON-encoded light-mode design token map.
    pub tokens_light: String,
    /// JSON-encoded dark-mode design token map, if present.
    pub tokens_dark: Option<String>,
    /// JSON-encoded list of unmapped token names, if any.
    pub unmapped: Option<String>,
    /// Whether this is the currently active theme for the project (0/1).
    pub is_active: i32,
}

/// Relations from `project_themes` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each theme belongs to exactly one project.
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
