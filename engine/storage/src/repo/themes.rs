// Themes repository for orqa-storage.
//
// Provides async read and write operations over `project_themes` and
// `project_theme_overrides`. Themes are design token maps extracted from
// source files (e.g., tailwind.config.ts).

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use crate::error::StorageError;
use crate::traits::ThemeRepository;

/// A raw theme row from the `project_themes` table.
#[derive(Debug, Clone)]
pub struct ThemeRow {
    /// Primary key.
    pub id: i64,
    /// Foreign key to the `projects` table.
    pub project_id: i64,
    /// Path to the source file this theme was extracted from.
    pub source_file: String,
    /// Hash of the source file at extraction time, for change detection.
    pub source_hash: String,
    /// ISO-8601 timestamp when this theme was extracted.
    pub extracted_at: String,
    /// JSON-encoded light-mode design token map.
    pub tokens_light: String,
    /// JSON-encoded dark-mode design token map, if present.
    pub tokens_dark: Option<String>,
    /// JSON-encoded list of unmapped token names, if any.
    pub unmapped: Option<String>,
    /// Whether this is the currently active theme for the project.
    pub is_active: bool,
}

/// A raw override row from the `project_theme_overrides` table.
#[derive(Debug, Clone)]
pub struct ThemeOverrideRow {
    /// Primary key.
    pub id: i64,
    /// Foreign key to the `projects` table.
    pub project_id: i64,
    /// Design token name being overridden (e.g., "primary").
    pub token_name: String,
    /// Override value for light mode.
    pub value_light: String,
    /// Override value for dark mode, if set.
    pub value_dark: Option<String>,
}

/// Async repository handle for theme tables.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::themes()`.
pub struct ThemeRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to a `ThemeRow`.
fn map_theme_row(row: &sea_orm::QueryResult) -> Result<ThemeRow, StorageError> {
    let is_active: i64 = row
        .try_get("", "is_active")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(ThemeRow {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        project_id: row
            .try_get("", "project_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        source_file: row
            .try_get("", "source_file")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        source_hash: row
            .try_get("", "source_hash")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        extracted_at: row
            .try_get("", "extracted_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tokens_light: row
            .try_get("", "tokens_light")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        tokens_dark: row
            .try_get("", "tokens_dark")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        unmapped: row
            .try_get("", "unmapped")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        is_active: is_active != 0,
    })
}

/// Map a SeaORM `QueryResult` row to a `ThemeOverrideRow`.
fn map_override_row(row: &sea_orm::QueryResult) -> Result<ThemeOverrideRow, StorageError> {
    Ok(ThemeOverrideRow {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        project_id: row
            .try_get("", "project_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        token_name: row
            .try_get("", "token_name")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        value_light: row
            .try_get("", "value_light")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        value_dark: row
            .try_get("", "value_dark")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

#[async_trait::async_trait]
impl ThemeRepository for ThemeRepo {
    /// Get all active themes for a project, ordered by source file path.
    async fn get_themes(&self, project_id: i64) -> Result<Vec<ThemeRow>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, project_id, source_file, source_hash, extracted_at, \
                        tokens_light, tokens_dark, unmapped, is_active \
                 FROM project_themes \
                 WHERE project_id = ? AND is_active = 1 \
                 ORDER BY source_file ASC",
                [project_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_theme_row).collect()
    }

    /// Get all theme overrides for a project, ordered by token name.
    async fn get_overrides(&self, project_id: i64) -> Result<Vec<ThemeOverrideRow>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, project_id, token_name, value_light, value_dark \
                 FROM project_theme_overrides \
                 WHERE project_id = ? \
                 ORDER BY token_name ASC",
                [project_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(map_override_row).collect()
    }

    /// Set (upsert) a theme override for a specific token.
    async fn set_override(
        &self,
        project_id: i64,
        token_name: &str,
        value_light: &str,
        value_dark: Option<&str>,
    ) -> Result<(), StorageError> {
        let dark_val: sea_orm::Value = match value_dark {
            Some(d) => d.into(),
            None => sea_orm::Value::String(None),
        };
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO project_theme_overrides \
                 (project_id, token_name, value_light, value_dark, updated_at) \
                 VALUES (?, ?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
                 ON CONFLICT(project_id, token_name) DO UPDATE SET \
                 value_light = excluded.value_light, \
                 value_dark = excluded.value_dark, \
                 updated_at = excluded.updated_at",
                [
                    project_id.into(),
                    token_name.into(),
                    value_light.into(),
                    dark_val,
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Clear all theme overrides for a project.
    async fn clear_overrides(&self, project_id: i64) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM project_theme_overrides WHERE project_id = ?",
                [project_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{ProjectRepository, ThemeRepository};
    use crate::Storage;

    async fn setup() -> Storage {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create project");
        storage
    }

    #[tokio::test]
    async fn empty_themes() {
        let storage = setup().await;
        let themes = storage.themes().get_themes(1).await.expect("get_themes");
        assert!(themes.is_empty());
    }

    #[tokio::test]
    async fn set_and_get_override() {
        let storage = setup().await;
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", Some("#00ff00"))
            .await
            .expect("set override");
        let overrides = storage
            .themes()
            .get_overrides(1)
            .await
            .expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].token_name, "primary");
        assert_eq!(overrides[0].value_light, "#ff0000");
        assert_eq!(overrides[0].value_dark.as_deref(), Some("#00ff00"));
    }

    #[tokio::test]
    async fn set_override_upserts() {
        let storage = setup().await;
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", None)
            .await
            .expect("first");
        storage
            .themes()
            .set_override(1, "primary", "#0000ff", Some("#ffffff"))
            .await
            .expect("second");
        let overrides = storage.themes().get_overrides(1).await.expect("get");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].value_light, "#0000ff");
    }

    #[tokio::test]
    async fn clear_overrides_works() {
        let storage = setup().await;
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", None)
            .await
            .expect("set 1");
        storage
            .themes()
            .set_override(1, "secondary", "#00ff00", None)
            .await
            .expect("set 2");
        storage.themes().clear_overrides(1).await.expect("clear");
        let after = storage.themes().get_overrides(1).await.expect("get");
        assert!(after.is_empty());
    }
}
