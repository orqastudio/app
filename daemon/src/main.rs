// OrqaStudio daemon — persistent background process.
//
// The daemon provides the always-on infrastructure layer for an OrqaStudio
// project: health monitoring, file watching, MCP/LSP server lifecycle
// management, and the system tray icon.
//
// Startup sequence:
//   1. Locate the project root (walk up from CWD until .orqa/ is found).
//   2. Initialise structured logging (file + optional TTY console).
//   3. Check for an existing live daemon instance via the PID file.
//   4. Write our own PID file.
//   5. Register Ctrl-C / SIGTERM handler for graceful shutdown.
//   6. Start the system tray (currently headless — see tray.rs).
//   7. Start the health HTTP endpoint.
//   8. Start file watchers on .orqa/ and plugins/.
//   9. Start MCP server subprocess (graceful degradation if binary absent).
//  10. Start LSP server subprocess in TCP mode (graceful degradation if binary absent).
//  11. Block until the shutdown signal fires.
//  12. Stop MCP and LSP subprocesses.
//  13. Clean up the PID file and exit.

mod health;
mod logging;
mod lsp;
mod mcp;
mod process;
mod subprocess;
mod tray;
mod watcher;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tracing::{error, info, warn};

/// Resolve the project root, exiting with a helpful message if none is found.
///
/// This is a startup-time failure — there is no useful work the daemon can do
/// outside a project directory.
fn resolve_project_root() -> PathBuf {
    let cwd = std::env::current_dir().expect("cannot read working directory");
    match process::find_project_root(&cwd) {
        Ok(root) => root,
        Err(e) => {
            eprintln!("error: no OrqaStudio project found — {e}");
            std::process::exit(1);
        }
    }
}

/// Guard against duplicate daemon instances for the same project.
///
/// Exits immediately if a live daemon process is already registered in the PID
/// file. Logs a warning if the PID file is unreadable and continues.
fn guard_single_instance(project_root: &Path) {
    match process::check_existing(project_root) {
        Ok(true) => {
            error!(
                subsystem = "health",
                pid_file = %project_root.join(".state/daemon.pid").display(),
                "[health] a daemon instance is already running — exiting"
            );
            std::process::exit(1);
        }
        Ok(false) => {}
        Err(e) => {
            warn!(
                subsystem = "health",
                error = %e,
                "[health] could not read PID file — proceeding"
            );
        }
    }
}

/// Write the PID file and log its location, exiting on failure.
///
/// A missing or unwritable `.state/` directory is a startup-time hard error —
/// the daemon cannot operate without a place to record its PID.
fn establish_pid(project_root: &Path) {
    if let Err(e) = process::write_pid(project_root) {
        error!(
            subsystem = "health",
            error = %e,
            "[health] failed to write PID file — exiting"
        );
        std::process::exit(1);
    }
    info!(
        subsystem = "health",
        pid = std::process::id(),
        pid_file = %project_root.join(".state/daemon.pid").display(),
        "[health] PID file written"
    );
}

/// Register the Ctrl-C / SIGTERM handler that triggers graceful shutdown.
///
/// Sets `shutdown_flag` to true and removes the PID file. The main event loop
/// polls the flag and exits when it sees `true`.
fn register_shutdown_handler(shutdown_flag: Arc<AtomicBool>, project_root: PathBuf) {
    ctrlc::set_handler(move || {
        info!(subsystem = "health", "[health] shutdown signal received");
        shutdown_flag.store(true, Ordering::SeqCst);
        if let Err(e) = process::cleanup_pid(&project_root) {
            eprintln!("failed to remove PID file during shutdown: {e}");
        }
    })
    .expect("failed to register Ctrl-C handler");
}

/// Start all daemon subsystems and block until the shutdown flag is set.
///
/// Spawns the health server as a background tokio task. Starts the file
/// watcher on `.orqa/` and `plugins/` (with a warning fallback if the watcher
/// cannot start). Spawns the MCP and LSP server subprocesses; each degrades
/// gracefully if its binary is not yet built. Runs the polling event loop
/// until the shutdown flag is set, then stops the subprocesses.
async fn run(project_root: PathBuf, shutdown_flag: Arc<AtomicBool>) {
    let daemon_port = health::resolve_port();

    tokio::spawn(async move {
        if let Err(e) = health::start(daemon_port).await {
            error!(subsystem = "health", error = %e, "[health] health server failed");
        }
    });

    // The watcher handle must be kept alive for the duration of the event loop.
    // If starting the watcher fails the daemon continues without file watching.
    let _watcher_handle = match watcher::start_watcher(&project_root) {
        Ok(handle) => {
            info!(subsystem = "watcher", "[watcher] file watchers started");
            Some(handle)
        }
        Err(e) => {
            warn!(
                subsystem = "watcher",
                error = %e,
                "[watcher] could not start file watcher — running without watching"
            );
            None
        }
    };

    // Start MCP and LSP subprocesses. Both degrade gracefully when their
    // binaries are not yet available (e.g. first run before a full build).
    let mut mcp_manager = mcp::start_mcp(&project_root, daemon_port);
    let mut lsp_manager = lsp::start_lsp(&project_root, daemon_port);

    run_event_loop(&shutdown_flag, &mut mcp_manager, &mut lsp_manager).await;

    // Graceful shutdown: stop subprocesses before exiting.
    mcp_manager.stop();
    lsp_manager.stop();
}

/// Poll the shutdown flag and yield to the tokio runtime between checks.
///
/// Also polls subprocess status on each iteration so crashes are logged
/// promptly. Polling every 250 ms adds negligible overhead for a long-running
/// background process.
async fn run_event_loop(
    shutdown_flag: &Arc<AtomicBool>,
    mcp: &mut crate::subprocess::SubprocessManager,
    lsp: &mut crate::subprocess::SubprocessManager,
) {
    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }
        // Poll subprocess status to detect crashes and log them.
        mcp.check_status();
        lsp.check_status();
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
}

#[tokio::main]
async fn main() {
    // Project root must be found before logging is initialised so we know
    // where to write the log file.
    let project_root = resolve_project_root();

    // Logging must be initialised before any other subsystem so that all
    // startup events are captured. The guard keeps the background log-writer
    // thread alive for the lifetime of the process.
    let _log_guard = logging::init(&project_root);

    info!(
        subsystem = "health",
        version = env!("CARGO_PKG_VERSION"),
        "[health] orqa-daemon starting"
    );
    info!(
        subsystem = "health",
        root = %project_root.display(),
        "[health] project root located"
    );

    guard_single_instance(&project_root);
    establish_pid(&project_root);

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    register_shutdown_handler(Arc::clone(&shutdown_flag), project_root.clone());

    let tray_status = tray::start();
    info!(
        subsystem = "health",
        status = ?tray_status,
        "[health] system tray initialised"
    );

    run(project_root.clone(), Arc::clone(&shutdown_flag)).await;

    // Final cleanup in case the signal handler did not run (e.g., clean exit
    // via another code path).
    let _ = process::cleanup_pid(&project_root);
    info!(subsystem = "health", "[health] orqa-daemon stopped");
}
