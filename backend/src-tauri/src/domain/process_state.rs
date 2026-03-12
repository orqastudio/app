use serde::{Deserialize, Serialize};

/// A process compliance violation detected during a session.
///
/// Violations are emitted as `StreamEvent::ProcessViolation` after each turn completes,
/// so the frontend can surface them to the user without blocking execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessViolation {
    /// Machine-readable check identifier (e.g. `"docs_before_code"`).
    pub check: String,
    /// Human-readable description of the violation.
    pub message: String,
    /// Severity level: `"warn"` or `"block"`.
    pub severity: String,
}

/// Tracks process compliance state across a single session.
///
/// Resets when a new session begins (i.e. `stream_send_message` is called with
/// a different `session_id` than the previous call).
///
/// Currently enforces two documentation-first checks:
/// - `docs_before_code`: documentation must be read before code is written
/// - `skills_before_code`: skills must be loaded before code is written
#[derive(Debug, Default)]
pub struct SessionProcessState {
    /// The session this state belongs to. `None` before any message is sent.
    pub session_id: Option<i64>,
    /// Set when any `read_file` call targets a path inside `docs/` or `.orqa/rules/`.
    pub docs_read: bool,
    /// Set when any `load_skill` tool call is made.
    pub skills_loaded: bool,
    /// Set when any `write_file` or `edit_file` call targets a `.rs`, `.ts`, or `.svelte` file.
    pub code_written: bool,
}

impl SessionProcessState {
    /// Reset state for a new session.
    pub fn reset(&mut self, session_id: i64) {
        self.session_id = Some(session_id);
        self.docs_read = false;
        self.skills_loaded = false;
        self.code_written = false;
    }

    /// Update state based on a completed tool call.
    ///
    /// `tool_name` is the name of the tool that was called.
    /// `input` is the parsed JSON input passed to the tool.
    pub fn track_tool_call(&mut self, tool_name: &str, input: &serde_json::Value) {
        match tool_name {
            "read_file" => {
                if let Some(path) = input["path"].as_str() {
                    if path.contains("docs/") || path.contains(".orqa/governance/rules/") {
                        self.docs_read = true;
                    }
                }
            }
            "load_skill" => self.skills_loaded = true,
            "write_file" | "edit_file" => {
                if let Some(path) = input["path"].as_str() {
                    if path.ends_with(".rs") || path.ends_with(".ts") || path.ends_with(".svelte") {
                        self.code_written = true;
                    }
                }
            }
            _ => {}
        }
    }

    /// Check for process compliance violations.
    ///
    /// Returns a list of violations that apply given the current state.
    /// An empty list means no violations were detected.
    pub fn check_violations(&self) -> Vec<ProcessViolation> {
        let mut violations = Vec::new();

        if self.code_written && !self.docs_read {
            violations.push(ProcessViolation {
                check: "docs_before_code".to_string(),
                message: "Code was written before reading documentation. \
                    Read docs/ or .orqa/rules/ before making code changes."
                    .to_string(),
                severity: "warn".to_string(),
            });
        }

        if self.code_written && !self.skills_loaded {
            violations.push(ProcessViolation {
                check: "skills_before_code".to_string(),
                message: "Code was written without loading any skills. \
                    Use load_skill to load relevant skills before making code changes."
                    .to_string(),
                severity: "warn".to_string(),
            });
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_path_input(path: &str) -> serde_json::Value {
        serde_json::json!({ "path": path })
    }

    // --- track_tool_call ---

    #[test]
    fn track_read_file_docs_sets_docs_read() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call(
            "read_file",
            &make_path_input("docs/architecture/decisions.md"),
        );
        assert!(ps.docs_read);
    }

    #[test]
    fn track_read_file_orqa_rules_sets_docs_read() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call(
            "read_file",
            &make_path_input(".orqa/governance/rules/coding-standards.md"),
        );
        assert!(ps.docs_read);
    }

    #[test]
    fn track_read_file_src_does_not_set_docs_read() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("read_file", &make_path_input("src-tauri/src/lib.rs"));
        assert!(!ps.docs_read);
    }

    #[test]
    fn track_load_skill_sets_skills_loaded() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call(
            "load_skill",
            &serde_json::json!({ "name": "rust-async-patterns" }),
        );
        assert!(ps.skills_loaded);
    }

    #[test]
    fn track_write_file_rs_sets_code_written() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("write_file", &make_path_input("src-tauri/src/foo.rs"));
        assert!(ps.code_written);
    }

    #[test]
    fn track_write_file_ts_sets_code_written() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("write_file", &make_path_input("ui/lib/store.ts"));
        assert!(ps.code_written);
    }

    #[test]
    fn track_write_file_svelte_sets_code_written() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("write_file", &make_path_input("ui/routes/+page.svelte"));
        assert!(ps.code_written);
    }

    #[test]
    fn track_edit_file_rs_sets_code_written() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("edit_file", &make_path_input("src-tauri/src/state.rs"));
        assert!(ps.code_written);
    }

    #[test]
    fn track_write_file_md_does_not_set_code_written() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("write_file", &make_path_input("docs/foo.md"));
        assert!(!ps.code_written);
    }

    #[test]
    fn track_unknown_tool_is_noop() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call("bash", &serde_json::json!({ "command": "ls" }));
        assert!(!ps.docs_read);
        assert!(!ps.skills_loaded);
        assert!(!ps.code_written);
    }

    // --- check_violations ---

    #[test]
    fn no_violations_when_nothing_happened() {
        let ps = SessionProcessState::default();
        assert!(ps.check_violations().is_empty());
    }

    #[test]
    fn no_violations_when_code_written_with_docs_and_skills() {
        let ps = SessionProcessState {
            docs_read: true,
            skills_loaded: true,
            code_written: true,
            ..Default::default()
        };
        assert!(ps.check_violations().is_empty());
    }

    #[test]
    fn violation_docs_before_code_when_no_docs_read() {
        let ps = SessionProcessState {
            skills_loaded: true,
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].check, "docs_before_code");
        assert_eq!(violations[0].severity, "warn");
    }

    #[test]
    fn violation_skills_before_code_when_no_skills_loaded() {
        let ps = SessionProcessState {
            docs_read: true,
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].check, "skills_before_code");
        assert_eq!(violations[0].severity, "warn");
    }

    #[test]
    fn both_violations_when_code_written_without_docs_or_skills() {
        let ps = SessionProcessState {
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 2);
        let checks: Vec<&str> = violations.iter().map(|v| v.check.as_str()).collect();
        assert!(checks.contains(&"docs_before_code"));
        assert!(checks.contains(&"skills_before_code"));
    }

    #[test]
    fn no_violations_when_code_not_written() {
        // Reading docs and loading skills without writing code should not trigger anything
        let ps = SessionProcessState {
            docs_read: true,
            skills_loaded: true,
            ..Default::default()
        };
        assert!(ps.check_violations().is_empty());
    }

    // --- reset ---

    #[test]
    fn reset_clears_all_flags() {
        let mut ps = SessionProcessState {
            session_id: Some(1),
            docs_read: true,
            skills_loaded: true,
            code_written: true,
        };
        ps.reset(2);
        assert_eq!(ps.session_id, Some(2));
        assert!(!ps.docs_read);
        assert!(!ps.skills_loaded);
        assert!(!ps.code_written);
    }
}
