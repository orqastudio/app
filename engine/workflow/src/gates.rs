//! Process gate evaluation for the OrqaStudio workflow engine.
//!
//! Evaluates process gates against the current session workflow state and returns
//! gate results. Gates inject thinking prompts into the agent context to guide agents
//! back toward correct process when they skip steps like research, documentation review,
//! or verification. Gates are enforced as warnings — they guide but do not block.
//!
//! Gate definitions are loaded from plugin-declared workflow config via `ProcessGateConfig`.
//! No gate names, conditions, or messages are hardcoded in this module.
//!
//! Two gate trigger types are supported:
//!   - `write` — evaluated once per file write event
//!   - `stop`  — evaluated once per turn-complete event

use orqa_engine_types::types::enforcement::{RuleAction, Verdict};
use orqa_engine_types::types::workflow::GateResult;
use crate::tracker::WorkflowTracker;

/// When a process gate fires relative to agent activity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateTrigger {
    /// Fires when a file is written.
    Write,
    /// Fires at turn end (stop event).
    Stop,
}

/// The condition that determines whether a process gate fires.
///
/// Built-in variants encode well-known session-observable facts. The `Custom`
/// variant allows plugin-declared gates to name a condition string without
/// requiring changes to Rust source. Unknown condition strings log a warning
/// and return `GateResult::Pass` (open) so that new plugin conditions degrade
/// gracefully rather than blocking agent work.
#[derive(Debug, Clone)]
pub enum GateCondition {
    /// Fires on first code write when no research has been done.
    /// Only fires once per session.
    FirstCodeWriteWithoutResearch,
    /// Fires on any code write when no docs have been consulted.
    CodeWriteWithoutDocs,
    /// Fires on any code write when no planning artifacts have been consulted.
    CodeWriteWithoutPlanning,
    /// Fires at stop when code was written but no verification command was run.
    CodeWrittenWithoutVerification,
    /// Fires at stop when more than `threshold` code writes occurred but lessons were not checked.
    SignificantWorkWithoutLessons {
        /// Minimum number of code writes that must have occurred before this gate fires.
        threshold: usize,
    },
    /// Plugin-declared condition identified by a string.
    ///
    /// Unknown condition IDs are not evaluated against session state. The engine
    /// logs a warning and returns a non-fired (pass) result, allowing plugins to
    /// declare forward-compatible condition types without blocking agent work.
    Custom(String),
}

/// Definition of a single process gate loaded from plugin workflow config.
///
/// Each gate has a name, trigger type, condition, and the thinking prompt to inject
/// when the gate fires. The engine evaluates all registered gates against the current
/// session state; gates that fire have their `message` injected into the agent context.
#[derive(Debug, Clone)]
pub struct ProcessGateConfig {
    /// Machine-readable gate identifier (e.g. `"understand-first"`).
    pub name: String,
    /// When this gate is evaluated — on file write or at turn end.
    pub trigger: GateTrigger,
    /// The condition that causes this gate to fire.
    pub condition: GateCondition,
    /// Thinking prompt injected into the agent context when the gate fires.
    pub message: String,
}

impl ProcessGateConfig {
    /// Create a new gate config entry.
    pub fn new(
        name: impl Into<String>,
        trigger: GateTrigger,
        condition: GateCondition,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            trigger,
            condition,
            message: message.into(),
        }
    }
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
        gate_name: name.to_owned(),
        message: if fired {
            message.to_owned()
        } else {
            String::new()
        },
        fired,
    }
}

/// Evaluate a single gate config entry against the current session state.
///
/// Returns a `GateResult` indicating whether the gate fired and, if so, the
/// thinking prompt to inject.
fn evaluate_gate(
    config: &ProcessGateConfig,
    tracker: &mut WorkflowTracker,
    trigger: GateTrigger,
    file_path: Option<&str>,
) -> Option<GateResult> {
    if config.trigger != trigger {
        return None;
    }

    let writing_code = file_path.is_some_and(is_code_file);

    let fired = match &config.condition {
        GateCondition::FirstCodeWriteWithoutResearch => {
            let fires = writing_code
                && !tracker.has_done_any_research()
                && !tracker.first_code_write_gated;
            if fires {
                tracker.first_code_write_gated = true;
            }
            fires
        }
        GateCondition::CodeWriteWithoutDocs => {
            writing_code && !tracker.has_read_any_docs()
        }
        GateCondition::CodeWriteWithoutPlanning => {
            writing_code && !tracker.has_read_any_planning()
        }
        GateCondition::CodeWrittenWithoutVerification => {
            tracker.has_written_code() && !tracker.has_run_verification()
        }
        GateCondition::SignificantWorkWithoutLessons { threshold } => {
            tracker.code_write_count() > *threshold && !tracker.has_checked_lessons()
        }
        GateCondition::Custom(condition_id) => {
            // Unknown plugin-declared conditions are not evaluated against session state.
            // Log a warning and return pass (not fired) so new condition types from plugins
            // degrade gracefully without blocking agent work.
            tracing::warn!(
                "[process_gates] gate '{}': unknown condition '{condition_id}', treating as pass",
                config.name
            );
            false
        }
    };

    Some(gate(&config.name, fired, &config.message))
}

/// Evaluate all registered process gates against the current session workflow state.
///
/// # Parameters
///
/// - `gates`: The gate definitions loaded from plugin workflow config.
/// - `tracker`: The current session's `WorkflowTracker`.
/// - `trigger`: The event that occurred — `GateTrigger::Write` or `GateTrigger::Stop`.
/// - `file_path`: The path of the file being written, when `trigger == GateTrigger::Write`.
///   `None` otherwise.
///
/// # Returns
///
/// A `Vec<GateResult>` of all evaluated gates. Each entry has `fired: true` when
/// the gate's condition was met and a thinking prompt should be injected. Gates
/// with `fired: false` are returned for observability but require no action.
pub fn evaluate_process_gates(
    gates: &[ProcessGateConfig],
    tracker: &mut WorkflowTracker,
    trigger: GateTrigger,
    file_path: Option<&str>,
) -> Vec<GateResult> {
    gates
        .iter()
        .filter_map(|g| evaluate_gate(g, tracker, trigger.clone(), file_path))
        .collect()
}

/// Return only the gates that fired from a full evaluation result.
///
/// Convenience helper for callers that only care about actionable gates.
pub fn fired_gates(results: Vec<GateResult>) -> Vec<GateResult> {
    results.into_iter().filter(|r| r.fired).collect()
}

/// Convert a fired `GateResult` into a `Verdict` with `RuleAction::Warn`.
///
/// Process gates are surfaced as warnings — they guide the agent toward
/// correct process but do not block tool execution.
fn gate_as_verdict(gate: GateResult) -> Verdict {
    Verdict {
        rule_name: gate.gate_name,
        action: RuleAction::Warn,
        message: gate.message,
        knowledge: Vec::new(),
    }
}

/// Evaluate write-event process gates and return only fired ones as `Verdict`s.
///
/// This is the enforcement-compatible output path. Callers that need to merge
/// gate results with `EnforcementEngine` verdicts use this instead of
/// `evaluate_process_gates`.
pub fn evaluate_write_verdicts(
    gates: &[ProcessGateConfig],
    tracker: &mut WorkflowTracker,
    file_path: &str,
) -> Vec<Verdict> {
    evaluate_process_gates(gates, tracker, GateTrigger::Write, Some(file_path))
        .into_iter()
        .filter(|r| r.fired)
        .map(gate_as_verdict)
        .collect()
}

/// Evaluate stop-event process gates and return only fired ones as `Verdict`s.
///
/// This is the enforcement-compatible output path for turn-complete events.
pub fn evaluate_stop_verdicts(
    gates: &[ProcessGateConfig],
    tracker: &WorkflowTracker,
) -> Vec<Verdict> {
    // Stop gates don't mutate the tracker so we use a shared ref.
    // Re-evaluate all stop-triggered gates directly without the mutable tracker path.
    gates
        .iter()
        .filter(|g| g.trigger == GateTrigger::Stop)
        .filter_map(|g| {
            let fired = match &g.condition {
                GateCondition::CodeWrittenWithoutVerification => {
                    tracker.has_written_code() && !tracker.has_run_verification()
                }
                GateCondition::SignificantWorkWithoutLessons { threshold } => {
                    tracker.code_write_count() > *threshold && !tracker.has_checked_lessons()
                }
                // Write-only conditions never fire at stop
                GateCondition::FirstCodeWriteWithoutResearch
                | GateCondition::CodeWriteWithoutDocs
                | GateCondition::CodeWriteWithoutPlanning => false,
                // Unknown plugin-declared conditions: log warning and treat as pass
                GateCondition::Custom(condition_id) => {
                    tracing::warn!(
                        "[process_gates] gate '{}': unknown condition '{condition_id}', treating as pass",
                        g.name
                    );
                    false
                }
            };
            if fired {
                Some(gate_as_verdict(gate(&g.name, true, &g.message)))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracker::{PathCategory, PathRule, TrackerConfig, WorkflowTracker};

    /// Build the standard set of process gates matching the original hardcoded gates.
    fn standard_gates() -> Vec<ProcessGateConfig> {
        vec![
            ProcessGateConfig::new(
                "understand-first",
                GateTrigger::Write,
                GateCondition::FirstCodeWriteWithoutResearch,
                "THINK FIRST: What is the system you're modifying? \
                 What are its boundaries? What depends on this? What could break? \
                 Read the governing docs and understand the context before writing code.",
            ),
            ProcessGateConfig::new(
                "docs-before-code",
                GateTrigger::Write,
                GateCondition::CodeWriteWithoutDocs,
                "DOCUMENTATION CHECK: Have you read the documentation that defines \
                 this area? Check .orqa/documentation/ for specs, patterns, and constraints \
                 before implementing.",
            ),
            ProcessGateConfig::new(
                "plan-before-build",
                GateTrigger::Write,
                GateCondition::CodeWriteWithoutPlanning,
                "PLANNING CHECK: Is there an epic or task that defines this work? \
                 Check .orqa/implementation/ for the scope, acceptance criteria, and \
                 implementation design.",
            ),
            ProcessGateConfig::new(
                "evidence-before-done",
                GateTrigger::Stop,
                GateCondition::CodeWrittenWithoutVerification,
                "VERIFICATION CHECK: You wrote code but didn't run make check \
                 or make test. Show evidence that the work is correct before completing.",
            ),
            ProcessGateConfig::new(
                "learn-after-doing",
                GateTrigger::Stop,
                GateCondition::SignificantWorkWithoutLessons { threshold: 3 },
                "LEARNING CHECK: Significant work was done this session. \
                 Check .orqa/learning/lessons/ for known patterns and consider \
                 if anything unexpected should be recorded.",
            ),
        ]
    }

    fn standard_tracker() -> WorkflowTracker {
        WorkflowTracker::new(TrackerConfig::new(vec![
            PathRule::new(".orqa/documentation/", PathCategory::Docs),
            PathRule::new(".orqa/implementation/", PathCategory::Planning),
            PathRule::new(".orqa/learning/lessons/", PathCategory::Lessons),
        ]))
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
        assert!(!is_code_file(".orqa/learning/rules/RULE-042.md"));
    }

    #[test]
    fn is_code_file_returns_false_for_orqa_absolute_path() {
        assert!(!is_code_file(
            "/home/user/project/.orqa/implementation/tasks/TASK-001.md"
        ));
    }

    // ── write event ──

    #[test]
    fn write_event_no_research_no_docs_no_planning_fires_all_three_gates() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
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
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        // Only write-triggered gates are returned (3 of 5).
        assert_eq!(results.len(), 3);
        let names = all_gate_names(&results);
        assert!(names.contains(&"understand-first"));
        assert!(names.contains(&"docs-before-code"));
        assert!(names.contains(&"plan-before-build"));
    }

    #[test]
    fn write_event_no_gates_fire_for_orqa_file() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(
            &gates,
            &mut t,
            GateTrigger::Write,
            Some(".orqa/implementation/tasks/TASK-001.md"),
        );
        let fired = gates_fired(&results);
        assert!(fired.is_empty(), "no gates should fire for .orqa/ writes");
    }

    #[test]
    fn understand_first_only_fires_once_per_session() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        // First write — gate fires
        let results1 = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired1 = gates_fired(&results1);
        assert!(fired1.contains(&"understand-first"));

        // Second write — gate must NOT fire again
        let results2 = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/bar.rs"));
        let fired2 = gates_fired(&results2);
        assert!(
            !fired2.contains(&"understand-first"),
            "understand-first must not fire twice"
        );
    }

    #[test]
    fn understand_first_does_not_fire_when_research_done() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_search();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"understand-first"));
    }

    #[test]
    fn understand_first_does_not_fire_when_docs_read() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"understand-first"));
    }

    #[test]
    fn docs_before_code_does_not_fire_when_docs_read() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"docs-before-code"));
    }

    #[test]
    fn plan_before_build_does_not_fire_when_planning_read() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_read(".orqa/implementation/epics/EPIC-042.md");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"plan-before-build"));
    }

    #[test]
    fn all_write_gates_silent_when_fully_prepared() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_read(".orqa/documentation/architecture/overview.md");
        t.record_read(".orqa/implementation/epics/EPIC-042.md");
        t.record_search();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let fired = gates_fired(&results);
        assert!(
            fired.is_empty(),
            "no gates should fire when fully prepared: {:?}",
            fired
        );
    }

    #[test]
    fn write_event_with_no_file_path_does_not_fire_write_gates() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, None);
        let fired = gates_fired(&results);
        assert!(fired.is_empty());
    }

    // ── stop event ──

    #[test]
    fn stop_event_returns_two_results() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        assert_eq!(results.len(), 2);
        let names = all_gate_names(&results);
        assert!(names.contains(&"evidence-before-done"));
        assert!(names.contains(&"learn-after-doing"));
    }

    #[test]
    fn evidence_before_done_fires_when_code_written_no_verification() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_write("src-tauri/src/foo.rs");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn evidence_before_done_does_not_fire_when_no_code_written() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_write(".orqa/learning/rules/RULE-042.md");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn evidence_before_done_does_not_fire_when_verification_ran() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_write("src-tauri/src/foo.rs");
        t.record_command("make check");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"evidence-before-done"));
    }

    #[test]
    fn learn_after_doing_fires_when_more_than_three_code_writes_no_lessons() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        for i in 0..4 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn learn_after_doing_does_not_fire_when_exactly_three_code_writes() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        for i in 0..3 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn learn_after_doing_does_not_fire_when_lessons_checked() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        for i in 0..5 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        t.record_read(".orqa/learning/lessons/IMPL-001.md");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(!fired.contains(&"learn-after-doing"));
    }

    #[test]
    fn all_stop_gates_silent_when_compliant() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let fired = gates_fired(&results);
        assert!(fired.is_empty());
    }

    // ── trigger filtering ──

    #[test]
    fn stop_trigger_returns_no_write_gate_results() {
        // Stop trigger evaluates only stop-triggered gates; write-only gates produce no results.
        // (Invalid trigger types are now impossible: the type system enforces this.)
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let names = all_gate_names(&results);
        assert!(!names.contains(&"understand-first"));
        assert!(!names.contains(&"docs-before-code"));
        assert!(!names.contains(&"plan-before-build"));
        assert_eq!(results.len(), 2, "only stop-triggered gates evaluated");
    }

    // ── fired_gates helper ──

    #[test]
    fn fired_gates_filters_to_only_fired() {
        let results = vec![
            GateResult {
                gate_name: "gate-a".to_owned(),
                message: "msg a".to_owned(),
                fired: true,
            },
            GateResult {
                gate_name: "gate-b".to_owned(),
                message: String::new(),
                fired: false,
            },
            GateResult {
                gate_name: "gate-c".to_owned(),
                message: "msg c".to_owned(),
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
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let g = results
            .iter()
            .find(|r| r.gate_name == "understand-first")
            .unwrap();
        assert!(g.fired);
        assert!(g.message.contains("THINK FIRST"), "message: {}", g.message);
    }

    #[test]
    fn docs_before_code_message_contains_key_phrase() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let g = results
            .iter()
            .find(|r| r.gate_name == "docs-before-code")
            .unwrap();
        assert!(g.fired);
        assert!(
            g.message.contains("DOCUMENTATION CHECK"),
            "message: {}",
            g.message
        );
    }

    #[test]
    fn plan_before_build_message_contains_key_phrase() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src-tauri/src/foo.rs"));
        let g = results
            .iter()
            .find(|r| r.gate_name == "plan-before-build")
            .unwrap();
        assert!(g.fired);
        assert!(
            g.message.contains("PLANNING CHECK"),
            "message: {}",
            g.message
        );
    }

    #[test]
    fn evidence_before_done_message_contains_key_phrase() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        t.record_write("src-tauri/src/foo.rs");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let g = results
            .iter()
            .find(|r| r.gate_name == "evidence-before-done")
            .unwrap();
        assert!(g.fired);
        assert!(
            g.message.contains("VERIFICATION CHECK"),
            "message: {}",
            g.message
        );
    }

    #[test]
    fn learn_after_doing_message_contains_key_phrase() {
        let gates = standard_gates();
        let mut t = standard_tracker();
        for i in 0..5 {
            t.record_write(&format!("src-tauri/src/file{i}.rs"));
        }
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        let g = results
            .iter()
            .find(|r| r.gate_name == "learn-after-doing")
            .unwrap();
        assert!(g.fired);
        assert!(
            g.message.contains("LEARNING CHECK"),
            "message: {}",
            g.message
        );
    }

    // ── custom gates config ──

    #[test]
    fn custom_gate_names_and_conditions_work() {
        let gates = vec![
            ProcessGateConfig::new(
                "my-custom-gate",
                GateTrigger::Stop,
                GateCondition::CodeWrittenWithoutVerification,
                "CUSTOM: run your tests!",
            ),
        ];
        let mut t = standard_tracker();
        t.record_write("src/lib.rs");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].gate_name, "my-custom-gate");
        assert!(results[0].fired);
        assert!(results[0].message.contains("CUSTOM"));
    }

    #[test]
    fn empty_gates_config_produces_no_results() {
        let gates: Vec<ProcessGateConfig> = vec![];
        let mut t = standard_tracker();
        let write_results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src/lib.rs"));
        let stop_results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        assert!(write_results.is_empty());
        assert!(stop_results.is_empty());
    }

    // ── Custom condition ──

    #[test]
    fn custom_condition_at_write_event_does_not_fire() {
        // Unknown plugin-declared conditions degrade gracefully — gate does not fire.
        let gates = vec![ProcessGateConfig::new(
            "plugin-custom-gate",
            GateTrigger::Write,
            GateCondition::Custom("some-plugin-condition".to_owned()),
            "PLUGIN: custom message",
        )];
        let mut t = standard_tracker();
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Write, Some("src/lib.rs"));
        assert_eq!(results.len(), 1);
        assert!(!results[0].fired, "custom condition must not fire (pass)");
    }

    #[test]
    fn custom_condition_at_stop_event_does_not_fire() {
        // Unknown plugin-declared conditions at stop event also degrade gracefully.
        let gates = vec![ProcessGateConfig::new(
            "plugin-stop-gate",
            GateTrigger::Stop,
            GateCondition::Custom("another-plugin-condition".to_owned()),
            "PLUGIN: stop message",
        )];
        let mut t = standard_tracker();
        t.record_write("src/lib.rs");
        let results = evaluate_process_gates(&gates, &mut t, GateTrigger::Stop, None);
        assert_eq!(results.len(), 1);
        assert!(!results[0].fired, "custom condition must not fire (pass)");
    }

    #[test]
    fn custom_condition_in_evaluate_stop_verdicts_returns_no_verdicts() {
        let gates = vec![ProcessGateConfig::new(
            "plugin-stop-gate",
            GateTrigger::Stop,
            GateCondition::Custom("plugin-defined-condition".to_owned()),
            "PLUGIN: stop message",
        )];
        let mut t = standard_tracker();
        t.record_write("src/lib.rs");
        let verdicts = evaluate_stop_verdicts(&gates, &t);
        assert!(verdicts.is_empty(), "custom condition must produce no verdicts");
    }
}
