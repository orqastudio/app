use tauri::State;

use crate::error::OrqaError;
use crate::startup::StartupSnapshot;
use crate::state::AppState;

/// Get the current status of all startup tasks.
///
/// Returns a snapshot of every registered startup task with its current
/// status and optional detail string (e.g. download percentage).
#[tauri::command]
pub async fn get_startup_status(state: State<'_, AppState>) -> Result<StartupSnapshot, OrqaError> {
    state
        .startup
        .tracker
        .snapshot()
        .map_err(|e| OrqaError::Search(e.to_string()))
}
