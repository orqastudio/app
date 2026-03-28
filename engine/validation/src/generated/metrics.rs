// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports, missing_docs)]

use serde::{Deserialize, Serialize};

/// Graph-theoretic health metrics for the artifact graph. All values are computed purely in Rust from the graph data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHealth {
    /// Total number of primary nodes (excluding alias nodes in org mode).
    pub total_nodes: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Number of weakly-connected components. 1 means fully connected.
    pub component_count: usize,
    /// Largest connected component size / total nodes (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Nodes with zero incoming references (excluding doc artifacts).
    pub orphan_count: usize,
    /// orphan_count / total_nodes * 100, rounded to 1 decimal place.
    pub orphan_percentage: f64,
    /// Average number of relationships per node (edges * 2 / nodes).
    pub avg_degree: f64,
    /// Edge density: edges / (nodes * (nodes - 1)), clamped 0.0–1.0.
    pub graph_density: f64,
    /// Percentage of non-doc nodes that can trace a path to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Ratio of typed relationship edges that have their inverse present (0.0–1.0).
    pub bidirectionality_ratio: f64,
    /// Number of broken references (target not in graph).
    pub broken_ref_count: usize,
}

/// A point-in-time snapshot of graph health metrics stored in SQLite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshot {
    /// Auto-incremented SQLite row ID.
    pub id: i64,
    /// Foreign key to the projects table.
    pub project_id: i64,
    /// ISO 8601 timestamp when this snapshot was recorded.
    pub created_at: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub orphan_count: usize,
    pub broken_ref_count: usize,
    /// Number of Error-severity integrity findings at snapshot time.
    pub error_count: usize,
    /// Number of Warning-severity integrity findings at snapshot time.
    pub warning_count: usize,
    pub largest_component_ratio: f64,
    pub orphan_percentage: f64,
    pub avg_degree: f64,
    pub graph_density: f64,
    pub component_count: usize,
    pub pillar_traceability: f64,
    pub bidirectionality_ratio: f64,
}

/// A single node in an ancestry chain, ordered from the query artifact up to the pillar or vision root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryNode {
    /// Artifact ID (e.g. 'EPIC-048').
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Artifact type string (e.g. 'epic', 'pillar').
    pub artifact_type: String,
    /// The relationship type connecting this node to the next node upward. Empty string for the terminal (pillar/vision) node.
    pub relationship: String,
}

/// An ordered path from the query artifact to a pillar or vision root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryChain {
    /// Ordered from current artifact (index 0) to pillar/vision root (last).
    pub path: Vec<AncestryNode>,
}

/// A downstream artifact with its BFS distance from the query artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracedArtifact {
    /// Artifact ID.
    pub id: String,
    /// BFS hops from the query artifact.
    pub depth: usize,
}

/// Full traceability result for a single artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityResult {
    /// All paths from the artifact upward to any pillar or vision.
    pub ancestry_chains: Vec<AncestryChain>,
    /// All downstream artifacts with their BFS distance.
    pub descendants: Vec<TracedArtifact>,
    /// IDs of artifacts that share at least one direct parent with this artifact.
    pub siblings: Vec<String>,
    /// Count of distinct descendants within 2 hops.
    pub impact_radius: usize,
    /// True when no path exists to any pillar or vision artifact.
    pub disconnected: bool,
}

