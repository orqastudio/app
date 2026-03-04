use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::domain::project::DetectedStack;
use crate::error::OrqaError;

/// Governance artifact counts for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceCounts {
    pub docs: u32,
    pub agents: u32,
    pub rules: u32,
    pub skills: u32,
    pub hooks: u32,
    pub has_claude_config: bool,
}

/// File-based project settings stored in `{project}/.orqa/project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_model")]
    pub default_model: String,
    #[serde(default = "default_excluded_paths")]
    pub excluded_paths: Vec<String>,
    #[serde(default)]
    pub stack: Option<DetectedStack>,
    #[serde(default)]
    pub governance: Option<GovernanceCounts>,
    #[serde(default)]
    pub icon: Option<String>,
}

fn default_model() -> String {
    "auto".to_string()
}

fn default_excluded_paths() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        ".git".to_string(),
        "target".to_string(),
        "dist".to_string(),
        "build".to_string(),
    ]
}

/// Read project settings from `{project_path}/.orqa/project.json`.
///
/// Returns `Ok(None)` if the file does not exist.
/// Returns `Err(OrqaError::Serialization(...))` if JSON is malformed.
pub fn read_settings(project_path: &str) -> Result<Option<ProjectSettings>, OrqaError> {
    let settings_file = Path::new(project_path).join(".orqa").join("project.json");

    if !settings_file.exists() {
        return Ok(None);
    }

    let contents = std::fs::read_to_string(&settings_file)?;
    let settings: ProjectSettings = serde_json::from_str(&contents)?;
    Ok(Some(settings))
}

/// Write project settings to `{project_path}/.orqa/project.json`.
///
/// Creates the `.orqa/` directory if it does not exist.
pub fn write_settings(project_path: &str, settings: &ProjectSettings) -> Result<(), OrqaError> {
    let orqa_dir = Path::new(project_path).join(".orqa");
    std::fs::create_dir_all(&orqa_dir)?;

    let settings_file = orqa_dir.join("project.json");
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write(&settings_file, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test-project".to_string(),
            description: Some("A test project".to_string()),
            default_model: "auto".to_string(),
            excluded_paths: default_excluded_paths(),
            stack: Some(DetectedStack {
                languages: vec!["rust".to_string()],
                frameworks: vec!["tauri".to_string()],
                package_manager: Some("cargo".to_string()),
                has_claude_config: true,
                has_design_tokens: false,
            }),
            governance: Some(GovernanceCounts {
                docs: 10,
                agents: 3,
                rules: 5,
                skills: 2,
                hooks: 1,
                has_claude_config: true,
            }),
            icon: None,
        }
    }

    #[test]
    fn roundtrip_serialization() {
        let settings = sample_settings();
        let json = serde_json::to_string_pretty(&settings).expect("serialization should succeed");
        let deserialized: ProjectSettings =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.name, settings.name);
        assert_eq!(deserialized.description, settings.description);
        assert_eq!(deserialized.default_model, settings.default_model);
        assert_eq!(deserialized.excluded_paths, settings.excluded_paths);
        assert!(deserialized.stack.is_some());
        assert!(deserialized.governance.is_some());

        let gov = deserialized.governance.as_ref().expect("governance");
        assert_eq!(gov.docs, 10);
        assert_eq!(gov.agents, 3);
        assert!(gov.has_claude_config);
    }

    #[test]
    fn read_settings_nonexistent_returns_none() {
        let result = read_settings("/nonexistent/path/that/does/not/exist");
        assert!(result.is_ok());
        assert!(result.expect("should be Ok").is_none());
    }

    #[test]
    fn write_and_read_roundtrip() {
        let tmp = std::env::temp_dir().join("forge_test_settings_roundtrip");
        // Clean up from any prior failed run
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("create temp dir");

        let settings = sample_settings();
        let tmp_str = tmp.to_str().expect("temp path as str");

        write_settings(tmp_str, &settings).expect("write should succeed");

        let read_back = read_settings(tmp_str)
            .expect("read should succeed")
            .expect("settings should exist");

        assert_eq!(read_back.name, "test-project");
        assert_eq!(read_back.default_model, "auto");
        assert_eq!(read_back.excluded_paths.len(), 5);

        // Cleanup
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn malformed_json_returns_serialization_error() {
        let tmp = std::env::temp_dir().join("forge_test_settings_malformed");
        let _ = std::fs::remove_dir_all(&tmp);
        let orqa_dir = tmp.join(".orqa");
        std::fs::create_dir_all(&orqa_dir).expect("create dirs");

        let settings_file = orqa_dir.join("project.json");
        std::fs::write(&settings_file, "{ invalid json }").expect("write bad json");

        let tmp_str = tmp.to_str().expect("temp path as str");
        let result = read_settings(tmp_str);
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, OrqaError::Serialization(_)));

        // Cleanup
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn serde_defaults_applied_for_missing_fields() {
        let json = r#"{"name": "minimal"}"#;
        let settings: ProjectSettings =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(settings.name, "minimal");
        assert!(settings.description.is_none());
        assert_eq!(settings.default_model, "auto");
        assert_eq!(settings.excluded_paths.len(), 5);
        assert!(settings.stack.is_none());
        assert!(settings.governance.is_none());
    }
}
