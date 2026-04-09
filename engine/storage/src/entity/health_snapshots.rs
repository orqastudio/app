// SeaORM entity for the `health_snapshots` table.
//
// Health snapshots are point-in-time captures of artifact graph metrics used
// to drive the governance dashboard trend sparklines. The outlier-based pipeline
// health model is reflected in the outlier_count / outlier_percentage columns.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `health_snapshots` table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "health_snapshots")]
pub struct Model {
    /// Database row ID (INTEGER PRIMARY KEY).
    #[sea_orm(primary_key)]
    pub id: i64,
    /// FK to `projects(id)` — ON DELETE CASCADE.
    pub project_id: i64,
    /// Total number of artifact nodes in the graph.
    pub node_count: i64,
    /// Total number of directed relationship edges.
    pub edge_count: i64,
    /// Number of relationship targets that do not resolve to known nodes.
    pub broken_ref_count: i64,
    /// Number of integrity check findings with severity Error.
    pub error_count: i64,
    /// Number of integrity check findings with severity Warning.
    pub warning_count: i64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Average number of edges per node (in + out combined).
    pub avg_degree: f64,
    /// Percentage of non-doc nodes tracing to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Number of active pipeline outliers past their grace period.
    pub outlier_count: i64,
    /// Percentage of outliers relative to total active nodes (0.0–100.0).
    pub outlier_percentage: f64,
    /// Fraction of delivery artifacts in the main delivery component (0.0–1.0).
    pub delivery_connectivity: f64,
    /// Fraction of learning artifacts connected to decisions (0.0–1.0).
    pub learning_connectivity: f64,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
}

/// Relations from `health_snapshots` to other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Each snapshot belongs to exactly one project.
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
