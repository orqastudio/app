// Path resolution for OrqaStudio projects.
//
// Provides constants for well-known paths within an OrqaStudio project, and
// the `ProjectPaths` struct that builds a runtime path cache from `project.json`.
// All path resolution goes through this module so path constants are defined
// in one place and the config-driven lookup has a single code path.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::{load_project_settings, ArtifactEntry, ProjectSettings};
use crate::error::EngineError;

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
    /// Artifact key -> relative path (e.g. "lessons" -> ".orqa/learning/lessons").
    artifact_paths: HashMap<String, String>,
}

impl ProjectPaths {
    /// Build a `ProjectPaths` from the project root directory.
    ///
    /// Delegates to [`load_project_settings`] for file I/O.
    /// Returns an error if the settings file cannot be read or parsed.
    /// Returns an empty path map if no settings file exists.
    pub fn load(project_root: &Path) -> Result<Self, EngineError> {
        match load_project_settings(project_root)? {
            Some(settings) => Ok(Self::from_settings(project_root, &settings)),
            None => Ok(Self {
                project_root: project_root.to_path_buf(),
                artifact_paths: HashMap::new(),
            }),
        }
    }

    /// Build a `ProjectPaths` from an already-loaded `ProjectSettings`.
    ///
    /// Useful when the settings have already been read (avoids double file I/O).
    pub fn from_settings(project_root: &Path, settings: &ProjectSettings) -> Self {
        // Flatten each entry to (key, path) pairs: groups expand their children,
        // single types contribute one pair each.
        let artifact_paths: HashMap<String, String> = settings
            .artifacts
            .iter()
            .flat_map(|entry| match entry {
                ArtifactEntry::Group { children, .. } => children
                    .iter()
                    .map(|child| (child.key.clone(), child.path.clone()))
                    .collect::<Vec<_>>(),
                ArtifactEntry::Type(config) => {
                    vec![(config.key.clone(), config.path.clone())]
                }
            })
            .collect();

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
    use crate::config::{ArtifactEntry, ArtifactTypeConfig, DeliveryConfig, ProjectSettings};

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test".to_owned(),
            organisation: false,
            projects: vec![],
            artifacts: vec![
                ArtifactEntry::Group {
                    key: "learning".to_owned(),
                    label: None,
                    icon: None,
                    children: vec![
                        ArtifactTypeConfig {
                            key: "lessons".to_owned(),
                            label: None,
                            icon: None,
                            path: ".orqa/learning/lessons".to_owned(),
                        },
                        ArtifactTypeConfig {
                            key: "decisions".to_owned(),
                            label: None,
                            icon: None,
                            path: ".orqa/learning/decisions".to_owned(),
                        },
                    ],
                },
                ArtifactEntry::Type(ArtifactTypeConfig {
                    key: "docs".to_owned(),
                    label: None,
                    icon: None,
                    path: ".orqa/documentation".to_owned(),
                }),
            ],
            statuses: vec![],
            delivery: DeliveryConfig::default(),
            relationships: vec![],
            plugins: HashMap::new(),
        }
    }

    #[test]
    fn from_settings_builds_flat_map() {
        let root = Path::new("/projects/my-app");
        let settings = sample_settings();
        let paths = ProjectPaths::from_settings(root, &settings);

        assert_eq!(
            paths.artifact_relative_path("lessons"),
            Some(".orqa/learning/lessons")
        );
        assert_eq!(
            paths.artifact_relative_path("decisions"),
            Some(".orqa/learning/decisions")
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
            Some(PathBuf::from("/projects/my-app/.orqa/learning/lessons"))
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
            name: "empty".to_owned(),
            organisation: false,
            projects: vec![],
            artifacts: vec![],
            statuses: vec![],
            delivery: DeliveryConfig::default(),
            relationships: vec![],
            plugins: HashMap::new(),
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
            Some(".orqa/learning/lessons")
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
