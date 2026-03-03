pub mod commands;
pub mod db;
pub mod domain;
pub mod error;
pub mod repo;
pub mod sidecar;
pub mod state;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| format!("failed to create app data dir: {e}"))?;

            let db_path = app_dir.join("forge.db");
            let db_path_str = db_path
                .to_str()
                .ok_or_else(|| "app data path is not valid UTF-8".to_string())?;

            let conn = db::init_db(db_path_str)
                .map_err(|e| format!("failed to initialize database: {e}"))?;

            app.manage(state::AppState {
                db: std::sync::Mutex::new(conn),
                sidecar: sidecar::manager::SidecarManager::new(),
            });

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
