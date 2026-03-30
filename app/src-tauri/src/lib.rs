//! OrqaStudio Tauri application entry point.
//!
//! Bootstraps the Tauri builder with all plugins, commands, and application state.

/// CLI tool runner and registration.
pub mod cli_tools;
/// Tauri IPC command handlers (all `#[tauri::command]` functions).
pub mod commands;
/// SQLite database initialization and migrations.
pub mod db;
/// Domain logic: session management, prompt building, tool dispatch, workflow tracking.
pub mod domain;
/// Application-level error type.
pub mod error;
/// Plugin hook lifecycle management.
pub mod hooks;
/// Structured logging initialization for the Tauri process.
pub mod logging;
/// Plugin discovery, installation, and lifecycle management.
pub mod plugins;
/// Persistence layer: SQLite-backed repositories for sessions, messages, and lessons.
pub mod repo;
/// ONNX-based semantic search: indexing, chunking, and querying.
pub mod search;
/// Background server processes: IPC socket for CLI integration.
pub mod servers;
/// Claude sidecar process manager: spawn, restart, and health monitoring.
pub mod sidecar;
/// Application startup sequencing and task progress tracking.
pub mod startup;
/// Tauri `manage`-d application state structs.
pub mod state;
/// File system watcher for `.orqa/` artifact change detection.
pub mod watcher;

use std::sync::Arc;

use tauri::Manager;

use crate::startup::TaskStatus;

/// Initialize the SQLite database at `db_path` and return the connection.
fn setup_database(db_path_str: &str) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    let conn =
        db::init_db(db_path_str).map_err(|e| format!("failed to initialize database: {e}"))?;
    Ok(conn)
}

/// Construct and return the `AppState`, registering startup tasks.
fn build_app_state(
    conn: rusqlite::Connection,
    tracker: &Arc<startup::StartupTracker>,
) -> Result<state::AppState, Box<dyn std::error::Error>> {
    tracker.register("sidecar", "Sidecar")?;
    tracker.register("embedding_model", "Embedding model")?;

    Ok(state::AppState {
        db: state::DbState {
            conn: std::sync::Mutex::new(conn),
        },
        sidecar: state::SidecarState {
            manager: sidecar::manager::SidecarManager::new(),
            pending_approvals: std::sync::Mutex::new(std::collections::HashMap::new()),
        },
        search: state::SearchState {
            engine: std::sync::Mutex::new(None),
        },
        startup: state::StartupState {
            tracker: Arc::clone(tracker),
        },
        enforcement: state::EnforcementState {
            engine: std::sync::Mutex::new(None),
        },
        session: state::SessionState {
            process_state: std::sync::Mutex::new(
                domain::process_state::SessionProcessState::default(),
            ),
            tracker_config: std::sync::Mutex::new(
                domain::workflow_config::default_tracker_config(),
            ),
            workflow_tracker: std::sync::Mutex::new(
                domain::workflow_tracker::WorkflowTracker::new(
                    domain::workflow_config::default_tracker_config(),
                ),
            ),
            process_gates: std::sync::Mutex::new(
                domain::workflow_config::default_process_gates(),
            ),
        },
        artifacts: state::ArtifactState {
            watcher: Arc::new(std::sync::Mutex::new(None)),
            graph: std::sync::Mutex::new(None),
            knowledge_injector: std::sync::Mutex::new(None),
        },
        cli_tools: state::CliToolState {
            runner: cli_tools::runner::CliToolRunner::new(),
        },
    })
}

/// Auto-start the sidecar process, updating the tracker with the result.
fn start_sidecar(app_state: &state::AppState, tracker: &Arc<startup::StartupTracker>) {
    tracker
        .update(
            "sidecar",
            TaskStatus::InProgress,
            Some("Starting...".into()),
        )
        .unwrap_or_else(|e| tracing::warn!("tracker update failed: {e}"));

    match commands::sidecar_commands::ensure_sidecar_running(app_state) {
        Ok(()) => {
            tracker
                .update("sidecar", TaskStatus::Done, None)
                .unwrap_or_else(|e| tracing::warn!("tracker update failed: {e}"));
        }
        Err(e) => {
            tracing::warn!("failed to auto-start sidecar: {e}");
            tracker
                .update("sidecar", TaskStatus::Error, Some(e.to_string()))
                .unwrap_or_else(|err| tracing::warn!("tracker update failed: {err}"));
        }
    }
}

/// Spawn a background task that pre-downloads the embedding model.
fn spawn_model_download(model_dir: std::path::PathBuf, tracker: Arc<startup::StartupTracker>) {
    tracker
        .update(
            "embedding_model",
            TaskStatus::InProgress,
            Some("Checking...".into()),
        )
        .unwrap_or_else(|e| tracing::warn!("tracker update failed: {e}"));

    tauri::async_runtime::spawn(async move {
        match search::embedder::ensure_model_exists(&model_dir, |_file, downloaded, total| {
            if let Some(total) = total {
                let pct = (downloaded as f64 / total as f64 * 100.0) as u32;
                tracker
                    .update(
                        "embedding_model",
                        TaskStatus::InProgress,
                        Some(format!("{pct}%")),
                    )
                    .unwrap_or_else(|e| tracing::warn!("tracker update failed: {e}"));
            }
        })
        .await
        {
            Ok(()) => {
                tracker
                    .update("embedding_model", TaskStatus::Done, None)
                    .unwrap_or_else(|e| tracing::warn!("tracker update failed: {e}"));
            }
            Err(e) => {
                tracing::warn!("failed to pre-download embedding model: {e}");
                tracker
                    .update("embedding_model", TaskStatus::Error, Some(e.to_string()))
                    .unwrap_or_else(|err| tracing::warn!("tracker update failed: {err}"));
            }
        }
    });
}

/// Run the Tauri setup callback: initialise logging, database, sidecar, and model download.
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

    start_sidecar(&app_state, &tracker);

    // Start IPC socket server for CLI ↔ App communication (orqa mcp / orqa lsp)
    servers::ipc_socket::start(None);

    app.manage(app_state);

    // When launched by `orqa dev`, ORQA_PROJECT_ROOT is set. In this mode:
    // 1. Auto-complete first-run setup (skip the wizard)
    // 2. Auto-open the project so a fresh DB is immediately usable
    if let Ok(project_root) = std::env::var("ORQA_PROJECT_ROOT") {
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

        // Auto-open the project so artifacts are immediately available.
        match commands::project_commands::project_open(project_root.clone(), state) {
            Ok(project) => {
                tracing::info!(
                    path = %project.path,
                    "auto-opened project from ORQA_PROJECT_ROOT"
                );
            }
            Err(e) => {
                tracing::warn!(
                    root = %project_root,
                    err = %e,
                    "failed to auto-open project from ORQA_PROJECT_ROOT"
                );
            }
        }
    }

    let model_dir = app_dir.join("models").join("all-MiniLM-L6-v2");
    spawn_model_download(model_dir, Arc::clone(&tracker));

    Ok(())
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
