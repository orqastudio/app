// File-backed project settings store for the orqa-project crate.
//
// Provides `FileProjectSettingsStore`, a concrete implementation of
// `ProjectSettingsStore` that reads and writes `project.json` as
// `serde_json::Value`. The engine trait intentionally uses `Value` so that
// both the current `ProjectSettings` shape and future schema evolution are
// supported without coupling the engine to a specific settings struct.

use std::path::Path;

use orqa_engine_types::paths::{ORQA_DIR, SETTINGS_FILE};
use orqa_engine_types::traits::storage::ProjectSettingsStore;

/// Concrete error type for `FileProjectSettingsStore` operations.
#[derive(Debug)]
pub enum ProjectSettingsStoreError {
    /// An I/O error occurred while reading or writing the settings file.
    Io(std::io::Error),
    /// The settings file exists but contains malformed JSON.
    Serialization(String),
}

impl std::fmt::Display for ProjectSettingsStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Serialization(msg) => write!(f, "serialization error: {msg}"),
        }
    }
}

impl std::error::Error for ProjectSettingsStoreError {}

impl From<std::io::Error> for ProjectSettingsStoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for ProjectSettingsStoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

/// A file-backed implementation of `ProjectSettingsStore`.
///
/// Reads and writes `{project_root}/.orqa/project.json`. The return type
/// is `serde_json::Value` so any settings shape can be loaded without
/// this store coupling to a specific Rust struct.
#[derive(Default)]
pub struct FileProjectSettingsStore;

impl FileProjectSettingsStore {
    /// Create a new `FileProjectSettingsStore`.
    pub fn new() -> Self {
        Self
    }

    /// Write project settings to `{root}/.orqa/project.json`.
    ///
    /// Creates the `.orqa/` directory if it does not exist.
    /// Serialises `settings` to pretty-printed JSON.
    pub fn save(
        &self,
        root: &Path,
        settings: &serde_json::Value,
    ) -> Result<(), ProjectSettingsStoreError> {
        let orqa_dir = root.join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir)?;

        let settings_file = orqa_dir.join("project.json");
        let json = serde_json::to_string_pretty(settings)?;
        std::fs::write(&settings_file, json)?;
        Ok(())
    }
}

/// Implement the abstract `ProjectSettingsStore` trait using file-backed reads.
///
/// Reads `{root}/.orqa/project.json` as a `serde_json::Value`.
impl ProjectSettingsStore for FileProjectSettingsStore {
    type Error = ProjectSettingsStoreError;

    fn load(&self, root: &Path) -> Result<Option<serde_json::Value>, Self::Error> {
        let settings_file = root.join(SETTINGS_FILE);

        if !settings_file.exists() {
            return Ok(None);
        }

        let contents = std::fs::read_to_string(&settings_file)?;
        let value: serde_json::Value = serde_json::from_str(&contents)?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_nonexistent_returns_none() {
        let store = FileProjectSettingsStore::new();
        let result = store
            .load(Path::new("/nonexistent/path/that/does/not/exist"))
            .expect("should be Ok");
        assert!(result.is_none());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let store = FileProjectSettingsStore::new();

        let settings = serde_json::json!({
            "name": "test-project",
            "default_model": "auto",
            "excluded_paths": ["node_modules", ".git"]
        });

        store
            .save(tmp.path(), &settings)
            .expect("save should succeed");

        let loaded = store
            .load(tmp.path())
            .expect("load should succeed")
            .expect("settings should exist");

        assert_eq!(loaded["name"], "test-project");
        assert_eq!(loaded["default_model"], "auto");
        assert_eq!(
            loaded["excluded_paths"]
                .as_array()
                .expect("should be array")
                .len(),
            2
        );
    }

    #[test]
    fn malformed_json_returns_serialization_error() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create dirs");

        let settings_file = orqa_dir.join("project.json");
        std::fs::write(&settings_file, "{ invalid json }").expect("write bad json");

        let store = FileProjectSettingsStore::new();
        let result = store.load(tmp.path());
        assert!(result.is_err());
        assert!(matches!(
            result.expect_err("should be error"),
            ProjectSettingsStoreError::Serialization(_)
        ));
    }

    #[test]
    fn save_creates_orqa_dir_if_missing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let store = FileProjectSettingsStore::new();

        let settings = serde_json::json!({"name": "new-project"});
        store
            .save(tmp.path(), &settings)
            .expect("save should succeed");

        let settings_file = tmp.path().join(SETTINGS_FILE);
        assert!(
            settings_file.exists(),
            ".orqa/project.json should be created"
        );
    }
}
