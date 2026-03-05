use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::process_state::SessionProcessState;
use crate::search::SearchEngine;
use crate::sidecar::manager::SidecarManager;
use crate::startup::StartupTracker;

/// Application state managed by Tauri.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
/// The `SidecarManager` uses interior mutability via its own `Mutex` fields.
/// The `SearchEngine` is lazily initialized when a project is first indexed.
/// The `StartupTracker` tracks long-running initialization tasks for the frontend.
/// The `pending_approvals` map holds one-shot channels keyed by `tool_call_id`.
/// When a write/execute tool requires user approval, the stream loop parks on a
/// sync channel receiver; the `stream_tool_approval_respond` command sends the
/// boolean decision onto the channel to unblock the stream loop.
/// The `enforcement` engine is loaded when a project is opened and enforces rules
/// on file and bash tool calls during Claude sessions.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    pub db: Mutex<Connection>,
    pub sidecar: SidecarManager,
    pub search: Mutex<Option<SearchEngine>>,
    pub startup: Arc<StartupTracker>,
    /// Pending tool approval channels: `tool_call_id` -> sender for the approval decision.
    ///
    /// The stream loop inserts a sender before blocking on the corresponding receiver.
    /// `stream_tool_approval_respond` looks up the sender by `tool_call_id`, sends the
    /// boolean, and removes the entry.
    pub pending_approvals: Mutex<HashMap<String, SyncSender<bool>>>,
    /// Rule enforcement engine, loaded when a project is opened.
    ///
    /// `None` until the first project is opened. Reloaded via `enforcement_rules_reload`.
    pub enforcement: Mutex<Option<EnforcementEngine>>,
    /// Session-level process compliance state.
    ///
    /// Tracks whether docs were read and skills were loaded before code was written.
    /// Resets when `stream_send_message` is called for a different session.
    pub process_state: Mutex<SessionProcessState>,
}
