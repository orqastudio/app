use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use crate::domain::project::DetectedStack;
use crate::domain::project_settings::GovernanceCounts;
use crate::error::OrqaError;

const MAX_SCAN_DEPTH: usize = 10;

/// Result of scanning a project's filesystem for stack and governance info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectScanResult {
    pub stack: DetectedStack,
    pub governance: GovernanceCounts,
    pub scan_duration_ms: u64,
}

/// Scan a project directory for language, framework, and governance info.
///
/// Walks the filesystem up to `MAX_SCAN_DEPTH` levels deep, skipping
/// directories listed in `excluded_paths`. Only reads directory entries
/// (never file contents) for speed.
pub fn scan_project(
    project_path: &str,
    excluded_paths: &[String],
) -> Result<ProjectScanResult, OrqaError> {
    let start = Instant::now();
    let root = Path::new(project_path);

    if !root.exists() || !root.is_dir() {
        return Err(OrqaError::Validation(format!(
            "project path does not exist or is not a directory: {project_path}"
        )));
    }

    let mut languages = HashSet::new();
    let mut frameworks = HashSet::new();
    let mut package_manager: Option<String> = None;

    // Walk the tree for language detection by file extension
    walk_for_languages(root, excluded_paths, 0, &mut languages);

    // Detect frameworks from root-level config files
    detect_root_frameworks(root, &mut frameworks);

    // Detect package manager from root-level lock files
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
fn walk_for_languages(
    dir: &Path,
    excluded: &[String],
    depth: usize,
    languages: &mut HashSet<String>,
) {
    if depth > MAX_SCAN_DEPTH {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if is_excluded(&name, excluded) {
            continue;
        }

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            walk_for_languages(&entry.path(), excluded, depth + 1, languages);
        } else if file_type.is_file() {
            detect_language_from_name(&name, languages);
        }
    }
}

fn is_excluded(name: &str, excluded: &[String]) -> bool {
    excluded.iter().any(|e| e == name)
}

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
    languages.insert(lang.to_string());
}

/// Detect frameworks by looking for well-known config files in the root.
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
                frameworks.insert((*framework).to_string());
                break;
            }
        }
    }
}

/// Detect package manager from lock files in the root (first found wins).
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
            return Some((*manager).to_string());
        }
    }
    None
}

/// Count governance artifacts in the project.
fn scan_governance(root: &Path) -> GovernanceCounts {
    let docs = count_md_files_recursive(&root.join("docs"));
    let agents = count_md_files_in_dir(&root.join(".claude").join("agents"));
    let rules = count_md_files_in_dir(&root.join(".claude").join("rules"));
    let skills = count_subdirs(&root.join(".claude").join("skills"));
    let hooks = count_files_in_dir(&root.join(".claude").join("hooks"));
    let has_claude_config = root.join(".claude").join("CLAUDE.md").exists();

    GovernanceCounts {
        docs,
        agents,
        rules,
        skills,
        hooks,
        has_claude_config,
    }
}

/// Count `.md` files recursively in a directory.
fn count_md_files_recursive(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    count_md_recursive_inner(dir, 0)
}

fn count_md_recursive_inner(dir: &Path, depth: usize) -> u32 {
    if depth > MAX_SCAN_DEPTH {
        return 0;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    let mut count = 0u32;
    for entry in entries.flatten() {
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if ft.is_file() {
            let name = entry.file_name();
            if name.to_string_lossy().ends_with(".md") {
                count = count.saturating_add(1);
            }
        } else if ft.is_dir() {
            count = count.saturating_add(count_md_recursive_inner(&entry.path(), depth + 1));
        }
    }
    count
}

/// Count `.md` files in a single directory (not recursive).
fn count_md_files_in_dir(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    entries
        .flatten()
        .filter(|e| {
            e.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                && e.file_name().to_string_lossy().ends_with(".md")
        })
        .count() as u32
}

/// Count subdirectories in a directory (not recursive).
fn count_subdirs(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    entries
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .count() as u32
}

/// Count all files in a directory (not recursive).
fn count_files_in_dir(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };

    entries
        .flatten()
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("forge_scanner_test_{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create test dir");
        dir
    }

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
        assert_eq!(result.governance.docs, 0);
        assert_eq!(result.governance.agents, 0);

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

        // Create docs/ with markdown files
        let docs_dir = dir.join("docs");
        fs::create_dir_all(docs_dir.join("sub")).expect("mkdir");
        fs::write(docs_dir.join("readme.md"), "# Docs").expect("write");
        fs::write(docs_dir.join("sub").join("page.md"), "# Page").expect("write");

        // Create .claude/ structure
        let claude_dir = dir.join(".claude");
        fs::create_dir_all(claude_dir.join("agents")).expect("mkdir");
        fs::create_dir_all(claude_dir.join("rules")).expect("mkdir");
        fs::create_dir_all(claude_dir.join("skills").join("chunkhound")).expect("mkdir");
        fs::create_dir_all(claude_dir.join("hooks")).expect("mkdir");

        fs::write(claude_dir.join("CLAUDE.md"), "# Config").expect("write");
        fs::write(claude_dir.join("agents").join("backend.md"), "# Agent").expect("write");
        fs::write(claude_dir.join("rules").join("no-stubs.md"), "# Rule").expect("write");
        fs::write(
            claude_dir.join("hooks").join("pre-commit.sh"),
            "#!/bin/bash",
        )
        .expect("write");

        let dir_str = dir.to_str().expect("path");
        let excluded = vec![".git".to_string()];
        let result = scan_project(dir_str, &excluded).expect("scan");

        assert_eq!(result.governance.docs, 2);
        assert_eq!(result.governance.agents, 1);
        assert_eq!(result.governance.rules, 1);
        assert_eq!(result.governance.skills, 1);
        assert_eq!(result.governance.hooks, 1);
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
        assert!(matches!(err, OrqaError::Validation(_)));
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
}
