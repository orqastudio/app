//! OrqaDev Tauri application setup and command registration.
//!
//! Bootstraps the Tauri builder with logging, commands, and application state
//! for the developer tools companion app.

use tracing_subscriber::EnvFilter;

/// Initialize structured logging for the devtools process.
fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

/// Run the Tauri setup callback: initialise logging and application state.
fn setup_app(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    tracing::info!("OrqaDev starting");
    Ok(())
}

/// Build and run the Tauri application event loop.
pub fn run() {
    tauri::Builder::default()
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running OrqaDev");
}
