use std::path::Path;

use tauri::State;

use crate::domain::enforcement::EnforcementRule;
use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::enforcement_violation::EnforcementViolation;
use crate::domain::governance::GovernanceScanResult;
use crate::domain::governance_scanner;
use crate::error::OrqaError;
use crate::repo::enforcement_rules_repo;
use crate::repo::project_repo;
use crate::repo::project_settings_repo;
use crate::repo::violations_repo;
use crate::state::AppState;

/// List the enforcement rules currently loaded for the active project.
///
/// Returns the full list of parsed rules including their enforcement entries.
/// Rules without YAML frontmatter are included with empty `entries`.
#[tauri::command]
pub fn enforcement_rules_list(
    state: State<'_, AppState>,
) -> Result<Vec<EnforcementRule>, OrqaError> {
    let guard = state
        .enforcement
        .engine
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;

    match guard.as_ref() {
        Some(engine) => Ok(engine.rules().to_vec()),
        None => Ok(Vec::new()),
    }
}

/// Reload the enforcement engine from the active project's `.orqa/rules/` directory.
///
/// Returns the number of rules loaded. Use this when rule files have been edited
/// and you want the engine to pick up the changes without restarting the app.
#[tauri::command]
pub fn enforcement_rules_reload(state: State<'_, AppState>) -> Result<usize, OrqaError> {
    let project_path = resolve_active_project_path(&state)?;
    let rules_dir = Path::new(&project_path).join(".orqa").join("rules");

    if !rules_dir.exists() {
        let mut guard = state
            .enforcement
            .engine
            .lock()
            .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
        *guard = None;
        return Ok(0);
    }

    let rules = enforcement_rules_repo::load_rules(&rules_dir)?;
    let engine = EnforcementEngine::new(rules);
    let count = engine.rules().len();

    let mut guard = state
        .enforcement
        .engine
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
    *guard = Some(engine);

    tracing::debug!(
        "[enforcement] reloaded {count} rules from '{}'",
        rules_dir.display()
    );
    Ok(count)
}

/// List enforcement violation history for the active project.
///
/// Returns all recorded violations ordered by most recent first. Use this to
/// surface historical enforcement events in the governance UI rather than relying
/// on the in-memory session-only violations in the enforcement store.
#[tauri::command]
pub fn enforcement_violations_list(
    state: State<'_, AppState>,
) -> Result<Vec<EnforcementViolation>, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("db lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_owned()))?;

    violations_repo::list_for_project(&conn, project.id, None)
}

/// Scan the active project directory for governance files.
///
/// Loads artifact paths from `project.json` so the set of scanned areas is driven
/// by plugin configuration (P1: Plugin-Composed Everything). Falls back to an empty
/// artifact list when no `project.json` exists — the scan still runs but reports zero
/// coverage, prompting the user to configure a project.
#[tauri::command]
pub fn governance_scan(state: State<'_, AppState>) -> Result<GovernanceScanResult, OrqaError> {
    let project_path = resolve_active_project_path(&state)?;

    // Load artifacts config from project.json; fall back to empty if missing or unreadable.
    let artifacts = match project_settings_repo::read(&project_path) {
        Ok(Some(settings)) => settings.artifacts,
        Ok(None) => {
            tracing::debug!("[governance_scan] no project.json at '{project_path}', scanning with empty artifact list");
            Vec::new()
        }
        Err(e) => {
            tracing::warn!("[governance_scan] failed to load project.json: {e}");
            Vec::new()
        }
    };

    governance_scanner::scan_governance(Path::new(&project_path), &artifacts)
}

/// Resolve the active project's path from the database.
fn resolve_active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("db lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_owned()))?;

    Ok(project.path)
}

#[cfg(test)]
mod tests {
    use crate::domain::enforcement::{EnforcementRule, EventType, RuleAction};
    use crate::domain::enforcement_engine::EnforcementEngine;
    use crate::repo::enforcement_rules_repo;

    #[test]
    fn engine_with_no_rules_returns_empty_list() {
        let engine = EnforcementEngine::new(vec![]);
        assert!(engine.rules().is_empty());
    }

    #[test]
    fn engine_with_rules_returns_all_rules() {
        let rule = EnforcementRule {
            name: "test-rule".to_owned(),
            scope: "project".to_owned(),
            prose: "# Test rule".to_owned(),
            entries: vec![],
        };
        let engine = EnforcementEngine::new(vec![rule]);
        assert_eq!(engine.rules().len(), 1);
        assert_eq!(engine.rules()[0].name, "test-rule");
    }

    #[test]
    fn load_rules_from_tempdir_with_valid_rule() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("test-rule.md"),
            r#"---
scope: project
enforcement:
  - event: bash
    action: warn
    pattern: "rm -rf"
---
# Test Rule

Do not use dangerous commands.
"#,
        )
        .expect("write");

        let rules = enforcement_rules_repo::load_rules(dir.path()).expect("should load");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "test-rule");
        assert_eq!(rules[0].entries.len(), 1);
        assert_eq!(rules[0].entries[0].event, EventType::Bash);
        assert_eq!(rules[0].entries[0].action, RuleAction::Warn);
    }

    #[test]
    fn load_rules_from_nonexistent_dir_returns_error() {
        let result = enforcement_rules_repo::load_rules(std::path::Path::new(
            "/nonexistent/rules/directory",
        ));
        assert!(result.is_err());
    }

    #[test]
    fn load_rules_skips_malformed_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Valid rule
        std::fs::write(dir.path().join("good.md"), "# Good Rule\n\nBody text.")
            .expect("write good");
        // Non-md file (should be skipped)
        std::fs::write(dir.path().join("not-a-rule.txt"), "not a rule").expect("write txt");

        let rules = enforcement_rules_repo::load_rules(dir.path()).expect("should load");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "good");
    }
}
