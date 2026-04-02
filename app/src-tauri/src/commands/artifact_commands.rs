use std::path::PathBuf;

use tauri::{AppHandle, Runtime, State};

use orqa_engine_types::types::artifact::NavTree;

use crate::error::OrqaError;
use crate::state::AppState;
use crate::watcher;

/// Fetch the artifact navigation tree from the daemon.
///
/// Delegates to the daemon `GET /artifacts/tree` endpoint, which derives the
/// tree from `.orqa/schema.composed.json` and scans the `.orqa/` directory.
/// Returns an empty tree when no active project is set or when the daemon is
/// unreachable.
#[tauri::command]
pub async fn artifact_scan_tree(state: State<'_, AppState>) -> Result<NavTree, OrqaError> {
    tracing::info!("artifact_scan_tree called — delegating to daemon");
    state.daemon.client.get_artifact_tree().await
}

/// Start (or replace) the `.orqa/` file-system watcher for a project.
///
/// Watches `<project_path>/.orqa/` recursively with a 500 ms debounce.
/// When any file changes a single `artifact-changed` Tauri event is emitted to
/// all windows so the frontend can invalidate its nav-tree cache.
///
/// Safe to call multiple times — each call replaces the previous watcher.
#[tauri::command]
pub fn artifact_watch_start<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    project_path: String,
) -> Result<(), OrqaError> {
    watcher::start(app, PathBuf::from(&project_path), &state.artifacts.watcher)
        .map_err(OrqaError::FileSystem)
}
