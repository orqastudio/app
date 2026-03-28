// File watcher for OrqaStudio project content.
//
// Watches two directories within the project root for changes:
//   - `.orqa/`    — governance artifact changes trigger graph rebuild and
//                   enforcement rule reload via the engine
//   - `plugins/`  — plugin source changes trigger plugin discovery reload
//
// Events are debounced with a 500 ms window so that rapid successive writes
// (e.g. editor autosave bursts) produce a single notification rather than a
// flood. Paths under `.git/`, `node_modules/`, `target/`, and `.state/` are
// silently ignored to avoid spurious events during build or tooling activity.
//
// When a relevant event fires, the engine APIs are called synchronously in the
// debouncer callback. All engine calls are best-effort: errors are logged at
// warn level and do not crash the daemon.

use std::path::{Path, PathBuf};
use std::time::Duration;

use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, RecommendedCache};
use tracing::{info, warn};

use orqa_engine::enforcement::engine::EnforcementEngine;
use orqa_engine::enforcement::store::load_rules;
use orqa_engine::graph::build_artifact_graph;
use orqa_engine::plugin::discovery::scan_plugins;

/// Directories to watch relative to the project root.
const WATCH_DIRS: &[&str] = &[".orqa", "plugins"];

/// Path components that trigger event filtering — changes under these
/// directories are ignored regardless of where in the tree they appear.
const IGNORED_COMPONENTS: &[&str] = &[".git", "node_modules", "target", ".state"];

/// Debounce window for file-system events.
const DEBOUNCE_MS: u64 = 500;

/// Relative path from the project root to the enforcement rules directory.
const RULES_DIR: &str = ".orqa/process/rules";

/// RAII handle that keeps the debounced watcher alive.
///
/// Dropping this value unregisters all OS watches and stops the background
/// event thread. Create via [`start_watcher`].
pub struct WatchHandle {
    /// The underlying debouncer — kept alive by this field.
    _debouncer: Debouncer<notify::RecommendedWatcher, RecommendedCache>,
}

/// Start file watching on `.orqa/` and `plugins/` under `project_root`.
///
/// Returns a [`WatchHandle`] that keeps the watcher alive for as long as it
/// is held. Drop the handle to stop watching. Directories that do not exist
/// are skipped with a warning rather than causing an error, so the watcher
/// starts successfully even in projects that have not yet created a `plugins/`
/// directory.
///
/// Events are debounced over a [`DEBOUNCE_MS`] window. Each event batch is
/// filtered for ignored path components. Events from `.orqa/` trigger graph
/// rebuild and enforcement reload; events from `plugins/` trigger plugin
/// discovery reload.
pub fn start_watcher(project_root: &Path) -> notify::Result<WatchHandle> {
    let root = project_root.to_path_buf();

    let mut debouncer = new_debouncer(
        Duration::from_millis(DEBOUNCE_MS),
        None,
        move |result: DebounceEventResult| handle_events(result, &root),
    )?;

    let mut watched_any = false;
    for dir_name in WATCH_DIRS {
        let dir = project_root.join(dir_name);
        if dir.exists() {
            match debouncer.watch(&dir, RecursiveMode::Recursive) {
                Ok(()) => {
                    info!(
                        subsystem = "watcher",
                        path = %dir.display(),
                        "[watcher] watching directory"
                    );
                    watched_any = true;
                }
                Err(e) => {
                    warn!(
                        subsystem = "watcher",
                        path = %dir.display(),
                        error = %e,
                        "[watcher] failed to watch directory — skipping"
                    );
                }
            }
        } else {
            info!(
                subsystem = "watcher",
                path = %dir.display(),
                "[watcher] directory does not exist — skipping"
            );
        }
    }

    if !watched_any {
        warn!(
            subsystem = "watcher",
            root = %project_root.display(),
            "[watcher] no directories could be watched"
        );
    }

    Ok(WatchHandle {
        _debouncer: debouncer,
    })
}

/// Process a batch of debounced file-system events.
///
/// Filters out any events whose paths pass through an ignored directory
/// component, then classifies each remaining event by its top-level watched
/// directory. Events under `.orqa/` trigger graph rebuild and enforcement
/// reload; events under `plugins/` trigger plugin discovery reload.
fn handle_events(result: DebounceEventResult, root: &Path) {
    match result {
        Ok(events) => {
            let mut orqa_changed = false;
            let mut plugins_changed = false;

            for event in &events {
                let relevant_paths: Vec<&PathBuf> =
                    event.paths.iter().filter(|p| !is_ignored(p)).collect();

                if relevant_paths.is_empty() {
                    continue;
                }

                for path in &relevant_paths {
                    if path_is_under(path, root, ".orqa") {
                        orqa_changed = true;
                    } else if path_is_under(path, root, "plugins") {
                        plugins_changed = true;
                    }
                }
            }

            if orqa_changed {
                rebuild_graph(root);
                reload_enforcement(root);
            }

            if plugins_changed {
                reload_plugins(root);
            }
        }
        Err(errors) => {
            for e in errors {
                warn!(
                    subsystem = "watcher",
                    error = %e,
                    "[watcher] notify error"
                );
            }
        }
    }
}

/// Return `true` if `path` is located under `root/subdir`.
///
/// Used to classify watch events by which top-level watched directory they
/// belong to without doing a full path prefix match that could fail on
/// symlinks or non-canonical paths.
fn path_is_under(path: &Path, root: &Path, subdir: &str) -> bool {
    path.starts_with(root.join(subdir))
}

/// Rebuild the artifact graph after `.orqa/` changes.
///
/// Calls `build_artifact_graph` from the engine and logs the result at info
/// level. Errors are logged at warn level so a broken artifact file does not
/// crash the daemon.
fn rebuild_graph(root: &Path) {
    match build_artifact_graph(root) {
        Ok(graph) => {
            info!(
                subsystem = "watcher",
                artifact_count = graph.nodes.len(),
                "[watcher] graph rebuilt ({} artifacts)",
                graph.nodes.len()
            );
        }
        Err(e) => {
            warn!(
                subsystem = "watcher",
                error = %e,
                "[watcher] graph rebuild failed"
            );
        }
    }
}

/// Reload enforcement rules after `.orqa/` changes.
///
/// Loads rules from `.orqa/process/rules`, compiles the enforcement engine,
/// and logs how many rules and compiled entries were loaded at info level.
/// Errors are logged at warn level — a missing rules directory or unparseable
/// rule file must not crash the daemon.
fn reload_enforcement(root: &Path) {
    let rules_dir = root.join(RULES_DIR);
    match load_rules(&rules_dir) {
        Ok(rules) => {
            let rule_count: usize = rules.len();
            let engine = EnforcementEngine::new(rules);
            let entry_count: usize = engine.rules().iter().map(|r| r.entries.len()).sum();
            info!(
                subsystem = "watcher",
                rule_count,
                entry_count,
                "[watcher] enforcement reloaded ({rule_count} rules, {entry_count} entries)"
            );
        }
        Err(e) => {
            warn!(
                subsystem = "watcher",
                error = %e,
                "[watcher] enforcement reload failed"
            );
        }
    }
}

/// Reload plugin discovery after `plugins/` changes.
///
/// Calls `scan_plugins` from the engine and logs the discovered count at info
/// level. Plugin discovery is infallible (returns an empty vec on any error),
/// so this function always succeeds.
fn reload_plugins(root: &Path) {
    let plugins = scan_plugins(root);
    info!(
        subsystem = "watcher",
        plugin_count = plugins.len(),
        "[watcher] plugins reloaded ({} plugins discovered)",
        plugins.len()
    );
}

/// Return `true` if any path component of `path` is an ignored directory.
///
/// Checks every component of the path string representation against the
/// [`IGNORED_COMPONENTS`] list. This catches paths like
/// `/project/node_modules/foo/bar.md` regardless of nesting depth.
fn is_ignored(path: &Path) -> bool {
    path.components().any(|comp| {
        let s = comp.as_os_str().to_string_lossy();
        IGNORED_COMPONENTS
            .iter()
            .any(|ignored| s.as_ref() == *ignored)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn node_modules_is_ignored() {
        let p = PathBuf::from("/project/node_modules/lodash/index.js");
        assert!(is_ignored(&p));
    }

    #[test]
    fn git_dir_is_ignored() {
        let p = PathBuf::from("/project/.git/COMMIT_EDITMSG");
        assert!(is_ignored(&p));
    }

    #[test]
    fn target_dir_is_ignored() {
        let p = PathBuf::from("/project/target/debug/build/orqa-daemon.d");
        assert!(is_ignored(&p));
    }

    #[test]
    fn state_dir_is_ignored() {
        let p = PathBuf::from("/project/.state/daemon.pid");
        assert!(is_ignored(&p));
    }

    #[test]
    fn orqa_artifact_is_not_ignored() {
        let p = PathBuf::from("/project/.orqa/planning/epics/EPIC-001.md");
        assert!(!is_ignored(&p));
    }

    #[test]
    fn plugin_file_is_not_ignored() {
        let p = PathBuf::from("/project/plugins/agile/orqa-plugin.json");
        assert!(!is_ignored(&p));
    }

    #[test]
    fn path_is_under_detects_orqa() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/.orqa/process/rules/rule-001.md");
        assert!(path_is_under(&path, &root, ".orqa"));
        assert!(!path_is_under(&path, &root, "plugins"));
    }

    #[test]
    fn path_is_under_detects_plugins() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/plugins/agile/orqa-plugin.json");
        assert!(path_is_under(&path, &root, "plugins"));
        assert!(!path_is_under(&path, &root, ".orqa"));
    }

    #[test]
    fn reload_plugins_does_not_crash_on_missing_project() {
        // scan_plugins is infallible — returns empty vec for nonexistent root.
        reload_plugins(Path::new("/nonexistent/project/root"));
    }

    #[test]
    fn rebuild_graph_logs_error_on_missing_project() {
        // rebuild_graph is best-effort — must not panic.
        rebuild_graph(Path::new("/nonexistent/project/root"));
    }

    #[test]
    fn reload_enforcement_logs_error_on_missing_rules_dir() {
        // reload_enforcement is best-effort — must not panic when rules dir absent.
        reload_enforcement(Path::new("/nonexistent/project/root"));
    }
}
