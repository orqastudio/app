use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::domain::project_settings::{ArtifactEntry, ProjectSettings};
use crate::error::OrqaError;

/// Central directory name for Orqa project configuration and metadata.
pub const ORQA_DIR: &str = ".orqa";

/// Path to the project settings file, relative to the project root.
pub const SETTINGS_FILE: &str = ".orqa/project.json";

/// Runtime path cache built from the `artifacts` array in `project.json`.
///
/// Replaces hardcoded path constants with config-driven lookup.
/// Constructed at startup by reading `project.json` and flattening
/// the artifacts tree into a key-to-path map.
#[derive(Debug, Clone)]
pub struct ProjectPaths {
    /// Absolute path to the project root.
    project_root: PathBuf,
    /// Artifact key → relative path (e.g. "lessons" → ".orqa/process/lessons").
    artifact_paths: HashMap<String, String>,
}

impl ProjectPaths {
    /// Build a `ProjectPaths` from the project root directory.
    ///
    /// Reads `project.json`, extracts artifact paths, and creates the lookup map.
    /// Returns an error if the settings file cannot be read or parsed.
    /// Returns an empty path map if no settings file exists.
    pub fn load(project_root: &Path) -> Result<Self, OrqaError> {
        let settings_file = project_root.join(SETTINGS_FILE);

        if !settings_file.exists() {
            return Ok(Self {
                project_root: project_root.to_path_buf(),
                artifact_paths: HashMap::new(),
            });
        }

        let contents = std::fs::read_to_string(&settings_file)?;
        let settings: ProjectSettings = serde_json::from_str(&contents)?;

        Ok(Self::from_settings(project_root, &settings))
    }

    /// Build a `ProjectPaths` from an already-loaded `ProjectSettings`.
    ///
    /// Useful when the settings have already been read (avoids double file I/O).
    pub fn from_settings(project_root: &Path, settings: &ProjectSettings) -> Self {
        let mut artifact_paths = HashMap::new();

        for entry in &settings.artifacts {
            match entry {
                ArtifactEntry::Group { children, .. } => {
                    for child in children {
                        artifact_paths.insert(child.key.clone(), child.path.clone());
                    }
                }
                ArtifactEntry::Type(config) => {
                    artifact_paths.insert(config.key.clone(), config.path.clone());
                }
            }
        }

        Self {
            project_root: project_root.to_path_buf(),
            artifact_paths,
        }
    }

    /// Resolve the absolute path for an artifact key.
    ///
    /// Returns `None` if the key is not found in the config.
    pub fn artifact_dir(&self, key: &str) -> Option<PathBuf> {
        self.artifact_paths
            .get(key)
            .map(|rel| self.project_root.join(rel))
    }

    /// Get the relative path string for an artifact key.
    ///
    /// Returns `None` if the key is not found in the config.
    pub fn artifact_relative_path(&self, key: &str) -> Option<&str> {
        self.artifact_paths.get(key).map(String::as_str)
    }

    /// Return the project root path.
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project_settings::{ArtifactEntry, ArtifactTypeConfig, ProjectSettings};

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test".to_string(),
            description: None,
            default_model: "auto".to_string(),
            excluded_paths: vec![],
            stack: None,
            governance: None,
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![
                ArtifactEntry::Group {
                    key: "process".to_string(),
                    label: None,
                    icon: None,
                    children: vec![
                        ArtifactTypeConfig {
                            key: "lessons".to_string(),
                            label: None,
                            icon: None,
                            path: ".orqa/process/lessons".to_string(),
                        },
                        ArtifactTypeConfig {
                            key: "decisions".to_string(),
                            label: None,
                            icon: None,
                            path: ".orqa/process/decisions".to_string(),
                        },
                    ],
                },
                ArtifactEntry::Type(ArtifactTypeConfig {
                    key: "docs".to_string(),
                    label: None,
                    icon: None,
                    path: ".orqa/documentation".to_string(),
                }),
            ],
            artifact_links: Default::default(),
            statuses: vec![],
            delivery: Default::default(),
        }
    }

    #[test]
    fn from_settings_builds_flat_map() {
        let root = Path::new("/projects/my-app");
        let settings = sample_settings();
        let paths = ProjectPaths::from_settings(root, &settings);

        assert_eq!(
            paths.artifact_relative_path("lessons"),
            Some(".orqa/process/lessons")
        );
        assert_eq!(
            paths.artifact_relative_path("decisions"),
            Some(".orqa/process/decisions")
        );
        assert_eq!(
            paths.artifact_relative_path("docs"),
            Some(".orqa/documentation")
        );
    }

    #[test]
    fn artifact_dir_resolves_absolute_path() {
        let root = Path::new("/projects/my-app");
        let settings = sample_settings();
        let paths = ProjectPaths::from_settings(root, &settings);

        assert_eq!(
            paths.artifact_dir("lessons"),
            Some(PathBuf::from("/projects/my-app/.orqa/process/lessons"))
        );
    }

    #[test]
    fn unknown_key_returns_none() {
        let root = Path::new("/projects/my-app");
        let settings = sample_settings();
        let paths = ProjectPaths::from_settings(root, &settings);

        assert!(paths.artifact_dir("nonexistent").is_none());
        assert!(paths.artifact_relative_path("nonexistent").is_none());
    }

    #[test]
    fn empty_artifacts_produces_empty_map() {
        let root = Path::new("/projects/my-app");
        let settings = ProjectSettings {
            name: "empty".to_string(),
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
        };
        let paths = ProjectPaths::from_settings(root, &settings);

        assert!(paths.artifact_dir("lessons").is_none());
    }

    #[test]
    fn load_returns_empty_when_no_settings_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let paths = ProjectPaths::load(tmp.path()).expect("load should succeed");
        assert!(paths.artifact_dir("lessons").is_none());
    }

    #[test]
    fn load_reads_from_disk() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");

        let settings = sample_settings();
        let json = serde_json::to_string_pretty(&settings).expect("serialize");
        std::fs::write(orqa_dir.join("project.json"), json).expect("write");

        let paths = ProjectPaths::load(tmp.path()).expect("load should succeed");
        assert_eq!(
            paths.artifact_relative_path("lessons"),
            Some(".orqa/process/lessons")
        );
    }

    #[test]
    fn project_root_returns_root_path() {
        let root = Path::new("/projects/my-app");
        let settings = sample_settings();
        let paths = ProjectPaths::from_settings(root, &settings);
        assert_eq!(paths.project_root(), root);
    }
}
