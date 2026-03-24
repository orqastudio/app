//! Centralised project configuration loader.
//!
//! All code that needs to read `.orqa/project.json` MUST go through this module.
//! This ensures a single code path for file I/O and deserialisation, making it
//! straightforward to add caching or change-notification later.

use std::path::Path;

use crate::domain::paths;
use crate::domain::project_settings::ProjectSettings;
use crate::error::OrqaError;

/// Load and parse project settings from `{project_root}/.orqa/project.json`.
///
/// Returns `Ok(None)` if the settings file does not exist.
/// Returns `Err` if the file exists but cannot be read or parsed.
pub fn load_project_settings(project_root: &Path) -> Result<Option<ProjectSettings>, OrqaError> {
    let settings_file = project_root.join(paths::SETTINGS_FILE);

    if !settings_file.exists() {
        return Ok(None);
    }

    let contents = std::fs::read_to_string(&settings_file)?;
    let settings: ProjectSettings = serde_json::from_str(&contents)?;
    Ok(Some(settings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::paths::ORQA_DIR;
    use crate::domain::project_settings::ProjectSettings;

    fn minimal_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test".to_string(),
            dogfood: false,
            description: None,
            default_model: "auto".to_string(),
            excluded_paths: vec![],
            stack: None,
            governance: None,
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![],
            artifact_links: Default::default(),
            statuses: vec![],
            delivery: Default::default(),
            relationships: vec![],
            plugins: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn returns_none_when_no_settings_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let result = load_project_settings(tmp.path());
        assert!(result.is_ok());
        assert!(result.expect("should be Ok").is_none());
    }

    #[test]
    fn loads_valid_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");

        let settings = minimal_settings();
        let json = serde_json::to_string_pretty(&settings).expect("serialize");
        std::fs::write(orqa_dir.join("project.json"), json).expect("write");

        let loaded = load_project_settings(tmp.path())
            .expect("load should succeed")
            .expect("settings should exist");
        assert_eq!(loaded.name, "test");
    }

    #[test]
    fn returns_error_on_malformed_json() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");

        std::fs::write(orqa_dir.join("project.json"), "{ invalid json }").expect("write");

        let result = load_project_settings(tmp.path());
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, OrqaError::Serialization(_)));
    }
}
