use crate::domain::workflow_tracker::WorkflowTracker;

/// The result of a single process gate evaluation.
///
/// When `fired` is `true`, `message` contains a thinking prompt to inject into
/// the agent's context to guide it back toward correct process.
#[derive(Debug, Clone)]
pub struct GateResult {
    /// Machine-readable gate identifier (e.g. `"understand-first"`).
    pub gate_name: String,
    /// Thinking prompt to inject into the agent context when the gate fires.
    pub message: String,
    /// Whether this gate fired (condition was met and action should be taken).
    pub fired: bool,
}

/// Returns true if the given file path should be treated as a code file.
///
/// Code files are anything that is NOT a governance artifact (`.orqa/`). This
/// mirrors the logic in `WorkflowTracker::has_written_code`.
fn is_code_file(path: &str) -> bool {
    !path.starts_with(".orqa/") && !path.contains("/.orqa/")
}

/// Create a `GateResult` that fired with the given message, or an unfired result.
fn gate(name: &str, fired: bool, message: &str) -> GateResult {
    GateResult {
        gate_name: name.to_string(),
        message: if fired {
            message.to_string()
        } else {
            String::new()
        },
        fired,
    }
}

/// Evaluate all process gates against the current session workflow state.
///
/// # Parameters
///
/// - `tracker`: The current session's `WorkflowTracker`.
/// - `event_type`: Either `"write"` (file write/edit tool call) or `"stop"` (turn complete).
/// - `file_path`: The path of the file being written, when `event_type == "write"`. `None`
///   otherwise.
///
/// # Returns
///
/// A `Vec<GateResult>` of all evaluated gates. Each entry has `fired: true` when
/// the gate's condition was met and a thinking prompt should be injected. Gates
/// with `fired: false` are returned for observability but require no action.
pub fn evaluate_process_gates(
    tracker: &mut WorkflowTracker,
    event_type: &str,
    file_path: Option<&str>,
) -> Vec<GateResult> {
    let mut results = Vec::new();

    match event_type {
        "write" => {
            let writing_code = file_path.is_some_and(is_code_file);

            // Gate: understand-first
            // Fires once per session, on the first code write, when no research has been done.
            let uf_fired = writing_code
                && !tracker.has_done_any_research()
                && !tracker.first_code_write_gated;
            if uf_fired {
                tracker.first_code_write_gated = true;
            }
            results.push(gate(
                "understand-first",
                uf_fired,
                "THINK FIRST: What is the system you're modifying? \
                 What are its boundaries? What depends on this? What could break? \
                 Read the governing docs and understand the context before writing code.",
            ));

            // Gate: docs-before-code
            // Fires on any code write when no docs have been read this session.
            results.push(gate(
                "docs-before-code",
                writing_code && !tracker.has_read_any_docs(),
                "DOCUMENTATION CHECK: Have you read the documentation that defines \
                 this area? Check .orqa/documentation/ for specs, patterns, and constraints \
                 before implementing.",
            ));

            // Gate: plan-before-build
            // Fires on any code write when no planning artifacts have been consulted.
            results.push(gate(
                "plan-before-build",
                writing_code && !tracker.has_read_any_planning(),
                "PLANNING CHECK: Is there an epic or task that defines this work? \
                 Check .orqa/planning/ for the scope, acceptance criteria, and \
                 implementation design.",
            ));
        }

        "stop" => {
            // Gate: evidence-before-done
            // Fires at turn end when code was written but no verification command was run.
            results.push(gate(
                "evidence-before-done",
                tracker.has_written_code() && !tracker.has_run_verification(),
                "VERIFICATION CHECK: You wrote code but didn't run make check \
                 or make test. Show evidence that the work is correct before completing.",
            ));

            // Gate: learn-after-doing
            // Fires at turn end when significant code was written but lessons were not checked.
            results.push(gate(
                "learn-after-doing",
                tracker.code_write_count() > 3 && !tracker.has_checked_lessons(),
                "LEARNING CHECK: Significant work was done this session. \
                 Check .orqa/governance/lessons/ for known patterns and consider \
                 if anything unexpected should be recorded.",
            ));
        }

        other => {
            tracing::warn!("[process_gates] unknown event_type: '{other}'");
        }
    }

    results
}

/// Return only the gates that fired from a full evaluation result.
///
/// Convenience helper for callers that only care about actionable gates.
pub fn fired_gates(results: Vec<GateResult>) -> Vec<GateResult> {
    results.into_iter().filter(|r| r.fired).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_tracker() -> WorkflowTracker {
        WorkflowTracker::new()
    }

    // ── helpers ──

    fn gates_fired(results: &[GateResult]) -> Vec<&str> {
        results
            .iter()
            .filter(|r| r.fired)
            .map(|r| r.gate_name.as_str())
            .collect()
    }

    fn all_gate_names(results: &[GateResult]) -> Vec<&str> {
        results.iter().map(|r| r.gate_name.as_str()).collect()
    }

    // ── is_code_file ──

    #[test]
    fn is_code_file_returns_true_for_rust_file() {
        assert!(is_code_file("src-tauri/src/main.rs"));
    }

    #[test]
    fn is_code_file_returns_true_for_svelte_file() {
        assert!(is_code_file("ui/lib/components/App.svelte"));
    }

    #[test]
    fn is_code_file_returns_false_for_orqa_relative_path() {
        assert!(!is_code_file(".orqa/governance/rules/RULE-042.md"));
    }

    #[test]
    fn is_code_file_returns_false_for_orqa_absolute_path() {
        assert!(!is_code_file(
            "/home/user/project/.orqa/planning/tasks/TASK-001.md"
        ));
    }

    // ── write event ──

    #[test]
    fn write_event_no_research_no_docs_no_planning_fires_all_three_gates() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(
            fired.contains(&"understand-first"),
            "understand-first should fire"
        );
        assert!(
            fired.contains(&"docs-before-code"),
            "docs-before-code should fire"
        );
        assert!(
            fired.contains(&"plan-before-build"),
            "plan-before-build should fire"
        );
    }

    #[test]
    fn write_event_returns_three_results_for_code_file() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        assert_eq!(results.len(), 3);
        let names = all_gate_names(&results);
        assert!(names.contains(&"understand-first"));
        assert!(names.contains(&"docs-before-code"));
        assert!(names.contains(&"plan-before-build"));
    }

    #[test]
    fn write_event_no_gates_fire_for_orqa_file() {
        let mut t = fresh_tracker();
        let results =
            evaluate_process_gates(&mut t, "write", Some(".orqa/planning/tasks/TASK-001.md"));
        let fired = gates_fired(&results);
        assert!(fired.is_empty(), "no gates should fire for .orqa/ writes");
    }

    #[test]
    fn understand_first_only_fires_once_per_session() {
        let mut t = fresh_tracker();
        // First write — gate fires
        let results1 = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired1 = gates_fired(&results1);
        assert!(fired1.contains(&"understand-first"));

        // Second write — gate must NOT fire again
        let results2 = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/bar.rs"));
        let fired2 = gates_fired(&results2);
        assert!(
            !fired2.contains(&"understand-first"),
            "understand-first must not fire twice"
        );
    }

    #[test]
    fn understand_first_does_not_fire_when_research_done() {
        let mut t = fresh_tracker();
        t.record_search(); // research performed
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"understand-first"));
    }

    #[test]
    fn understand_first_does_not_fire_when_docs_read() {
        let mut t = fresh_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"understand-first"));
    }

    #[test]
    fn docs_before_code_does_not_fire_when_docs_read() {
        let mut t = fresh_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"docs-before-code"));
    }

    #[test]
    fn plan_before_build_does_not_fire_when_planning_read() {
        let mut t = fresh_tracker();
        t.record_read(".orqa/planning/epics/EPIC-042.md");
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"plan-before-build"));
    }

    #[test]
    fn all_write_gates_silent_when_fully_prepared() {
        let mut t = fresh_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        t.record_read(".orqa/planning/epics/EPIC-042.md");
        t.record_search();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(
            fired.is_empty(),
            "no gates should fire when fully prepared: {:?}",
            fired
        );
    }

    #[test]
    fn write_event_with_no_file_path_does_not_fire_write_gates() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", None);
        let fired = gates_fired(&results);
        assert!(fired.is_empty());
    }

    // ── stop event ──

    #[test]
    fn stop_event_returns_two_results() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "stop", None);
        assert_eq!(results.len(), 2);
        let names = all_gate_names(&results);
        assert!(names.contains(&"evidence-before-done"));
        assert!(names.contains(&"learn-after-doing"));
    }

    #[test]
    fn evidence_before_done_fires_when_code_written_no_verification() {
        let mut t = fresh_tracker();
        t.record_write("src-tauri/src/foo.rs");
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn evidence_before_done_does_not_fire_when_no_code_written() {
        let mut t = fresh_tracker();
        // Only wrote to .orqa/ (governance artifact, not code)
        t.record_write(".orqa/governance/rules/RULE-042.md");
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn evidence_before_done_does_not_fire_when_verification_ran() {
        let mut t = fresh_tracker();
        t.record_write("src-tauri/src/foo.rs");
        t.record_command("make check");
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn learn_after_doing_fires_when_more_than_three_code_writes_no_lessons() {
        let mut t = fresh_tracker();
        for i in 0..4 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn learn_after_doing_does_not_fire_when_exactly_three_code_writes() {
        let mut t = fresh_tracker();
        for i in 0..3 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn learn_after_doing_does_not_fire_when_lessons_checked() {
        let mut t = fresh_tracker();
        for i in 0..5 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        t.record_read(".orqa/governance/lessons/IMPL-001.md");
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn all_stop_gates_silent_when_compliant() {
        let mut t = fresh_tracker();
        // No code written, no lessons needed
        let results = evaluate_process_gates(&mut t, "stop", None);
        let fired = gates_fired(&results);
        assert!(fired.is_empty());
    }

    // ── unknown event ──

    #[test]
    fn unknown_event_type_returns_empty_results() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "unknown", None);
        assert!(results.is_empty());
    }

    // ── fired_gates helper ──

    #[test]
    fn fired_gates_filters_to_only_fired() {
        let results = vec![
            GateResult {
                gate_name: "gate-a".to_string(),
                message: "msg a".to_string(),
                fired: true,
            },
            GateResult {
                gate_name: "gate-b".to_string(),
                message: String::new(),
                fired: false,
            },
            GateResult {
                gate_name: "gate-c".to_string(),
                message: "msg c".to_string(),
                fired: true,
            },
        ];
        let fired = fired_gates(results);
        assert_eq!(fired.len(), 2);
        assert_eq!(fired[0].gate_name, "gate-a");
        assert_eq!(fired[1].gate_name, "gate-c");
    }

    // ── gate message content ──

    #[test]
    fn understand_first_message_contains_key_phrase() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let gate = results
            .iter()
            .find(|r| r.gate_name == "understand-first")
            .unwrap();
        assert!(gate.fired);
        assert!(
            gate.message.contains("THINK FIRST"),
            "message: {}",
            gate.message
        );
    }

    #[test]
    fn docs_before_code_message_contains_key_phrase() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let gate = results
            .iter()
            .find(|r| r.gate_name == "docs-before-code")
            .unwrap();
        assert!(gate.fired);
        assert!(
            gate.message.contains("DOCUMENTATION CHECK"),
            "message: {}",
            gate.message
        );
    }

    #[test]
    fn plan_before_build_message_contains_key_phrase() {
        let mut t = fresh_tracker();
        let results = evaluate_process_gates(&mut t, "write", Some("src-tauri/src/foo.rs"));
        let gate = results
            .iter()
            .find(|r| r.gate_name == "plan-before-build")
            .unwrap();
        assert!(gate.fired);
        assert!(
            gate.message.contains("PLANNING CHECK"),
            "message: {}",
            gate.message
        );
    }

    #[test]
    fn evidence_before_done_message_contains_key_phrase() {
        let mut t = fresh_tracker();
        t.record_write("src-tauri/src/foo.rs");
        let results = evaluate_process_gates(&mut t, "stop", None);
        let gate = results
            .iter()
            .find(|r| r.gate_name == "evidence-before-done")
            .unwrap();
        assert!(gate.fired);
        assert!(
            gate.message.contains("VERIFICATION CHECK"),
            "message: {}",
            gate.message
        );
    }

    #[test]
    fn learn_after_doing_message_contains_key_phrase() {
        let mut t = fresh_tracker();
        for i in 0..5 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&mut t, "stop", None);
        let gate = results
            .iter()
            .find(|r| r.gate_name == "learn-after-doing")
            .unwrap();
        assert!(gate.fired);
        assert!(
            gate.message.contains("LEARNING CHECK"),
            "message: {}",
            gate.message
        );
    }
}
