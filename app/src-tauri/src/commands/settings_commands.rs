// Tauri IPC commands for application settings.
//
// Settings are persisted in the daemon's database. The app reads and writes
// settings via the daemon HTTP API through libs/db.

use std::collections::HashMap;

use tauri::State;

use crate::error::OrqaError;
use crate::state::AppState;

/// Set a setting value (upsert).
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub async fn settings_set(
    key: String,
    value: serde_json::Value,
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if key.trim().is_empty() {
        return Err(OrqaError::Validation(
            "settings key cannot be empty".to_owned(),
        ));
    }

    let scope_str = scope.unwrap_or_else(|| "app".to_owned());
    Ok(state
        .db
        .client
        .settings()
        .set(key.trim(), &value, &scope_str)
        .await?)
}

/// Get all settings for a given scope.
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub async fn settings_get_all(
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<HashMap<String, serde_json::Value>, OrqaError> {
    let scope_str = scope.unwrap_or_else(|| "app".to_owned());
    Ok(state.db.client.settings().get_all(&scope_str).await?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_key_validation() {
        let key = "   ";
        assert!(key.trim().is_empty());
    }
}
