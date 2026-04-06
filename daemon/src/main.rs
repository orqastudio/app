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
mod graph_state;
mod health;
mod knowledge;
mod logging;
mod lsp;
mod mcp;
mod process;
mod prompt;
mod routes;
mod session_start;
mod subprocess;
mod tray;
mod watcher;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use orqa_engine_types::types::event::LogEvent;
use orqa_storage::Storage;
use tokio::sync::broadcast;

use crate::graph_state::GraphState;
use crate::subprocess::{ProcessSnapshot, SubprocessManager};
use crate::tray::SubprocessStatuses;

/// Maximum events to accumulate before flushing to SQLite.
const EVENT_BATCH_SIZE: usize = 100;

/// Maximum time to wait before flushing a non-empty event batch.
const EVENT_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

/// Spawn a background tokio task that drains the event bus into unified storage.
///
/// Subscribes to `bus`, accumulates events into a local batch, and flushes
/// to the `log_events` table via `spawn_blocking` when the batch reaches
/// `EVENT_BATCH_SIZE` or `EVENT_FLUSH_INTERVAL` elapses — whichever comes first.
/// Exits automatically when the bus sender is dropped (daemon shutdown),
/// after flushing any remaining events.
fn spawn_event_batch_writer(bus: Arc<event_bus::EventBus>, storage: Arc<Storage>) {
    tokio::spawn(async move {
        let mut rx: broadcast::Receiver<LogEvent> = bus.subscribe();
        let mut batch: Vec<LogEvent> = Vec::with_capacity(EVENT_BATCH_SIZE);
        let flush_interval = tokio::time::interval(EVENT_FLUSH_INTERVAL);
        tokio::pin!(flush_interval);

        loop {
            tokio::select! {
                biased;

                result = rx.recv() => {
                    match result {
                        Ok(event) => {
                            batch.push(event);
                            if batch.len() >= EVENT_BATCH_SIZE {
                                flush_event_batch(&storage, &mut batch).await;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!(
                                subsystem = "storage",
                                dropped = n,
                                "[storage] event bus lagged — {n} events lost"
                            );
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            if !batch.is_empty() {
                                flush_event_batch(&storage, &mut batch).await;
                            }
                            info!(
                                subsystem = "storage",
                                "[storage] event bus closed, batch writer exiting"
                            );
                            break;
                        }
                    }
                }

                _ = flush_interval.tick() => {
                    if !batch.is_empty() {
                        flush_event_batch(&storage, &mut batch).await;
                    }
                }
            }
        }
    });
}

/// Move `batch` into `spawn_blocking` and flush it to the events repo.
///
/// Clears `batch` after dispatching. The `Arc<Storage>` clone ensures the
/// storage outlives the blocking closure.
async fn flush_event_batch(storage: &Arc<Storage>, batch: &mut Vec<LogEvent>) {
    let storage_clone = Arc::clone(storage);
    let events = std::mem::take(batch);
    if let Err(e) = tokio::task::spawn_blocking(move || {
        if let Err(e) = storage_clone.events().insert_batch(events) {
            error!(
                subsystem = "storage",
                error = %e,
                "[storage] event batch insert failed"
            );
        }
    })
    .await
    {
        error!(
            subsystem = "storage",
            error = ?e,
            "[storage] spawn_blocking panicked during event flush"
        );
    }
}

use tracing::{error, info, warn};

/// Install a panic hook that logs panics through the tracing subscriber before
/// chaining to the default hook (which writes to stderr).
///
/// Without this, panics in `tokio::spawn`'d tasks are silently swallowed: the
/// tokio executor catches the unwind and drops the task without surfacing
/// anything to tracing, and the default panic hook only writes to stderr —
/// which, when the daemon runs detached without a TTY, goes nowhere. That
/// turned a routing-syntax panic in the health router into a mysterious hang
/// with no log trail. This hook ensures every panic hits `.state/daemon.log`.
///
/// Must be called after `logging::init` so the tracing subscriber is live.
/// Panics before that point still fall through to the default hook.
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let location = info.location().map_or_else(
            || "unknown".to_owned(),
            |loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()),
        );
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .map(ToOwned::to_owned)
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<non-string panic payload>".to_owned());
        let thread = std::thread::current()
            .name()
            .unwrap_or("<unnamed>")
            .to_owned();
        error!(
            subsystem = "panic",
            thread = %thread,
            location = %location,
            "thread '{thread}' panicked at {location}: {message}"
        );
        // Chain to the default hook so stderr still gets the full backtrace.
        default_hook(info);
    }));
}

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
            {
                eprintln!("error: no OrqaStudio project found — {e}");
            }
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

    // Open the unified SQLite storage at .state/orqa.db. This replaces the
    // separate daemon.db and events.db that previously lived in .state/.
    // Degrades gracefully if the database cannot be opened.
    let storage = match Storage::open(&project_root) {
        Ok(s) => {
            // Spawn background batch writer: drains the event bus into the
            // unified storage's log_events table via batched inserts.
            spawn_event_batch_writer(Arc::clone(&event_bus), Arc::clone(&s));

            // Spawn a background task that purges expired events every 6 hours.
            let purge_storage = Arc::clone(&s);
            let retention_days = daemon_config.event_retention_days;
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(6 * 3600));
                // Skip the immediate first tick — startup purge already ran inside open().
                interval.tick().await;
                loop {
                    interval.tick().await;
                    let s = Arc::clone(&purge_storage);
                    if let Err(e) =
                        tokio::task::spawn_blocking(move || s.events().purge(retention_days)).await
                    {
                        error!(
                            subsystem = "storage",
                            error = ?e,
                            "[storage] spawn_blocking panicked during event purge"
                        );
                    }
                }
            });

            Some(s)
        }
        Err(e) => {
            warn!(
                subsystem = "storage",
                error = %e,
                "[storage] could not open orqa.db — running without persistence"
            );
            None
        }
    };

    // Build the cached artifact graph and validation context. Failures are
    // non-fatal — the daemon starts without graph state and attempts a reload
    // on the next file change.
    let graph_state = match GraphState::build(&project_root) {
        Ok(gs) => {
            info!(
                subsystem = "graph-state",
                artifact_count = gs.artifact_count(),
                "[graph-state] initial graph built ({} artifacts)",
                gs.artifact_count()
            );
            gs
        }
        Err(e) => {
            warn!(
                subsystem = "graph-state",
                error = %e,
                "[graph-state] could not build initial graph — starting without cached state"
            );
            // Build an empty graph state using a fresh empty graph.
            // This is safe because all route handlers guard against empty state.
            GraphState::build_empty(&project_root)
        }
    };

    tokio::spawn({
        let bus = Arc::clone(&event_bus);
        let snapshots = Arc::clone(&process_snapshots);
        let gs = graph_state.clone();
        let st = storage.clone();
        async move {
            if let Err(e) = health::start(daemon_port, daemon_config, bus, st, snapshots, gs).await
            {
                error!(subsystem = "health", error = %e, "[health] health server failed");
            }
        }
    });

    // The watcher handle must be kept alive for the duration of the event loop.
    // If starting the watcher fails the daemon continues without file watching.
    let _watcher_handle = match watcher::start_watcher(&project_root, graph_state) {
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
            *guard = vec![lsp.snapshot("lsp"), mcp.snapshot("mcp")];
        }

        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
}

/// All daemon startup and async runtime logic, run on the background thread.
///
/// On Windows the main OS thread stack is 1 MB. The layered
/// `tracing_subscriber` with multiple `EnvFilter` layers involves deeply-
/// nested generic types whose initialisation overflows 1 MB.  By executing
/// ALL pre-flight work here, on a thread with an explicit 8 MB stack, we
/// avoid the overflow entirely.  The main thread only receives the two
/// `Arc`s it needs for the tray loop via a `std::sync::mpsc` channel, then
/// blocks in `tray::run_tray_loop` which has a shallow stack profile.
///
/// Returns the `(shutdown_flag, subprocess_statuses)` pair via `tx` as soon
/// as they are created so the main thread can start the tray without waiting
/// for async subsystems.
fn run_daemon(
    project_root: PathBuf,
    tx: std::sync::mpsc::SyncSender<(Arc<AtomicBool>, Arc<Mutex<SubprocessStatuses>>)>,
) {
    // Create the event bus before logging is initialised so that the
    // `EventBusLayer` can be registered in the subscriber stack.  All tracing
    // events emitted after `logging::init` will flow through the bus.
    let event_bus = event_bus::EventBus::new();

    // Logging must be initialised before any other subsystem so that all
    // startup events are captured. The guard keeps the background log-writer
    // thread alive for the lifetime of the process.
    let _log_guard = logging::init(&project_root, Some(Arc::clone(&event_bus)));

    // Install the panic hook immediately after logging — from this point on,
    // panics in any thread or tokio task are logged to daemon.log before the
    // default hook writes to stderr.
    install_panic_hook();

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

    // Shared subprocess status snapshot: written by the async event loop,
    // read by the main-thread tray loop to display LSP/MCP status.
    let subprocess_statuses = Arc::new(Mutex::new(SubprocessStatuses::default()));

    // Send the two shared Arcs to the main thread so the tray can start.
    // Use a rendezvous send (capacity 0) — the main thread is already blocked
    // on recv() so this completes immediately.
    let _ = tx.send((Arc::clone(&shutdown_flag), Arc::clone(&subprocess_statuses)));

    // Start the tokio runtime and block until shutdown.
    // Pin the run() future on the heap so its state machine (which captures
    // all local variables across await points) does not consume stack space.
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(Box::pin(run(
        project_root.clone(),
        Arc::clone(&shutdown_flag),
        Arc::clone(&subprocess_statuses),
        Arc::clone(&event_bus),
    )));

    // Final cleanup in case the signal handler did not run (e.g., clean exit
    // via another code path).
    let _ = process::cleanup_pid(&project_root);
    info!(subsystem = "health", "[health] orqa-daemon stopped");
}

/// Entry point for the daemon.
///
/// The system tray on Windows requires the GUI message pump on the main OS
/// thread.  On Windows, the default main thread stack is only 1 MB, which is
/// too small for the deeply-nested generic types in `tracing_subscriber`.  To
/// satisfy both constraints:
///
///   1. Create a rendezvous channel.
///   2. Spawn a background thread with 8 MB stack (`daemon-runtime`) that runs
///      ALL pre-flight setup (logging, PID, signal handler) and the tokio
///      runtime.  As soon as the shared `Arc`s are ready it sends them over
///      the channel and continues running the async event loop.
///   3. The main thread receives the `Arc`s and enters the tray loop.
///   4. On tray exit (Quit or shutdown signal) the main thread sets the
///      shutdown flag and joins the background thread.
fn main() {
    // Project root resolution is lightweight (stat calls only) and safe on 1 MB.
    let project_root = resolve_project_root();

    // Rendezvous channel (capacity 0): the background thread sends the shared
    // Arcs exactly once; the main thread blocks here until they arrive.
    let (tx, rx) = std::sync::mpsc::sync_channel(0);

    let bg_root = project_root.clone();
    // Spawn with 8 MB stack. The main thread's 1 MB Windows stack is too
    // small for daemon startup; 8 MB provides comfortable headroom.
    let runtime_thread = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .name("daemon-runtime".to_owned())
        .spawn(move || run_daemon(bg_root, tx))
        .expect("failed to spawn runtime thread");

    // Block until the background thread has finished pre-flight and sent the
    // shared Arcs.  This is quick (< 100 ms in practice).
    let (shutdown_flag, subprocess_statuses) = rx
        .recv()
        .expect("daemon-runtime thread exited before sending tray handles");

    // Run the tray loop on the main thread (Win32 message pump requirement).
    // Returns when Quit is selected or the shutdown flag is set externally.
    let tray_status = tray::run_tray_loop(Arc::clone(&shutdown_flag), subprocess_statuses);

    // Ensure the shutdown flag is set so the async event loop exits even if
    // the tray returned Headless (no tray support on this platform).
    shutdown_flag.store(true, Ordering::SeqCst);

    // tray_status was already logged inside run_daemon via the tracing subscriber.
    let _ = tray_status;

    // Wait for all async subsystems to finish cleanly.
    if let Err(e) = runtime_thread.join() {
        // tracing may not be initialised if run_daemon panicked during logging::init,
        // so fall back to stderr.
        #[allow(clippy::print_stderr)]
        {
            eprintln!("[health] runtime thread panicked: {e:?}");
        }
    }
}
