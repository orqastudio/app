//! OrqaDev Tauri application setup and command registration.
//!
//! Bootstraps the Tauri builder with logging, shared state (event ring buffer),
//! and IPC commands for the developer tools companion app.

/// Dev environment controller — spawns `orqa dev start-processes` and pipes output.
pub mod dev_controller;

/// SSE event consumer — connects to daemon, buffers events, exposes IPC commands.
pub mod events;

/// IPC command for querying process status from the daemon health endpoint.
pub mod process_status;

use tauri::Manager as _;
use tracing_subscriber::EnvFilter;

/// Initialize structured logging for the devtools process.
fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

/// Run the Tauri setup callback: initialise logging, shared state, and the
/// background SSE consumer that connects to the daemon event bus.
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    tracing::info!("OrqaDev starting");

    let consumer_state = events::EventConsumerState::new();
    app.manage(std::sync::Arc::clone(&consumer_state));
    events::spawn_consumer(app.handle().clone(), consumer_state);

    let dev_ctrl_state = dev_controller::DevControllerState::new();
    app.manage(dev_ctrl_state);

    Ok(())
}

/// Build and run the Tauri application event loop.
pub fn run() {
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running OrqaDev");
}
