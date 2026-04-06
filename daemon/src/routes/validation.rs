// Validation routes: integrity scan, auto-fix, and hook evaluation.
//
// All scan and fix handlers use the cached GraphState. Hook evaluation uses
// the cached graph to load active rules.
//
// Endpoints:
//   POST /validation/scan  — run all integrity checks on the cached graph
//   POST /validation/fix   — run checks and apply auto-fixes
//   POST /validation/hook  — evaluate hook lifecycle rules

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use orqa_validation::hooks::evaluate_hook;
use orqa_validation::metrics::compute_health;
use orqa_validation::types::{GraphHealth, HookContext, HookResult, IntegrityCheck};
use orqa_validation::PipelineCategories;
use orqa_validation::{auto_fix, validate, AppliedFix};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Response body for POST /validation/scan.
#[derive(Debug, Serialize)]
pub struct ValidationScanResponse {
    /// All integrity check findings from the graph.
    pub checks: Vec<IntegrityCheck>,
    /// Graph health metrics computed alongside the checks.
    pub health: GraphHealth,
}

/// Request body for POST /validation/fix.
#[derive(Debug, Deserialize)]
pub struct ValidationFixRequest {
    /// Must be `true` to apply fixes. `false` is a dry-run that returns what
    /// would be fixed without writing to disk.
    pub fix: bool,
}

/// Response body for POST /validation/fix.
#[derive(Debug, Serialize)]
pub struct ValidationFixResponse {
    /// All integrity check findings (same as scan).
    pub checks: Vec<IntegrityCheck>,
    /// Graph health metrics.
    pub health: GraphHealth,
    /// Fixes that were applied to disk. Empty on dry-run.
    pub fixes_applied: Vec<AppliedFix>,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /validation/scan — run all integrity checks against the cached graph.
///
/// Uses the cached graph and validation context. Returns findings and current
/// health metrics. No disk I/O beyond what is already cached.
pub async fn validation_scan(State(state): State<GraphState>) -> Json<ValidationScanResponse> {
    let Ok(guard) = state.0.read() else {
        return Json(ValidationScanResponse {
            checks: Vec::new(),
            health: GraphHealth::default(),
        });
    };

    let checks = validate(&guard.graph, &guard.ctx);
    let owned = guard.owned_pipeline_categories();
    let (d, l, es, et, rt) = owned.as_str_vecs();
    let health = compute_health(
        &guard.graph,
        &PipelineCategories {
            delivery: &d,
            learning: &l,
            excluded_statuses: &es,
            excluded_types: &et,
            root_types: &rt,
        },
    );

    Json(ValidationScanResponse { checks, health })
}

/// Handle POST /validation/fix — run integrity checks and optionally apply fixes.
///
/// When `fix: true`, applies auto-fixes to files on disk and reloads the graph.
/// When `fix: false`, behaves like /validation/scan with an empty `fixes_applied`.
/// The reload after fixing is wrapped in `spawn_blocking` because it does a full
/// directory scan and must not block the tokio thread pool.
pub async fn validation_fix(
    State(state): State<GraphState>,
    Json(req): Json<ValidationFixRequest>,
) -> Result<Json<ValidationFixResponse>, (StatusCode, Json<serde_json::Value>)> {
    let (checks, health, project_root) = {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        let checks = validate(&guard.graph, &guard.ctx);
        let owned = guard.owned_pipeline_categories();
        let (d, l, es, et, rt) = owned.as_str_vecs();
        let health = compute_health(
            &guard.graph,
            &PipelineCategories {
                delivery: &d,
                learning: &l,
                excluded_statuses: &es,
                excluded_types: &et,
                root_types: &rt,
            },
        );
        (checks, health, guard.project_root.clone())
    };

    let fixes_applied = if req.fix {
        // Run auto_fix inside spawn_blocking: it borrows the graph state
        // synchronously and writes to disk, so it must not run on the tokio thread.
        let state_clone = state.clone();
        let root_clone = project_root.clone();
        let checks_clone = checks.clone();
        let (result, _) = tokio::task::spawn_blocking(move || {
            let guard = state_clone.0.read().map_err(|e| e.to_string())?;
            let fixes =
                auto_fix(&guard.graph, &checks_clone, &root_clone).map_err(|e| e.to_string())?;
            drop(guard);
            // Reload graph so subsequent requests see the updated state.
            // reload() walks the full .orqa/ directory tree — must stay in spawn_blocking.
            let count = state_clone.reload(&root_clone);
            Ok::<_, String>((fixes, count))
        })
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
            )
        })?
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "FIX_FAILED" })),
            )
        })?;
        result
    } else {
        Vec::new()
    };

    Ok(Json(ValidationFixResponse {
        checks,
        health,
        fixes_applied,
    }))
}

/// Handle POST /validation/hook — evaluate a hook lifecycle event against active rules.
///
/// Uses `evaluate_hook` from the validation crate which scans active rules in the
/// project root. The project root is taken from the cached GraphState.
pub async fn validation_hook(
    State(state): State<GraphState>,
    Json(ctx): Json<HookContext>,
) -> Result<Json<HookResult>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let guard = state.0.read().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            )
        })?;
        guard.project_root.clone()
    };

    let result = evaluate_hook(&ctx, &project_root);
    Ok(Json(result))
}
