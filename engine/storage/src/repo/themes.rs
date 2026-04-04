// Themes repository for orqa-storage.
//
// Provides read and write operations over `project_themes` and
// `project_theme_overrides`. Themes are design token maps extracted from source
// files (e.g., tailwind.config.ts). All SQL is ported from
// app/src-tauri/src/repo/theme_repo.rs.

use rusqlite::params;

use crate::Storage;
use crate::error::StorageError;

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

/// Zero-cost repository handle for theme tables.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::themes()`.
pub struct ThemeRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl ThemeRepo<'_> {
    /// Get all active themes for a project, ordered by source file path.
    pub fn get_themes(&self, project_id: i64) -> Result<Vec<ThemeRow>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, source_file, source_hash, extracted_at, \
                    tokens_light, tokens_dark, unmapped, is_active \
             FROM project_themes \
             WHERE project_id = ?1 AND is_active = 1 \
             ORDER BY source_file ASC",
        )?;

        let rows = stmt.query_map(params![project_id], |row| {
            let is_active: i32 = row.get(8)?;
            Ok(ThemeRow {
                id: row.get(0)?,
                project_id: row.get(1)?,
                source_file: row.get(2)?,
                source_hash: row.get(3)?,
                extracted_at: row.get(4)?,
                tokens_light: row.get(5)?,
                tokens_dark: row.get(6)?,
                unmapped: row.get(7)?,
                is_active: is_active != 0,
            })
        })?;

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Get all theme overrides for a project, ordered by token name.
    pub fn get_overrides(&self, project_id: i64) -> Result<Vec<ThemeOverrideRow>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, project_id, token_name, value_light, value_dark \
             FROM project_theme_overrides \
             WHERE project_id = ?1 \
             ORDER BY token_name ASC",
        )?;

        let rows = stmt.query_map(params![project_id], |row| {
            Ok(ThemeOverrideRow {
                id: row.get(0)?,
                project_id: row.get(1)?,
                token_name: row.get(2)?,
                value_light: row.get(3)?,
                value_dark: row.get(4)?,
            })
        })?;

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Set (upsert) a theme override for a specific token.
    pub fn set_override(
        &self,
        project_id: i64,
        token_name: &str,
        value_light: &str,
        value_dark: Option<&str>,
    ) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO project_theme_overrides \
             (project_id, token_name, value_light, value_dark, updated_at) \
             VALUES (?1, ?2, ?3, ?4, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
             ON CONFLICT(project_id, token_name) DO UPDATE SET \
             value_light = excluded.value_light, \
             value_dark = excluded.value_dark, \
             updated_at = excluded.updated_at",
            params![project_id, token_name, value_light, value_dark],
        )?;
        Ok(())
    }

    /// Clear all theme overrides for a project.
    pub fn clear_overrides(&self, project_id: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "DELETE FROM project_theme_overrides WHERE project_id = ?1",
            params![project_id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    fn setup() -> Storage {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .expect("create project");
        storage
    }

    #[test]
    fn empty_themes() {
        let storage = setup();
        let themes = storage.themes().get_themes(1).expect("get_themes");
        assert!(themes.is_empty());
    }

    #[test]
    fn set_and_get_override() {
        let storage = setup();
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", Some("#00ff00"))
            .expect("set override");
        let overrides = storage.themes().get_overrides(1).expect("get_overrides");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].token_name, "primary");
        assert_eq!(overrides[0].value_light, "#ff0000");
        assert_eq!(overrides[0].value_dark.as_deref(), Some("#00ff00"));
    }

    #[test]
    fn set_override_upserts() {
        let storage = setup();
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", None)
            .expect("first");
        storage
            .themes()
            .set_override(1, "primary", "#0000ff", Some("#ffffff"))
            .expect("second");
        let overrides = storage.themes().get_overrides(1).expect("get");
        assert_eq!(overrides.len(), 1);
        assert_eq!(overrides[0].value_light, "#0000ff");
    }

    #[test]
    fn clear_overrides_works() {
        let storage = setup();
        storage
            .themes()
            .set_override(1, "primary", "#ff0000", None)
            .expect("set 1");
        storage
            .themes()
            .set_override(1, "secondary", "#00ff00", None)
            .expect("set 2");
        storage.themes().clear_overrides(1).expect("clear");
        let after = storage.themes().get_overrides(1).expect("get");
        assert!(after.is_empty());
    }
}
