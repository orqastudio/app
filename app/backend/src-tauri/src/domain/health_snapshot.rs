use serde::{Deserialize, Serialize};

/// A point-in-time snapshot of graph health metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshot {
    pub id: i64,
    pub project_id: i64,
    pub node_count: i64,
    pub edge_count: i64,
    pub orphan_count: i64,
    pub broken_ref_count: i64,
    pub error_count: i64,
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
    pub created_at: String,
}

/// Input for creating a new health snapshot (no id or timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHealthSnapshot {
    pub node_count: i64,
    pub edge_count: i64,
    pub orphan_count: i64,
    pub broken_ref_count: i64,
    pub error_count: i64,
    pub warning_count: i64,
    pub largest_component_ratio: f64,
    pub orphan_percentage: f64,
    pub avg_degree: f64,
    pub graph_density: f64,
    pub component_count: i64,
    pub pillar_traceability: f64,
    pub bidirectionality_ratio: f64,
}
