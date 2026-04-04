// Settings repository for orqa-storage.
//
// Provides get/set/list operations over the `settings` table. Settings are
// key-value pairs with a scope (e.g., "app", "project:123"). Values are stored
// as JSON strings. All SQL is ported directly from
// app/src-tauri/src/repo/settings_repo.rs.

use std::collections::HashMap;

use rusqlite::{OptionalExtension, params};

use crate::Storage;
use crate::error::StorageError;

/// Zero-cost repository handle for the `settings` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::settings()`.
pub struct SettingsRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl SettingsRepo<'_> {
    /// Get a single setting value by key and scope.
    ///
    /// Returns `None` if the key does not exist in the given scope.
    pub fn get(
        &self,
        key: &str,
        scope: &str,
    ) -> Result<Option<serde_json::Value>, StorageError> {
        let conn = self.storage.conn()?;
        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1 AND scope = ?2",
                params![key, scope],
                |row| row.get(0),
            )
            .optional()?;

        match value {
            Some(v) => {
                let parsed: serde_json::Value = serde_json::from_str(&v)?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// Set a setting value (upsert by key + scope).
    pub fn set(
        &self,
        key: &str,
        value: &serde_json::Value,
        scope: &str,
    ) -> Result<(), StorageError> {
        let value_str = serde_json::to_string(value)?;
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO settings (key, value, scope, updated_at) \
             VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
             ON CONFLICT(key, scope) DO UPDATE SET \
             value = excluded.value, updated_at = excluded.updated_at",
            params![key, value_str, scope],
        )?;
        Ok(())
    }

    /// Get all settings regardless of scope, ordered by key.
    ///
    /// Used by the daemon settings endpoint when no scope filter is requested.
    /// Keys from different scopes may collide — callers should prefer `get_all`
    /// with an explicit scope when the scope is known.
    pub fn get_all_any_scope(&self) -> Result<HashMap<String, serde_json::Value>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt =
            conn.prepare("SELECT key, value FROM settings ORDER BY key ASC")?;

        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value_str: String = row.get(1)?;
            Ok((key, value_str))
        })?;

        rows.map(|row| {
            let (key, value_str) = row.map_err(|e| StorageError::Database(e.to_string()))?;
            let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
            Ok((key, parsed))
        })
        .collect::<Result<HashMap<_, _>, StorageError>>()
    }

    /// Get all settings for a given scope, ordered by key.
    pub fn get_all(
        &self,
        scope: &str,
    ) -> Result<HashMap<String, serde_json::Value>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt =
            conn.prepare("SELECT key, value FROM settings WHERE scope = ?1 ORDER BY key ASC")?;

        let rows = stmt.query_map(params![scope], |row| {
            let key: String = row.get(0)?;
            let value_str: String = row.get(1)?;
            Ok((key, value_str))
        })?;

        rows.map(|row| {
            let (key, value_str) = row.map_err(|e| StorageError::Database(e.to_string()))?;
            let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
            Ok((key, parsed))
        })
        .collect::<Result<HashMap<_, _>, StorageError>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    #[test]
    fn get_nonexistent_returns_none() {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        let result = storage.settings().get("theme", "app").expect("get");
        assert!(result.is_none());
    }

    #[test]
    fn set_and_get_string() {
        let storage = Storage::open_in_memory().expect("in-memory storage");
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
    fn set_overwrites_existing() {
        let storage = Storage::open_in_memory().expect("in-memory storage");
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
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .expect("set app");
        storage
            .settings()
            .set("theme", &serde_json::json!("light"), "project:1")
            .expect("set project");
        let app = storage
            .settings()
            .get("theme", "app")
            .expect("get app")
            .expect("should exist");
        let proj = storage
            .settings()
            .get("theme", "project:1")
            .expect("get project")
            .expect("should exist");
        assert_eq!(app, serde_json::json!("dark"));
        assert_eq!(proj, serde_json::json!("light"));
    }

    #[test]
    fn get_all_returns_scope_only() {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .expect("set");
        storage
            .settings()
            .set("font_size", &serde_json::json!(14), "app")
            .expect("set");
        storage
            .settings()
            .set("other", &serde_json::json!("x"), "project:1")
            .expect("set other");
        let all = storage.settings().get_all("app").expect("get_all");
        assert_eq!(all.len(), 2);
        assert_eq!(all["theme"], serde_json::json!("dark"));
    }
}
