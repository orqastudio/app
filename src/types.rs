//! Public integrity check types exported from the validation library.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::settings::DeliveryConfig;

// ---------------------------------------------------------------------------
// Schema types
// ---------------------------------------------------------------------------

/// A status rule constraint from the relationship schema.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusRule {
    /// Which side to evaluate: `"source"` or `"target"`.
    pub evaluate: String,
    /// Condition to test: `"all-targets-in"`, `"any-target-in"`, `"no-targets-in"`.
    pub condition: String,
    /// The status values to check against.
    pub statuses: Vec<String>,
    /// The status to propose when the condition is met.
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    /// Human-readable description of this rule.
    pub description: String,
}

/// Constraint block on a relationship definition.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RelationshipConstraints {
    /// Whether artifacts of the `from` type must have at least one instance of this relationship.
    #[serde(default)]
    pub required: Option<bool>,
    /// Minimum number of relationships of this type required (only when `required` is true).
    #[serde(rename = "minCount", default)]
    pub min_count: Option<usize>,
    /// Maximum number of relationships of this type allowed.
    #[serde(rename = "maxCount", default)]
    pub max_count: Option<usize>,
    /// Whether the inverse edge must exist.
    #[serde(rename = "requireInverse", default)]
    pub require_inverse: Option<bool>,
    /// Status-based transition rules.
    #[serde(rename = "statusRules", default)]
    pub status_rules: Vec<StatusRule>,
}

/// A relationship schema entry — combines platform, project, and plugin definitions.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipSchema {
    pub key: String,
    pub inverse: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub semantic: Option<String>,
    #[serde(default)]
    pub constraints: Option<RelationshipConstraints>,
}

/// The full validation context loaded from core.json + project.json + plugins.
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// All relationship schemas, keyed by relationship key for fast lookup.
    pub relationships: Vec<RelationshipSchema>,
    /// Maps each relationship key to its inverse.
    pub inverse_map: HashMap<String, String>,
    /// The valid status values from project.json.
    pub valid_statuses: Vec<String>,
    /// The delivery config from project.json (kept for delivery-path checks).
    pub delivery: DeliveryConfig,
    /// Relationship keys that have the "dependency" semantic.
    pub dependency_keys: HashSet<String>,
}

// ---------------------------------------------------------------------------
// Integrity finding types
// ---------------------------------------------------------------------------

/// Category of integrity issue found in the artifact graph.
///
/// Generic categories derived from schema-driven checks. No relationship keys
/// or artifact types are hardcoded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrityCategory {
    /// Target of a reference does not exist in the graph.
    BrokenLink,
    /// Inverse relationship edge is missing.
    MissingInverse,
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
    /// Body text references an artifact without a corresponding relationship edge.
    BodyTextRefWithoutRelationship,
    /// Child artifact is further along the status progression than its parent.
    ParentChildInconsistency,
    /// Delivery path does not match the delivery config hierarchy.
    DeliveryPathMismatch,
}

/// Severity of an integrity finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegritySeverity {
    Error,
    Warning,
}

/// A single integrity finding from the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityCheck {
    pub category: IntegrityCategory,
    pub severity: IntegritySeverity,
    pub artifact_id: String,
    pub message: String,
    pub auto_fixable: bool,
    pub fix_description: Option<String>,
}

/// A fix that was applied to resolve an integrity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    pub artifact_id: String,
    pub description: String,
    pub file_path: String,
}

// ---------------------------------------------------------------------------
// Graph health metrics
// ---------------------------------------------------------------------------

/// Graph-theoretic health metrics for the artifact graph.
///
/// These are computed purely in Rust from the graph data structure —
/// no delegation to JavaScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHealth {
    /// Number of weakly connected components in the graph.
    pub component_count: usize,
    /// Number of nodes with no edges in either direction (excluding docs).
    pub orphan_count: usize,
    /// Percentage of nodes that are orphans (0.0–100.0).
    pub orphan_percentage: f64,
    /// Average number of edges per node (in + out combined, undirected).
    pub avg_degree: f64,
    /// Graph density: actual edges / maximum possible edges (0.0–1.0).
    pub graph_density: f64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Total number of primary nodes.
    pub total_nodes: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Percentage of non-doc nodes that can trace a path to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Ratio of relationship edges that have their inverse (0.0–1.0).
    pub bidirectionality_ratio: f64,
}
