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
/// (never file contents) for speed. Delegates stack classification to
/// [`classify_project`].
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

    // Collect file names from the directory tree for language detection.
    let mut all_file_names: Vec<String> = Vec::new();
    collect_file_names(root, excluded_paths, 0, &mut all_file_names);

    // Collect root-level file names for framework and package manager detection.
    let root_file_names: Vec<String> = collect_root_file_names(root);

    let has_claude_config = root.join(".claude").join("CLAUDE.md").exists();

    let stack = classify_project(&all_file_names, &root_file_names, has_claude_config);
    let governance = scan_governance(root);
    let elapsed = start.elapsed().as_millis() as u64;

    Ok(ProjectScanResult {
        stack,
        governance,
        scan_duration_ms: elapsed,
    })
}

/// Classify the project stack from pre-collected file name lists with no I/O.
///
/// This is the pure core of stack detection. The caller is responsible for
/// collecting file names by walking the filesystem. This function performs only
/// pure in-memory classification from the supplied name lists.
///
/// `all_file_names` — file names (basename only, any depth) for language detection via extension.
/// `root_file_names` — file names at the project root for framework and package manager detection.
/// `has_claude_config` — whether `.claude/CLAUDE.md` exists (caller checks this).
///
/// Returns a [`DetectedStack`] with sorted languages and frameworks.
pub fn classify_project(
    all_file_names: &[String],
    root_file_names: &[String],
    has_claude_config: bool,
) -> DetectedStack {
    let languages: HashSet<String> = all_file_names
        .iter()
        .filter_map(|name| infer_language_from_name(name).map(str::to_owned))
        .collect();

    let frameworks: HashSet<String> = collect_frameworks(root_file_names);
    let package_manager = classify_package_manager(root_file_names);

    let mut lang_vec: Vec<String> = languages.into_iter().collect();
    lang_vec.sort();
    let mut fw_vec: Vec<String> = frameworks.into_iter().collect();
    fw_vec.sort();

    DetectedStack {
        languages: lang_vec,
        frameworks: fw_vec,
        package_manager,
        has_claude_config,
        has_design_tokens: false,
    }
}

/// Recursively walk directories and collect all file names for classification.
///
/// Stops recursion at `MAX_SCAN_DEPTH` to bound execution time on large trees.
/// Populates `names` with basename strings of all files found.
fn collect_file_names(dir: &Path, excluded: &[String], depth: usize, names: &mut Vec<String>) {
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
            collect_file_names(&entry.path(), excluded, depth + 1, names);
        } else if file_type.is_file() {
            names.push(name.to_string());
        }
    }
}

/// Collect file names from the project root directory (non-recursive).
///
/// Returns only the basenames of files at the root level. Used to supply
/// root files to the pure [`classify_project`] function for framework and
/// package manager detection.
fn collect_root_file_names(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|ft| ft.is_file()))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

/// Check if a directory or file name matches the exclusion list.
fn is_excluded(name: &str, excluded: &[String]) -> bool {
    excluded.iter().any(|e| e == name)
}

/// Infer a programming language from a file's extension.
///
/// Returns the language name when the extension maps unambiguously to a single language,
/// or `None` for unrecognised extensions. Pure function — no side effects.
fn infer_language_from_name(name: &str) -> Option<&'static str> {
    match name.rsplit('.').next() {
        Some("rs") => Some("rust"),
        Some("ts" | "tsx") => Some("typescript"),
        Some("js" | "jsx") => Some("javascript"),
        Some("py") => Some("python"),
        Some("go") => Some("go"),
        Some("svelte") => Some("svelte"),
        Some("java") => Some("java"),
        Some("kt") => Some("kotlin"),
        Some("rb") => Some("ruby"),
        Some("cs") => Some("c#"),
        Some("cpp" | "cc" | "cxx") => Some("c++"),
        Some("c" | "h") => Some("c"),
        _ => None,
    }
}

/// Detect a programming language from a file name and insert it into the provided set.
///
/// Delegates to `infer_language_from_name` for the extension lookup. Unrecognised
/// extensions are silently ignored. Used by tests to exercise the language detection
/// logic through the accumulator API.
#[cfg(test)]
fn detect_language_from_name(name: &str, languages: &mut HashSet<String>) {
    if let Some(lang) = infer_language_from_name(name) {
        languages.insert(lang.to_owned());
    }
}

/// Detect frameworks from a list of root-level file names and return them as a set.
///
/// Pure function — accepts pre-collected root file names rather than walking the filesystem.
/// Only considers root-level config files because framework config files
/// are always at the project root by convention.
fn collect_frameworks(root_file_names: &[String]) -> HashSet<String> {
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

    framework_indicators
        .iter()
        .filter_map(|(files, framework)| {
            if files.iter().any(|f| root_file_names.iter().any(|n| n == f)) {
                Some((*framework).to_owned())
            } else {
                None
            }
        })
        .collect()
}

/// Detect frameworks from a list of root-level file names and insert them into the provided set.
///
/// Delegates to `collect_frameworks` for the detection logic. Mutating-output variant
/// used by tests to exercise the framework detection logic through the accumulator API.
#[cfg(test)]
fn classify_root_frameworks(root_file_names: &[String], frameworks: &mut HashSet<String>) {
    frameworks.extend(collect_frameworks(root_file_names));
}

/// Detect the package manager from a list of root-level file names.
///
/// Pure function — accepts pre-collected root file names rather than walking the filesystem.
/// Returns the first match found; precedence follows the order of the lock-file list.
fn classify_package_manager(root_file_names: &[String]) -> Option<String> {
    let lock_files: &[(&str, &str)] = &[
        ("Cargo.lock", "cargo"),
        ("package-lock.json", "npm"),
        ("yarn.lock", "yarn"),
        ("pnpm-lock.yaml", "pnpm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    for (file, manager) in lock_files {
        if root_file_names.iter().any(|n| n == file) {
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

    // -----------------------------------------------------------------------
    // classify_project unit tests (pure function — no I/O)
    // -----------------------------------------------------------------------

    #[test]
    fn classify_project_detects_rust_language() {
        let all_names = vec!["main.rs".to_owned(), "lib.rs".to_owned()];
        let stack = classify_project(&all_names, &[], false);
        assert!(stack.languages.contains(&"rust".to_owned()));
    }

    #[test]
    fn classify_project_detects_multiple_languages() {
        let all_names = vec![
            "app.ts".to_owned(),
            "main.rs".to_owned(),
            "script.py".to_owned(),
        ];
        let stack = classify_project(&all_names, &[], false);
        assert!(stack.languages.contains(&"rust".to_owned()));
        assert!(stack.languages.contains(&"typescript".to_owned()));
        assert!(stack.languages.contains(&"python".to_owned()));
    }

    #[test]
    fn classify_project_detects_cargo_framework() {
        let root_names = vec!["Cargo.toml".to_owned(), "Cargo.lock".to_owned()];
        let stack = classify_project(&[], &root_names, false);
        assert!(stack.frameworks.contains(&"cargo".to_owned()));
        assert_eq!(stack.package_manager, Some("cargo".to_owned()));
    }

    #[test]
    fn classify_project_detects_svelte_and_vite() {
        let root_names = vec!["svelte.config.js".to_owned(), "vite.config.ts".to_owned()];
        let stack = classify_project(&[], &root_names, false);
        assert!(stack.frameworks.contains(&"svelte".to_owned()));
        assert!(stack.frameworks.contains(&"vite".to_owned()));
    }

    #[test]
    fn classify_project_respects_has_claude_config() {
        let stack = classify_project(&[], &[], true);
        assert!(stack.has_claude_config);
        let stack = classify_project(&[], &[], false);
        assert!(!stack.has_claude_config);
    }

    #[test]
    fn classify_project_languages_sorted() {
        let all_names = vec!["z.ts".to_owned(), "a.rs".to_owned(), "m.py".to_owned()];
        let stack = classify_project(&all_names, &[], false);
        // Languages should be in sorted order
        let sorted: Vec<String> = {
            let mut v = stack.languages.clone();
            v.sort();
            v
        };
        assert_eq!(stack.languages, sorted);
    }

    #[test]
    fn classify_project_empty_inputs() {
        let stack = classify_project(&[], &[], false);
        assert!(stack.languages.is_empty());
        assert!(stack.frameworks.is_empty());
        assert!(stack.package_manager.is_none());
        assert!(!stack.has_claude_config);
        assert!(!stack.has_design_tokens);
    }

    #[test]
    fn classify_package_manager_priority_cargo_over_npm() {
        let names = vec!["Cargo.lock".to_owned(), "package-lock.json".to_owned()];
        let pm = classify_package_manager(&names);
        assert_eq!(pm, Some("cargo".to_owned()));
    }

    #[test]
    fn classify_package_manager_npm() {
        let names = vec!["package-lock.json".to_owned()];
        assert_eq!(classify_package_manager(&names), Some("npm".to_owned()));
    }

    #[test]
    fn classify_package_manager_none_when_no_lock() {
        assert_eq!(classify_package_manager(&[]), None);
    }

    #[test]
    fn classify_package_manager_yarn() {
        let names = vec!["yarn.lock".to_owned()];
        assert_eq!(classify_package_manager(&names), Some("yarn".to_owned()));
    }

    #[test]
    fn classify_package_manager_pnpm() {
        let names = vec!["pnpm-lock.yaml".to_owned()];
        assert_eq!(classify_package_manager(&names), Some("pnpm".to_owned()));
    }

    #[test]
    fn classify_root_frameworks_svelte_and_vite() {
        let names = vec!["svelte.config.js".to_owned(), "vite.config.ts".to_owned()];
        let mut frameworks = HashSet::new();
        classify_root_frameworks(&names, &mut frameworks);
        assert!(frameworks.contains("svelte"));
        assert!(frameworks.contains("vite"));
    }

    #[test]
    fn classify_root_frameworks_nextjs() {
        let names = vec!["next.config.js".to_owned()];
        let mut frameworks = HashSet::new();
        classify_root_frameworks(&names, &mut frameworks);
        assert!(frameworks.contains("nextjs"));
    }

    #[test]
    fn classify_root_frameworks_angular() {
        let names = vec!["angular.json".to_owned()];
        let mut frameworks = HashSet::new();
        classify_root_frameworks(&names, &mut frameworks);
        assert!(frameworks.contains("angular"));
    }

    #[test]
    fn classify_root_frameworks_empty_list_no_frameworks() {
        let mut frameworks = HashSet::new();
        classify_root_frameworks(&[], &mut frameworks);
        assert!(frameworks.is_empty());
    }

    // -----------------------------------------------------------------------
    // Integration tests (use the filesystem)
    // -----------------------------------------------------------------------

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

        assert!(result.stack.languages.contains(&"rust".to_owned()));
        assert!(result.stack.frameworks.contains(&"cargo".to_owned()));
        assert_eq!(result.stack.package_manager, Some("cargo".to_owned()));

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
        let excluded = vec![".git".to_owned()];
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
        let excluded = vec!["node_modules".to_owned()];
        let result = scan_project(dir_str, &excluded).expect("scan");

        // Should find TypeScript but NOT JavaScript (from node_modules)
        assert!(result.stack.languages.contains(&"typescript".to_owned()));
        assert!(!result.stack.languages.contains(&"javascript".to_owned()));

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

    #[test]
    fn is_excluded_matches_exact_name() {
        let excluded = vec!["node_modules".to_owned(), ".git".to_owned()];
        assert!(is_excluded("node_modules", &excluded));
        assert!(is_excluded(".git", &excluded));
        assert!(!is_excluded("src", &excluded));
    }

    #[test]
    fn is_excluded_empty_list_never_excludes() {
        assert!(!is_excluded("anything", &[]));
    }

    #[test]
    fn count_md_files_in_dir_counts_correctly() {
        let dir = create_test_dir("count_md");
        fs::write(dir.join("a.md"), "").expect("write");
        fs::write(dir.join("b.md"), "").expect("write");
        fs::write(dir.join("c.txt"), "").expect("write"); // ignored

        assert_eq!(count_md_files_in_dir(&dir), 2);
        cleanup(&dir);
    }

    #[test]
    fn count_md_files_in_dir_nonexistent_returns_zero() {
        let dir = std::env::temp_dir().join("orqa_test_nonexistent_dir_xyz987");
        assert_eq!(count_md_files_in_dir(&dir), 0);
    }
}
