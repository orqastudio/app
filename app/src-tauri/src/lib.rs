//! OrqaStudio Tauri application entry point.
//!
//! Bootstraps the Tauri builder with all plugins, commands, and application state.
//! The app is a pure UI layer — all engine operations are delegated to the daemon.

/// Tauri IPC command handlers (all `#[tauri::command]` functions).
pub mod commands;
/// HTTP client for the OrqaStudio daemon REST API.
pub mod daemon_client;
/// SQLite database initialization and migrations.
pub mod db;
/// Domain logic: artifact types, session management, settings, and setup checks.
pub mod domain;
/// Application-level error type.
pub mod error;
/// Structured logging initialization for the Tauri process.
pub mod logging;
/// Persistence layer: SQLite-backed repositories for sessions, messages, and settings.
pub mod repo;
/// Background server processes: IPC socket for CLI integration.
pub mod servers;
/// Application startup sequencing and task progress tracking.
pub mod startup;
/// Tauri `manage`-d application state structs.
pub mod state;
/// File system watcher for `.orqa/` artifact change detection.
pub mod watcher;

use std::sync::Arc;

use tauri::Manager;

/// Initialize the SQLite database at `db_path` and return the connection.
fn setup_database(db_path_str: &str) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    let conn =
        db::init_db(db_path_str).map_err(|e| format!("failed to initialize database: {e}"))?;
    Ok(conn)
}

/// Construct and return the `AppState`.
fn build_app_state(
    conn: rusqlite::Connection,
    tracker: &Arc<startup::StartupTracker>,
) -> Result<state::AppState, Box<dyn std::error::Error>> {
    Ok(state::AppState {
        db: state::DbState {
            conn: std::sync::Mutex::new(conn),
        },
        daemon: state::DaemonState {
            client: daemon_client::DaemonClient::new()?,
        },
        startup: state::StartupState {
            tracker: Arc::clone(tracker),
        },
        artifacts: state::ArtifactState {
            watcher: Arc::new(std::sync::Mutex::new(None)),
        },
    })
}

/// Run the Tauri setup callback: initialise logging and database.
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging(app.handle());

    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("failed to create app data dir: {e}"))?;

    let db_path = app_dir.join("orqa.db");
    let db_path_str = db_path
        .to_str()
        .ok_or_else(|| "app data path is not valid UTF-8".to_owned())?;

    let conn = setup_database(db_path_str).map_err(|e| e.to_string())?;

    let tracker = startup::StartupTracker::new();
    let app_state = build_app_state(conn, &tracker).map_err(|e| e.to_string())?;

    // Start IPC socket server for CLI ↔ App communication (orqa mcp / orqa lsp)
    servers::ipc_socket::start(None);

    app.manage(app_state);

    // When launched by `orqa dev`, ORQA_PROJECT_ROOT is set. Auto-complete setup
    // and open the project so a fresh DB is immediately usable.
    if let Ok(project_root) = std::env::var("ORQA_PROJECT_ROOT") {
        auto_open_dev_project(app, &project_root);
    }

    Ok(())
}

/// Auto-complete first-run setup and open the project when launched via `orqa dev`.
///
/// Called only when `ORQA_PROJECT_ROOT` is set. Marks setup as complete so the
/// wizard is skipped, then calls `project_open` so artifacts are immediately available.
fn auto_open_dev_project(app: &tauri::App, project_root: &str) {
    let state: tauri::State<'_, state::AppState> = app.state();

    // Mark first-run setup as complete — dev environment is already configured.
    {
        let conn = state
            .db
            .conn
            .lock()
            .expect("db lock for setup auto-complete");
        if let Err(e) = repo::settings_repo::set(
            &conn,
            "setup_version",
            &serde_json::json!(1u32),
            "app",
        ) {
            tracing::warn!(err = %e, "failed to auto-complete setup");
        } else {
            tracing::info!("auto-completed first-run setup (dev mode)");
        }
    }

    match commands::project_commands::project_open(project_root.to_owned(), state) {
        Ok(project) => {
            tracing::info!(path = %project.path, "auto-opened project from ORQA_PROJECT_ROOT");
        }
        Err(e) => {
            tracing::warn!(root = %project_root, err = %e, "failed to auto-open project from ORQA_PROJECT_ROOT");
        }
    }
}

/// Register all Tauri plugins on the builder.
fn register_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::all()
                        & !tauri_plugin_window_state::StateFlags::DECORATIONS,
                )
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
}

/// Register all Tauri command handlers on the builder.
#[allow(clippy::too_many_lines)]
fn register_commands(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.invoke_handler(tauri::generate_handler![
        commands::daemon_commands::daemon_health,
        commands::sidecar_commands::sidecar_status,
        commands::sidecar_commands::sidecar_restart,
        commands::stream_commands::stream_send_message,
        commands::stream_commands::stream_stop,
        commands::stream_commands::stream_tool_approval_respond,
        commands::project_commands::project_open,
        commands::project_commands::project_get_active,
        commands::project_commands::project_list,
        commands::session_commands::session_create,
        commands::session_commands::session_list,
        commands::session_commands::session_get,
        commands::session_commands::session_update_title,
        commands::session_commands::session_end,
        commands::session_commands::session_delete,
        commands::message_commands::message_list,
        commands::artifact_commands::artifact_scan_tree,
        commands::artifact_commands::artifact_watch_start,
        commands::project_settings_commands::project_settings_read,
        commands::project_settings_commands::project_settings_write,
        commands::project_settings_commands::project_scan,
        commands::project_settings_commands::project_icon_upload,
        commands::project_settings_commands::project_icon_read,
        commands::settings_commands::settings_set,
        commands::settings_commands::settings_get_all,
        commands::search_commands::get_startup_status,
        commands::setup_commands::get_setup_status,
        commands::setup_commands::check_claude_cli,
        commands::setup_commands::check_claude_auth,
        commands::setup_commands::check_embedding_model,
        commands::setup_commands::complete_setup,
        commands::setup_commands::reauthenticate_claude,
        commands::lesson_commands::lessons_list,
        commands::lesson_commands::lessons_create,
        commands::lesson_commands::lesson_increment_recurrence,
        commands::enforcement_commands::enforcement_rules_list,
        commands::enforcement_commands::enforcement_rules_reload,
        commands::enforcement_commands::enforcement_violations_list,
        commands::enforcement_commands::governance_scan,
        commands::graph_commands::get_artifacts_by_type,
        commands::graph_commands::read_artifact_content,
        commands::graph_commands::get_graph_stats,
        commands::graph_commands::get_graph_health,
        commands::graph_commands::get_artifact_traceability,
        commands::graph_commands::refresh_artifact_graph,
        commands::graph_commands::run_integrity_scan,
        commands::graph_commands::apply_auto_fixes,
        commands::graph_commands::store_health_snapshot,
        commands::graph_commands::get_health_snapshots,
        commands::graph_commands::update_artifact_field,
        commands::status_transition_commands::evaluate_status_transitions,
        commands::status_transition_commands::apply_status_transition,
        commands::cli_tool_commands::get_registered_cli_tools,
        commands::cli_tool_commands::run_cli_tool,
        commands::cli_tool_commands::cli_tool_status,
        commands::plugin_commands::plugin_list_installed,
        commands::plugin_commands::plugin_registry_list,
        commands::plugin_commands::plugin_install_local,
        commands::plugin_commands::plugin_install_github,
        commands::plugin_commands::plugin_uninstall,
        commands::plugin_commands::plugin_check_updates,
        commands::plugin_commands::plugin_get_path,
        commands::plugin_commands::plugin_get_manifest,
        commands::hook_commands::get_registered_hooks,
        commands::hook_commands::generate_hook_dispatchers,
        commands::devtools_commands::launch_devtools,
        commands::devtools_commands::is_devtools_running,
    ])
}

/// Build and run the Tauri application event loop.
pub fn run() {
    let builder = tauri::Builder::default().setup(setup_app);
    let builder = register_plugins(builder);
    register_commands(builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
