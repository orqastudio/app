//! OrqaStudio daemon — persistent background process.
//
//! The daemon provides the always-on infrastructure layer for an OrqaStudio
//! project: health monitoring, file watching, LSP and MCP server lifecycle
//! management, and the system tray icon.
//!
//! Both LSP and MCP servers are managed by the daemon in TCP mode. This allows
//! a single persistent process to serve all connected clients (editors for LSP,
//! LLM clients for MCP) without each client managing its own subprocess.
//!
//! Startup sequence:
//!   1. Locate the project root (walk up from CWD until .orqa/ is found).
//!   2. Initialise structured logging (file + optional TTY console).
//!   3. Check for an existing live daemon instance via the PID file.
//!   4. Write our own PID file.
//!   5. Register Ctrl-C / SIGTERM handler for graceful shutdown.
//!   6. Start the system tray (currently headless — see tray.rs).
//!   7. Start the health HTTP endpoint.
//!   8. Start file watchers on .orqa/ and plugins/.
//!   9. Start LSP server subprocess in TCP mode (graceful degradation if binary absent).
//!  10. Start MCP server subprocess in TCP mode (graceful degradation if binary absent).
//!  11. Block until the shutdown signal fires.
//!  12. Stop the MCP and LSP subprocesses.
//!  13. Clean up the PID file and exit.

mod compact_context;
mod config;
mod context;
mod event_bus;
mod event_store;
mod health;
mod knowledge;
mod logging;
mod lsp;
mod mcp;
mod parse;
mod process;
mod prompt;
mod session_start;
mod subprocess;
mod tray;
mod watcher;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::subprocess::{ProcessSnapshot, SubprocessManager};
use crate::tray::SubprocessStatuses;

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
            // Logging is not yet initialised at this point — print directly to stderr.
            #[allow(clippy::print_stderr)]
            { eprintln!("error: no OrqaStudio project found — {e}"); }
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
            error!(subsystem = "health", error = %e, "[health] failed to remove PID file during shutdown");
        }
    })
    .expect("failed to register Ctrl-C handler");
}

/// Start all daemon subsystems and block until the shutdown flag is set.
///
/// Loads DaemonConfig from orqa.toml at the project root. Spawns the health
/// server as a background tokio task. Starts the file watcher on `.orqa/` and
/// `plugins/` (with a warning fallback if the watcher cannot start). Starts the
/// LSP and MCP server subprocesses in TCP mode; both degrade gracefully if their
/// binaries are not yet built. Runs the polling event loop until the shutdown
/// flag is set, then stops both subprocesses. Writes subprocess statuses to
/// `subprocess_statuses` on every polling cycle for the tray thread to read.
///
/// `event_bus` is created in `main()` before logging is initialised so the
/// `EventBusLayer` can be registered in the tracing subscriber stack.  The same
/// arc is passed here so all subsystems share a single bus instance.
///
/// Called from `main()` on a background thread that owns the tokio runtime.
#[allow(clippy::too_many_lines)]
async fn run(
    project_root: PathBuf,
    shutdown_flag: Arc<AtomicBool>,
    subprocess_statuses: Arc<Mutex<SubprocessStatuses>>,
    event_bus: Arc<event_bus::EventBus>,
) {
    let daemon_port = health::resolve_port();

    // Load runtime config from orqa.toml. Falls back to defaults when absent.
    let daemon_config = config::DaemonConfig::load(&project_root);

    // Shared registry of process snapshots written by the event loop on every
    // polling cycle. The health endpoint reads this to populate the `processes`
    // array in GET /health, enabling OrqaDev auto-discovery.
    let process_snapshots: Arc<Mutex<Vec<ProcessSnapshot>>> = Arc::new(Mutex::new(Vec::new()));

    // Open the SQLite event store and spawn the background batch writer.
    // Degrades gracefully if the database cannot be opened.
    let event_store = match event_store::EventStore::open(&project_root, daemon_config.event_retention_days) {
        Ok(store) => {
            event_store::spawn_batch_writer(Arc::clone(&event_bus), Arc::clone(&store));

            // Spawn a background task that purges expired events every 6 hours.
            // This runs independently of the batch writer and uses its own interval.
            let purge_store = Arc::clone(&store);
            let retention_days = daemon_config.event_retention_days;
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(6 * 3600));
                // Skip the immediate first tick — startup purge already ran inside open().
                interval.tick().await;
                loop {
                    interval.tick().await;
                    event_store::EventStore::purge(Arc::clone(&purge_store), retention_days).await;
                }
            });

            Some(store)
        }
        Err(e) => {
            warn!(
                subsystem = "event-store",
                error = %e,
                "[event-store] could not open events.db — running without persistence"
            );
            None
        }
    };

    tokio::spawn({
        let bus = Arc::clone(&event_bus);
        let snapshots = Arc::clone(&process_snapshots);
        async move {
            if let Err(e) = health::start(daemon_port, daemon_config, bus, event_store, snapshots).await {
                error!(subsystem = "health", error = %e, "[health] health server failed");
            }
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

    // Start the LSP server subprocess in TCP mode. A single process serves all
    // connected editors simultaneously.
    let mut lsp_manager = lsp::start_lsp(&project_root, daemon_port);

    // Start the MCP server subprocess in TCP mode. A single process serves all
    // connected LLM clients simultaneously. Degrades gracefully if the binary
    // is absent — LLM clients can still spawn orqa-mcp-server in stdio mode.
    let mut mcp_manager = mcp::start_mcp(&project_root, daemon_port);

    run_event_loop(
        &shutdown_flag,
        &mut lsp_manager,
        &mut mcp_manager,
        &subprocess_statuses,
        &process_snapshots,
    )
    .await;

    // Graceful shutdown: stop both subprocesses before exiting.
    mcp_manager.stop();
    lsp_manager.stop();
}

/// Poll the shutdown flag and yield to the tokio runtime between checks.
///
/// Polls both LSP and MCP subprocess statuses on each iteration so crashes are
/// logged promptly. Writes the latest statuses to `subprocess_statuses` for the
/// tray thread and to `process_snapshots` for the health endpoint. Polling every
/// 250 ms adds negligible overhead for a long-running background process.
async fn run_event_loop(
    shutdown_flag: &Arc<AtomicBool>,
    lsp: &mut SubprocessManager,
    mcp: &mut SubprocessManager,
    subprocess_statuses: &Arc<Mutex<SubprocessStatuses>>,
    process_snapshots: &Arc<Mutex<Vec<ProcessSnapshot>>>,
) {
    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }
        // Poll subprocess statuses to detect crashes and log them.
        let lsp_status = lsp.check_status();
        let mcp_status = mcp.check_status();

        // Update the shared status snapshot for the tray thread to read.
        if let Ok(mut guard) = subprocess_statuses.lock() {
            guard.lsp = lsp_status;
            guard.mcp = mcp_status;
        }

        // Update the process snapshot registry for the health endpoint.
        // Each manager generates its own snapshot so the health handler never
        // needs to access subprocess managers directly.
        if let Ok(mut guard) = process_snapshots.lock() {
            *guard = vec![
                lsp.snapshot("lsp"),
                mcp.snapshot("mcp"),
            ];
        }

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

    // Create the event bus before logging is initialised so that the
    // `EventBusLayer` can be registered in the subscriber stack.  All tracing
    // events emitted after `logging::init` will flow through the bus.
    let event_bus = event_bus::EventBus::new();

    // Logging must be initialised before any other subsystem so that all
    // startup events are captured. The guard keeps the background log-writer
    // thread alive for the lifetime of the process.
    let _log_guard = logging::init(&project_root, Some(Arc::clone(&event_bus)));

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

    // Shared subprocess status snapshot: written by the background event loop,
    // read by the tray thread to display LSP and MCP status in the menu.
    let subprocess_statuses = Arc::new(Mutex::new(SubprocessStatuses::default()));

    // Spawn the tokio runtime on a background thread so the main thread
    // remains free for the OS GUI message pump (required by tray-icon).
    let runtime_root = project_root.clone();
    let runtime_flag = Arc::clone(&shutdown_flag);
    let runtime_statuses = Arc::clone(&subprocess_statuses);
    let runtime_bus = Arc::clone(&event_bus);
    let runtime_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
        rt.block_on(run(runtime_root, runtime_flag, runtime_statuses, runtime_bus));
    });

    // Run the tray loop on the main thread. Returns when Quit is selected or
    // the shutdown flag is set externally.
    let tray_status = tray::run_tray_loop(Arc::clone(&shutdown_flag), subprocess_statuses);
    info!(
        subsystem = "health",
        status = ?tray_status,
        "[health] system tray exited"
    );

    // Ensure the shutdown flag is set so the background tokio runtime exits
    // its event loop even if the tray returned Headless.
    shutdown_flag.store(true, Ordering::SeqCst);

    // Wait for all async subsystems to finish cleanly.
    if let Err(e) = runtime_thread.join() {
        error!(subsystem = "health", error = ?e, "[health] runtime thread panicked");
    }

    // Final cleanup in case the signal handler did not run (e.g., clean exit
    // via another code path).
    let _ = process::cleanup_pid(&project_root);
    info!(subsystem = "health", "[health] orqa-daemon stopped");
}
