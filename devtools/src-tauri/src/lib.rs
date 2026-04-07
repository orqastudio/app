//! OrqaDev Tauri application setup and command registration.
//!
//! Bootstraps the Tauri builder with logging, shared state (event ring buffer,
//! unified storage), and IPC commands for the developer tools companion app.

/// Dev environment controller — spawns `orqa dev start-processes` and pipes output.
pub mod dev_controller;

/// SSE event consumer — connects to daemon, buffers events, exposes IPC commands.
pub mod events;

/// IPC command for querying process status from the daemon health endpoint.
pub mod process_status;

/// IPC command wrappers for the session database backed by orqa-storage.
pub mod session_commands;

/// IPC command wrappers for issue group queries backed by orqa-storage.
pub mod issue_group_commands;

use std::sync::Arc;

use orqa_storage::Storage;
use tauri::Manager as _;
use tracing_subscriber::EnvFilter;

/// Initialize structured logging for the devtools process.
fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

/// Run the Tauri setup callback: initialise logging, shared state, and the
/// background SSE consumer that connects to the daemon event bus.
///
/// Also opens the unified storage, keying off `ORQA_PROJECT_ROOT` env var
/// with a fallback to the current working directory.
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    tracing::info!("OrqaDev starting");

    // Resolve the project root from env var or fall back to cwd.
    // The CLI sets ORQA_PROJECT_ROOT when launching devtools so the DB lands
    // in the correct .state/ directory for the current project.
    let project_root = std::env::var("ORQA_PROJECT_ROOT").map_or_else(
        // BINARY ENTRY POINT: current_dir() failure is an unrecoverable OS error —
        // OrqaDev cannot determine which project to open without a working directory.
        |_| std::env::current_dir().expect("cannot read cwd"),
        std::path::PathBuf::from,
    );

    // Open the unified storage and register it as managed state.
    let storage =
        Storage::open(&project_root).map_err(|e| format!("failed to open storage: {e}"))?;

    // Mark any orphaned sessions from a previous crash as interrupted
    // before creating the new session.
    storage
        .devtools()
        .mark_orphaned_sessions_interrupted()
        .map_err(|e| format!("failed to mark orphaned sessions: {e}"))?;

    // Create the session for this devtools open.
    let session_id = uuid::Uuid::new_v4().to_string();
    let started_at = now_ms();
    storage
        .devtools()
        .create_session(&session_id, started_at)
        .map_err(|e| format!("failed to create session: {e}"))?;

    // Purge sessions older than 30 days.
    let _ = storage.devtools().purge_old_sessions(30);

    app.manage(Arc::clone(&storage));

    // Store the active session id as managed state so commands can retrieve it.
    app.manage(Arc::new(ActiveSession(session_id.clone())));

    let consumer_state = events::EventConsumerState::new();
    app.manage(Arc::clone(&consumer_state));

    let batch_writer = events::EventBatchWriter::new(Arc::clone(&storage), session_id);
    app.manage(Arc::new(batch_writer));

    let dev_ctrl_state = dev_controller::DevControllerState::new();
    app.manage(dev_ctrl_state);

    let graph_topo = Arc::new(dev_controller::GraphTopologyState(tokio::sync::Mutex::new(
        None,
    )));
    app.manage(graph_topo);

    // Dev mode: set by the CLI via ORQA_DEV_MODE=1. When absent, the devtools
    // runs in attach/production mode (no process management, runtime events only).
    let dev_mode = std::env::var("ORQA_DEV_MODE").is_ok_and(|v| v == "1" || v == "true");
    app.manage(DevModeFlag(dev_mode));

    // Always connect the SSE consumer on startup. In both dev and production
    // modes, the daemon is already running by the time the devtools window opens
    // (started by the CLI process manager or the production app respectively).
    // The SSE consumer has retry logic with exponential backoff, so it handles
    // the brief window where the daemon may still be starting.
    events::spawn_consumer(app.handle().clone(), Arc::clone(&consumer_state));

    Ok(())
}

/// Returns the current Unix timestamp in milliseconds.
fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Managed state holding the UUID of the currently active devtools session.
pub struct ActiveSession(pub String);

/// Whether the devtools is running in dev mode (launched by `orqa dev`) or
/// production/attach mode (opened from the main app to inspect a running daemon).
pub struct DevModeFlag(pub bool);

/// IPC command — returns true when the devtools is running in dev mode.
#[tauri::command]
fn devtools_is_dev_mode(flag: tauri::State<'_, DevModeFlag>) -> bool {
    flag.0
}

/// IPC command — returns the process manager dependency graph topology, if available.
///
/// First checks the in-memory state (populated when dev_controller parses the
/// graph-topology JSON event from start-processes). If empty, falls back to
/// reading `.state/graph-topology.json` from disk (written by the CLI when it
/// orchestrates the build directly in the new `orqa dev` flow).
#[tauri::command]
async fn devtools_graph_topology(
    state: tauri::State<'_, Arc<dev_controller::GraphTopologyState>>,
) -> Result<Option<serde_json::Value>, String> {
    // Check in-memory state first (populated via start-processes stdout parsing).
    let in_memory = state.0.lock().await.clone();
    if in_memory.is_some() {
        return Ok(in_memory);
    }

    // Fall back to reading from disk (written by CLI's emitGraphTopology).
    let project_root = std::env::var("ORQA_PROJECT_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let topo_path = project_root.join(".state/graph-topology.json");

    match std::fs::read_to_string(&topo_path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

/// Build and run the Tauri application event loop.
///
/// Uses `.build(generate_context!()).run(callback)` so that the `RunEvent::Exit`
/// handler can call `storage.devtools().end_session()` for a clean session close.
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::all()
                        & !tauri_plugin_window_state::StateFlags::DECORATIONS,
                )
                .build(),
        )
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            events::get_events,
            events::clear_events,
            events::event_buffer_stats,
            events::devtools_query_history,
            process_status::devtools_process_status,
            dev_controller::devtools_start_dev,
            dev_controller::devtools_stop_dev,
            dev_controller::devtools_dev_status,
            session_commands::list_sessions,
            session_commands::query_session_events,
            session_commands::get_current_session,
            session_commands::rename_session,
            session_commands::delete_session,
            issue_group_commands::devtools_list_issue_groups,
            issue_group_commands::devtools_get_issue_group,
            devtools_is_dev_mode,
            devtools_graph_topology,
        ])
        .build(tauri::generate_context!())
        // BINARY ENTRY POINT: Tauri's builder `.build()` returns Result but if it
        // fails the process cannot proceed — there is no OrqaDev without the window.
        .expect("error while building OrqaDev");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            // End the active session so it has a proper ended_at timestamp.
            if let (Some(storage), Some(active)) = (
                app_handle.try_state::<Arc<Storage>>(),
                app_handle.try_state::<Arc<ActiveSession>>(),
            ) {
                let ended_at = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as i64;
                let _ = storage.devtools().end_session(&active.0, ended_at);
            }
        }
    });
}
