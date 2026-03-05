use serde::{Deserialize, Serialize};

/// The type of tool event an enforcement entry applies to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Applies to file write or edit tool calls.
    File,
    /// Applies to bash tool calls.
    Bash,
}

/// What happens when an enforcement entry matches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    /// Block the tool call and return an error to the model.
    Block,
    /// Log a warning but allow the tool call to proceed.
    Warn,
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
    /// Whether this applies to file or bash events.
    pub event: EventType,
    /// Whether to block or warn on match.
    pub action: RuleAction,
    /// Conditions for file events (all must match).
    #[serde(default)]
    pub conditions: Vec<Condition>,
    /// Pattern for bash events (single regex against the full command).
    pub pattern: Option<String>,
}

/// A parsed enforcement rule from a `.claude/rules/*.md` file.
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
    /// Whether to block or warn.
    pub action: RuleAction,
    /// An excerpt of the rule prose for the error message (first ~200 chars).
    pub message: String,
}
