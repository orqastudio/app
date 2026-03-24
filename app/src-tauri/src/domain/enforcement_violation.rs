use serde::{Deserialize, Serialize};

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
