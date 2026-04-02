// CLI tool routes: list registered tools, run them, and check status.
//
// CLI tools are registered by plugins via `plugin-cli-tools.json`. The
// CliToolRunner discovers, executes, and tracks the last run status of each
// registered tool. All calls use spawn_blocking because process execution
// is synchronous.
//
// Endpoints:
//   GET  /cli-tools                       — list registered CLI tools
//   POST /cli-tools/:plugin/:key/run      — execute a plugin CLI tool
//   GET  /cli-tools/status                — get last-run status of all tools

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use orqa_plugin::cli_runner::{CliToolRunner, CliToolResult};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /cli-tools — list all CLI tools registered by installed plugins.
///
/// Reads `plugin-cli-tools.json` from the project root and returns the full
/// list of discovered tools.
pub async fn list_cli_tools(
    State(state): State<GraphState>,
) -> Json<serde_json::Value> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(serde_json::json!({ "tools": [] }));
        };
        guard.project_root.clone()
    };

    let result = tokio::task::spawn_blocking(move || {
        let runner = CliToolRunner::new();
        runner.registered_cli_tools(&project_root)
    })
    .await
    .unwrap_or_default();

    Json(serde_json::json!({ "tools": result }))
}

/// Handle POST /cli-tools/:plugin/:key/run — execute a registered CLI tool.
///
/// Looks up the tool by plugin and key, spawns the process synchronously,
/// and returns the captured output. Returns 404 when the plugin/key pair
/// is not registered.
pub async fn run_cli_tool(
    State(state): State<GraphState>,
    Path((plugin, key)): Path<(String, String)>,
) -> Result<Json<CliToolResult>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    tokio::task::spawn_blocking(move || {
        let runner = CliToolRunner::new();
        let tools = runner.registered_cli_tools(&project_root);
        let tool = tools.iter().find(|t| t.plugin == plugin && t.key == key);

        match tool {
            None => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("CLI tool {plugin}/{key} not found"),
                    "code": "NOT_FOUND"
                })),
            )),
            Some(t) => {
                runner.run(t, &project_root)
                    .map(Json)
                    .map_err(|e| (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": e, "code": "RUN_FAILED" })),
                    ))
            }
        }
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /cli-tools/status — return the last-run result for each tool.
///
/// Returns a map of plugin/key -> CliToolStatus representing the last known
/// run outcome for every registered tool. Absent means never run.
pub async fn get_cli_tool_status(
    State(state): State<GraphState>,
) -> Json<serde_json::Value> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(serde_json::json!({ "statuses": [] }));
        };
        guard.project_root.clone()
    };

    let result = tokio::task::spawn_blocking(move || {
        let runner = CliToolRunner::new();
        runner.statuses(&project_root)
    })
    .await
    .unwrap_or_default();

    Json(serde_json::json!({ "statuses": result }))
}
