//! Content installation helpers — runtime file copy and uninstall for plugin content entries.
//!
//! This module handles the `target: "runtime"` half of the two-sink install split.
//! Files classified as `target: "surrealdb"` are ingested by the daemon layer (which has
//! SurrealDB access); this module only performs filesystem operations.

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::Path;

use orqa_engine_types::error::EngineError;

use super::manifest::{ContentEntry, ContentTarget};

/// Summary of a runtime content install run for one plugin.
#[derive(Debug, Clone)]
pub struct ContentInstallSummary {
    /// Number of runtime files copied to `install_path`.
    pub runtime_copied: usize,
    /// Number of runtime files skipped (content hash unchanged).
    pub runtime_skipped: usize,
    /// Number of surrealdb-target entries found (handled by the daemon layer, not here).
    pub surrealdb_entries: usize,
}

/// Copy all `target: "runtime"` content entries from the installed plugin dir to their
/// `install_path` destinations, using content-hash deduplication to skip unchanged files.
///
/// `plugin_dir` is the installed plugin directory (`plugins/<short-name>/`).
/// `project_root` is the root against which `install_path` values are resolved.
/// `content` is the map of content entries from the plugin manifest.
///
/// Entries with `target: "surrealdb"` are counted in the summary but not touched here —
/// the daemon layer handles SurrealDB ingest after this function returns.
pub fn install_runtime_content<S: BuildHasher>(
    plugin_dir: &Path,
    project_root: &Path,
    content: &HashMap<String, ContentEntry, S>,
) -> Result<ContentInstallSummary, EngineError> {
    let mut summary = ContentInstallSummary {
        runtime_copied: 0,
        runtime_skipped: 0,
        surrealdb_entries: 0,
    };

    for (key, entry) in content {
        match entry.target {
            ContentTarget::Surrealdb => {
                summary.surrealdb_entries += 1;
            }
            ContentTarget::Runtime => {
                let src_dir = plugin_dir.join(&entry.source);
                if !src_dir.exists() {
                    tracing::warn!(
                        key = %key,
                        source = %entry.source,
                        "[plugins] runtime content source dir not found, skipping"
                    );
                    continue;
                }

                let dst_dir = project_root.join(&entry.install_path);
                std::fs::create_dir_all(&dst_dir)?;

                let (copied, skipped) = copy_dir_content_hashed(&src_dir, &dst_dir)?;
                summary.runtime_copied += copied;
                summary.runtime_skipped += skipped;

                tracing::debug!(
                    key = %key,
                    src = %src_dir.display(),
                    dst = %dst_dir.display(),
                    copied,
                    skipped,
                    "[plugins] runtime content installed"
                );
            }
        }
    }

    Ok(summary)
}

/// Return the list of files that `uninstall_runtime_content` would remove,
/// without performing any deletion.
///
/// Returns a list of absolute paths.
pub fn preview_uninstall_content<S: BuildHasher>(
    project_root: &Path,
    content: &HashMap<String, ContentEntry, S>,
) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();

    for entry in content.values() {
        if entry.target != ContentTarget::Runtime {
            continue;
        }
        let dst_dir = project_root.join(&entry.install_path);
        if dst_dir.exists() {
            collect_files(&dst_dir, &mut paths);
        }
    }

    paths
}

/// Remove all files that were installed to `install_path` by this plugin's runtime entries.
///
/// Leaves the destination directories in place (they may be shared). Only the files
/// within each `install_path` directory are removed — subdirectories are not pruned.
pub fn uninstall_runtime_content<S: BuildHasher>(
    project_root: &Path,
    content: &HashMap<String, ContentEntry, S>,
) -> Result<usize, EngineError> {
    let mut removed = 0;

    for (key, entry) in content {
        if entry.target != ContentTarget::Runtime {
            continue;
        }
        let dst_dir = project_root.join(&entry.install_path);
        if !dst_dir.exists() {
            continue;
        }

        let mut paths = Vec::new();
        collect_files(&dst_dir, &mut paths);

        for path in &paths {
            match std::fs::remove_file(path) {
                Ok(()) => removed += 1,
                Err(e) => {
                    tracing::warn!(
                        key = %key,
                        path = %path.display(),
                        error = %e,
                        "[plugins] failed to remove runtime content file"
                    );
                }
            }
        }
    }

    Ok(removed)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Recursively copy files from `src` to `dst`, skipping files whose SHA-256 hash
/// matches the existing destination file.
///
/// Returns `(copied, skipped)`.
fn copy_dir_content_hashed(src: &Path, dst: &Path) -> Result<(usize, usize), EngineError> {
    let mut copied = 0;
    let mut skipped = 0;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)?;
            let (c, s) = copy_dir_content_hashed(&src_path, &dst_path)?;
            copied += c;
            skipped += s;
        } else {
            let src_bytes = std::fs::read(&src_path)?;
            let src_hash = Sha256::digest(&src_bytes);

            if dst_path.exists() {
                let dst_bytes = std::fs::read(&dst_path)?;
                let dst_hash = Sha256::digest(&dst_bytes);
                if src_hash == dst_hash {
                    skipped += 1;
                    continue;
                }
            }

            std::fs::write(&dst_path, &src_bytes)?;
            copied += 1;
        }
    }

    Ok((copied, skipped))
}

/// Recursively collect all file paths under `dir` into `out`.
fn collect_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out);
        } else {
            out.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{ContentEntry, ContentTarget};
    use std::collections::HashMap;

    fn make_entry(source: &str, install_path: &str, target: ContentTarget) -> ContentEntry {
        ContentEntry {
            source: source.to_owned(),
            install_path: install_path.to_owned(),
            target,
        }
    }

    #[test]
    fn install_runtime_copies_files() {
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        // Write a source file in the plugin dir.
        let agents_dir = plugin_dir.path().join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();
        std::fs::write(agents_dir.join("AGENT-test.md"), "# Agent").unwrap();

        let mut content = HashMap::new();
        content.insert(
            "agents".to_owned(),
            make_entry("agents", ".claude/agents", ContentTarget::Runtime),
        );

        let summary =
            install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();

        assert_eq!(summary.runtime_copied, 1);
        assert_eq!(summary.runtime_skipped, 0);
        assert!(project_dir
            .path()
            .join(".claude/agents/AGENT-test.md")
            .exists());
    }

    #[test]
    fn install_runtime_skips_unchanged_file() {
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        let agents_dir = plugin_dir.path().join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();
        std::fs::write(agents_dir.join("AGENT-test.md"), "# Agent").unwrap();

        let mut content = HashMap::new();
        content.insert(
            "agents".to_owned(),
            make_entry("agents", ".claude/agents", ContentTarget::Runtime),
        );

        // First install — copies.
        install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();
        // Second install — skips (hashes match).
        let summary =
            install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();

        assert_eq!(summary.runtime_copied, 0);
        assert_eq!(summary.runtime_skipped, 1);
    }

    #[test]
    fn install_runtime_counts_surrealdb_entries() {
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        let mut content = HashMap::new();
        content.insert(
            "knowledge".to_owned(),
            make_entry("knowledge", ".orqa/knowledge", ContentTarget::Surrealdb),
        );

        let summary =
            install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();
        assert_eq!(summary.surrealdb_entries, 1);
        assert_eq!(summary.runtime_copied, 0);
    }

    #[test]
    fn uninstall_runtime_removes_files() {
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        let agents_dir = plugin_dir.path().join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();
        std::fs::write(agents_dir.join("AGENT-test.md"), "# Agent").unwrap();

        let mut content = HashMap::new();
        content.insert(
            "agents".to_owned(),
            make_entry("agents", ".claude/agents", ContentTarget::Runtime),
        );

        install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();
        assert!(project_dir
            .path()
            .join(".claude/agents/AGENT-test.md")
            .exists());

        let removed = uninstall_runtime_content(project_dir.path(), &content).unwrap();
        assert_eq!(removed, 1);
        assert!(!project_dir
            .path()
            .join(".claude/agents/AGENT-test.md")
            .exists());
    }

    #[test]
    fn preview_uninstall_returns_paths_without_deletion() {
        let plugin_dir = tempfile::tempdir().unwrap();
        let project_dir = tempfile::tempdir().unwrap();

        let agents_dir = plugin_dir.path().join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();
        std::fs::write(agents_dir.join("AGENT-test.md"), "# Agent").unwrap();

        let mut content = HashMap::new();
        content.insert(
            "agents".to_owned(),
            make_entry("agents", ".claude/agents", ContentTarget::Runtime),
        );

        install_runtime_content(plugin_dir.path(), project_dir.path(), &content).unwrap();

        let preview = preview_uninstall_content(project_dir.path(), &content);
        assert_eq!(preview.len(), 1);
        // File still exists — preview did not delete it.
        assert!(project_dir
            .path()
            .join(".claude/agents/AGENT-test.md")
            .exists());
    }
}
