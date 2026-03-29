// Resolved workflow JSON loader for the OrqaStudio Tauri backend.
//
// Reads `process_gates:` sections from `.orqa/workflows/*.resolved.json` and
// converts them into `ProcessGateConfig` values consumed by the session gate
// evaluator. Gate definitions are owned by workflow plugins and expressed in
// the resolved JSON — this loader is the bridge between the JSON on disk and
// the in-memory `Vec<ProcessGateConfig>` held in `AppState`.
//
// The loader searches all resolved workflow files in `.orqa/workflows/` and
// collects every `process_gates:` entry it finds. The first file that declares
// process gates wins; duplicate gate names across files are ignored (first-wins).
// If no resolved workflow file declares any process gates, the function returns
// `None` and the caller falls back to `default_process_gates()`.

use std::collections::HashSet;
use std::path::Path;

use serde::Deserialize;

use crate::domain::process_gates::{GateCondition, GateTrigger, ProcessGateConfig};

/// JSON representation of a single process gate entry.
///
/// Matches the `process_gates:` list items in a resolved workflow JSON file.
#[derive(Debug, Deserialize)]
struct JsonProcessGate {
    /// Machine-readable gate identifier (e.g. `"understand-first"`).
    name: String,
    /// When this gate fires — `"write"` or `"stop"`.
    trigger: String,
    /// The condition that causes this gate to fire.
    ///
    /// Can be either a plain string (e.g. `"code_write_without_docs"`) for
    /// parameterless conditions or an object with a `type` key and optional
    /// parameters (e.g. `{ "type": "significant_work_without_lessons", "threshold": 3 }`).
    condition: serde_json::Value,
    /// Thinking prompt injected into the agent context when the gate fires.
    message: String,
}

/// Minimal JSON structure for a resolved workflow file.
///
/// Only the `process_gates` field is extracted; all other workflow content
/// is ignored by this loader.
#[derive(Debug, Deserialize)]
struct JsonWorkflow {
    #[serde(default)]
    process_gates: Vec<JsonProcessGate>,
}

/// Parse a `GateTrigger` from a JSON string value.
///
/// Returns `None` for unrecognised trigger strings so the loader can skip
/// malformed entries without failing the entire load.
fn parse_trigger(s: &str) -> Option<GateTrigger> {
    match s {
        "write" => Some(GateTrigger::Write),
        "stop" => Some(GateTrigger::Stop),
        other => {
            tracing::warn!("[workflow_loader] unknown gate trigger '{other}', skipping gate");
            None
        }
    }
}

/// Parse a `GateCondition` from a JSON value.
///
/// Accepts either a plain string or an object with a `type` key:
///
/// - `"first_code_write_without_research"` or `{"type": "first_code_write_without_research"}`
/// - `"code_write_without_docs"` or `{"type": "code_write_without_docs"}`
/// - `"code_write_without_planning"` or `{"type": "code_write_without_planning"}`
/// - `"code_written_without_verification"` or `{"type": "code_written_without_verification"}`
/// - `{"type": "significant_work_without_lessons", "threshold": <usize>}`
///
/// Returns `None` for unrecognised conditions so the loader skips malformed entries.
fn parse_condition(value: &serde_json::Value) -> Option<GateCondition> {
    let (type_str, params) = match value {
        serde_json::Value::String(s) => (s.as_str(), None),
        serde_json::Value::Object(map) => {
            let type_val = map.get("type")?;
            let type_str = type_val.as_str()?;
            (type_str, Some(map))
        }
        other => {
            tracing::warn!("[workflow_loader] unexpected condition value type: {other:?}");
            return None;
        }
    };

    match type_str {
        "first_code_write_without_research" => Some(GateCondition::FirstCodeWriteWithoutResearch),
        "code_write_without_docs" => Some(GateCondition::CodeWriteWithoutDocs),
        "code_write_without_planning" => Some(GateCondition::CodeWriteWithoutPlanning),
        "code_written_without_verification" => Some(GateCondition::CodeWrittenWithoutVerification),
        "significant_work_without_lessons" => {
            let threshold = params
                .and_then(|m| m.get("threshold"))
                .and_then(serde_json::Value::as_u64)
                .map_or(3, |n| n as usize);
            Some(GateCondition::SignificantWorkWithoutLessons { threshold })
        }
        other => {
            tracing::warn!("[workflow_loader] unknown gate condition '{other}', skipping gate");
            None
        }
    }
}

/// Convert a `JsonProcessGate` to a `ProcessGateConfig`.
///
/// Returns `None` if the trigger or condition cannot be parsed so the loader
/// can skip individual malformed entries without aborting the full load.
fn json_gate_to_config(json: JsonProcessGate) -> Option<ProcessGateConfig> {
    let trigger = parse_trigger(&json.trigger)?;
    let condition = parse_condition(&json.condition)?;
    // Collapse multi-line message strings to a single space-separated line.
    let message = json.message.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(ProcessGateConfig::new(json.name, trigger, condition, message))
}

/// Collect process gates from a single resolved workflow file into `gates`.
///
/// Skips gates whose names are already in `seen_names`. Adds new gate names
/// to `seen_names` as they are inserted. Failures reading or parsing the file
/// are logged as warnings; malformed individual gate entries are skipped.
fn collect_gates_from_file(
    path: &Path,
    seen_names: &mut HashSet<String>,
    gates: &mut Vec<ProcessGateConfig>,
) {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            tracing::warn!("[workflow_loader] could not read '{}': {err}", path.display());
            return;
        }
    };

    let workflow: JsonWorkflow = match serde_json::from_str(&contents) {
        Ok(w) => w,
        Err(err) => {
            tracing::warn!("[workflow_loader] failed to parse '{}': {err}", path.display());
            return;
        }
    };

    if workflow.process_gates.is_empty() {
        return;
    }

    tracing::debug!(
        "[workflow_loader] found {} process gate(s) in '{}'",
        workflow.process_gates.len(),
        path.display()
    );

    for json_gate in workflow.process_gates {
        let name = json_gate.name.clone();
        if seen_names.contains(&name) {
            tracing::debug!(
                "[workflow_loader] duplicate gate '{name}' in '{}', skipping",
                path.display()
            );
            continue;
        }
        if let Some(config) = json_gate_to_config(json_gate) {
            seen_names.insert(name);
            gates.push(config);
        }
    }
}

/// Load process gate definitions from resolved workflow JSON files.
///
/// Scans `.orqa/workflows/*.resolved.json` in `project_root` and collects
/// all `process_gates:` entries. Gate names are deduplicated — the first
/// occurrence wins. Returns `None` when no resolved workflow files exist or
/// none of them declare any process gates. The caller falls back to
/// `default_process_gates()` on `None`.
///
/// Failures (missing directory, unreadable files, parse errors) are logged as
/// warnings and do not propagate — a broken workflow file must not prevent the
/// project from opening.
pub fn load_process_gates_from_workflows(project_root: &Path) -> Option<Vec<ProcessGateConfig>> {
    let workflows_dir = project_root.join(".orqa").join("workflows");
    if !workflows_dir.exists() {
        tracing::debug!(
            "[workflow_loader] no workflows directory at '{}'",
            workflows_dir.display()
        );
        return None;
    }

    let entries = match std::fs::read_dir(&workflows_dir) {
        Ok(e) => e,
        Err(err) => {
            tracing::warn!(
                "[workflow_loader] failed to read workflows directory '{}': {err}",
                workflows_dir.display()
            );
            return None;
        }
    };

    let mut seen_names: HashSet<String> = HashSet::new();
    let mut gates: Vec<ProcessGateConfig> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let is_resolved = path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(".resolved.json"));
        if !is_resolved {
            continue;
        }
        collect_gates_from_file(&path, &mut seen_names, &mut gates);
    }

    if gates.is_empty() {
        None
    } else {
        tracing::debug!(
            "[workflow_loader] loaded {} process gate(s) from resolved workflows",
            gates.len()
        );
        Some(gates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;
    use tempfile::TempDir;

    fn write_resolved_json(dir: &Path, filename: &str, content: &str) {
        let path = dir.join(filename);
        let mut file = std::fs::File::create(&path).expect("create file");
        file.write_all(content.as_bytes()).expect("write file");
    }

    fn setup_workflows_dir(tmp: &TempDir) -> std::path::PathBuf {
        let workflows = tmp.path().join(".orqa").join("workflows");
        std::fs::create_dir_all(&workflows).expect("create workflows dir");
        workflows
    }

    // ── no workflows directory ──

    #[test]
    fn returns_none_when_no_workflows_directory() {
        let tmp = TempDir::new().unwrap();
        let result = load_process_gates_from_workflows(tmp.path());
        assert!(result.is_none());
    }

    // ── empty or no process_gates ──

    #[test]
    fn returns_none_when_no_resolved_json_files() {
        let tmp = TempDir::new().unwrap();
        setup_workflows_dir(&tmp);
        let result = load_process_gates_from_workflows(tmp.path());
        assert!(result.is_none());
    }

    #[test]
    fn returns_none_when_resolved_json_has_no_process_gates() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "task.resolved.json",
            r#"{"name": "task", "states": {"open": {"category": "active"}}}"#,
        );
        let result = load_process_gates_from_workflows(tmp.path());
        assert!(result.is_none());
    }

    // ── successful loads ──

    #[test]
    fn loads_write_trigger_string_condition() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "docs-before-code", "trigger": "write", "condition": "code_write_without_docs", "message": "Check docs first."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "docs-before-code");
        assert!(matches!(gates[0].trigger, GateTrigger::Write));
        assert!(matches!(gates[0].condition, GateCondition::CodeWriteWithoutDocs));
        assert_eq!(gates[0].message, "Check docs first.");
    }

    #[test]
    fn loads_stop_trigger_string_condition() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "evidence-before-done", "trigger": "stop", "condition": "code_written_without_verification", "message": "Run tests."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].trigger, GateTrigger::Stop));
        assert!(matches!(gates[0].condition, GateCondition::CodeWrittenWithoutVerification));
    }

    #[test]
    fn loads_significant_work_condition_with_threshold() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "learn-after-doing", "trigger": "stop", "condition": {"type": "significant_work_without_lessons", "threshold": 5}, "message": "Record lessons."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1);
        match &gates[0].condition {
            GateCondition::SignificantWorkWithoutLessons { threshold } => {
                assert_eq!(*threshold, 5);
            }
            other => panic!("expected SignificantWorkWithoutLessons, got: {other:?}"),
        }
    }

    #[test]
    fn loads_significant_work_condition_default_threshold_when_absent() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "learn-after-doing", "trigger": "stop", "condition": {"type": "significant_work_without_lessons"}, "message": "Record lessons."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        match &gates[0].condition {
            GateCondition::SignificantWorkWithoutLessons { threshold } => {
                assert_eq!(*threshold, 3, "default threshold should be 3");
            }
            other => panic!("expected SignificantWorkWithoutLessons, got: {other:?}"),
        }
    }

    #[test]
    fn deduplicates_gate_names_across_files() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        // Write two files with the same gate name — first wins.
        write_resolved_json(
            &workflows,
            "a.resolved.json",
            r#"{"name": "a", "process_gates": [{"name": "my-gate", "trigger": "write", "condition": "code_write_without_docs", "message": "From file A."}]}"#,
        );
        write_resolved_json(
            &workflows,
            "b.resolved.json",
            r#"{"name": "b", "process_gates": [{"name": "my-gate", "trigger": "stop", "condition": "code_written_without_verification", "message": "From file B."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1, "duplicate gate name should be deduplicated");
    }

    #[test]
    fn skips_unknown_trigger() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "bad-gate", "trigger": "unknown_trigger", "condition": "code_write_without_docs", "message": "Never fires."}, {"name": "good-gate", "trigger": "write", "condition": "code_write_without_docs", "message": "Fires."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "good-gate");
    }

    #[test]
    fn skips_unknown_condition() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{"name": "test", "process_gates": [{"name": "bad-gate", "trigger": "write", "condition": "unknown_condition_type", "message": "Never fires."}, {"name": "good-gate", "trigger": "write", "condition": "code_write_without_docs", "message": "Fires."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load gates");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "good-gate");
    }

    #[test]
    fn skips_malformed_json_file_and_loads_valid_one() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "bad.resolved.json",
            "{ not valid json",
        );
        write_resolved_json(
            &workflows,
            "good.resolved.json",
            r#"{"name": "good", "process_gates": [{"name": "my-gate", "trigger": "write", "condition": "code_write_without_docs", "message": "Check docs."}]}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load from good file");
        assert_eq!(gates.len(), 1);
    }

    #[test]
    fn non_resolved_json_files_are_ignored() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "task.workflow.json",
            r#"{"name": "task", "process_gates": [{"name": "my-gate", "trigger": "write", "condition": "code_write_without_docs", "message": "Should not load."}]}"#,
        );
        let result = load_process_gates_from_workflows(tmp.path());
        assert!(result.is_none(), "non-resolved JSON files should be ignored");
    }

    #[test]
    fn message_whitespace_is_collapsed() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            "{\"name\": \"test\", \"process_gates\": [{\"name\": \"my-gate\", \"trigger\": \"write\", \"condition\": \"code_write_without_docs\", \"message\": \"Line one.\\n      Line two.\"}]}",
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load");
        assert_eq!(gates[0].message, "Line one. Line two.");
    }

    // ── all five standard gates ──

    #[test]
    fn loads_all_five_standard_gate_conditions() {
        let tmp = TempDir::new().unwrap();
        let workflows = setup_workflows_dir(&tmp);
        write_resolved_json(
            &workflows,
            "workflow.resolved.json",
            r#"{
  "name": "test",
  "process_gates": [
    {"name": "understand-first", "trigger": "write", "condition": "first_code_write_without_research", "message": "Think first."},
    {"name": "docs-before-code", "trigger": "write", "condition": "code_write_without_docs", "message": "Read docs."},
    {"name": "plan-before-build", "trigger": "write", "condition": "code_write_without_planning", "message": "Check plan."},
    {"name": "evidence-before-done", "trigger": "stop", "condition": "code_written_without_verification", "message": "Run tests."},
    {"name": "learn-after-doing", "trigger": "stop", "condition": {"type": "significant_work_without_lessons", "threshold": 3}, "message": "Record lessons."}
  ]
}"#,
        );
        let gates = load_process_gates_from_workflows(tmp.path()).expect("should load all 5 gates");
        assert_eq!(gates.len(), 5);

        let names: Vec<&str> = gates.iter().map(|g| g.name.as_str()).collect();
        assert!(names.contains(&"understand-first"));
        assert!(names.contains(&"docs-before-code"));
        assert!(names.contains(&"plan-before-build"));
        assert!(names.contains(&"evidence-before-done"));
        assert!(names.contains(&"learn-after-doing"));
    }
}
