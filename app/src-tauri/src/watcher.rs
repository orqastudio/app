/// File-system watcher for the `.orqa/` directory.
///
/// When the project path is known (a project is opened), callers invoke
/// [`start`] to begin watching `.orqa/` recursively with a 500 ms debounce.
/// Any create/modify/remove event on a `.md` file causes two Tauri events to be
/// emitted to all windows:
///
/// - `artifact-changed`: used by the frontend nav-tree to invalidate its cache.
/// - `artifact-graph-updated`: used by the Artifact Graph SDK to auto-refresh.
///
/// Only `.md` file events that are not inside hidden directories (path segments
/// starting with `.` or `_`) trigger emission.  Non-`.md` changes (e.g. JSON,
/// shell scripts) are silently ignored.
///
/// Only one watcher runs at a time.  Calling [`start`] again replaces any
/// previously active watcher.
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use tauri::{AppHandle, Emitter, Runtime};

/// Event name emitted to all windows when `.orqa/` content changes.
pub const ARTIFACT_CHANGED_EVENT: &str = "artifact-changed";

/// Event name emitted after the artifact graph cache is invalidated.
///
/// The Artifact Graph SDK listens for this event and triggers an auto-refresh
/// of its in-memory graph cache.
pub const ARTIFACT_GRAPH_UPDATED_EVENT: &str = "artifact-graph-updated";

/// A running watcher handle.  Dropping this value stops the watcher.
///
/// We keep this opaque so callers don't need to know which watcher backend
/// the platform chose.
pub struct WatcherHandle {
    // The debouncer owns the underlying watcher; keeping it alive is enough.
    _debouncer: notify_debouncer_full::Debouncer<
        notify::RecommendedWatcher,
        notify_debouncer_full::RecommendedCache,
    >,
}

/// Shared storage for the active watcher, held in `AppState`.
pub type SharedWatcher = Arc<Mutex<Option<WatcherHandle>>>;

/// Start (or replace) the `.orqa/` file watcher for the given project root.
///
/// Events are debounced with a 500 ms window.  On any change inside the
/// `.orqa/` directory a single `artifact-changed` Tauri event is emitted.
///
/// Returns an error string if the watcher cannot be initialised.
pub fn start<R: Runtime>(
    app: AppHandle<R>,
    project_path: PathBuf,
    shared: &SharedWatcher,
) -> Result<(), String> {
    let orqa_dir = project_path.join(".orqa");

    if !orqa_dir.exists() {
        return Err(format!(
            "cannot watch: {} does not exist",
            orqa_dir.display()
        ));
    }

    let mut debouncer = create_debouncer(app, &orqa_dir)?;

    // Watch the project's .orqa/ directory.
    debouncer
        .watch(&orqa_dir, RecursiveMode::Recursive)
        .map_err(|e| format!("failed to watch {}: {e}", orqa_dir.display()))?;

    tracing::info!("[watcher] watching {} for changes", orqa_dir.display());

    let mut guard = shared
        .lock()
        .map_err(|e| format!("watcher lock poisoned: {e}"))?;

    // Dropping the previous handle stops the old watcher automatically.
    *guard = Some(WatcherHandle {
        _debouncer: debouncer,
    });

    Ok(())
}

/// Create a debounced file watcher that emits Tauri events on relevant `.orqa/` changes.
fn create_debouncer<R: Runtime>(
    app: AppHandle<R>,
    orqa_dir: &Path,
) -> Result<
    notify_debouncer_full::Debouncer<
        notify::RecommendedWatcher,
        notify_debouncer_full::RecommendedCache,
    >,
    String,
> {
    let orqa_dir_for_closure = orqa_dir.to_path_buf();
    new_debouncer(
        Duration::from_millis(500),
        None,
        move |result: Result<Vec<notify_debouncer_full::DebouncedEvent>, Vec<notify::Error>>| {
            let events = match result {
                Ok(evts) => evts,
                Err(errors) => {
                    for e in errors {
                        tracing::warn!("[watcher] notify error: {e}");
                    }
                    return;
                }
            };

            let has_relevant_change = events.iter().any(|evt| {
                // Only react to content-modifying events: Create, Write,
                // Remove, Rename.  Metadata-only changes (atime, permissions)
                // happen on Windows/NTFS when the daemon reads files through
                // a Docker bind mount — they do NOT indicate content changed
                // and must be filtered out to avoid triggering 3.3-second
                // full graph rebuilds on every artifact view.
                if !is_content_event(evt.kind) {
                    return false;
                }
                evt.paths
                    .iter()
                    .any(|p| is_relevant_path(p, &orqa_dir_for_closure))
            });

            if !has_relevant_change {
                return;
            }

            invalidate_and_emit(&app);
        },
    )
    .map_err(|e| format!("failed to create debouncer: {e}"))
}

/// Return `true` when the event kind indicates content was actually modified.
///
/// Excludes metadata-only changes (access-time updates, permission changes)
/// which fire on Windows when other processes read a file through a Docker
/// bind mount but don't indicate that the artifact's content was changed.
fn is_content_event(kind: notify::EventKind) -> bool {
    use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
    use notify::EventKind;

    matches!(
        kind,
        EventKind::Create(CreateKind::File | CreateKind::Any)
            | EventKind::Modify(
                ModifyKind::Data(_)
                    | ModifyKind::Name(RenameMode::Any | RenameMode::To | RenameMode::From)
                    | ModifyKind::Any
            )
            | EventKind::Remove(RemoveKind::File | RemoveKind::Any)
    )
}

/// Emit artifact change events to all windows.
///
/// The daemon maintains its own file watcher and rebuilds the graph internally.
/// The app watcher only needs to notify the frontend so it can re-fetch from
/// the daemon after the next artifact query.
fn invalidate_and_emit<R: Runtime>(app: &AppHandle<R>) {
    if let Err(e) = app.emit(ARTIFACT_CHANGED_EVENT, ()) {
        tracing::warn!("[watcher] failed to emit artifact-changed event: {e}");
    }
    if let Err(e) = app.emit(ARTIFACT_GRAPH_UPDATED_EVENT, ()) {
        tracing::warn!("[watcher] failed to emit artifact-graph-updated event: {e}");
    }
}

/// Stop the active watcher, if any.
pub fn stop(shared: &SharedWatcher) {
    if let Ok(mut guard) = shared.lock() {
        if guard.take().is_some() {
            tracing::info!("[watcher] stopped");
        }
    }
}

// ---------------------------------------------------------------------------
// Path filtering helpers
// ---------------------------------------------------------------------------

/// Returns `true` when `path` should trigger a graph invalidation.
///
/// A path is relevant when:
/// - Its file name ends with `.md`.
/// - No path component *inside* `orqa_root` (i.e. after stripping the prefix)
///   starts with `.` or `_` (hidden/private directories within `.orqa/`).
///
/// The `orqa_root` itself is excluded from the hidden-directory check because
/// `.orqa/` is the intentionally-hidden governance directory we watch.
fn is_relevant_path(path: &Path, orqa_root: &Path) -> bool {
    // Must be a .md file.
    let is_md = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));

    if !is_md {
        return false;
    }

    // Strip the orqa_root prefix so we only inspect the sub-path inside it.
    // If stripping fails (path is not under orqa_root), check all components.
    let relative = path.strip_prefix(orqa_root).unwrap_or(path);

    // Reject paths that pass through hidden or private sub-directories.
    // We check the parent directories only (not the file name itself) because
    // `.md` files with a leading dot are not expected, but checking `parent()`
    // is cleaner.  In practice, all relevant checks are on directory names.
    !relative.components().any(|comp| {
        let s = comp.as_os_str().to_string_lossy();
        s.starts_with('.') || s.starts_with('_')
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn orqa(project: &str) -> PathBuf {
        PathBuf::from(project).join(".orqa")
    }

    #[test]
    fn md_file_under_orqa_is_relevant() {
        let root = orqa("/project");
        let p = root.join("planning/epics/EPIC-001.md");
        assert!(is_relevant_path(&p, &root));
    }

    #[test]
    fn non_md_file_is_not_relevant() {
        let root = orqa("/project");
        let p = root.join("planning/epics/schema.json");
        assert!(!is_relevant_path(&p, &root));
    }

    #[test]
    fn hidden_subdir_makes_path_irrelevant() {
        let root = orqa("/project");
        let p = root.join("planning/.hidden/EPIC-001.md");
        assert!(!is_relevant_path(&p, &root));
    }

    #[test]
    fn underscore_subdir_makes_path_irrelevant() {
        let root = orqa("/project");
        let p = root.join("planning/_draft/EPIC-001.md");
        assert!(!is_relevant_path(&p, &root));
    }

    #[test]
    fn md_in_deeply_nested_normal_dirs_is_relevant() {
        let root = orqa("/project");
        let p = root.join("governance/rules/RULE-001.md");
        assert!(is_relevant_path(&p, &root));
    }

    #[test]
    fn md_directly_in_orqa_is_relevant() {
        let root = orqa("/project");
        let p = root.join("TASK-001.md");
        assert!(is_relevant_path(&p, &root));
    }
}
