// Session start endpoint for the OrqaStudio daemon.
//
// Absorbs the connector's session-start.sh checks into a structured HTTP
// endpoint so the connector can delegate startup validation to the daemon
// rather than reimplementing them in shell. Returns a typed response that
// the connector can format without parsing strings.
//
// Route: POST /session-start
// Body: { "project_path": "/abs/path/to/project" }

use std::path::Path;
use std::process::Command;

use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Request body for POST /session-start.
#[derive(Deserialize)]
pub struct SessionStartRequest {
    /// Absolute path to the OrqaStudio project directory.
    pub project_path: String,
}

/// The result of an individual startup check.
#[derive(Serialize)]
pub struct CheckResult {
    /// Short identifier for this check (e.g. "installation", "git_state").
    pub name: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Human-readable detail — explains what failed or confirms what passed.
    pub message: String,
}

/// Response body for POST /session-start.
#[derive(Serialize)]
pub struct SessionStartResponse {
    /// Results of each named startup check.
    pub checks: Vec<CheckResult>,
    /// Actionable warnings (git stashes, uncommitted files, etc.).
    pub warnings: Vec<String>,
    /// Contents of .state/session-state.md, if the file exists.
    pub session_state: Option<String>,
    /// Contents of .state/migration-context.md, if the file exists.
    pub migration_context: Option<String>,
    /// Contents of .state/governance-context.md, if the file exists.
    pub governance_context: Option<String>,
    /// Ordered checklist items for the agent to follow at session start.
    pub checklist: Vec<String>,
}

/// Verify that the required connector installation artefacts are present.
///
/// Checks for .claude/agents, .claude/rules, and .claude/CLAUDE.md. Reports
/// each missing item individually so the connector can surface precise guidance.
fn check_installation(project_path: &Path) -> CheckResult {
    let claude_dir = project_path.join(".claude");
    let mut missing: Vec<&str> = Vec::new();

    if !claude_dir.join("agents").exists() {
        missing.push(".claude/agents");
    }
    if !claude_dir.join("rules").exists() {
        missing.push(".claude/rules");
    }
    if !claude_dir.join("CLAUDE.md").exists() {
        missing.push(".claude/CLAUDE.md");
    }

    if missing.is_empty() {
        CheckResult {
            name: "installation".into(),
            passed: true,
            message: "connector installation verified".into(),
        }
    } else {
        CheckResult {
            name: "installation".into(),
            passed: false,
            message: format!(
                "missing: {} — run orqa install",
                missing.join(", ")
            ),
        }
    }
}

/// Report the daemon health check.
///
/// The daemon is always healthy from its own perspective — if the endpoint is
/// responding, the daemon is running. This check exists for completeness in the
/// structured response so callers get a uniform check list.
fn check_daemon_health() -> CheckResult {
    CheckResult {
        name: "daemon".into(),
        passed: true,
        message: "daemon is running".into(),
    }
}

/// Run a basic graph integrity check by scanning .orqa/ for malformed YAML.
///
/// Validates that JSON artifact files in .orqa/ are parseable. A full
/// `orqa enforce --fix` requires the CLI binary; this lighter check catches
/// obviously broken artifacts without a subprocess dependency.
fn check_graph_integrity(project_path: &Path) -> CheckResult {
    let orqa_dir = project_path.join(".orqa");
    if !orqa_dir.exists() {
        return CheckResult {
            name: "graph_integrity".into(),
            passed: false,
            message: ".orqa/ directory not found".into(),
        };
    }

    // Walk .orqa/ and attempt to parse every .json file.
    let mut errors: Vec<String> = Vec::new();
    if let Ok(entries) = walk_json_files(&orqa_dir) {
        for path in entries {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if serde_json::from_str::<serde_json::Value>(&contents).is_err() {
                    errors.push(format!(
                        "malformed JSON: {}",
                        path.strip_prefix(project_path)
                            .unwrap_or(&path)
                            .display()
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        CheckResult {
            name: "graph_integrity".into(),
            passed: true,
            message: "graph artifacts are well-formed".into(),
        }
    } else {
        CheckResult {
            name: "graph_integrity".into(),
            passed: false,
            message: errors.join("; "),
        }
    }
}

/// Collect all .json file paths under a directory tree.
///
/// Returns an empty Vec rather than an error if the directory is unreadable,
/// so the integrity check degrades gracefully.
fn walk_json_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut results = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Ok(mut sub) = walk_json_files(&path) {
                results.append(&mut sub);
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            results.push(path);
        }
    }
    Ok(results)
}

/// Append a warning for each git stash found in the project.
///
/// Runs `git stash list` and pushes a summary into `warnings` if any stashes
/// exist. Silent on failure — a missing git binary or non-git directory is not
/// a hard error for the daemon.
fn check_git_stashes(project_path: &Path, warnings: &mut Vec<String>) {
    let stash_output = Command::new("git")
        .args(["stash", "list"])
        .current_dir(project_path)
        .output();

    match stash_output {
        Ok(out) if out.status.success() => {
            let stash_text = String::from_utf8_lossy(&out.stdout);
            let stash_text = stash_text.trim();
            if !stash_text.is_empty() {
                warnings.push(format!("Git stashes found:\n{stash_text}"));
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            warn!(subsystem = "session_start", "git stash list failed: {stderr}");
        }
        Err(e) => {
            warn!(subsystem = "session_start", "could not run git stash list: {e}");
        }
    }
}

/// Append a warning when there are uncommitted changes on the main branch.
///
/// Runs `git branch --show-current` to detect whether the working directory
/// is on main, then `git status --porcelain` to count modified files. Silent
/// on failure — a missing git binary or non-git directory is not a hard error.
fn check_uncommitted_on_main(project_path: &Path, warnings: &mut Vec<String>) {
    let branch_output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(project_path)
        .output();

    let on_main = branch_output
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .is_some_and(|s| s.trim() == "main");

    if !on_main {
        return;
    }

    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_path)
        .output();

    match status_output {
        Ok(out) if out.status.success() => {
            let lines: Vec<&str> = std::str::from_utf8(&out.stdout)
                .unwrap_or("")
                .lines()
                .filter(|l| !l.trim().is_empty())
                .collect();
            if !lines.is_empty() {
                warnings.push(format!("{} uncommitted file(s) on main", lines.len()));
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            warn!(subsystem = "session_start", "git status failed: {stderr}");
        }
        Err(e) => {
            warn!(subsystem = "session_start", "could not run git status: {e}");
        }
    }
}

/// Check git state: stash list and uncommitted changes on main.
///
/// Delegates to `check_git_stashes` and `check_uncommitted_on_main`, then
/// returns a CheckResult summarising any warnings found.
fn check_git_state(project_path: &Path, warnings: &mut Vec<String>) -> CheckResult {
    check_git_stashes(project_path, warnings);
    check_uncommitted_on_main(project_path, warnings);

    CheckResult {
        name: "git_state".into(),
        passed: true,
        message: if warnings.is_empty() {
            "git state clean".into()
        } else {
            format!("{} warning(s) — see warnings field", warnings.len())
        },
    }
}

/// Read a state file and return its contents, or None if the file is absent.
///
/// Logs a warning and returns None rather than propagating errors so that a
/// missing or unreadable state file does not block the session start response.
fn read_state_file(project_path: &Path, relative: &str) -> Option<String> {
    let path = project_path.join(relative);
    if !path.exists() {
        return None;
    }
    match std::fs::read_to_string(&path) {
        Ok(contents) => Some(contents),
        Err(e) => {
            warn!(
                subsystem = "session_start",
                file = %path.display(),
                "could not read state file: {e}"
            );
            None
        }
    }
}

/// Detect whether dogfood mode is active from .orqa/project.json.
///
/// Dogfood mode means the project being edited IS OrqaStudio itself. Returns
/// true if the file contains `"dogfood": true`.
fn detect_dogfood(project_path: &Path) -> bool {
    let path = project_path.join(".orqa/project.json");
    if !path.exists() {
        return false;
    }
    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            warn!(
                subsystem = "session_start",
                file = %path.display(),
                "could not read project.json: {e}"
            );
            return false;
        }
    };
    match serde_json::from_str::<serde_json::Value>(&contents) {
        Ok(v) => v
            .get("dogfood")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        Err(e) => {
            warn!(
                subsystem = "session_start",
                file = %path.display(),
                "malformed project.json: {e}"
            );
            false
        }
    }
}

/// Build the session start checklist.
///
/// Returns an ordered list of items for the agent to action at the start of
/// each session. Dogfood mode adds additional items.
fn build_checklist(dogfood: bool) -> Vec<String> {
    let mut items = vec![
        "Read context above (migration context + session state)".into(),
        "Set scope: which epic/task is the focus?".into(),
        "Keep .state/session-state.md up to date as you work".into(),
    ];
    if dogfood {
        items.push("DOGFOOD MODE ACTIVE: you are editing the app from the CLI".into());
        items.push("Ensure dev environment is running: orqa dev (separate terminal)".into());
        items.push("See RULE-998da8ea for dogfood rules".into());
    }
    items
}

/// Handle POST /session-start.
///
/// Runs all startup checks against the given project path and returns a
/// structured response. All checks are non-blocking — failures are reported
/// in the response rather than causing a 4xx/5xx error, so the connector
/// can surface them as warnings rather than hard errors.
pub async fn session_start_handler(
    Json(req): Json<SessionStartRequest>,
) -> Json<SessionStartResponse> {
    let project_path = Path::new(&req.project_path);
    let mut warnings: Vec<String> = Vec::new();

    let installation_check = check_installation(project_path);
    let daemon_check = check_daemon_health();
    let integrity_check = check_graph_integrity(project_path);
    let git_check = check_git_state(project_path, &mut warnings);

    let session_state = read_state_file(project_path, ".state/session-state.md");
    let migration_context = read_state_file(project_path, ".state/migration-context.md");
    let governance_context = read_state_file(project_path, ".state/governance-context.md");

    let dogfood = detect_dogfood(project_path);
    let checklist = build_checklist(dogfood);

    Json(SessionStartResponse {
        checks: vec![installation_check, daemon_check, integrity_check, git_check],
        warnings,
        session_state,
        migration_context,
        governance_context,
        checklist,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Create a minimal project directory tree for testing.
    fn make_project(tmp: &PathBuf) {
        fs::create_dir_all(tmp.join(".claude/agents")).unwrap();
        fs::create_dir_all(tmp.join(".claude/rules")).unwrap();
        fs::write(tmp.join(".claude/CLAUDE.md"), "# test").unwrap();
        fs::create_dir_all(tmp.join(".orqa")).unwrap();
        fs::create_dir_all(tmp.join(".state")).unwrap();
    }

    #[test]
    fn installation_check_passes_when_all_present() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_install_pass");
        make_project(&tmp);
        let result = check_installation(&tmp);
        assert!(result.passed, "expected pass, got: {}", result.message);
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn installation_check_fails_when_agents_missing() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_install_fail");
        make_project(&tmp);
        fs::remove_dir_all(tmp.join(".claude/agents")).unwrap();
        let result = check_installation(&tmp);
        assert!(!result.passed);
        assert!(result.message.contains(".claude/agents"));
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn daemon_health_always_passes() {
        let result = check_daemon_health();
        assert!(result.passed);
    }

    #[test]
    fn graph_integrity_passes_on_valid_json() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_graph_pass");
        make_project(&tmp);
        fs::write(tmp.join(".orqa/artifact.json"), r#"{"id":"test"}"#).unwrap();
        let result = check_graph_integrity(&tmp);
        assert!(result.passed, "expected pass, got: {}", result.message);
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn graph_integrity_fails_on_malformed_json() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_graph_fail");
        make_project(&tmp);
        fs::write(tmp.join(".orqa/broken.json"), "{ not json }").unwrap();
        let result = check_graph_integrity(&tmp);
        assert!(!result.passed);
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn read_state_file_returns_none_when_absent() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_state_absent");
        make_project(&tmp);
        let result = read_state_file(&tmp, ".state/session-state.md");
        assert!(result.is_none());
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn read_state_file_returns_contents_when_present() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_state_present");
        make_project(&tmp);
        fs::write(tmp.join(".state/session-state.md"), "# session").unwrap();
        let result = read_state_file(&tmp, ".state/session-state.md");
        assert_eq!(result.as_deref(), Some("# session"));
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn dogfood_detected_when_flag_true() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_dogfood");
        make_project(&tmp);
        fs::write(
            tmp.join(".orqa/project.json"),
            r#"{"dogfood":true}"#,
        )
        .unwrap();
        assert!(detect_dogfood(&tmp));
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn dogfood_not_detected_when_flag_false() {
        let tmp = PathBuf::from(std::env::temp_dir()).join("orqa_test_no_dogfood");
        make_project(&tmp);
        fs::write(tmp.join(".orqa/project.json"), r#"{"dogfood":false}"#).unwrap();
        assert!(!detect_dogfood(&tmp));
        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn checklist_includes_dogfood_items_when_active() {
        let items = build_checklist(true);
        assert!(items.iter().any(|i| i.contains("DOGFOOD MODE")));
    }

    #[test]
    fn checklist_excludes_dogfood_items_when_inactive() {
        let items = build_checklist(false);
        assert!(!items.iter().any(|i| i.contains("DOGFOOD MODE")));
    }
}
