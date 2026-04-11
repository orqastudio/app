// Enforcement routes: rule listing, violation scanning, and governance scan.
//
// Enforcement rules are cached in `GraphState` and rebuilt by the file watcher
// whenever `.orqa/` changes — route handlers never hit disk. This keeps the
// hot path O(1) and eliminates the 3-second reparse loop that previously
// fired on every request.
//
// Endpoints:
//   GET  /enforcement/rules         — list all parsed enforcement rules
//   POST /enforcement/rules/reload  — force an immediate reload from disk
//   GET  /enforcement/violations    — list scan findings classified as errors
//   POST /enforcement/scan          — full governance scan across all areas

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use orqa_enforcement::engine::EnforcementEngine;
use orqa_enforcement::scanner::scan_governance;
use orqa_engine_types::config::load_project_settings;
use orqa_engine_types::types::enforcement::{EnforcementRule, ScanFinding};
use orqa_engine_types::types::governance::GovernanceScanResult;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Response shapes
// ---------------------------------------------------------------------------

/// Response body for POST /enforcement/scan.
#[derive(Debug, Serialize)]
pub struct EnforcementScanResponse {
    /// Parsed rules from the learning/rules area.
    pub rules: Vec<EnforcementRule>,
    /// Governance scan result across all artifact areas.
    pub governance: GovernanceScanResult,
    /// Total artifact count across scanned areas.
    pub total_artifacts: usize,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read the cached project root from graph state.
///
/// Returns `None` if the state lock is poisoned — callers fall back to an
/// empty response so the endpoint never blocks.
fn project_root(state: &GraphState) -> Option<std::path::PathBuf> {
    state.0.read().ok().map(|guard| guard.project_root.clone())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /enforcement/rules — return all cached enforcement rules.
///
/// Reads from the in-memory cache populated by the file watcher.  The cache
/// is invalidated automatically whenever a file under `.orqa/` changes, so
/// callers see up-to-date rules without ever paying the parse cost on the
/// request path.
pub async fn list_enforcement_rules(State(state): State<GraphState>) -> Json<Vec<EnforcementRule>> {
    Json(state.enforcement_rules())
}

/// Handle POST /enforcement/rules/reload — force an immediate reload from disk.
///
/// Bypasses the watcher-driven invalidation for callers that want synchronous
/// freshness (e.g. a manual "Reload rules" button in the UI).  After rebuild,
/// returns the new cached list.
pub async fn reload_enforcement_rules(
    State(state): State<GraphState>,
) -> Json<Vec<EnforcementRule>> {
    let Some(root) = project_root(&state) else {
        return Json(Vec::new());
    };
    state.reload(&root);
    Json(state.enforcement_rules())
}

/// Handle GET /enforcement/violations — return scan findings classified as errors.
///
/// Builds an enforcement engine from the cached rules and runs the project
/// scan.  Only findings whose action is `Block` are returned as violations.
pub async fn list_enforcement_violations(
    State(state): State<GraphState>,
) -> Json<Vec<ScanFinding>> {
    let Some(root) = project_root(&state) else {
        return Json(Vec::new());
    };

    let rules = state.enforcement_rules();
    let engine = EnforcementEngine::new(rules);
    let findings = engine.scan(&root).unwrap_or_default();
    use orqa_engine_types::types::enforcement::RuleAction;
    let violations: Vec<ScanFinding> = findings
        .into_iter()
        .filter(|f| matches!(f.action, RuleAction::Block))
        .collect();
    Json(violations)
}

/// Handle POST /enforcement/scan — full governance scan across all artifact areas.
///
/// Uses `scan_governance` with the project's artifact config to discover
/// coverage across all governance areas. Returns cached rules, governance
/// scan result, and total artifact count.
pub async fn enforcement_scan(
    State(state): State<GraphState>,
) -> Result<Json<EnforcementScanResponse>, (StatusCode, Json<serde_json::Value>)> {
    let Some(root) = project_root(&state) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
        ));
    };

    let rules = state.enforcement_rules();

    // Load artifact config for governance scan coverage computation.
    let artifacts = load_project_settings(&root)
        .ok()
        .flatten()
        .map(|s| s.artifacts)
        .unwrap_or_default();

    let governance = scan_governance(&root, &artifacts).unwrap_or(GovernanceScanResult {
        areas: Vec::new(),
        coverage_ratio: 0.0,
    });
    let total_artifacts = governance.areas.iter().map(|a| a.files.len()).sum();

    Ok(Json(EnforcementScanResponse {
        rules,
        governance,
        total_artifacts,
    }))
}
