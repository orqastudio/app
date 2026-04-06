//! Governance scanner for the OrqaStudio enforcement engine.
//!
//! Scans a project directory for governance files across the artifact areas defined
//! in the project's artifacts config. The set of areas is entirely driven by the
//! `ArtifactEntry` config passed by the caller — no governance areas are hardcoded.
//! Used by the enforcement pipeline to determine governance coverage and surface files
//! to the governance health UI.

use std::path::Path;

use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::governance::{GovernanceArea, GovernanceFile, GovernanceScanResult};
use orqa_validation::settings::ArtifactEntry;

/// Maximum number of characters to include in a content preview.
const CONTENT_PREVIEW_CHARS: usize = 500;

/// Scan a project directory for governance files across the artifact areas defined
/// in the project's artifacts config.
///
/// Each leaf `ArtifactTypeConfig` in `artifacts` becomes one `GovernanceArea`. Groups
/// are flattened to their children. Coverage ratio is computed as covered areas divided
/// by total areas from the config. When `artifacts` is empty the result has zero areas
/// and a coverage ratio of 0.0.
///
/// # Filesystem dependency
///
/// This function performs filesystem I/O (directory listing, file metadata reads, and
/// content previews). The dependency is intentional — governance scanning is inherently
/// a filesystem operation whose purpose is to walk and inspect local project files.
/// It does not access any database or network resource.
pub fn scan_governance(
    project_path: &Path,
    artifacts: &[ArtifactEntry],
) -> Result<GovernanceScanResult, EngineError> {
    if !project_path.exists() || !project_path.is_dir() {
        return Err(EngineError::Validation(format!(
            "project path does not exist or is not a directory: {}",
            project_path.display()
        )));
    }

    let areas = scan_artifact_areas(project_path, artifacts);
    let total = areas.len();
    let covered = areas.iter().filter(|a| a.covered).count();
    let coverage_ratio = if total == 0 {
        0.0
    } else {
        covered as f64 / total as f64
    };

    Ok(GovernanceScanResult {
        areas,
        coverage_ratio,
    })
}

/// Flatten artifact entries to leaf types and scan each as a governance area.
///
/// Groups contribute their children as individual areas. Type entries contribute
/// one area each. Each area is scanned recursively so that nested directory layouts
/// (e.g. documentation with architecture/product subdirs) are fully covered.
fn scan_artifact_areas(project_path: &Path, artifacts: &[ArtifactEntry]) -> Vec<GovernanceArea> {
    artifacts
        .iter()
        .flat_map(|entry| match entry {
            ArtifactEntry::Group {
                key: _,
                label: _,
                icon: _,
                children,
            } => children
                .iter()
                .map(|child| {
                    let dir = project_path.join(&child.path);
                    scan_recursive_area(&child.key, &dir)
                })
                .collect::<Vec<_>>(),
            ArtifactEntry::Type(type_config) => {
                let dir = project_path.join(&type_config.path);
                vec![scan_recursive_area(&type_config.key, &dir)]
            }
        })
        .collect()
}

/// Scan a directory tree recursively for `.md` governance files.
///
/// The area is considered covered if at least one `.md` file is found. Files at
/// any depth below `dir` are included.
fn scan_recursive_area(name: &str, dir: &Path) -> GovernanceArea {
    let files = if dir.is_dir() {
        let mut collected = collect_files_recursive(dir);
        collected.sort_by(|a, b| a.path.cmp(&b.path));
        collected
    } else {
        Vec::new()
    };

    let covered = !files.is_empty();
    GovernanceArea {
        name: name.to_owned(),
        source: "orqa".to_owned(),
        files,
        covered,
    }
}

/// Walk `dir` recursively and return all `.md` files found.
fn collect_files_recursive(dir: &Path) -> Vec<GovernanceFile> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                collect_files_recursive(&path)
            } else if path.is_file() {
                let is_md = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e == "md");
                if is_md {
                    read_governance_file(&path)
                        .map(|f| vec![f])
                        .unwrap_or_default()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        })
        .collect()
}

/// Read a governance file using its absolute path as the stored path.
///
/// Returns `None` if the file metadata cannot be read (e.g. permissions error).
/// If the file content cannot be read as UTF-8, the preview is left empty and
/// `size_bytes` still reflects the true file size from metadata.
fn read_governance_file(path: &Path) -> Option<GovernanceFile> {
    let metadata = std::fs::metadata(path).ok()?;
    let size_bytes = metadata.len();
    let content_preview = read_preview(path);
    Some(GovernanceFile {
        path: path.to_string_lossy().into_owned(),
        size_bytes,
        content_preview,
    })
}

/// Read and truncate file content to `CONTENT_PREVIEW_CHARS` characters.
///
/// Returns an empty string if the file cannot be read or is not valid UTF-8,
/// rather than silently producing a `GovernanceFile` whose `content_preview`
/// does not match the non-zero `size_bytes`.
fn read_preview(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(raw) => truncate_to_chars(&raw, CONTENT_PREVIEW_CHARS),
        Err(_) => String::new(),
    }
}

/// Truncate a string to at most `max_chars` Unicode scalar values.
fn truncate_to_chars(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_validation::settings::ArtifactTypeConfig;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("orqa_gov_scanner_test_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    /// Build a minimal 3-area artifacts config for tests: rules, lessons, docs.
    fn test_artifacts() -> Vec<ArtifactEntry> {
        vec![
            ArtifactEntry::Group {
                key: "learning".to_owned(),
                label: Some("Learning".to_owned()),
                icon: None,
                children: vec![
                    ArtifactTypeConfig {
                        key: "rules".to_owned(),
                        label: Some("Rules".to_owned()),
                        icon: None,
                        path: ".orqa/learning/rules".to_owned(),
                    },
                    ArtifactTypeConfig {
                        key: "lessons".to_owned(),
                        label: Some("Lessons".to_owned()),
                        icon: None,
                        path: ".orqa/learning/lessons".to_owned(),
                    },
                ],
            },
            ArtifactEntry::Type(ArtifactTypeConfig {
                key: "docs".to_owned(),
                label: Some("Documentation".to_owned()),
                icon: None,
                path: ".orqa/documentation".to_owned(),
            }),
        ]
    }

    #[test]
    fn empty_project_has_zero_coverage() {
        let dir = create_test_dir("empty");
        let artifacts = test_artifacts();
        let result = scan_governance(&dir, &artifacts).expect("scan");

        assert!((result.coverage_ratio - 0.0_f64).abs() < f64::EPSILON);
        assert_eq!(result.areas.len(), 3);
        for area in &result.areas {
            assert!(!area.covered);
        }

        cleanup(&dir);
    }

    #[test]
    fn empty_artifacts_config_returns_zero_areas() {
        let dir = create_test_dir("no_artifacts");
        let result = scan_governance(&dir, &[]).expect("scan");

        assert_eq!(result.areas.len(), 0);
        assert!((result.coverage_ratio - 0.0_f64).abs() < f64::EPSILON);

        cleanup(&dir);
    }

    #[test]
    fn full_coverage_when_all_areas_have_files() {
        let dir = create_test_dir("full");
        let artifacts = test_artifacts();

        fs::create_dir_all(dir.join(".orqa/learning/rules")).expect("mkdir");
        fs::write(dir.join(".orqa/learning/rules/no-stubs.md"), "# Rule").expect("write");

        fs::create_dir_all(dir.join(".orqa/learning/lessons")).expect("mkdir");
        fs::write(dir.join(".orqa/learning/lessons/IMPL-001.md"), "# Lesson").expect("write");

        fs::create_dir_all(dir.join(".orqa/documentation/architecture")).expect("mkdir");
        fs::write(
            dir.join(".orqa/documentation/architecture/overview.md"),
            "# Arch",
        )
        .expect("write");

        let result = scan_governance(&dir, &artifacts).expect("scan");
        assert_eq!(result.areas.len(), 3);
        assert!((result.coverage_ratio - 1.0_f64).abs() < f64::EPSILON);

        cleanup(&dir);
    }

    #[test]
    fn partial_coverage_computed_correctly() {
        let dir = create_test_dir("partial");
        let artifacts = test_artifacts();

        // Only rules covered (1 of 3)
        fs::create_dir_all(dir.join(".orqa/learning/rules")).expect("mkdir");
        fs::write(dir.join(".orqa/learning/rules/rule.md"), "# Rule").expect("write");

        let result = scan_governance(&dir, &artifacts).expect("scan");
        let expected = 1.0 / 3.0;
        assert!(
            (result.coverage_ratio - expected).abs() < 1e-9,
            "expected ratio ~{expected:.4}, got {:.4}",
            result.coverage_ratio
        );

        cleanup(&dir);
    }

    #[test]
    fn content_preview_truncated_at_500_chars() {
        let dir = create_test_dir("preview");
        let artifacts = test_artifacts();

        let rules_dir = dir.join(".orqa/learning/rules");
        fs::create_dir_all(&rules_dir).expect("mkdir");

        let long_content = "x".repeat(1000);
        fs::write(rules_dir.join("long.md"), &long_content).expect("write");

        let result = scan_governance(&dir, &artifacts).expect("scan");
        let rules_area = result
            .areas
            .iter()
            .find(|a| a.name == "rules")
            .expect("rules area");
        assert!(rules_area.covered);
        let file = &rules_area.files[0];
        assert_eq!(file.content_preview.len(), 500);
        assert_eq!(file.size_bytes, 1000);

        cleanup(&dir);
    }

    #[test]
    fn nonexistent_path_returns_error() {
        let result = scan_governance(Path::new("/nonexistent/governance/test/path/xyz"), &[]);
        assert!(result.is_err());
        assert!(matches!(result, Err(EngineError::Validation(_))));
    }

    #[test]
    fn docs_area_scans_recursively() {
        let dir = create_test_dir("doc_recursive");
        let artifacts = vec![ArtifactEntry::Type(ArtifactTypeConfig {
            key: "docs".to_owned(),
            label: Some("Documentation".to_owned()),
            icon: None,
            path: ".orqa/documentation".to_owned(),
        })];

        fs::create_dir_all(dir.join(".orqa/documentation/architecture")).expect("mkdir");
        fs::create_dir_all(dir.join(".orqa/documentation/product")).expect("mkdir");
        fs::write(
            dir.join(".orqa/documentation/architecture/decisions.md"),
            "# Decisions",
        )
        .expect("write");
        fs::write(
            dir.join(".orqa/documentation/product/vision.md"),
            "# Vision",
        )
        .expect("write");

        let result = scan_governance(&dir, &artifacts).expect("scan");
        let doc_area = result
            .areas
            .iter()
            .find(|a| a.name == "docs")
            .expect("docs area");

        assert!(doc_area.covered);
        assert_eq!(doc_area.files.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn area_names_come_from_config_keys() {
        let dir = create_test_dir("names");
        let artifacts = test_artifacts();
        let result = scan_governance(&dir, &artifacts).expect("scan");

        let names: Vec<&str> = result.areas.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(names, ["rules", "lessons", "docs"]);

        cleanup(&dir);
    }

    #[test]
    fn custom_artifact_config_defines_areas() {
        let dir = create_test_dir("custom");
        let artifacts = vec![
            ArtifactEntry::Type(ArtifactTypeConfig {
                key: "agents".to_owned(),
                label: Some("Agents".to_owned()),
                icon: None,
                path: ".claude/agents".to_owned(),
            }),
            ArtifactEntry::Type(ArtifactTypeConfig {
                key: "knowledge".to_owned(),
                label: Some("Knowledge".to_owned()),
                icon: None,
                path: ".orqa/documentation/knowledge".to_owned(),
            }),
        ];

        fs::create_dir_all(dir.join(".claude/agents")).expect("mkdir");
        fs::write(dir.join(".claude/agents/backend.md"), "# Agent").expect("write");

        let result = scan_governance(&dir, &artifacts).expect("scan");
        assert_eq!(result.areas.len(), 2);

        let agents_area = result
            .areas
            .iter()
            .find(|a| a.name == "agents")
            .expect("agents");
        assert!(agents_area.covered);

        let knowledge_area = result
            .areas
            .iter()
            .find(|a| a.name == "knowledge")
            .expect("knowledge");
        assert!(!knowledge_area.covered);

        assert!((result.coverage_ratio - 0.5).abs() < 1e-9);

        cleanup(&dir);
    }
}
