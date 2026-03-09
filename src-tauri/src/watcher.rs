/// File-system watcher for the `.orqa/` directory.
///
/// When the project path is known (a project is opened), callers invoke
/// [`start`] to begin watching `.orqa/` recursively with a 500 ms debounce.
/// Any create/modify/remove event causes a `artifact-changed` Tauri event to be
/// emitted to all windows, which the frontend uses to invalidate the nav-tree
/// cache and reload artifact data.
///
/// Only one watcher runs at a time.  Calling [`start`] again replaces any
/// previously active watcher.
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use tauri::{AppHandle, Emitter, Runtime};

/// Event name emitted to all windows when `.orqa/` content changes.
pub const ARTIFACT_CHANGED_EVENT: &str = "artifact-changed";

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

    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        None,
        move |result: Result<Vec<notify_debouncer_full::DebouncedEvent>, Vec<notify::Error>>| {
            if result.is_ok() {
                // Emit a single signal regardless of how many events were batched.
                if let Err(e) = app.emit(ARTIFACT_CHANGED_EVENT, ()) {
                    tracing::warn!("[watcher] failed to emit artifact-changed event: {e}");
                }
            }
        },
    )
    .map_err(|e| format!("failed to create debouncer: {e}"))?;

    // Watch recursively so nested directories (planning/, governance/, …) are covered.
    // `Debouncer` itself implements `Watcher` in notify-debouncer-full 0.6+.
    debouncer
        .watch(&orqa_dir, RecursiveMode::Recursive)
        .map_err(|e| format!("failed to watch {}: {e}", orqa_dir.display()))?;

    let handle = WatcherHandle {
        _debouncer: debouncer,
    };

    let mut guard = shared
        .lock()
        .map_err(|e| format!("watcher lock poisoned: {e}"))?;

    // Dropping the previous handle stops the old watcher automatically.
    *guard = Some(handle);

    tracing::info!(
        "[watcher] watching {} for changes",
        orqa_dir.display()
    );

    Ok(())
}

/// Stop the active watcher, if any.
pub fn stop(shared: &SharedWatcher) {
    if let Ok(mut guard) = shared.lock() {
        if guard.take().is_some() {
            tracing::info!("[watcher] stopped");
        }
    }
}
