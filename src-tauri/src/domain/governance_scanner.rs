use std::path::Path;

use crate::domain::governance::{GovernanceArea, GovernanceFile, GovernanceScanResult};
use crate::error::OrqaError;

/// Maximum number of characters to include in a content preview.
const CONTENT_PREVIEW_CHARS: usize = 500;

/// Total number of canonical governance areas checked for coverage ratio.
const TOTAL_AREAS: usize = 7;

/// Scan a project directory for governance files across all known tool ecosystems.
///
/// Returns a `GovernanceScanResult` covering 7 canonical Claude governance areas
/// plus detection of non-Claude governance tools (Cursor, Copilot, Continue).
/// The `coverage_ratio` is computed over the 7 canonical areas only.
pub fn scan_governance(project_path: &Path) -> Result<GovernanceScanResult, OrqaError> {
    if !project_path.exists() || !project_path.is_dir() {
        return Err(OrqaError::Validation(format!(
            "project path does not exist or is not a directory: {}",
            project_path.display()
        )));
    }

    let mut areas = scan_claude_areas(project_path);
    areas.extend(scan_non_claude_areas(project_path));

    let covered_canonical = areas
        .iter()
        .filter(|a| a.source == "claude" && a.covered)
        .count();

    let coverage_ratio = covered_canonical as f64 / TOTAL_AREAS as f64;

    Ok(GovernanceScanResult {
        areas,
        coverage_ratio,
    })
}

/// Scan all 7 canonical Claude governance areas.
fn scan_claude_areas(project_path: &Path) -> Vec<GovernanceArea> {
    let claude_dir = project_path.join(".claude");
    vec![
        scan_directory_area(
            "claude_rules",
            "claude",
            &claude_dir.join("rules"),
            Some(".md"),
        ),
        scan_directory_area(
            "claude_agents",
            "claude",
            &claude_dir.join("agents"),
            Some(".md"),
        ),
        scan_skills_area(project_path, &claude_dir.join("skills")),
        scan_directory_area("claude_hooks", "claude", &claude_dir.join("hooks"), None),
        scan_single_file_area(
            "claude_settings",
            "claude",
            &claude_dir.join("settings.json"),
        ),
        scan_claude_md_area(project_path),
        scan_single_file_area("agents_md", "claude", &project_path.join("AGENTS.md")),
    ]
}

/// Scan a directory for governance files with an optional extension filter.
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
                        .map(|e| format!(".{e}") == required_ext)
                        .unwrap_or(false);
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

/// Scan the skills directory — each subdirectory with a `SKILL.md` is one skill.
fn scan_skills_area(project_root: &Path, skills_dir: &Path) -> GovernanceArea {
    let mut files = Vec::new();

    if skills_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(skills_dir) {
            for entry in entries.flatten() {
                if !entry.path().is_dir() {
                    continue;
                }
                let skill_md = entry.path().join("SKILL.md");
                if skill_md.is_file() {
                    if let Some(f) = read_governance_file_relative(&skill_md, project_root) {
                        files.push(f);
                    }
                }
            }
        }
        files.sort_by(|a, b| a.path.cmp(&b.path));
    }

    let covered = !files.is_empty();
    GovernanceArea {
        name: "claude_skills".to_string(),
        source: "claude".to_string(),
        files,
        covered,
    }
}

/// Scan for CLAUDE.md — checks `.claude/CLAUDE.md` then root `CLAUDE.md`.
fn scan_claude_md_area(project_root: &Path) -> GovernanceArea {
    let candidates = [
        project_root.join(".claude").join("CLAUDE.md"),
        project_root.join("CLAUDE.md"),
    ];

    let mut files = Vec::new();
    for candidate in &candidates {
        if candidate.is_file() {
            if let Some(f) = read_governance_file_relative(candidate, project_root) {
                files.push(f);
            }
            break; // Only include the first match
        }
    }

    let covered = !files.is_empty();
    GovernanceArea {
        name: "claude_md".to_string(),
        source: "claude".to_string(),
        files,
        covered,
    }
}

/// Scan a single file as a governance area.
fn scan_single_file_area(name: &str, source: &str, path: &Path) -> GovernanceArea {
    let mut files = Vec::new();
    if path.is_file() {
        if let Some(parent) = path.parent() {
            if let Some(f) = read_governance_file_relative(path, parent) {
                files.push(f);
            }
        }
    }
    let covered = !files.is_empty();
    GovernanceArea {
        name: name.to_string(),
        source: source.to_string(),
        files,
        covered,
    }
}

/// Scan for non-Claude governance tools (Cursor, Copilot, Continue).
fn scan_non_claude_areas(project_root: &Path) -> Vec<GovernanceArea> {
    vec![
        scan_cursor_area(project_root),
        scan_copilot_area(project_root),
        scan_continue_area(project_root),
    ]
}

/// Scan for Cursor rules (`.cursor/rules/` directory and `.cursorrules` file).
fn scan_cursor_area(project_root: &Path) -> GovernanceArea {
    let cursor_rules_dir = project_root.join(".cursor").join("rules");
    let cursorrules = project_root.join(".cursorrules");
    let mut files = Vec::new();

    if cursor_rules_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&cursor_rules_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(f) = read_governance_file(&path) {
                        files.push(f);
                    }
                }
            }
        }
    }
    if cursorrules.is_file() {
        if let Some(f) = read_governance_file(&cursorrules) {
            files.push(f);
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    let covered = !files.is_empty();
    GovernanceArea {
        name: "cursor".to_string(),
        source: "cursor".to_string(),
        files,
        covered,
    }
}

/// Scan for GitHub Copilot instructions (`.github/copilot-instructions.md`).
fn scan_copilot_area(project_root: &Path) -> GovernanceArea {
    let copilot_path = project_root.join(".github").join("copilot-instructions.md");
    let files = if copilot_path.is_file() {
        read_governance_file(&copilot_path)
            .into_iter()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let covered = !files.is_empty();
    GovernanceArea {
        name: "copilot".to_string(),
        source: "copilot".to_string(),
        files,
        covered,
    }
}

/// Scan for Continue IDE extension configuration (`.continue/` directory).
fn scan_continue_area(project_root: &Path) -> GovernanceArea {
    let continue_dir = project_root.join(".continue");
    let files = if continue_dir.is_dir() {
        collect_files_in_dir(&continue_dir)
    } else {
        Vec::new()
    };
    let covered = !files.is_empty();
    GovernanceArea {
        name: "continue".to_string(),
        source: "continue".to_string(),
        files,
        covered,
    }
}

/// Collect all files directly in a directory (non-recursive).
fn collect_files_in_dir(dir: &Path) -> Vec<GovernanceFile> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Some(f) = read_governance_file(&entry.path()) {
                    files.push(f);
                }
            }
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
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
    let display_path = path
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string());
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
        // 7 canonical + 3 non-Claude areas
        assert_eq!(result.areas.len(), 10);
        for area in result.areas.iter().filter(|a| a.source == "claude") {
            assert!(!area.covered);
        }

        cleanup(&dir);
    }

    #[test]
    fn full_claude_governance_has_full_coverage() {
        let dir = create_test_dir("full");
        let claude_dir = dir.join(".claude");

        // claude_rules
        fs::create_dir_all(claude_dir.join("rules")).expect("mkdir");
        fs::write(claude_dir.join("rules").join("no-stubs.md"), "# Rule").expect("write");

        // claude_agents
        fs::create_dir_all(claude_dir.join("agents")).expect("mkdir");
        fs::write(claude_dir.join("agents").join("backend.md"), "# Agent").expect("write");

        // claude_skills
        fs::create_dir_all(claude_dir.join("skills").join("chunkhound")).expect("mkdir");
        fs::write(
            claude_dir
                .join("skills")
                .join("chunkhound")
                .join("SKILL.md"),
            "# Skill",
        )
        .expect("write");

        // claude_hooks
        fs::create_dir_all(claude_dir.join("hooks")).expect("mkdir");
        fs::write(
            claude_dir.join("hooks").join("pre-commit.sh"),
            "#!/bin/bash",
        )
        .expect("write");

        // claude_settings
        fs::write(claude_dir.join("settings.json"), "{}").expect("write");

        // claude_md
        fs::write(claude_dir.join("CLAUDE.md"), "# Config").expect("write");

        // agents_md
        fs::write(dir.join("AGENTS.md"), "# Agents").expect("write");

        let result = scan_governance(&dir).expect("scan");
        assert_eq!(result.coverage_ratio, 1.0);

        cleanup(&dir);
    }

    #[test]
    fn partial_coverage_computed_correctly() {
        let dir = create_test_dir("partial");
        let claude_dir = dir.join(".claude");

        // Only rules and agents covered (2 of 7)
        fs::create_dir_all(claude_dir.join("rules")).expect("mkdir");
        fs::write(claude_dir.join("rules").join("rule.md"), "# Rule").expect("write");

        fs::create_dir_all(claude_dir.join("agents")).expect("mkdir");
        fs::write(claude_dir.join("agents").join("agent.md"), "# Agent").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let expected = 2.0 / 7.0;
        assert!(
            (result.coverage_ratio - expected).abs() < 1e-9,
            "expected ratio ~{expected:.4}, got {:.4}",
            result.coverage_ratio
        );

        cleanup(&dir);
    }

    #[test]
    fn cursor_rules_detected() {
        let dir = create_test_dir("cursor");
        let cursor_dir = dir.join(".cursor").join("rules");
        fs::create_dir_all(&cursor_dir).expect("mkdir");
        fs::write(cursor_dir.join("my-rule.md"), "# Cursor Rule").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let cursor_area = result
            .areas
            .iter()
            .find(|a| a.name == "cursor")
            .expect("cursor area");
        assert!(cursor_area.covered);
        assert_eq!(cursor_area.files.len(), 1);

        cleanup(&dir);
    }

    #[test]
    fn copilot_instructions_detected() {
        let dir = create_test_dir("copilot");
        let github_dir = dir.join(".github");
        fs::create_dir_all(&github_dir).expect("mkdir");
        fs::write(github_dir.join("copilot-instructions.md"), "# Copilot").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let area = result
            .areas
            .iter()
            .find(|a| a.name == "copilot")
            .expect("copilot area");
        assert!(area.covered);

        cleanup(&dir);
    }

    #[test]
    fn content_preview_truncated_at_500_chars() {
        let dir = create_test_dir("preview");
        let claude_dir = dir.join(".claude").join("rules");
        fs::create_dir_all(&claude_dir).expect("mkdir");

        let long_content = "x".repeat(1000);
        fs::write(claude_dir.join("long.md"), &long_content).expect("write");

        let result = scan_governance(&dir).expect("scan");
        let rules_area = result
            .areas
            .iter()
            .find(|a| a.name == "claude_rules")
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
    fn claude_md_found_in_root_when_not_in_claude_dir() {
        let dir = create_test_dir("claude_md_root");
        fs::write(dir.join("CLAUDE.md"), "# Root Config").expect("write");

        let result = scan_governance(&dir).expect("scan");
        let area = result
            .areas
            .iter()
            .find(|a| a.name == "claude_md")
            .expect("claude_md area");
        assert!(area.covered);

        cleanup(&dir);
    }
}
