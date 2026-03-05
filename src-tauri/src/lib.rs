pub mod commands;
pub mod db;
pub mod domain;
pub mod error;
pub mod repo;
pub mod search;
pub mod sidecar;
pub mod startup;
pub mod state;

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
        db: std::sync::Mutex::new(conn),
        sidecar: sidecar::manager::SidecarManager::new(),
        search: std::sync::Mutex::new(None),
        startup: Arc::clone(tracker),
        pending_approvals: std::sync::Mutex::new(std::collections::HashMap::new()),
        enforcement: std::sync::Mutex::new(None),
        process_state: std::sync::Mutex::new(domain::process_state::SessionProcessState::default()),
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| format!("failed to create app data dir: {e}"))?;

            let db_path = app_dir.join("orqa.db");
            let db_path_str = db_path
                .to_str()
                .ok_or_else(|| "app data path is not valid UTF-8".to_string())?;

            let conn = setup_database(db_path_str).map_err(|e| e.to_string())?;

            let tracker = startup::StartupTracker::new();
            let app_state = build_app_state(conn, &tracker).map_err(|e| e.to_string())?;

            start_sidecar(&app_state, &tracker);

            app.manage(app_state);

            let model_dir = app_dir.join("models").join("bge-small-en-v1.5");
            spawn_model_download(model_dir, Arc::clone(&tracker));

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            // Sidecar commands
            commands::sidecar_commands::sidecar_status,
            commands::sidecar_commands::sidecar_restart,
            // Stream commands
            commands::stream_commands::stream_send_message,
            commands::stream_commands::stream_stop,
            commands::stream_commands::stream_tool_approval_respond,
            // Project commands
            commands::project_commands::project_open,
            commands::project_commands::project_create,
            commands::project_commands::project_get,
            commands::project_commands::project_get_active,
            commands::project_commands::project_list,
            // Session commands
            commands::session_commands::session_create,
            commands::session_commands::session_list,
            commands::session_commands::session_get,
            commands::session_commands::session_update_title,
            commands::session_commands::session_end,
            commands::session_commands::session_delete,
            // Message commands
            commands::message_commands::message_list,
            commands::message_commands::message_search,
            // Artifact commands
            commands::artifact_commands::artifact_list,
            commands::artifact_commands::artifact_get,
            commands::artifact_commands::artifact_get_by_path,
            commands::artifact_commands::artifact_create,
            commands::artifact_commands::artifact_update,
            commands::artifact_commands::artifact_delete,
            commands::artifact_commands::doc_read,
            commands::artifact_commands::doc_tree_scan,
            commands::artifact_commands::governance_list,
            commands::artifact_commands::governance_read,
            // Project settings commands (file-based)
            commands::project_settings_commands::project_settings_read,
            commands::project_settings_commands::project_settings_write,
            commands::project_settings_commands::project_scan,
            commands::project_settings_commands::project_icon_upload,
            commands::project_settings_commands::project_icon_read,
            // Settings commands
            commands::settings_commands::settings_get,
            commands::settings_commands::settings_set,
            commands::settings_commands::settings_get_all,
            // Theme commands
            commands::theme_commands::theme_get_project,
            commands::theme_commands::theme_set_override,
            commands::theme_commands::theme_clear_overrides,
            // Search commands
            commands::search_commands::index_codebase,
            commands::search_commands::search_regex,
            commands::search_commands::search_semantic,
            commands::search_commands::get_index_status,
            commands::search_commands::init_embedder,
            // Startup commands
            commands::search_commands::get_startup_status,
            // Setup commands
            commands::setup_commands::get_setup_status,
            commands::setup_commands::check_claude_cli,
            commands::setup_commands::check_claude_auth,
            commands::setup_commands::check_embedding_model,
            commands::setup_commands::complete_setup,
            commands::setup_commands::reauthenticate_claude,
            // Governance commands
            commands::governance_commands::governance_scan,
            commands::governance_commands::governance_analyze,
            commands::governance_commands::governance_analysis_get,
            commands::governance_commands::recommendations_list,
            commands::governance_commands::recommendation_update,
            commands::governance_commands::recommendation_apply,
            commands::governance_commands::recommendations_apply_all,
            // Enforcement commands
            commands::enforcement_commands::enforcement_rules_list,
            commands::enforcement_commands::enforcement_rules_reload,
            commands::enforcement_commands::enforcement_scan_governance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
