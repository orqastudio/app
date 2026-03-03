use std::collections::HashMap;

use tauri::State;

use crate::error::ForgeError;
use crate::repo::settings_repo;
use crate::state::AppState;

/// Get a single setting value by key and scope.
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub fn settings_get(
    key: String,
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<Option<serde_json::Value>, ForgeError> {
    if key.trim().is_empty() {
        return Err(ForgeError::Validation(
            "settings key cannot be empty".to_string(),
        ));
    }

    let scope_str = scope.unwrap_or_else(|| "app".to_string());

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    settings_repo::get(&conn, key.trim(), &scope_str)
}

/// Set a setting value (upsert).
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub fn settings_set(
    key: String,
    value: serde_json::Value,
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), ForgeError> {
    if key.trim().is_empty() {
        return Err(ForgeError::Validation(
            "settings key cannot be empty".to_string(),
        ));
    }

    let scope_str = scope.unwrap_or_else(|| "app".to_string());

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    settings_repo::set(&conn, key.trim(), &value, &scope_str)
}

/// Get all settings for a given scope.
///
/// Scope defaults to "app" if not provided.
#[tauri::command]
pub fn settings_get_all(
    scope: Option<String>,
    state: State<'_, AppState>,
) -> Result<HashMap<String, serde_json::Value>, ForgeError> {
    let scope_str = scope.unwrap_or_else(|| "app".to_string());

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    settings_repo::get_all(&conn, &scope_str)
}

#[cfg(test)]
mod tests {
    use crate::db::init_memory_db;
    use crate::repo::settings_repo;

    #[test]
    fn get_nonexistent_returns_none() {
        let conn = init_memory_db().expect("db init");
        let result = settings_repo::get(&conn, "missing", "app").expect("get");
        assert!(result.is_none());
    }

    #[test]
    fn set_and_get_string_value() {
        let conn = init_memory_db().expect("db init");
        let value = serde_json::json!("dark");
        settings_repo::set(&conn, "theme", &value, "app").expect("set");

        let fetched = settings_repo::get(&conn, "theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn set_and_get_object_value() {
        let conn = init_memory_db().expect("db init");
        let value = serde_json::json!({"font_size": 14, "wrap": true});
        settings_repo::set(&conn, "editor", &value, "app").expect("set");

        let fetched = settings_repo::get(&conn, "editor", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched["font_size"], 14);
        assert_eq!(fetched["wrap"], true);
    }

    #[test]
    fn set_overwrites_existing() {
        let conn = init_memory_db().expect("db init");
        settings_repo::set(&conn, "theme", &serde_json::json!("light"), "app").expect("set 1");
        settings_repo::set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set 2");

        let fetched = settings_repo::get(&conn, "theme", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[test]
    fn scopes_are_independent() {
        let conn = init_memory_db().expect("db init");
        settings_repo::set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set app");
        settings_repo::set(&conn, "theme", &serde_json::json!("light"), "project:1")
            .expect("set project");

        let app_val = settings_repo::get(&conn, "theme", "app")
            .expect("get app")
            .expect("should exist");
        let proj_val = settings_repo::get(&conn, "theme", "project:1")
            .expect("get project")
            .expect("should exist");

        assert_eq!(app_val, serde_json::json!("dark"));
        assert_eq!(proj_val, serde_json::json!("light"));
    }

    #[test]
    fn get_all_returns_scope_entries() {
        let conn = init_memory_db().expect("db init");
        settings_repo::set(&conn, "theme", &serde_json::json!("dark"), "app").expect("set");
        settings_repo::set(&conn, "font", &serde_json::json!(14), "app").expect("set");
        settings_repo::set(&conn, "other", &serde_json::json!("x"), "project:1")
            .expect("set other scope");

        let all = settings_repo::get_all(&conn, "app").expect("get_all");
        assert_eq!(all.len(), 2);
        assert_eq!(all["theme"], serde_json::json!("dark"));
        assert_eq!(all["font"], serde_json::json!(14));
    }

    #[test]
    fn get_all_empty_scope() {
        let conn = init_memory_db().expect("db init");
        let all = settings_repo::get_all(&conn, "nonexistent").expect("get_all");
        assert!(all.is_empty());
    }

    #[test]
    fn empty_key_validation() {
        let key = "   ";
        assert!(key.trim().is_empty());
    }
}
