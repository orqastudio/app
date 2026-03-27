// System tray integration for the OrqaStudio daemon.
//
// The tray icon represents the running daemon and provides a context menu with
// quick actions: opening the app and quitting the daemon. This module is a
// placeholder that documents the intended integration point. Full tray
// integration requires a native event loop (e.g., via tray-icon + an OS
// message pump on Windows) which must run on the main thread.
//
// TODO(P3): Implement full system tray with tray-icon crate.
//   - Create a 16x16 RGBA icon programmatically (green = running).
//   - Build a context menu with "Open App" and "Quit" items.
//   - Drive the OS message pump on the main thread while the tokio runtime
//     runs on a background thread pool.
//   - "Open App" should call `orqa-studio` or open the app URL.
//   - "Quit" should trigger the graceful shutdown channel.
//
// The daemon is fully functional without the tray — the tray is an optional
// UX layer that does not affect the health endpoint or file watchers.

/// Tray integration status used by the main loop to report state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayStatus {
    /// Tray is not running — daemon operates headless.
    Headless,
}

/// Attempt to start the system tray. Currently always returns `Headless`
/// because the tray implementation is pending (see module TODO). When the
/// tray is implemented this function will either return `Running` or an error
/// that causes the daemon to fall back to headless mode.
///
/// The caller should log the returned status so operators know whether the
/// tray is active.
pub fn start() -> TrayStatus {
    tracing::info!(
        "system tray not yet implemented — daemon running headless. \
        See daemon/src/tray.rs for the integration plan."
    );
    TrayStatus::Headless
}
