// OrqaStudio daemon — persistent background process.
//
// The daemon provides the always-on infrastructure layer for an OrqaStudio
// project: health monitoring, file watching, LSP server lifecycle management,
// and the system tray icon.
//
// MCP is NOT managed by the daemon. MCP uses stdio transport — each LLM client
// (e.g., Claude Code) spawns its own `orqa-mcp-server` process. See mcp.rs for
// the full rationale. LSP is managed here because it uses TCP and legitimately
// runs as a single shared persistent process.
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
//   9. Start LSP server subprocess in TCP mode (graceful degradation if binary absent).
//  10. Block until the shutdown signal fires.
//  11. Stop the LSP subprocess.
//  12. Clean up the PID file and exit.

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
/// cannot start). Starts the LSP server subprocess in TCP mode; it degrades
/// gracefully if the binary is not yet built. Runs the polling event loop until
/// the shutdown flag is set, then stops the LSP subprocess.
///
/// MCP is not started here — see mcp.rs for why MCP is client-managed.
///
/// Called from `main()` on a background thread that owns the tokio runtime.
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

    // Start the LSP server subprocess in TCP mode. LSP is legitimately
    // persistent — a single process serves all connected editors over TCP.
    // MCP is not started here; clients spawn their own orqa-mcp-server process.
    let mut lsp_manager = lsp::start_lsp(&project_root, daemon_port);

    run_event_loop(&shutdown_flag, &mut lsp_manager).await;

    // Graceful shutdown: stop the LSP subprocess before exiting.
    lsp_manager.stop();
}

/// Poll the shutdown flag and yield to the tokio runtime between checks.
///
/// Also polls LSP subprocess status on each iteration so crashes are logged
/// promptly. Polling every 250 ms adds negligible overhead for a long-running
/// background process.
async fn run_event_loop(
    shutdown_flag: &Arc<AtomicBool>,
    lsp: &mut crate::subprocess::SubprocessManager,
) {
    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }
        // Poll LSP subprocess status to detect crashes and log them.
        lsp.check_status();
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
}

/// Entry point for the daemon.
///
/// The system tray on Windows requires the GUI message pump on the main OS
/// thread. Tokio's `#[tokio::main]` would own the main thread for the async
/// runtime, blocking the message pump. To satisfy both requirements:
///
///   1. Perform all pre-flight setup synchronously on the main thread.
///   2. Spawn the tokio runtime on a background thread (`runtime_thread`).
///   3. Run the tray event loop on the main thread via `tray::run_tray_loop`.
///   4. On tray exit (Quit or shutdown signal), join the background thread.
///
/// This arrangement means the tokio `run()` function and the tray loop share
/// the `shutdown_flag` `AtomicBool` as the synchronisation primitive.
fn main() {
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

    // Spawn the tokio runtime on a background thread so the main thread
    // remains free for the OS GUI message pump (required by tray-icon).
    let runtime_root = project_root.clone();
    let runtime_flag = Arc::clone(&shutdown_flag);
    let runtime_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
        rt.block_on(run(runtime_root, runtime_flag));
    });

    // Run the tray loop on the main thread. Returns when Quit is selected or
    // the shutdown flag is set externally.
    let tray_status = tray::run_tray_loop(Arc::clone(&shutdown_flag));
    info!(
        subsystem = "health",
        status = ?tray_status,
        "[health] system tray exited"
    );

    // Ensure the shutdown flag is set so the background tokio runtime exits
    // its event loop even if the tray returned Headless.
    shutdown_flag.store(true, std::sync::atomic::Ordering::SeqCst);

    // Wait for all async subsystems to finish cleanly.
    if let Err(e) = runtime_thread.join() {
        eprintln!("runtime thread panicked: {e:?}");
    }

    // Final cleanup in case the signal handler did not run (e.g., clean exit
    // via another code path).
    let _ = process::cleanup_pid(&project_root);
    info!(subsystem = "health", "[health] orqa-daemon stopped");
}
