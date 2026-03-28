// Enforcement domain types for the OrqaStudio engine.
//
// Defines structs and enums for the enforcement rule system: event types, rule actions,
// conditions, rule entries, scan findings, parsed rules, verdicts, and violation records.
// These types flow between the enforcement engine, MCP hooks, and the governance UI.

use serde::{Deserialize, Serialize};

/// The type of tool event an enforcement entry applies to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Applies to file write or edit tool calls.
    File,
    /// Applies to bash tool calls.
    Bash,
    /// Applies to on-demand governance scans across project files.
    Scan,
    /// Documents linter delegation — declarative only, not executed by the engine.
    Lint,
}

/// What happens when an enforcement entry matches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    /// Block the tool call and return an error to the model.
    Block,
    /// Log a warning but allow the tool call to proceed.
    Warn,
    /// Inject knowledge content into the agent context (non-blocking).
    Inject,
}

/// A single field+pattern condition within a file enforcement entry.
///
/// All conditions in an entry are ANDed together — every condition must
/// match for the entry to trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// The field to match against: `"file_path"` or `"new_text"`.
    pub field: String,
    /// A regex pattern that must match the field value.
    pub pattern: String,
}

/// One enforcement entry within a rule file's frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementEntry {
    /// Whether this applies to file, bash, or scan events.
    pub event: EventType,
    /// Whether to block, warn, or inject on match.
    pub action: RuleAction,
    /// Conditions for file and scan events (all must match).
    #[serde(default)]
    pub conditions: Vec<Condition>,
    /// Pattern for bash events (single regex against the full command).
    pub pattern: Option<String>,
    /// Glob pattern for scan events (e.g., `.orqa/agents/*.md`).
    ///
    /// Defines which project files are scanned when this entry is evaluated.
    /// Resolved relative to the project root at scan time.
    #[serde(default)]
    pub scope: Option<String>,
    /// Knowledge artifacts to inject when action is `inject`.
    ///
    /// The YAML frontmatter field is `skills` for backward compatibility with existing rule files.
    #[serde(rename = "skills", default)]
    pub knowledge: Vec<String>,
}

/// A finding produced by a governance scan entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFinding {
    /// The name of the rule that produced this finding.
    pub rule_name: String,
    /// Whether this finding is a block or warn.
    pub action: RuleAction,
    /// The path to the file where the violation was found.
    pub file_path: String,
    /// The 1-based line number of the matching line.
    pub line: usize,
    /// The content of the matching line (trimmed).
    pub content: String,
    /// An excerpt of the rule prose for context.
    pub message: String,
}

/// A parsed enforcement rule from a `.orqa/rules/*.md` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementRule {
    /// The filename stem (e.g., `"coding-standards"`).
    pub name: String,
    /// The rule scope: `"system"` or `"project"`.
    pub scope: String,
    /// Parsed enforcement entries from the YAML frontmatter.
    pub entries: Vec<EnforcementEntry>,
    /// The markdown prose body, used in error messages.
    pub prose: String,
}

/// The verdict returned when an enforcement entry matches a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verdict {
    /// The name of the rule that triggered.
    pub rule_name: String,
    /// Whether to block, warn, or inject.
    pub action: RuleAction,
    /// An excerpt of the rule prose for the error message (first ~200 chars).
    pub message: String,
    /// Knowledge artifacts to inject when action is `inject`.
    ///
    /// Populated from the matching entry's `knowledge` field. Empty for block/warn verdicts.
    #[serde(default)]
    pub knowledge: Vec<String>,
}

/// A recorded enforcement rule violation from the `enforcement_violations` table.
///
/// Populated when the enforcement engine blocks or warns on a tool call.
/// Used to surface violation history in the governance UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementViolation {
    /// Database primary key.
    pub id: i64,
    /// Foreign key to the `projects` table.
    pub project_id: i64,
    /// The name of the enforcement rule that triggered (e.g. `"RULE-006"`).
    pub rule_name: String,
    /// Whether the rule blocked or warned: `"block"` or `"warn"`.
    pub action: String,
    /// The tool that triggered the violation (e.g. `"write_file"`, `"bash"`).
    pub tool_name: String,
    /// Additional context: file path, command snippet, or other detail.
    pub detail: Option<String>,
    /// ISO 8601 timestamp from the database default (`datetime('now')`).
    pub created_at: String,
}
