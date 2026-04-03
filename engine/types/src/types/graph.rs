//! Graph domain types for the OrqaStudio artifact graph.
//!
//! Contains the core data shapes for the artifact graph — the bidirectional
//! relationship graph built from `.orqa/` markdown files. These types are shared
//! between `engine/graph` (which builds the graph) and `engine/validation` (which
//! runs integrity checks against it), so they live in `engine/types` to avoid
//! circular dependencies.
//!
//! This module contains ONLY type definitions — no graph construction logic.
//! Graph building lives in `engine/graph`, health computation in `engine/validation`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Core graph types
// ---------------------------------------------------------------------------

/// A bidirectional graph of all governance artifacts in `.orqa/`.
///
/// Built by scanning every `.md` file under the project root that carries a
/// YAML `id` field. References between artifacts are extracted from well-known
/// frontmatter fields and inverted in a second pass to produce `references_in`
/// backlinks on each node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactGraph {
    /// All artifact nodes, keyed by their `id` frontmatter value (e.g. "EPIC-048").
    pub nodes: HashMap<String, ArtifactNode>,
    /// Reverse-lookup index: relative file path → artifact ID.
    pub path_index: HashMap<String, String>,
}

/// A single artifact node in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactNode {
    /// Frontmatter `id` field (e.g. "EPIC-048").
    pub id: String,
    /// Source project name in organisation mode, or `None` for single-project mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Relative path from the project root (e.g. ".orqa/implementation/epics/EPIC-048.md").
    pub path: String,
    /// Inferred category string (e.g. "epic", "task", "milestone", "idea", "decision").
    pub artifact_type: String,
    /// Frontmatter `title` field, or a humanized fallback from the filename.
    pub title: String,
    /// Frontmatter `description` field.
    pub description: Option<String>,
    /// Frontmatter `status` field.
    pub status: Option<String>,
    /// Frontmatter `priority` field (e.g. "P1", "P2", "P3").
    pub priority: Option<String>,
    /// Full YAML frontmatter parsed into JSON for generic access.
    pub frontmatter: serde_json::Value,
    /// Markdown body content (everything after the YAML frontmatter block).
    /// Cached at graph-build time to avoid re-reading files during queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Forward references declared in this node's frontmatter.
    pub references_out: Vec<ArtifactRef>,
    /// Backlinks computed from other nodes' `references_out` during pass 2.
    pub references_in: Vec<ArtifactRef>,
}

/// A directed reference from one artifact to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    /// The artifact ID that is referenced (the link target).
    pub target_id: String,
    /// Name of the frontmatter field that contains this reference.
    pub field: String,
    /// ID of the artifact that declares this reference (the link source).
    pub source_id: String,
    /// Semantic relationship type (e.g. "enforced-by", "grounded-by", "delivers").
    /// Only populated for refs from the `relationships` frontmatter array.
    pub relationship_type: Option<String>,
}

/// Summary statistics about the artifact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    /// Total number of nodes (artifacts with an `id` field).
    pub node_count: usize,
    /// Total number of directed edges (sum of all `references_out` lengths).
    pub edge_count: usize,
    /// Nodes that have no `references_out` and no `references_in`.
    pub orphan_count: usize,
    /// References whose `target_id` does not exist in the graph.
    pub broken_ref_count: usize,
}

// ---------------------------------------------------------------------------
// Graph health types
// ---------------------------------------------------------------------------

/// Age distribution of pipeline outliers, bucketed by how long ago they were created.
///
/// Only artifacts that pass all outlier filters (active, in-scope type, past grace period)
/// are counted here. Artifacts without a `created` field are placed in the `stale` bucket
/// because their age is unknown and they should be investigated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutlierAgeDistribution {
    /// Outliers created within the last 7 days — within grace period, informational only.
    pub fresh: usize,
    /// Outliers created 7–30 days ago — aging, should be connected or archived soon.
    pub aging: usize,
    /// Outliers created more than 30 days ago (or with no `created` date) — priority action items.
    pub stale: usize,
}

/// Graph-theoretic health metrics for the artifact graph.
///
/// These are computed purely in Rust from the graph data structure —
/// no delegation to JavaScript.
///
/// The model tracks two pipelines:
/// - Delivery: task → epic → milestone → idea → research → decision → wireframe
/// - Learning: lesson → rule, both connecting back to decisions
///
/// Artifacts outside both pipelines (excluding archived, surpassed, knowledge,
/// and doc types) are counted as outliers. Each outlier is only counted once it
/// has exceeded the grace period for its artifact type. The `outlier_age_distribution`
/// breaks down all age-eligible outliers by recency bucket.
///
/// The `Default` implementation returns a zeroed struct, suitable for an empty graph.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphHealth {
    /// Number of active pipeline outliers past their type-specific grace period.
    pub outlier_count: usize,
    /// Percentage of outliers relative to total active (non-archived, non-knowledge) nodes (0.0–100.0).
    pub outlier_percentage: f64,
    /// Age distribution of outliers: fresh (≤7d), aging (7–30d), stale (30d+ or no date).
    pub outlier_age_distribution: OutlierAgeDistribution,
    /// Fraction of delivery artifacts (task, epic, milestone, idea, research, decision, wireframe)
    /// that are connected to the main delivery component (0.0–1.0).
    pub delivery_connectivity: f64,
    /// Fraction of learning artifacts (lesson, rule) connected to each other or to decisions (0.0–1.0).
    pub learning_connectivity: f64,
    /// Average number of edges per node (in + out combined, undirected).
    pub avg_degree: f64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Total number of primary nodes.
    pub total_nodes: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Percentage of non-doc nodes that can trace a path to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Number of broken references (target not in graph).
    pub broken_ref_count: usize,
}

// ---------------------------------------------------------------------------
// Integrity finding types
// ---------------------------------------------------------------------------

/// Category of integrity issue found in the artifact graph.
///
/// Generic categories derived from schema-driven checks. No relationship keys
/// or artifact types are hardcoded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrityCategory {
    /// Target of a reference does not exist in the graph.
    BrokenLink,
    /// From/to type constraints on a relationship are violated.
    TypeConstraintViolation,
    /// A required relationship (constraints.required) is missing or below minCount.
    RequiredRelationshipMissing,
    /// A maxCount cardinality constraint is exceeded.
    CardinalityViolation,
    /// A cycle was detected on a relationship with "dependency" semantic.
    CircularDependency,
    /// Status value is not in the valid status list.
    InvalidStatus,
    /// The explicit `type:` field does not match the type implied by the ID prefix.
    TypePrefixMismatch,
    /// Child artifact is further along the status progression than its parent.
    ParentChildInconsistency,
    /// Delivery path does not match the delivery config hierarchy.
    DeliveryPathMismatch,
    /// The artifact has no `type:` field in its frontmatter.
    MissingType,
    /// The artifact has no `status:` field in its frontmatter.
    MissingStatus,
    /// The same target + relationship type appears more than once in `relationships`.
    DuplicateRelationship,
    /// The filename does not match the artifact's frontmatter `id`.
    FilenameMismatch,
    /// A required frontmatter field is absent, or the artifact status is not permitted
    /// by the type's declared status transition schema.
    SchemaViolation,
    /// Body text contains a reference to an artifact but no formal relationship edge exists.
    BodyTextRefWithoutRelationship,
}

/// Severity of an integrity finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegritySeverity {
    /// Blocking violation — must be resolved for the graph to be valid.
    Error,
    /// Reserved — no checks currently emit Warning. All graph integrity
    /// violations are errors because without relationships there is no graph.
    Warning,
    /// Informational note — no action required.
    Info,
}

/// A single integrity finding from the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheck {
    /// Category of the integrity violation.
    pub category: IntegrityCategory,
    /// Severity level of this finding.
    pub severity: IntegritySeverity,
    /// ID of the artifact that triggered this finding.
    pub artifact_id: String,
    /// Human-readable description of the violation.
    pub message: String,
    /// Whether the engine can automatically repair this violation.
    pub auto_fixable: bool,
    /// Description of what the auto-fix would do, if applicable.
    pub fix_description: Option<String>,
}

/// A fix that was applied to resolve an integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    /// ID of the artifact that was modified.
    pub artifact_id: String,
    /// Human-readable description of what was changed.
    pub description: String,
    /// Path to the file that was modified.
    pub file_path: String,
}

// ---------------------------------------------------------------------------
// Traceability types
// ---------------------------------------------------------------------------

/// A single node in an ancestry chain, ordered from the query artifact up to
/// the pillar or vision root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryNode {
    /// Artifact ID (e.g. "EPIC-048").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Artifact type string (e.g. "epic", "pillar").
    pub artifact_type: String,
    /// The relationship type that connects this node to the *next* node in the
    /// chain (i.e. the edge leading upward toward the pillar).
    /// Empty string for the terminal node.
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
    /// All downstream artifacts (following references_out), with distance.
    pub descendants: Vec<TracedArtifact>,
    /// Artifacts that share at least one direct parent with the query artifact.
    pub siblings: Vec<String>,
    /// Count of distinct descendants within 2 hops (impact radius).
    pub impact_radius: usize,
    /// True when no path exists to any pillar or vision artifact.
    pub disconnected: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, artifact_type: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: format!("{id} Title"),
            description: Some("A test node.".to_owned()),
            status: Some("active".to_owned()),
            priority: Some("P1".to_owned()),
            frontmatter: serde_json::json!({"id": id, "type": artifact_type}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    // -----------------------------------------------------------------------
    // ArtifactGraph default / construction
    // -----------------------------------------------------------------------

    #[test]
    fn artifact_graph_default_is_empty() {
        let graph = ArtifactGraph::default();
        assert!(graph.nodes.is_empty());
        assert!(graph.path_index.is_empty());
    }

    #[test]
    fn artifact_graph_serde_round_trip() {
        let mut graph = ArtifactGraph::default();
        graph.nodes.insert("TASK-001".to_owned(), make_node("TASK-001", "task"));
        graph.path_index.insert(".orqa/tasks/TASK-001.md".to_owned(), "TASK-001".to_owned());

        let json = serde_json::to_string(&graph).expect("serialize");
        let restored: ArtifactGraph = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.nodes.len(), 1);
        assert!(restored.nodes.contains_key("TASK-001"));
        assert_eq!(
            restored.path_index.get(".orqa/tasks/TASK-001.md"),
            Some(&"TASK-001".to_owned())
        );
    }

    // -----------------------------------------------------------------------
    // ArtifactNode serde
    // -----------------------------------------------------------------------

    #[test]
    fn artifact_node_serde_round_trip() {
        let node = make_node("EPIC-001", "epic");
        let json = serde_json::to_string(&node).expect("serialize");
        let restored: ArtifactNode = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(restored.id, "EPIC-001");
        assert_eq!(restored.artifact_type, "epic");
        assert_eq!(restored.status, Some("active".to_owned()));
        assert_eq!(restored.priority, Some("P1".to_owned()));
    }

    #[test]
    fn artifact_node_optional_project_omitted_in_json() {
        let node = make_node("TASK-001", "task"); // project is None
        let json = serde_json::to_string(&node).expect("serialize");
        // skip_serializing_if = "Option::is_none" means the key should be absent
        assert!(!json.contains("\"project\""));
    }

    #[test]
    fn artifact_node_project_present_when_set() {
        let mut node = make_node("TASK-001", "task");
        node.project = Some("my-app".to_owned());
        let json = serde_json::to_string(&node).expect("serialize");
        assert!(json.contains("my-app"));
    }

    // -----------------------------------------------------------------------
    // ArtifactRef serde
    // -----------------------------------------------------------------------

    #[test]
    fn artifact_ref_serde_round_trip() {
        let aref = ArtifactRef {
            target_id: "EPIC-001".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-001".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        };
        let json = serde_json::to_string(&aref).expect("serialize");
        let restored: ArtifactRef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.target_id, "EPIC-001");
        assert_eq!(restored.relationship_type, Some("delivers".to_owned()));
    }

    // -----------------------------------------------------------------------
    // GraphHealth default
    // -----------------------------------------------------------------------

    #[test]
    fn graph_health_default_is_zero() {
        let h = GraphHealth::default();
        assert_eq!(h.outlier_count, 0);
        assert_eq!(h.total_nodes, 0);
        assert_eq!(h.total_edges, 0);
        assert_eq!(h.avg_degree, 0.0);
        assert_eq!(h.broken_ref_count, 0);
    }

    #[test]
    fn graph_health_serde_round_trip() {
        let h = GraphHealth {
            outlier_count: 5,
            outlier_percentage: 12.5,
            outlier_age_distribution: OutlierAgeDistribution {
                fresh: 1,
                aging: 2,
                stale: 2,
            },
            delivery_connectivity: 0.8,
            learning_connectivity: 0.9,
            avg_degree: 3.2,
            largest_component_ratio: 0.95,
            total_nodes: 40,
            total_edges: 64,
            pillar_traceability: 75.0,
            broken_ref_count: 2,
        };
        let json = serde_json::to_string(&h).expect("serialize");
        let restored: GraphHealth = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.outlier_count, 5);
        assert_eq!(restored.total_nodes, 40);
        assert_eq!(restored.outlier_age_distribution.aging, 2);
    }

    // -----------------------------------------------------------------------
    // IntegrityCategory / IntegritySeverity equality
    // -----------------------------------------------------------------------

    #[test]
    fn integrity_category_equality() {
        assert_eq!(IntegrityCategory::BrokenLink, IntegrityCategory::BrokenLink);
        assert_ne!(IntegrityCategory::BrokenLink, IntegrityCategory::InvalidStatus);
    }

    #[test]
    fn integrity_severity_equality() {
        assert_eq!(IntegritySeverity::Error, IntegritySeverity::Error);
        assert_ne!(IntegritySeverity::Error, IntegritySeverity::Warning);
    }

    // -----------------------------------------------------------------------
    // GraphStats
    // -----------------------------------------------------------------------

    #[test]
    fn graph_stats_serde_round_trip() {
        let stats = GraphStats {
            node_count: 100,
            edge_count: 250,
            orphan_count: 10,
            broken_ref_count: 3,
        };
        let json = serde_json::to_string(&stats).expect("serialize");
        let restored: GraphStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.node_count, 100);
        assert_eq!(restored.broken_ref_count, 3);
    }
}
