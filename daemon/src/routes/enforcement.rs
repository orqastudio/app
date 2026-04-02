// Enforcement routes: rule listing, violation scanning, and governance scan.
//
// All handlers call into the orqa-enforcement crate. Rules are loaded from the
// .orqa/learning/rules directory under the project root.
//
// Endpoints:
//   GET  /enforcement/rules         — list all parsed enforcement rules
//   POST /enforcement/rules/reload  — reload rules from disk (same as list)
//   GET  /enforcement/violations    — list scan findings classified as errors
//   POST /enforcement/scan          — full governance scan across all areas

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use orqa_enforcement::engine::EnforcementEngine;
use orqa_enforcement::scanner::scan_governance;
use orqa_enforcement::store::load_rules;
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

/// Load enforcement rules from the project root's learning/rules directory.
///
/// Returns an empty list if the directory is absent or unreadable.
fn load_project_rules(project_root: &std::path::Path) -> Vec<EnforcementRule> {
    let rules_dir = project_root.join(".orqa/learning/rules");
    load_rules(&rules_dir).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /enforcement/rules — return all enforcement rules from .orqa/.
///
/// Loads rules from the learning/rules directory under the project root.
/// Returns an empty list if the directory is absent.
pub async fn list_enforcement_rules(
    State(state): State<GraphState>,
) -> Json<Vec<EnforcementRule>> {
    let Ok(guard) = state.0.read() else {
        return Json(Vec::new());
    };
    let project_root = guard.project_root.clone();
    drop(guard);
    Json(load_project_rules(&project_root))
}

/// Handle POST /enforcement/rules/reload — reload rules from disk.
///
/// Re-reads rule files from disk. Identical to GET /enforcement/rules but
/// semantically signals to the caller that a refresh has been performed.
pub async fn reload_enforcement_rules(
    State(state): State<GraphState>,
) -> Json<Vec<EnforcementRule>> {
    let Ok(guard) = state.0.read() else {
        return Json(Vec::new());
    };
    let project_root = guard.project_root.clone();
    drop(guard);
    Json(load_project_rules(&project_root))
}

/// Handle GET /enforcement/violations — return scan findings classified as errors.
///
/// Builds an enforcement engine from all loaded rules and runs the project scan.
/// Only findings whose severity is "error" are returned as violations.
pub async fn list_enforcement_violations(
    State(state): State<GraphState>,
) -> Json<Vec<ScanFinding>> {
    let Ok(guard) = state.0.read() else {
        return Json(Vec::new());
    };
    let project_root = guard.project_root.clone();
    drop(guard);

    let rules = load_project_rules(&project_root);
    let engine = EnforcementEngine::new(rules);
    let findings = engine.scan(&project_root).unwrap_or_default();
    // Return only Block-action findings as "violations".
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
/// coverage across all governance areas. Returns rules, governance scan result,
/// and total artifact count.
pub async fn enforcement_scan(
    State(state): State<GraphState>,
) -> Result<Json<EnforcementScanResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let rules = load_project_rules(&project_root);

    // Load artifact config for governance scan coverage computation.
    let artifacts = load_project_settings(&project_root)
        .ok()
        .flatten()
        .map(|s| s.artifacts)
        .unwrap_or_default();

    let governance = scan_governance(&project_root, &artifacts).unwrap_or(GovernanceScanResult {
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
