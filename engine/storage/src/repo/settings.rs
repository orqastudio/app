// Settings repository for orqa-storage.
//
// Provides async get/set/list operations over the `settings` table. Settings
// are key-value pairs with a scope (e.g., "app", "project:123"). Values are
// stored as JSON strings and deserialized on read.

use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use crate::error::StorageError;
use crate::traits::SettingsRepository;

/// Async repository handle for the `settings` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::settings()`.
pub struct SettingsRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepo {
    /// Get a single setting value by key and scope.
    ///
    /// Returns `None` if the key does not exist in the given scope.
    async fn get(&self, key: &str, scope: &str) -> Result<Option<serde_json::Value>, StorageError> {
        let row = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT value FROM settings WHERE key = ? AND scope = ?",
                [key.into(), scope.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(r) => {
                let value_str: String = r
                    .try_get("", "value")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
                Ok(Some(parsed))
            }
        }
    }

    /// Set a setting value (upsert by key + scope).
    async fn set(
        &self,
        key: &str,
        value: &serde_json::Value,
        scope: &str,
    ) -> Result<(), StorageError> {
        let value_str = serde_json::to_string(value)?;
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO settings (key, value, scope, updated_at) \
                 VALUES (?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
                 ON CONFLICT(key, scope) DO UPDATE SET \
                 value = excluded.value, updated_at = excluded.updated_at",
                [key.into(), value_str.into(), scope.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Get all settings regardless of scope, ordered by key.
    ///
    /// Keys from different scopes may collide — callers should prefer `get_all`
    /// with an explicit scope when the scope is known.
    async fn get_all_any_scope(&self) -> Result<HashMap<String, serde_json::Value>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT key, value FROM settings ORDER BY key ASC".to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter()
            .map(|row| {
                let key: String = row
                    .try_get("", "key")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let value_str: String = row
                    .try_get("", "value")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
                Ok((key, parsed))
            })
            .collect()
    }

    /// Get all settings for a given scope, ordered by key.
    async fn get_all(
        &self,
        scope: &str,
    ) -> Result<HashMap<String, serde_json::Value>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT key, value FROM settings WHERE scope = ? ORDER BY key ASC",
                [scope.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter()
            .map(|row| {
                let key: String = row
                    .try_get("", "key")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let value_str: String = row
                    .try_get("", "value")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
                Ok((key, parsed))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::SettingsRepository;
    use crate::Storage;

    #[tokio::test]
    async fn get_nonexistent_returns_none() {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        let result = storage.settings().get("theme", "app").await.expect("get");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn set_and_get_string() {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        let value = serde_json::json!("dark");
        storage
            .settings()
            .set("theme", &value, "app")
            .await
            .expect("set");
        let fetched = storage
            .settings()
            .get("theme", "app")
            .await
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[tokio::test]
    async fn set_overwrites_existing() {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .settings()
            .set("theme", &serde_json::json!("light"), "app")
            .await
            .expect("set 1");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .await
            .expect("set 2");
        let fetched = storage
            .settings()
            .get("theme", "app")
            .await
            .expect("get")
            .expect("should exist");
        assert_eq!(fetched, serde_json::json!("dark"));
    }

    #[tokio::test]
    async fn scopes_are_independent() {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .await
            .expect("set app");
        storage
            .settings()
            .set("theme", &serde_json::json!("light"), "project:1")
            .await
            .expect("set project");
        let app = storage
            .settings()
            .get("theme", "app")
            .await
            .expect("get app")
            .expect("should exist");
        let proj = storage
            .settings()
            .get("theme", "project:1")
            .await
            .expect("get project")
            .expect("should exist");
        assert_eq!(app, serde_json::json!("dark"));
        assert_eq!(proj, serde_json::json!("light"));
    }

    #[tokio::test]
    async fn get_all_returns_scope_only() {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .settings()
            .set("theme", &serde_json::json!("dark"), "app")
            .await
            .expect("set");
        storage
            .settings()
            .set("font_size", &serde_json::json!(14), "app")
            .await
            .expect("set");
        storage
            .settings()
            .set("other", &serde_json::json!("x"), "project:1")
            .await
            .expect("set other");
        let all = storage.settings().get_all("app").await.expect("get_all");
        assert_eq!(all.len(), 2);
        assert_eq!(all["theme"], serde_json::json!("dark"));
    }
}
