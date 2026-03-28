// Process state tracking for the OrqaStudio workflow engine.
//
// `ProcessViolation` and `SessionProcessState` are defined in `crate::types::workflow`.
// This module provides the `ProcessStateExt` extension trait that adds business
// logic methods (reset, track_tool_call, check_violations) to `SessionProcessState`.
// All callers use this trait to interact with process state.

pub use crate::types::workflow::{ProcessViolation, SessionProcessState};

/// Extension trait providing business logic for `SessionProcessState`.
///
/// Because `SessionProcessState` is defined as a plain data struct in
/// `crate::types::workflow`, methods cannot be added via inherent impl without
/// placing them in that same module. This trait provides the full interface
/// while keeping the type definition separate from the logic.
pub trait ProcessStateExt {
    /// Reset state for a new session, clearing all tracking flags.
    fn reset(&mut self, session_id: i64);
    /// Update state based on a completed tool call.
    fn track_tool_call(&mut self, tool_name: &str, input: &serde_json::Value);
    /// Check for process compliance violations given current state.
    fn check_violations(&self) -> Vec<ProcessViolation>;
}

impl ProcessStateExt for SessionProcessState {
    /// Reset state for a new session, clearing all flags.
    fn reset(&mut self, session_id: i64) {
        self.session_id = Some(session_id);
        self.docs_read = false;
        self.knowledge_loaded = false;
        self.code_written = false;
    }

    /// Update state based on a completed tool call.
    ///
    /// `tool_name` is the name of the tool that was called.
    /// `input` is the parsed JSON input passed to the tool.
    fn track_tool_call(&mut self, tool_name: &str, input: &serde_json::Value) {
        match tool_name {
            "read_file" => {
                if let Some(path) = input["path"].as_str() {
                    if path.contains("docs/") || path.contains(".orqa/learning/rules/") {
                        self.docs_read = true;
                    }
                }
            }
            "load_knowledge" => self.knowledge_loaded = true,
            "write_file" | "edit_file" => {
                if let Some(path) = input["path"].as_str() {
                    if std::path::Path::new(path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|ext| {
                            ext.eq_ignore_ascii_case("rs")
                                || ext.eq_ignore_ascii_case("ts")
                                || ext.eq_ignore_ascii_case("svelte")
                        })
                    {
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
    fn check_violations(&self) -> Vec<ProcessViolation> {
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

        if self.code_written && !self.knowledge_loaded {
            violations.push(ProcessViolation {
                check: "knowledge_before_code".to_string(),
                message: "Code was written without loading any knowledge. \
                    Use load_knowledge to load relevant knowledge before making code changes."
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
            &make_path_input(".orqa/learning/rules/coding-standards.md"),
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
    fn track_load_knowledge_sets_knowledge_loaded() {
        let mut ps = SessionProcessState::default();
        ps.track_tool_call(
            "load_knowledge",
            &serde_json::json!({ "name": "rust-async-patterns" }),
        );
        assert!(ps.knowledge_loaded);
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
        assert!(!ps.knowledge_loaded);
        assert!(!ps.code_written);
    }

    // --- check_violations ---

    #[test]
    fn no_violations_when_nothing_happened() {
        let ps = SessionProcessState::default();
        assert!(ps.check_violations().is_empty());
    }

    #[test]
    fn no_violations_when_code_written_with_docs_and_knowledge() {
        let ps = SessionProcessState {
            docs_read: true,
            knowledge_loaded: true,
            code_written: true,
            ..Default::default()
        };
        assert!(ps.check_violations().is_empty());
    }

    #[test]
    fn violation_docs_before_code_when_no_docs_read() {
        let ps = SessionProcessState {
            knowledge_loaded: true,
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].check, "docs_before_code");
        assert_eq!(violations[0].severity, "warn");
    }

    #[test]
    fn violation_knowledge_before_code_when_no_knowledge_loaded() {
        let ps = SessionProcessState {
            docs_read: true,
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].check, "knowledge_before_code");
        assert_eq!(violations[0].severity, "warn");
    }

    #[test]
    fn both_violations_when_code_written_without_docs_or_knowledge() {
        let ps = SessionProcessState {
            code_written: true,
            ..Default::default()
        };
        let violations = ps.check_violations();
        assert_eq!(violations.len(), 2);
        let checks: Vec<&str> = violations.iter().map(|v| v.check.as_str()).collect();
        assert!(checks.contains(&"docs_before_code"));
        assert!(checks.contains(&"knowledge_before_code"));
    }

    #[test]
    fn no_violations_when_code_not_written() {
        let ps = SessionProcessState {
            docs_read: true,
            knowledge_loaded: true,
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
            knowledge_loaded: true,
            code_written: true,
        };
        ps.reset(2);
        assert_eq!(ps.session_id, Some(2));
        assert!(!ps.docs_read);
        assert!(!ps.knowledge_loaded);
        assert!(!ps.code_written);
    }
}
