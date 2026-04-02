// Tauri IPC commands for enforcement rule management and governance scanning.
//
// All enforcement operations are delegated to the daemon via HTTP. The daemon
// owns the enforcement engine and governance scanner. The app is a thin client.
//
// Endpoints used:
//   GET  /enforcement/rules         — list all parsed enforcement rules
//   POST /enforcement/rules/reload  — reload rules from disk
//   GET  /enforcement/violations    — list recorded violations
//   POST /enforcement/scan          — full governance scan

use tauri::State;

use crate::daemon_client::EnforcementScanResponse;
use crate::error::OrqaError;
use crate::state::AppState;
use orqa_engine_types::types::enforcement::{EnforcementRule, EnforcementViolation};
use orqa_engine_types::types::governance::GovernanceScanResult;

/// List the enforcement rules currently loaded in the daemon.
///
/// Returns the full list of parsed rules including their enforcement entries.
#[tauri::command]
pub async fn enforcement_rules_list(
    state: State<'_, AppState>,
) -> Result<Vec<EnforcementRule>, OrqaError> {
    state.daemon.client.list_enforcement_rules().await
}

/// Reload the enforcement engine in the daemon from disk.
///
/// Returns the number of rules loaded after reload.
#[tauri::command]
pub async fn enforcement_rules_reload(state: State<'_, AppState>) -> Result<usize, OrqaError> {
    let rules = state.daemon.client.reload_enforcement_rules().await?;
    Ok(rules.len())
}

/// List enforcement violation history from the daemon.
///
/// Returns all recorded violations ordered by most recent first.
#[tauri::command]
pub async fn enforcement_violations_list(
    state: State<'_, AppState>,
) -> Result<Vec<EnforcementViolation>, OrqaError> {
    state.daemon.client.list_enforcement_violations().await
}

/// Run a full governance scan via the daemon.
///
/// Delegates to the daemon's enforcement scanner which uses the project root
/// path already known to the daemon. Returns the governance scan result.
#[tauri::command]
pub async fn governance_scan(
    state: State<'_, AppState>,
) -> Result<GovernanceScanResult, OrqaError> {
    let resp: EnforcementScanResponse = state.daemon.client.enforcement_scan().await?;
    Ok(resp.governance)
}

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::enforcement::{EnforcementEntry, EnforcementRule, EventType, RuleAction};

    #[test]
    fn enforcement_rule_serialization() {
        let rule = EnforcementRule {
            name: "test-rule".to_owned(),
            scope: "project".to_owned(),
            prose: "# Test rule".to_owned(),
            entries: vec![EnforcementEntry {
                event: EventType::Bash,
                action: RuleAction::Warn,
                conditions: vec![],
                pattern: Some("rm -rf".to_owned()),
                scope: None,
                knowledge: vec![],
            }],
        };
        let json = serde_json::to_value(&rule).expect("should serialize");
        assert_eq!(json["name"], "test-rule");
        assert_eq!(json["entries"][0]["event"], "bash");
        assert_eq!(json["entries"][0]["action"], "warn");
    }
}
