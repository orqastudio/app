//! Health snapshot domain types for the OrqaStudio engine.
//!
//! Defines structs representing point-in-time snapshots of artifact graph health metrics.
//! Snapshots are persisted by the daemon and surfaced in the governance dashboard.

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
    /// Number of nodes with no edges in either direction.
    pub orphan_count: i64,
    /// Number of relationship targets that do not exist in the graph.
    pub broken_ref_count: i64,
    /// Number of integrity check findings with severity Error.
    pub error_count: i64,
    /// Number of integrity check findings with severity Warning.
    pub warning_count: i64,
    /// Largest connected component size / total nodes (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Orphan count as a percentage of total nodes (0.0–100.0).
    pub orphan_percentage: f64,
    /// Average degree: (edges * 2) / nodes.
    pub avg_degree: f64,
    /// Edge density: edges / (nodes * (nodes - 1)).
    pub graph_density: f64,
    /// Number of weakly-connected components.
    pub component_count: i64,
    /// Percentage of rules with at least one grounded-by → pillar relationship.
    pub pillar_traceability: f64,
    /// Ratio of typed relationship edges that have their inverse present.
    pub bidirectionality_ratio: f64,
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
    /// Number of nodes with no edges in either direction.
    pub orphan_count: i64,
    /// Number of relationship targets that do not exist in the graph.
    pub broken_ref_count: i64,
    /// Number of integrity check findings with severity Error.
    pub error_count: i64,
    /// Number of integrity check findings with severity Warning.
    pub warning_count: i64,
    /// Largest connected component size / total nodes (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Orphan count as a percentage of total nodes (0.0–100.0).
    pub orphan_percentage: f64,
    /// Average degree: (edges * 2) / nodes.
    pub avg_degree: f64,
    /// Edge density: edges / (nodes * (nodes - 1)).
    pub graph_density: f64,
    /// Number of weakly-connected components.
    pub component_count: i64,
    /// Percentage of rules with at least one grounded-by → pillar relationship.
    pub pillar_traceability: f64,
    /// Ratio of typed relationship edges that have their inverse present.
    pub bidirectionality_ratio: f64,
}
