//! Project scanner for the OrqaStudio engine.
//!
//! Walks a project's filesystem to detect the technology stack (language, framework,
//! package manager) and counts governance artifacts. This is a pure filesystem domain
//! service — it performs no database or network I/O.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use orqa_engine_types::error::EngineError;
use orqa_engine_types::types::project::DetectedStack;

use crate::settings::GovernanceCounts;

/// Maximum directory recursion depth during language detection.
const MAX_SCAN_DEPTH: usize = 10;

/// Result of scanning a project's filesystem for stack and governance info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectScanResult {
    /// The detected technology stack (languages, frameworks, package manager).
    pub stack: DetectedStack,
    /// Counts of governance artifacts found in the project's `.orqa/` directory.
    pub governance: GovernanceCounts,
    /// Time taken to complete the scan, in milliseconds.
    pub scan_duration_ms: u64,
}

/// Scan a project directory for language, framework, and governance info.
///
/// Walks the filesystem up to `MAX_SCAN_DEPTH` levels deep, skipping
/// directories listed in `excluded_paths`. Only reads directory entries
/// (never file contents) for speed.
///
/// Returns `EngineError::Validation` if the path does not exist or is not a directory.
/// Returns `EngineError::FileSystem` if directory traversal encounters an I/O error.
pub fn scan_project(
    project_path: &str,
    excluded_paths: &[String],
) -> Result<ProjectScanResult, EngineError> {
    let start = Instant::now();
    let root = Path::new(project_path);

    if !root.exists() || !root.is_dir() {
        return Err(EngineError::Validation(format!(
            "project path does not exist or is not a directory: {project_path}"
        )));
    }

    let mut languages = HashSet::new();
    let mut frameworks = HashSet::new();
    let mut package_manager: Option<String> = None;

    // Walk the tree for language detection by file extension.
    walk_for_languages(root, excluded_paths, 0, &mut languages);

    // Detect frameworks from root-level config files.
    detect_root_frameworks(root, &mut frameworks);

    // Detect package manager from root-level lock files.
    if package_manager.is_none() {
        package_manager = detect_package_manager(root);
    }

    let has_claude_config = root.join(".claude").join("CLAUDE.md").exists();

    let mut lang_vec: Vec<String> = languages.into_iter().collect();
    lang_vec.sort();
    let mut fw_vec: Vec<String> = frameworks.into_iter().collect();
    fw_vec.sort();

    let stack = DetectedStack {
        languages: lang_vec,
        frameworks: fw_vec,
        package_manager,
        has_claude_config,
        has_design_tokens: false,
    };

    let governance = scan_governance(root);
    let elapsed = start.elapsed().as_millis() as u64;

    Ok(ProjectScanResult {
        stack,
        governance,
        scan_duration_ms: elapsed,
    })
}

/// Recursively walk directories to detect languages by file extension.
///
/// Stops recursion at `MAX_SCAN_DEPTH` to bound execution time on large trees.
fn walk_for_languages(
    dir: &Path,
    excluded: &[String],
    depth: usize,
    languages: &mut HashSet<String>,
) {
    if depth > MAX_SCAN_DEPTH {
        return;
    }

    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if is_excluded(&name, excluded) {
            continue;
        }

        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        if file_type.is_dir() {
            walk_for_languages(&entry.path(), excluded, depth + 1, languages);
        } else if file_type.is_file() {
            detect_language_from_name(&name, languages);
        }
    }
}

/// Check if a directory or file name matches the exclusion list.
fn is_excluded(name: &str, excluded: &[String]) -> bool {
    excluded.iter().any(|e| e == name)
}

/// Infer a programming language from a file's extension and insert it into the set.
///
/// Only recognises extensions that map unambiguously to a single language.
/// Unrecognised extensions are silently ignored.
fn detect_language_from_name(name: &str, languages: &mut HashSet<String>) {
    let lang = match name.rsplit('.').next() {
        Some("rs") => "rust",
        Some("ts" | "tsx") => "typescript",
        Some("js" | "jsx") => "javascript",
        Some("py") => "python",
        Some("go") => "go",
        Some("svelte") => "svelte",
        Some("java") => "java",
        Some("kt") => "kotlin",
        Some("rb") => "ruby",
        Some("cs") => "c#",
        Some("cpp" | "cc" | "cxx") => "c++",
        Some("c" | "h") => "c",
        _ => return,
    };
    languages.insert(lang.to_owned());
}

/// Detect frameworks by looking for well-known config files in the project root.
///
/// Only inspects the root directory (not recursive) because framework config files
/// are always at the project root by convention.
fn detect_root_frameworks(root: &Path, frameworks: &mut HashSet<String>) {
    let framework_indicators: &[(&[&str], &str)] = &[
        (&["Cargo.toml"], "cargo"),
        (&["svelte.config.js", "svelte.config.ts"], "svelte"),
        (&["tauri.conf.json"], "tauri"),
        (
            &["next.config.js", "next.config.ts", "next.config.mjs"],
            "nextjs",
        ),
        (&["tailwind.config.js", "tailwind.config.ts"], "tailwindcss"),
        (&["vite.config.js", "vite.config.ts"], "vite"),
        (&["tsconfig.json"], "typescript"),
        (&["angular.json"], "angular"),
        (&["vue.config.js"], "vue"),
    ];

    for (files, framework) in framework_indicators {
        for file in *files {
            if root.join(file).exists() {
                frameworks.insert((*framework).to_owned());
                break;
            }
        }
    }
}

/// Detect the package manager from well-known lock files in the project root.
///
/// Returns the first match found; precedence follows the order of the lock-file list.
fn detect_package_manager(root: &Path) -> Option<String> {
    let lock_files: &[(&str, &str)] = &[
        ("Cargo.lock", "cargo"),
        ("package-lock.json", "npm"),
        ("yarn.lock", "yarn"),
        ("pnpm-lock.yaml", "pnpm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    for (file, manager) in lock_files {
        if root.join(file).exists() {
            return Some((*manager).to_owned());
        }
    }
    None
}

/// Count governance artifacts in the project's `.orqa/` directory.
///
/// Reads artifact paths from `project.json` when available, so the scan
/// locations are driven by `ProjectSettings.artifacts` rather than hardcoded
/// paths (P1: Plugin-Composed Everything). Falls back to well-known defaults
/// when the config is absent or an artifact type is not configured.
fn scan_governance(root: &Path) -> GovernanceCounts {
    // Load project.json to resolve configured artifact paths.
    let config = load_project_config(root);
    let get_path = |key: &str, default: &str| -> std::path::PathBuf {
        config
            .as_ref()
            .and_then(|v| find_artifact_path_in_config(v, key))
            .map_or_else(|| root.join(default), |p| root.join(p))
    };

    let lessons_dir = get_path("lessons", ".orqa/learning/lessons");
    let decisions_dir = get_path("decisions", ".orqa/learning/decisions");
    let rules_dir = get_path("rules", ".orqa/learning/rules");

    let lessons = count_md_files_in_dir(&lessons_dir);
    let decisions = count_md_files_in_dir(&decisions_dir);
    let rules = count_md_files_in_dir(&rules_dir);

    // Documentation uses the docs subtree from project.json, or defaults to
    // the standard documentation directory.
    let docs_dir = get_path("docs", ".orqa/documentation");
    let documentation = count_md_files_in_docs(&docs_dir);

    let has_claude_config = root.join(".claude").join("CLAUDE.md").exists();

    GovernanceCounts {
        lessons,
        decisions,
        rules,
        documentation,
        has_claude_config,
    }
}

/// Load `project.json` as a generic JSON value for path resolution.
///
/// Returns `None` when the file is absent or unparseable — callers fall back
/// to default paths in that case.
fn load_project_config(root: &Path) -> Option<serde_json::Value> {
    let path = root.join(".orqa").join("project.json");
    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Walk the `artifacts` array in a parsed `project.json` value and return the
/// `path` string for an entry with the given `key`.
///
/// Searches both top-level `Type` entries and children of `Group` entries.
/// Returns `None` if no matching entry is found.
fn find_artifact_path_in_config(value: &serde_json::Value, key: &str) -> Option<String> {
    let artifacts = value.get("artifacts")?.as_array()?;
    for entry in artifacts {
        if entry.get("key").and_then(|v| v.as_str()) == Some(key) {
            if let Some(path) = entry.get("path").and_then(|v| v.as_str()) {
                return Some(path.to_owned());
            }
        }
        if let Some(children) = entry.get("children").and_then(|v| v.as_array()) {
            for child in children {
                if child.get("key").and_then(|v| v.as_str()) == Some(key) {
                    if let Some(path) = child.get("path").and_then(|v| v.as_str()) {
                        return Some(path.to_owned());
                    }
                }
            }
        }
    }
    None
}

/// Count `.md` files recursively under the documentation tree.
///
/// The documentation directory has subdirectories (architecture, development,
/// guides, etc.) each containing `.md` files. Counts all `.md` files across all
/// subdirectories to give a total documentation artifact count.
fn count_md_files_in_docs(docs_dir: &Path) -> u32 {
    if !docs_dir.is_dir() {
        return 0;
    }
    let mut count = 0u32;
    if let Ok(entries) = std::fs::read_dir(docs_dir) {
        for entry in entries.flatten() {
            let sub = entry.path();
            if sub.is_dir() {
                count += count_md_files_in_dir(&sub);
            } else if sub.extension().is_some_and(|e| e == "md") {
                count += 1;
            }
        }
    }
    count
}

/// Count `.md` files in a single directory (not recursive).
///
/// Returns 0 if the directory does not exist or cannot be read.
fn count_md_files_in_dir(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };

    entries
        .flatten()
        .filter(|e| {
            e.file_type().is_ok_and(|ft| ft.is_file())
                && e.file_name().to_string_lossy().ends_with(".md")
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Create a temporary test directory with a unique name, removing any prior remnant.
    fn create_test_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("orqa_engine_scanner_test_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

    /// Remove a test directory, ignoring errors (best-effort cleanup).
    fn cleanup(dir: &Path) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn empty_directory_returns_empty_results() {
        let dir = create_test_dir("empty");
        let dir_str = dir.to_str().expect("path");

        let result = scan_project(dir_str, &[]).expect("scan");
        assert!(result.stack.languages.is_empty());
        assert!(result.stack.frameworks.is_empty());
        assert!(result.stack.package_manager.is_none());
        assert!(!result.stack.has_claude_config);
        assert_eq!(result.governance.lessons, 0);
        assert_eq!(result.governance.documentation, 0);

        cleanup(&dir);
    }

    #[test]
    fn detects_rust_with_cargo() {
        let dir = create_test_dir("rust");

        // Create Cargo.toml (framework) and Cargo.lock (package manager)
        fs::write(dir.join("Cargo.toml"), "[package]").expect("write");
        fs::write(dir.join("Cargo.lock"), "").expect("write");

        // Create a .rs file in src/
        fs::create_dir_all(dir.join("src")).expect("mkdir");
        fs::write(dir.join("src").join("main.rs"), "fn main() {}").expect("write");

        let dir_str = dir.to_str().expect("path");
        let result = scan_project(dir_str, &[]).expect("scan");

        assert!(result.stack.languages.contains(&"rust".to_string()));
        assert!(result.stack.frameworks.contains(&"cargo".to_string()));
        assert_eq!(result.stack.package_manager, Some("cargo".to_string()));

        cleanup(&dir);
    }

    #[test]
    fn detects_governance_artifacts() {
        let dir = create_test_dir("governance");

        // Create .orqa/ structure matching the Phase 7 migrated layout.
        let learning_dir = dir.join(".orqa").join("learning");
        let docs_dir = dir.join(".orqa").join("documentation");
        fs::create_dir_all(learning_dir.join("rules")).expect("mkdir");
        fs::create_dir_all(learning_dir.join("lessons")).expect("mkdir");
        fs::create_dir_all(learning_dir.join("decisions")).expect("mkdir");
        fs::create_dir_all(docs_dir.join("reference")).expect("mkdir");

        fs::write(learning_dir.join("rules").join("no-stubs.md"), "# Rule").expect("write");
        fs::write(learning_dir.join("lessons").join("IMPL-001.md"), "# Lesson").expect("write");
        fs::write(
            learning_dir.join("decisions").join("AD-001.md"),
            "# Decision",
        )
        .expect("write");
        fs::write(
            docs_dir.join("reference").join("DOC-001.md"),
            "# Documentation",
        )
        .expect("write");

        // Create .claude/ for platform config (has_claude_config check)
        let claude_dir = dir.join(".claude");
        fs::create_dir_all(&claude_dir).expect("mkdir claude");
        fs::write(claude_dir.join("CLAUDE.md"), "# Config").expect("write");

        let dir_str = dir.to_str().expect("path");
        let excluded = vec![".git".to_string()];
        let result = scan_project(dir_str, &excluded).expect("scan");

        assert_eq!(result.governance.rules, 1);
        assert_eq!(result.governance.lessons, 1);
        assert_eq!(result.governance.decisions, 1);
        assert_eq!(result.governance.documentation, 1);
        assert!(result.governance.has_claude_config);

        cleanup(&dir);
    }

    #[test]
    fn excluded_paths_are_skipped() {
        let dir = create_test_dir("excluded");

        // Create node_modules/ with JS files that should be excluded
        let nm_dir = dir.join("node_modules");
        fs::create_dir_all(&nm_dir).expect("mkdir");
        fs::write(nm_dir.join("lib.js"), "module.exports = {}").expect("write");

        // Create a real source file
        fs::write(dir.join("app.ts"), "const x = 1").expect("write");

        let dir_str = dir.to_str().expect("path");
        let excluded = vec!["node_modules".to_string()];
        let result = scan_project(dir_str, &excluded).expect("scan");

        // Should find TypeScript but NOT JavaScript (from node_modules)
        assert!(result.stack.languages.contains(&"typescript".to_string()));
        assert!(!result.stack.languages.contains(&"javascript".to_string()));

        cleanup(&dir);
    }

    #[test]
    fn nonexistent_path_returns_validation_error() {
        let result = scan_project("/nonexistent/scanner/test/path", &[]);
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, EngineError::Validation(_)));
    }

    #[test]
    fn detect_language_coverage() {
        let mut langs = HashSet::new();

        detect_language_from_name("main.rs", &mut langs);
        detect_language_from_name("app.ts", &mut langs);
        detect_language_from_name("comp.tsx", &mut langs);
        detect_language_from_name("index.js", &mut langs);
        detect_language_from_name("comp.jsx", &mut langs);
        detect_language_from_name("script.py", &mut langs);
        detect_language_from_name("main.go", &mut langs);
        detect_language_from_name("App.svelte", &mut langs);
        detect_language_from_name("Main.java", &mut langs);
        detect_language_from_name("Main.kt", &mut langs);
        detect_language_from_name("app.rb", &mut langs);
        detect_language_from_name("Program.cs", &mut langs);
        detect_language_from_name("main.cpp", &mut langs);
        detect_language_from_name("lib.cc", &mut langs);
        detect_language_from_name("util.cxx", &mut langs);
        detect_language_from_name("main.c", &mut langs);
        detect_language_from_name("header.h", &mut langs);
        detect_language_from_name("readme.md", &mut langs); // not a language

        assert!(langs.contains("rust"));
        assert!(langs.contains("typescript"));
        assert!(langs.contains("javascript"));
        assert!(langs.contains("python"));
        assert!(langs.contains("go"));
        assert!(langs.contains("svelte"));
        assert!(langs.contains("java"));
        assert!(langs.contains("kotlin"));
        assert!(langs.contains("ruby"));
        assert!(langs.contains("c#"));
        assert!(langs.contains("c++"));
        assert!(langs.contains("c"));
        assert!(!langs.contains("markdown"));
        assert_eq!(langs.len(), 12);
    }

    #[test]
    fn governance_scan_uses_paths_from_project_json() {
        // When project.json specifies custom artifact paths, scan_governance should
        // count files in those locations rather than the default paths.
        let dir = create_test_dir("governance_config_driven");

        // Write a project.json with custom paths.
        let orqa_dir = dir.join(".orqa");
        fs::create_dir_all(&orqa_dir).expect("mkdir .orqa");
        let project_json = serde_json::json!({
            "name": "test",
            "artifacts": [
                {
                    "key": "learning",
                    "children": [
                        { "key": "rules", "path": ".orqa/custom/my-rules" },
                        { "key": "lessons", "path": ".orqa/custom/my-lessons" },
                        { "key": "decisions", "path": ".orqa/custom/my-decisions" }
                    ]
                },
                { "key": "docs", "path": ".orqa/custom/my-docs" }
            ]
        });
        fs::write(
            orqa_dir.join("project.json"),
            serde_json::to_string_pretty(&project_json).unwrap(),
        )
        .expect("write project.json");

        // Create files in the configured paths.
        let rules_dir = dir.join(".orqa").join("custom").join("my-rules");
        let lessons_dir = dir.join(".orqa").join("custom").join("my-lessons");
        fs::create_dir_all(&rules_dir).expect("mkdir rules");
        fs::create_dir_all(&lessons_dir).expect("mkdir lessons");
        fs::write(rules_dir.join("rule1.md"), "# Rule 1").expect("write rule");
        fs::write(lessons_dir.join("lesson1.md"), "# Lesson 1").expect("write lesson");

        let governance = scan_governance(&dir);
        assert_eq!(governance.rules, 1);
        assert_eq!(governance.lessons, 1);
        assert_eq!(governance.decisions, 0);

        cleanup(&dir);
    }

    #[test]
    fn find_artifact_path_in_config_finds_group_child() {
        let config = serde_json::json!({
            "artifacts": [
                {
                    "key": "learning",
                    "children": [
                        { "key": "rules", "path": ".orqa/learning/rules" }
                    ]
                }
            ]
        });
        let path = find_artifact_path_in_config(&config, "rules");
        assert_eq!(path, Some(".orqa/learning/rules".to_owned()));
    }

    #[test]
    fn find_artifact_path_in_config_finds_top_level() {
        let config = serde_json::json!({
            "artifacts": [
                { "key": "docs", "path": ".orqa/documentation" }
            ]
        });
        let path = find_artifact_path_in_config(&config, "docs");
        assert_eq!(path, Some(".orqa/documentation".to_owned()));
    }

    #[test]
    fn find_artifact_path_in_config_returns_none_for_missing_key() {
        let config = serde_json::json!({ "artifacts": [] });
        let path = find_artifact_path_in_config(&config, "rules");
        assert!(path.is_none());
    }
}
