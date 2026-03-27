// File watcher for OrqaStudio project content.
//
// Watches two directories within the project root for changes:
//   - `.orqa/`    — governance artifact changes (trigger re-evaluation)
//   - `plugins/`  — plugin source changes (trigger recomposition)
//
// Events are debounced with a 500 ms window so that rapid successive writes
// (e.g. editor autosave bursts) produce a single notification rather than a
// flood. Paths under `.git/`, `node_modules/`, `target/`, and `.state/` are
// silently ignored to avoid spurious events during build or tooling activity.
//
// Change events are currently logged at debug level. Wiring actual engine
// re-evaluation to these events is tracked as a later-phase deliverable.

use std::path::{Path, PathBuf};
use std::time::Duration;

use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, RecommendedCache};
use tracing::{debug, warn};

/// Directories to watch relative to the project root.
const WATCH_DIRS: &[&str] = &[".orqa", "plugins"];

/// Path components that trigger event filtering — changes under these
/// directories are ignored regardless of where in the tree they appear.
const IGNORED_COMPONENTS: &[&str] = &[".git", "node_modules", "target", ".state"];

/// Debounce window for file-system events.
const DEBOUNCE_MS: u64 = 500;

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
/// filtered for ignored path components before being logged at debug level.
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
                    tracing::info!(
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
            tracing::info!(
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
/// component, then logs each remaining event at debug level. Events are
/// logged with the path, event kind, and `[watcher]` subsystem tag.
fn handle_events(result: DebounceEventResult, _root: &Path) {
    match result {
        Ok(events) => {
            for event in events {
                let relevant_paths: Vec<&PathBuf> =
                    event.paths.iter().filter(|p| !is_ignored(p)).collect();

                if relevant_paths.is_empty() {
                    continue;
                }

                debug!(
                    subsystem = "watcher",
                    kind = ?event.kind,
                    paths = ?relevant_paths,
                    "[watcher] file changed"
                );
                // Phase 4: trigger connector regeneration and enforcement
                // evaluation based on which paths changed.
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
}
