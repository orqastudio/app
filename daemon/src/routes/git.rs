// Git integration routes: stash list and uncommitted file status.
//
// These routes expose git state for the active project. They delegate to
// `orqa_project::git` which runs `git` commands as child processes.
// All handlers use `spawn_blocking` to keep the tokio runtime free.
//
// Endpoints:
//   GET /git/stashes — list git stashes for the active project
//   GET /git/status  — get uncommitted file status for the active project

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use orqa_project::git::{stash_list, uncommitted_status};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /git/stashes — list all git stashes in the project.
///
/// Returns the stash list from `git stash list`. Returns an empty list
/// when the project is not a git repository or git is not installed.
pub async fn get_git_stashes(
    State(state): State<GraphState>,
) -> Json<serde_json::Value> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(serde_json::json!({ "stashes": [] }));
        };
        guard.project_root.clone()
    };

    let result = tokio::task::spawn_blocking(move || stash_list(&project_root))
        .await
        .ok()
        .flatten();

    match result {
        Some(list) => {
            let entries: Vec<&str> = list.output.lines().collect();
            Json(serde_json::json!({ "stashes": entries, "count": entries.len() }))
        }
        None => Json(serde_json::json!({ "stashes": [], "count": 0 })),
    }
}

/// Handle GET /git/status — return uncommitted file status for the project.
///
/// Returns the output of `git status --short`. Returns an empty status
/// when the project is not a git repository.
pub async fn get_git_status(
    State(state): State<GraphState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let result = tokio::task::spawn_blocking(move || uncommitted_status(&project_root))
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        ))?;

    match result {
        Some(status) => Ok(Json(serde_json::json!({
            "branch": status.branch,
            "uncommitted_count": status.uncommitted_count,
            "clean": status.uncommitted_count == 0,
        }))),
        None => Ok(Json(serde_json::json!({ "branch": null, "uncommitted_count": 0, "clean": true }))),
    }
}
