//! Public integrity check types exported from the validation library.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::platform::{ArtifactTypeDef, EnforcementMechanism, SchemaExtension};
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

/// The full validation context loaded from project.json + plugins.
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
    /// Artifact type definitions contributed by plugins (frontmatter requirements,
    /// status transitions, id prefixes). Used by schema-violation checks.
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// Schema extensions from plugins that extend other types' frontmatter schemas.
    pub schema_extensions: Vec<SchemaExtension>,
    /// Enforcement mechanisms registered by installed plugins.
    pub enforcement_mechanisms: Vec<EnforcementMechanism>,
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
    Error,
    /// Reserved — no checks currently emit Warning. All graph integrity
    /// violations are errors because without relationships there is no graph.
    Warning,
    Info,
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
// Enforcement event types
// ---------------------------------------------------------------------------

/// Result of an enforcement check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnforcementResult {
    /// No violation found.
    Pass,
    /// Violation detected, enforcement triggered.
    Fail,
    /// Potential issue, not blocking.
    Warn,
    /// Enforcement check itself failed.
    Error,
}

/// Known ONNX inference check types for the `onnx` enforcement mechanism.
///
/// Rules declare these in their enforcement entries:
/// ```yaml
/// enforcement:
///   - mechanism: onnx
///     check: lesson-recurrence
///     threshold: 0.85
///     action: warn
/// ```
pub const ONNX_CHECK_TYPES: &[&str] = &[
    "lesson-recurrence",
    "duplicate-detection",
    "compliance-check",
    "quality-signal",
];

/// A single enforcement event produced by a validation check.
///
/// Every enforcement check — regardless of source — produces one event
/// per check per artifact. These are serialised to the centralised
/// enforcement log (`.state/enforcement-log.jsonl`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementEvent {
    /// Mechanism key that produced this event.
    pub mechanism: String,
    /// Check type within the mechanism (e.g. "frontmatter", "PreToolUse").
    #[serde(rename = "type")]
    pub check_type: String,
    /// Rule ID that triggered this enforcement, if applicable.
    pub rule_id: Option<String>,
    /// Artifact ID being checked, if applicable.
    pub artifact_id: Option<String>,
    /// Check result.
    pub result: EnforcementResult,
    /// Human-readable message describing the finding.
    pub message: String,
}

// ---------------------------------------------------------------------------
// Artifact parse types
// ---------------------------------------------------------------------------

/// The result of validating a parsed artifact's frontmatter against its type schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// A fully parsed artifact: frontmatter, body, type inference, and schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedArtifact {
    pub id: String,
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub status: Option<String>,
    pub title: String,
    pub frontmatter: serde_json::Value,
    pub content: String,
    pub validation: ValidationResult,
}

// ---------------------------------------------------------------------------
// Hook lifecycle types
// ---------------------------------------------------------------------------

/// Context passed to the hook evaluation engine.
///
/// Carries all the information the evaluator needs to match rule enforcement
/// entries against the current lifecycle event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookContext {
    /// The lifecycle event name, e.g. `"PreAction"`, `"PostAction"`, `"PromptSubmit"`.
    pub event: String,
    /// The tool being invoked (populated for `PreAction` / `PostAction` events).
    pub tool_name: Option<String>,
    /// The raw tool input as JSON (populated for `PreAction` / `PostAction` events).
    pub tool_input: Option<serde_json::Value>,
    /// The file path being written or read, if applicable.
    pub file_path: Option<String>,
    /// The user message text, for `PromptSubmit` events.
    pub user_message: Option<String>,
    /// The agent type running the hook, for agent-scoped rules.
    pub agent_type: Option<String>,
}

/// The result of evaluating all active rules against a [`HookContext`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResult {
    /// Overall action: `"allow"`, `"warn"`, or `"block"`.
    ///
    /// Precedence: any `"block"` violation → `"block"`;
    /// otherwise any `"warn"` → `"warn"`; else `"allow"`.
    pub action: String,
    /// Human-readable summary messages (one per violation).
    pub messages: Vec<String>,
    /// Structured violation list for programmatic consumption.
    pub violations: Vec<HookViolation>,
}

/// A single rule violation found during hook evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookViolation {
    /// The rule artifact ID that produced this violation.
    pub rule_id: String,
    /// Enforcement action declared by the rule: `"block"`, `"warn"`, or `"inject"`.
    pub action: String,
    /// Human-readable description from the rule enforcement entry.
    pub message: String,
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
    /// Number of broken references (target not in graph).
    pub broken_ref_count: usize,
}
