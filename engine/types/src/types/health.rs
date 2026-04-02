//! Health snapshot domain types for the OrqaStudio engine.
//!
//! Defines structs representing point-in-time snapshots of artifact graph health metrics.
//! Snapshots are persisted in the local SQLite database and surfaced in the governance dashboard.
//!
//! The model tracks the outlier-based pipeline health model:
//! - Outliers: artifacts outside both delivery and learning pipelines, past their grace period.
//! - Delivery connectivity: fraction of delivery artifacts connected to the main pipeline.
//! - Learning connectivity: fraction of learning artifacts (lessons, rules) connected to decisions.

use serde::{Deserialize, Serialize};

/// A point-in-time snapshot of graph health metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshot {
    /// Database row ID.
    pub id: i64,
    /// ID of the project this snapshot belongs to.
    pub project_id: i64,
    /// Total number of artifact nodes in the graph.
    pub node_count: i64,
    /// Total number of directed relationship edges.
    pub edge_count: i64,
    /// Number of relationship targets that do not exist in the graph.
    pub broken_ref_count: i64,
    /// Number of integrity check findings with severity Error.
    pub error_count: i64,
    /// Number of integrity check findings with severity Warning.
    pub warning_count: i64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Average number of edges per node (in + out combined).
    pub avg_degree: f64,
    /// Percentage of non-doc nodes that trace to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Number of active pipeline outliers past their type-specific grace period.
    pub outlier_count: i64,
    /// Percentage of outliers relative to total active nodes (0.0–100.0).
    pub outlier_percentage: f64,
    /// Fraction of delivery artifacts connected to the main delivery component (0.0–1.0).
    pub delivery_connectivity: f64,
    /// Fraction of learning artifacts connected to each other or to decisions (0.0–1.0).
    pub learning_connectivity: f64,
    /// ISO-8601 timestamp when this snapshot was created.
    pub created_at: String,
}

/// Input for creating a new health snapshot (no id or timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHealthSnapshot {
    /// Total number of artifact nodes in the graph.
    pub node_count: i64,
    /// Total number of directed relationship edges.
    pub edge_count: i64,
    /// Number of relationship targets that do not exist in the graph.
    pub broken_ref_count: i64,
    /// Number of integrity check findings with severity Error.
    pub error_count: i64,
    /// Number of integrity check findings with severity Warning.
    pub warning_count: i64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Average number of edges per node (in + out combined).
    pub avg_degree: f64,
    /// Percentage of non-doc nodes that trace to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Number of active pipeline outliers past their type-specific grace period.
    pub outlier_count: i64,
    /// Percentage of outliers relative to total active nodes (0.0–100.0).
    pub outlier_percentage: f64,
    /// Fraction of delivery artifacts connected to the main delivery component (0.0–1.0).
    pub delivery_connectivity: f64,
    /// Fraction of learning artifacts connected to each other or to decisions (0.0–1.0).
    pub learning_connectivity: f64,
}
