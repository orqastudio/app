use std::path::{Path, PathBuf};

use tauri::{AppHandle, Runtime, State};

use crate::domain::artifact::NavTree;
use crate::error::OrqaError;
use crate::state::AppState;
use crate::watcher;

use super::helpers::active_project_path;

/// Scan the active project and return a unified navigation tree.
///
/// Reads artifact layout from the project's `.orqa/project.json` `artifacts` field.
/// Each entry drives the scan — no folder-guessing. Returns an empty tree when no
/// active project is set or when the `artifacts` config is absent.
#[tauri::command]
pub fn artifact_scan_tree(state: State<'_, AppState>) -> Result<NavTree, OrqaError> {
    let project_path = active_project_path(&state)?;

    // Load artifacts config from project.json — empty list if missing or unset.
    let artifacts = crate::repo::project_settings_repo::read(&project_path)
        .unwrap_or(None)
        .map(|s| s.artifacts)
        .unwrap_or_default();

    crate::domain::artifact_reader::artifact_scan_tree(Path::new(&project_path), &artifacts)
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
