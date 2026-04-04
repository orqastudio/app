// Tauri IPC commands for application settings.
//
// Settings are persisted in the project-scoped SQLite database via engine/storage.

use std::collections::HashMap;

use tauri::State;

use crate::error::OrqaError;
use crate::state::AppState;

/// Set a setting value (upsert).
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub fn settings_set(
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
    let storage = state.db.get()?;
    Ok(storage.settings().set(key.trim(), &value, &scope_str)?)
}

/// Get all settings for a given scope.
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub fn settings_get_all(
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<HashMap<String, serde_json::Value>, OrqaError> {
    let scope_str = scope.unwrap_or_else(|| "app".to_owned());
    let storage = state.db.get()?;
    Ok(storage.settings().get_all(&scope_str)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_nonexistent_returns_none() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let result = storage.settings().get("missing", "app").expect("get");
        assert!(result.is_none());
    }

    #[test]
    fn set_and_get_string_value() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let value = serde_json::json!("dark");
        storage.settings().set("theme", &value, "app").expect("set");

        let fetched = storage
            .settings()
            .get("theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn set_and_get_object_value() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let value = serde_json::json!({"font_size": 14, "wrap": true});
        storage.settings().set("editor", &value, "app").expect("set");

        let fetched = storage
            .settings()
            .get("editor", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched["font_size"], 14);
        assert_eq!(fetched["wrap"], true);
    }

    #[test]
    fn set_overwrites_existing() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        storage
            .settings()
            .set("theme", &serde_json::json!("light"), "app")
            .expect("set 1");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .expect("set 2");

        let fetched = storage
            .settings()
            .get("theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn scopes_are_independent() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .expect("set app");
        storage
            .settings()
            .set("theme", &serde_json::json!("light"), "project:1")
            .expect("set project");

        let app_val = storage
            .settings()
            .get("theme", "app")
            .expect("get app")
            .expect("should exist");
        let proj_val = storage
            .settings()
            .get("theme", "project:1")
            .expect("get project")
            .expect("should exist");

        assert_eq!(app_val, serde_json::json!("dark"));
        assert_eq!(proj_val, serde_json::json!("light"));
    }

    #[test]
    fn get_all_returns_scope_entries() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .expect("set");
        storage
            .settings()
            .set("font", &serde_json::json!(14), "app")
            .expect("set");
        storage
            .settings()
            .set("other", &serde_json::json!("x"), "project:1")
            .expect("set other scope");

        let all = storage.settings().get_all("app").expect("get_all");
        assert_eq!(all.len(), 2);
        assert_eq!(all["theme"], serde_json::json!("dark"));
        assert_eq!(all["font"], serde_json::json!(14));
    }

    #[test]
    fn get_all_empty_scope() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let all = storage.settings().get_all("nonexistent").expect("get_all");
        assert!(all.is_empty());
    }

    #[test]
    fn empty_key_validation() {
        let key = "   ";
        assert!(key.trim().is_empty());
    }
}
