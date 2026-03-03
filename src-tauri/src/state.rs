use std::sync::Mutex;

use rusqlite::Connection;

use crate::sidecar::manager::SidecarManager;

/// Application state managed by Tauri.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
/// The `SidecarManager` uses interior mutability via its own `Mutex` fields.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    pub db: Mutex<Connection>,
    pub sidecar: SidecarManager,
}
