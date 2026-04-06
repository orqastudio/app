//! Health snapshot domain types for the OrqaStudio engine.
//!
//! Defines structs representing point-in-time snapshots of artifact graph health metrics.
//! Snapshots are persisted in the local SQLite database and surfaced in the governance dashboard.
//!
//! The model tracks the outlier-based pipeline health model:
//! - Outliers: artifacts outside both delivery and learning pipelines, past their grace period.
//! - Delivery connectivity: fraction of delivery artifacts connected to the main pipeline.
//! - Learning connectivity: fraction of learning artifacts (lessons, rules) connected to decisions.
//!
//! # ID representation
//!
//! `id` and `project_id` are raw `i64` SQLite rowids, consistent with the rest of the
//! engine types. See the note in `types::project` for the rationale.

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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> HealthSnapshot {
        HealthSnapshot {
            id: 1,
            project_id: 42,
            node_count: 100,
            edge_count: 200,
            broken_ref_count: 3,
            error_count: 5,
            warning_count: 2,
            largest_component_ratio: 0.92,
            avg_degree: 4.0,
            pillar_traceability: 80.0,
            outlier_count: 7,
            outlier_percentage: 7.0,
            delivery_connectivity: 0.85,
            learning_connectivity: 0.75,
            created_at: "2026-04-01T00:00:00Z".to_owned(),
        }
    }

    fn sample_new_snapshot() -> NewHealthSnapshot {
        NewHealthSnapshot {
            node_count: 50,
            edge_count: 80,
            broken_ref_count: 1,
            error_count: 2,
            warning_count: 0,
            largest_component_ratio: 0.98,
            avg_degree: 3.2,
            pillar_traceability: 90.0,
            outlier_count: 2,
            outlier_percentage: 4.0,
            delivery_connectivity: 0.95,
            learning_connectivity: 0.88,
        }
    }

    #[test]
    fn health_snapshot_serde_round_trip() {
        let snap = sample_snapshot();
        let json = serde_json::to_string(&snap).expect("serialize");
        let restored: HealthSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.id, 1);
        assert_eq!(restored.project_id, 42);
        assert_eq!(restored.node_count, 100);
        assert_eq!(restored.broken_ref_count, 3);
        assert!((restored.largest_component_ratio - 0.92).abs() < 1e-9);
        assert_eq!(restored.created_at, "2026-04-01T00:00:00Z");
    }

    #[test]
    fn new_health_snapshot_serde_round_trip() {
        let snap = sample_new_snapshot();
        let json = serde_json::to_string(&snap).expect("serialize");
        let restored: NewHealthSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.node_count, 50);
        assert_eq!(restored.error_count, 2);
        assert!((restored.delivery_connectivity - 0.95).abs() < 1e-9);
    }

    #[test]
    fn health_snapshot_fields_preserve_precision() {
        let mut snap = sample_snapshot();
        snap.outlier_percentage = 33.333_333_333;
        let json = serde_json::to_string(&snap).expect("serialize");
        let restored: HealthSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert!((restored.outlier_percentage - 33.333_333_333).abs() < 1e-6);
    }

    #[test]
    fn new_health_snapshot_zero_values_round_trip() {
        let snap = NewHealthSnapshot {
            node_count: 0,
            edge_count: 0,
            broken_ref_count: 0,
            error_count: 0,
            warning_count: 0,
            largest_component_ratio: 0.0,
            avg_degree: 0.0,
            pillar_traceability: 0.0,
            outlier_count: 0,
            outlier_percentage: 0.0,
            delivery_connectivity: 0.0,
            learning_connectivity: 0.0,
        };
        let json = serde_json::to_string(&snap).expect("serialize");
        let restored: NewHealthSnapshot = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.node_count, 0);
        assert!((restored.avg_degree - 0.0_f64).abs() < f64::EPSILON);
    }

    #[test]
    fn health_snapshot_clone_produces_equal_values() {
        let snap = sample_snapshot();
        let cloned = snap.clone();
        assert_eq!(cloned.id, snap.id);
        assert_eq!(cloned.project_id, snap.project_id);
        assert_eq!(cloned.node_count, snap.node_count);
        assert_eq!(cloned.created_at, snap.created_at);
    }
}
