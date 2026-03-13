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
}
