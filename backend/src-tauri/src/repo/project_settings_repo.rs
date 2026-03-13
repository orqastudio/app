use std::path::Path;

use crate::domain::paths;
use crate::domain::project_settings::ProjectSettings;
use crate::error::OrqaError;

/// Read project settings from `{project_path}/.orqa/project.json`.
///
/// Returns `Ok(None)` if the file does not exist.
/// Returns `Err(OrqaError::Serialization(...))` if JSON is malformed.
pub fn read(project_path: &str) -> Result<Option<ProjectSettings>, OrqaError> {
    let settings_file = Path::new(project_path).join(paths::SETTINGS_FILE);

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
pub fn write(project_path: &str, settings: &ProjectSettings) -> Result<(), OrqaError> {
    let orqa_dir = Path::new(project_path).join(paths::ORQA_DIR);
    std::fs::create_dir_all(&orqa_dir)?;

    let settings_file = orqa_dir.join("project.json");
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write(&settings_file, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project::DetectedStack;
    use crate::domain::project_settings::GovernanceCounts;

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test-project".to_string(),
            description: Some("A test project".to_string()),
            default_model: "auto".to_string(),
            excluded_paths: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            stack: Some(DetectedStack {
                languages: vec!["rust".to_string()],
                frameworks: vec!["tauri".to_string()],
                package_manager: Some("cargo".to_string()),
                has_claude_config: true,
                has_design_tokens: false,
            }),
            governance: Some(GovernanceCounts {
                lessons: 10,
                decisions: 5,
                agents: 3,
                rules: 5,
                skills: 2,
                has_claude_config: true,
            }),
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![],
        }
    }

    #[test]
    fn read_nonexistent_returns_none() {
        let result = read("/nonexistent/path/that/does/not/exist");
        assert!(result.is_ok());
        assert!(result.expect("should be Ok").is_none());
    }

    #[test]
    fn write_and_read_roundtrip() {
        let tmp = std::env::temp_dir().join("orqa_repo_settings_roundtrip");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("create temp dir");

        let settings = sample_settings();
        let tmp_str = tmp.to_str().expect("temp path as str");

        write(tmp_str, &settings).expect("write should succeed");

        let read_back = read(tmp_str)
            .expect("read should succeed")
            .expect("settings should exist");

        assert_eq!(read_back.name, "test-project");
        assert_eq!(read_back.default_model, "auto");
        assert_eq!(read_back.excluded_paths.len(), 5);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn malformed_json_returns_serialization_error() {
        let tmp = std::env::temp_dir().join("orqa_repo_settings_malformed");
        let _ = std::fs::remove_dir_all(&tmp);
        let orqa_dir = tmp.join(paths::ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create dirs");

        let settings_file = orqa_dir.join("project.json");
        std::fs::write(&settings_file, "{ invalid json }").expect("write bad json");

        let tmp_str = tmp.to_str().expect("temp path as str");
        let result = read(tmp_str);
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, OrqaError::Serialization(_)));

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
