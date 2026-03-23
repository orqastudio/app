use std::path::Path;

use crate::domain::governance::{GovernanceArea, GovernanceFile, GovernanceScanResult};
use crate::error::OrqaError;

/// Maximum number of characters to include in a content preview.
const CONTENT_PREVIEW_CHARS: usize = 500;

/// Total number of canonical governance areas checked for coverage ratio.
///
/// The 6 areas map directly to the process/documentation directories in `.orqa/`:
/// rules, agents, knowledge, lessons, decisions, documentation.
const TOTAL_AREAS: usize = 6;

/// Scan a project directory for governance files across the 7 canonical OrqaStudio governance areas.
///
/// The areas correspond to the artifact config in `.orqa/project.json`:
/// - `.orqa/process/rules` — enforcement rules (`.md` files)
/// - `.orqa/process/agents` — agent definitions (`.md` files)
/// - `.orqa/process/knowledge` — knowledge artifacts (`.md` files)
/// - `.orqa/process/lessons` — implementation lessons (`.md` files)
/// - `.orqa/process/decisions` — architecture decisions (`.md` files)
/// - `.orqa/documentation` — project documentation (`.md` files, recursive)
///
/// The `coverage_ratio` is computed as covered areas / `TOTAL_AREAS`.
///
/// # Filesystem dependency
///
/// This function performs filesystem I/O (directory listing, file metadata reads, and content
/// previews). The dependency is intentional — governance scanning is inherently a filesystem
/// operation whose purpose is to walk and inspect local project files. It does not access
/// the database or any network resource.
pub fn scan_governance(project_path: &Path) -> Result<GovernanceScanResult, OrqaError> {
    if !project_path.exists() || !project_path.is_dir() {
        return Err(OrqaError::Validation(format!(
            "project path does not exist or is not a directory: {}",
            project_path.display()
        )));
    }

    let areas = scan_orqa_areas(project_path);

    let covered = areas.iter().filter(|a| a.covered).count();
    let coverage_ratio = covered as f64 / TOTAL_AREAS as f64;

    Ok(GovernanceScanResult {
        areas,
        coverage_ratio,
    })
}

/// Scan all 7 canonical governance areas from the `.orqa/` directory tree.
fn scan_orqa_areas(project_path: &Path) -> Vec<GovernanceArea> {
    let orqa_dir = project_path.join(".orqa");
    let process_dir = orqa_dir.join("process");

    vec![
        scan_directory_area("rules", "orqa", &process_dir.join("rules"), Some(".md")),
        scan_directory_area("agents", "orqa", &process_dir.join("agents"), Some(".md")),
        scan_knowledge_area(project_path, &process_dir.join("knowledge")),
        scan_directory_area("lessons", "orqa", &process_dir.join("lessons"), Some(".md")),
        scan_directory_area(
            "decisions",
            "orqa",
            &process_dir.join("decisions"),
            Some(".md"),
        ),
        scan_recursive_area(
            "documentation",
            "orqa",
            &orqa_dir.join("documentation"),
            Some(".md"),
        ),
    ]
}

/// Scan a flat directory for governance files with an optional extension filter.
///
/// Only files directly inside `dir` are included. For recursive scanning use
/// [`scan_recursive_area`].
fn scan_directory_area(name: &str, source: &str, dir: &Path, ext: Option<&str>) -> GovernanceArea {
    let mut files = Vec::new();

    if dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                if let Some(required_ext) = ext {
                    let matches = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|e| format!(".{e}") == required_ext);
                    if !matches {
                        continue;
                    }
                }
                if let Some(f) = read_governance_file(&path) {
                    files.push(f);
                }
            }
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
    }

    let covered = !files.is_empty();
    GovernanceArea {
        name: name.to_string(),
        source: source.to_string(),
        files,
        covered,
    }
}

/// Scan a directory tree recursively for governance files with an optional extension filter.
///
/// Descends into subdirectories. Files matching the extension filter (or all files if `ext` is
/// `None`) at any depth below `dir` are included.
fn scan_recursive_area(name: &str, source: &str, dir: &Path, ext: Option<&str>) -> GovernanceArea {
    let mut files = Vec::new();

    if dir.is_dir() {
        collect_files_recursive(dir, ext, &mut files);
        files.sort_by(|a, b| a.path.cmp(&b.path));
    }

    let covered = !files.is_empty();
    GovernanceArea {
        name: name.to_string(),
        source: source.to_string(),
        files,
        covered,
    }
}

/// Walk `dir` recursively, appending matching files to `out`.
fn collect_files_recursive(dir: &Path, ext: Option<&str>, out: &mut Vec<GovernanceFile>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, ext, out);
        } else if path.is_file() {
            if let Some(required_ext) = ext {
                let matches = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| format!(".{e}") == required_ext);
                if !matches {
                    continue;
                }
            }
            if let Some(f) = read_governance_file(&path) {
                out.push(f);
            }
        }
    }
}

/// Scan the knowledge directory — each `.md` file is one knowledge artifact.
fn scan_knowledge_area(project_root: &Path, knowledge_dir: &Path) -> GovernanceArea {
    let mut files = Vec::new();

    if knowledge_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(knowledge_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                    if let Some(f) = read_governance_file_relative(&path, project_root) {
                        files.push(f);
                    }
                }
            }
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
    }

    let covered = !files.is_empty();
    GovernanceArea {
        name: "knowledge".to_string(),
        source: "orqa".to_string(),
        files,
        covered,
    }
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
        path: path.to_string_lossy().to_string(),
        size_bytes,
        content_preview,
    })
}

/// Read a governance file, storing the path relative to `root`.
///
/// Returns `None` if the file metadata cannot be read (e.g. permissions error).
/// If the file content cannot be read as UTF-8, the preview is left empty and
/// `size_bytes` still reflects the true file size from metadata.
fn read_governance_file_relative(path: &Path, root: &Path) -> Option<GovernanceFile> {
    let metadata = std::fs::metadata(path).ok()?;
    let size_bytes = metadata.len();
    let content_preview = read_preview(path);
    let display_path = path.strip_prefix(root).map_or_else(
        |_| path.to_string_lossy().to_string(),
        |p| p.to_string_lossy().to_string(),
    );
    Some(GovernanceFile {
        path: display_path,
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

    #[test]
    fn empty_project_has_zero_coverage() {
        let dir = create_test_dir("empty");
        let result = scan_governance(&dir).expect("scan");

        assert_eq!(result.coverage_ratio, 0.0);
        assert_eq!(result.areas.len(), 6);
        for area in &result.areas {
            assert!(!area.covered);
        }

        cleanup(&dir);
    }

    #[test]
    fn full_orqa_governance_has_full_coverage() {
        let dir = create_test_dir("full");
        let process_dir = dir.join(".orqa").join("process");
        let doc_dir = dir.join(".orqa").join("documentation");

        // rules
        fs::create_dir_all(process_dir.join("rules")).expect("mkdir");
        fs::write(process_dir.join("rules").join("no-stubs.md"), "# Rule").expect("write");

        // agents
        fs::create_dir_all(process_dir.join("agents")).expect("mkdir");
        fs::write(process_dir.join("agents").join("backend.md"), "# Agent").expect("write");

        // knowledge artifacts
        fs::create_dir_all(process_dir.join("knowledge")).expect("mkdir");
        fs::write(
            process_dir.join("knowledge").join("chunkhound.md"),
            "# Knowledge",
        )
        .expect("write");

        // lessons
        fs::create_dir_all(process_dir.join("lessons")).expect("mkdir");
        fs::write(process_dir.join("lessons").join("IMPL-001.md"), "# Lesson").expect("write");

        // decisions
        fs::create_dir_all(process_dir.join("decisions")).expect("mkdir");
        fs::write(
            process_dir.join("decisions").join("AD-001.md"),
            "# Decision",
        )
        .expect("write");

        // documentation
        fs::create_dir_all(doc_dir.join("architecture")).expect("mkdir");
        fs::write(doc_dir.join("architecture").join("overview.md"), "# Arch").expect("write");

        let result = scan_governance(&dir).expect("scan");
        assert_eq!(result.areas.len(), 6);
        assert_eq!(result.coverage_ratio, 1.0);

        cleanup(&dir);
    }

    #[test]
    fn partial_coverage_computed_correctly() {
        let dir = create_test_dir("partial");
        let process_dir = dir.join(".orqa").join("process");

        // Only rules and agents covered (2 of 6)
        fs::create_dir_all(process_dir.join("rules")).expect("mkdir");
        fs::write(process_dir.join("rules").join("rule.md"), "# Rule").expect("write");

        fs::create_dir_all(process_dir.join("agents")).expect("mkdir");
        fs::write(process_dir.join("agents").join("agent.md"), "# Agent").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let expected = 2.0 / 6.0;
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
        let rules_dir = dir.join(".orqa").join("process").join("rules");
        fs::create_dir_all(&rules_dir).expect("mkdir");

        let long_content = "x".repeat(1000);
        fs::write(rules_dir.join("long.md"), &long_content).expect("write");

        let result = scan_governance(&dir).expect("scan");
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
        let result = scan_governance(Path::new("/nonexistent/governance/test/path/xyz"));
        assert!(result.is_err());
        assert!(matches!(result, Err(OrqaError::Validation(_))));
    }

    #[test]
    fn documentation_area_scans_recursively() {
        let dir = create_test_dir("doc_recursive");
        let doc_dir = dir.join(".orqa").join("documentation");

        // Create nested structure
        fs::create_dir_all(doc_dir.join("architecture")).expect("mkdir");
        fs::create_dir_all(doc_dir.join("product")).expect("mkdir");
        fs::write(
            doc_dir.join("architecture").join("decisions.md"),
            "# Decisions",
        )
        .expect("write");
        fs::write(doc_dir.join("product").join("vision.md"), "# Vision").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let doc_area = result
            .areas
            .iter()
            .find(|a| a.name == "documentation")
            .expect("documentation area");

        assert!(doc_area.covered);
        assert_eq!(doc_area.files.len(), 2);

        cleanup(&dir);
    }

    #[test]
    fn area_names_match_expected_keys() {
        let dir = create_test_dir("names");
        let result = scan_governance(&dir).expect("scan");

        let names: Vec<&str> = result.areas.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "rules",
                "agents",
                "knowledge",
                "lessons",
                "decisions",
                "documentation"
            ]
        );

        cleanup(&dir);
    }
}
