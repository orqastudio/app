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
    /// Unique relationship key (e.g. `"delivers"`).
    pub key: String,
    /// Inverse relationship key (e.g. `"delivered-by"`).
    pub inverse: String,
    /// Human-readable description of the relationship's meaning.
    #[serde(default)]
    pub description: String,
    /// Allowed source artifact types (empty = unconstrained).
    #[serde(default)]
    pub from: Vec<String>,
    /// Allowed target artifact types (empty = unconstrained).
    #[serde(default)]
    pub to: Vec<String>,
    /// Semantic category key (e.g. `"dependency"`, `"delivery"`).
    #[serde(default)]
    pub semantic: Option<String>,
    /// Validation constraints block for this relationship.
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
    /// Whether the frontmatter passed all schema checks.
    pub valid: bool,
    /// List of validation error messages (empty when `valid` is true).
    pub errors: Vec<String>,
}

/// A fully parsed artifact: frontmatter, body, type inference, and schema validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedArtifact {
    /// Artifact ID from frontmatter (e.g. `"TASK-abc12345"`).
    pub id: String,
    /// Artifact type key inferred from ID prefix (e.g. `"task"`).
    #[serde(rename = "type")]
    pub artifact_type: String,
    /// Current status value from frontmatter, if present.
    pub status: Option<String>,
    /// Title from the first `# Heading` line or frontmatter `title` field.
    pub title: String,
    /// All frontmatter fields as a JSON object.
    pub frontmatter: serde_json::Value,
    /// Full markdown body (everything after the frontmatter block).
    pub content: String,
    /// Schema validation result for this artifact's frontmatter.
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
