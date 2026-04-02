// Tauri IPC commands for git hook management.
//
// All hook operations are delegated to the daemon via HTTP. The daemon
// owns the hook registry and dispatcher generation. The app is a thin client.
//
// Endpoints used:
//   GET  /hooks          — list registered hooks from plugin manifests
//   POST /hooks/generate — generate git hook dispatcher scripts

use tauri::State;

use crate::daemon_client::{HookGenerationResult, RegisteredHook};
use crate::error::OrqaError;
use crate::state::AppState;

/// Get all registered hooks from plugin manifests.
#[tauri::command]
pub async fn get_registered_hooks(
    state: State<'_, AppState>,
) -> Result<Vec<RegisteredHook>, OrqaError> {
    state.daemon.client.list_hooks().await
}

/// Regenerate git hook dispatcher scripts from plugin registrations.
///
/// Delegates to the daemon which reads hook registrations, groups by event,
/// and writes dispatcher scripts to `.githooks/`.
#[tauri::command]
pub async fn generate_hook_dispatchers(
    state: State<'_, AppState>,
) -> Result<HookGenerationResult, OrqaError> {
    state.daemon.client.generate_hook_dispatchers().await
}

#[cfg(test)]
mod tests {
    // Hook commands require daemon connectivity. Covered by integration tests.
}
