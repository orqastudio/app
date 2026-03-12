use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::domain::artifact_graph::ArtifactGraph;
use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::process_state::SessionProcessState;
use crate::domain::skill_injector::SkillInjector;
use crate::domain::workflow_tracker::WorkflowTracker;
use crate::search::SearchEngine;
use crate::sidecar::manager::SidecarManager;
use crate::startup::StartupTracker;
use crate::watcher::SharedWatcher;

// ---------------------------------------------------------------------------
// Sub-structs — each groups a logically related slice of application state.
// ---------------------------------------------------------------------------

/// Database connection state.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
pub struct DbState {
    pub conn: Mutex<Connection>,
}

/// Sidecar process state.
///
/// The `SidecarManager` uses interior mutability via its own `Mutex` fields.
/// `pending_approvals` holds one-shot channels keyed by `tool_call_id`.
/// When a write/execute tool requires user approval, the stream loop parks on a
/// sync channel receiver; the `stream_tool_approval_respond` command sends the
/// boolean decision onto the channel to unblock the stream loop.
pub struct SidecarState {
    pub manager: SidecarManager,
    /// Pending tool approval channels: `tool_call_id` -> sender for the approval decision.
    ///
    /// The stream loop inserts a sender before blocking on the corresponding receiver.
    /// `stream_tool_approval_respond` looks up the sender by `tool_call_id`, sends the
    /// boolean, and removes the entry.
    pub pending_approvals: Mutex<HashMap<String, SyncSender<bool>>>,
}

/// Code search engine state.
///
/// The `SearchEngine` is lazily initialized when a project is first indexed.
pub struct SearchState {
    pub engine: Mutex<Option<SearchEngine>>,
}

/// Long-running initialization task tracking.
///
/// The `StartupTracker` tracks long-running initialization tasks for the frontend.
pub struct StartupState {
    pub tracker: Arc<StartupTracker>,
}

/// Rule enforcement engine state.
///
/// `None` until the first project is opened. Reloaded via `enforcement_rules_reload`.
pub struct EnforcementState {
    pub engine: Mutex<Option<EnforcementEngine>>,
}

/// Session-level process compliance and workflow tracking.
///
/// Tracks whether docs were read and skills were loaded before code was written.
/// Accumulates reads, writes, searches, and commands over the session lifetime.
/// Both reset when `stream_send_message` is called for a different session.
pub struct SessionState {
    /// Session-level process compliance state.
    pub process_state: Mutex<SessionProcessState>,
    /// Session-level workflow tracker for process gate evaluation.
    pub workflow_tracker: Mutex<WorkflowTracker>,
}

/// Artifact graph and related filesystem state.
///
/// Includes the file watcher, cached bidirectional graph, and skill injector.
pub struct ArtifactState {
    /// Active `.orqa/` file-system watcher.
    ///
    /// Replaced via `artifact_watch_start` whenever a different project is opened.
    /// Dropping the inner value stops the underlying watcher.
    pub watcher: SharedWatcher,
    /// Cached bidirectional artifact graph.
    ///
    /// `None` until the first graph query or an explicit `refresh_artifact_graph` call.
    /// Invalidated (set to `None`) by the artifact watcher when `.orqa/` files change,
    /// so the next query triggers a fresh build from disk.
    pub graph: Mutex<Option<ArtifactGraph>>,
    /// Prompt-based skill injector using semantic similarity.
    ///
    /// `None` until the embedder is ready and a project with skills is opened.
    /// When available, the system prompt builder embeds the user's message and
    /// injects the most relevant skills automatically.
    pub skill_injector: Mutex<Option<SkillInjector>>,
}

// ---------------------------------------------------------------------------
// Top-level application state
// ---------------------------------------------------------------------------

/// Application state managed by Tauri.
///
/// Decomposed into logical sub-structs for clarity and reduced lock contention.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    pub db: DbState,
    pub sidecar: SidecarState,
    pub search: SearchState,
    pub startup: StartupState,
    pub enforcement: EnforcementState,
    pub session: SessionState,
    pub artifacts: ArtifactState,
}
