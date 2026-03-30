use std::path::{Path, PathBuf};

use tauri::{AppHandle, Runtime, State};

use crate::domain::artifact::NavTree;
use crate::error::OrqaError;
use crate::state::AppState;
use crate::watcher;

use super::helpers::active_project_path;

/// Scan the active project and return a unified navigation tree.
///
/// Derives artifact layout from `.orqa/schema.composed.json` — the schema is the
/// single source of truth for which artifact types exist and where they live.
/// No manual configuration in project.json is needed (P1: Plugin-Composed Everything).
/// Returns an empty tree when no active project is set or when the schema is absent.
#[tauri::command]
pub fn artifact_scan_tree(state: State<'_, AppState>) -> Result<NavTree, OrqaError> {
    let project_path = active_project_path(&state)?;
    let path = Path::new(&project_path);

    tracing::info!(project_path = %project_path, "artifact_scan_tree called");

    // Derive artifact entries from the composed schema — schema is authoritative.
    let entries = orqa_engine::artifact::artifact_entries_from_schema(path);
    tracing::info!(entries = entries.len(), "schema entries loaded");

    let tree = crate::domain::artifact_reader::artifact_scan_tree(path, &entries)?;
    tracing::info!(
        groups = tree.groups.len(),
        total_nodes = tree.groups.iter().map(|g| g.types.iter().map(|t| t.nodes.len()).sum::<usize>()).sum::<usize>(),
        "artifact_scan_tree result"
    );

    Ok(tree)
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
