//! Public integrity check types exported from the validation library.
//!
//! Integrity finding types (`IntegrityCheck`, `AppliedFix`, etc.) and graph health
//! types (`GraphHealth`, `OutlierAgeDistribution`) live in `orqa_engine_types` and
//! are re-exported here so callers of this module get them without a direct dep on
//! `orqa_engine_types`.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::platform::{ArtifactTypeDef, EnforcementMechanism, SchemaExtension};
use crate::settings::DeliveryConfig;

// Re-export graph health and integrity types from engine/types so callers of
// this module get them without adding a direct orqa_engine_types dependency.
pub use orqa_engine_types::{
    AppliedFix, GraphHealth, IntegrityCategory, IntegrityCheck, IntegritySeverity,
    OutlierAgeDistribution,
};

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

// GraphHealth and OutlierAgeDistribution are re-exported from orqa_engine_types above.
