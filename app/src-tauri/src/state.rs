use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::cli_tools::runner::CliToolRunner;
use crate::domain::artifact_graph::ArtifactGraph;
use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::knowledge_injector::KnowledgeInjector;
use crate::domain::process_state::SessionProcessState;
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
    /// The SQLite connection, guarded by a mutex for single-writer access.
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
    /// Manages the Claude sidecar child process lifecycle.
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
    /// ONNX-based semantic search engine; `None` until a project is indexed.
    pub engine: Mutex<Option<SearchEngine>>,
}

/// Long-running initialization task tracking.
///
/// The `StartupTracker` tracks long-running initialization tasks for the frontend.
pub struct StartupState {
    /// Shared reference to the startup tracker, shared with background init tasks.
    pub tracker: Arc<StartupTracker>,
}

/// Rule enforcement engine state.
///
/// `None` until the first project is opened. Reloaded via `enforcement_rules_reload`.
pub struct EnforcementState {
    /// Compiled enforcement engine; `None` until a project is opened.
    pub engine: Mutex<Option<EnforcementEngine>>,
}

/// Session-level process compliance and workflow tracking.
///
/// Tracks whether docs were read and knowledge was loaded before code was written.
/// Accumulates reads, writes, searches, and commands over the session lifetime.
/// Both reset when `stream_send_message` is called for a different session.
pub struct SessionState {
    /// Session-level process compliance state.
    pub process_state: Mutex<SessionProcessState>,
    /// Session-level workflow tracker for process gate evaluation.
    pub workflow_tracker: Mutex<WorkflowTracker>,
}

/// Plugin CLI tool runner state.
///
/// Manages one-shot CLI tool execution and caches last-run results.
pub struct CliToolState {
    /// Executes plugin-contributed CLI tools and tracks their output.
    pub runner: CliToolRunner,
}

/// Artifact graph and related filesystem state.
///
/// Includes the file watcher, cached bidirectional graph, and knowledge injector.
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
    /// Prompt-based knowledge injector using semantic similarity.
    ///
    /// `None` until the embedder is ready and a project with knowledge artifacts is opened.
    /// When available, the system prompt builder embeds the user's message and
    /// injects the most relevant knowledge automatically.
    pub knowledge_injector: Mutex<Option<KnowledgeInjector>>,
}

// ---------------------------------------------------------------------------
// Top-level application state
// ---------------------------------------------------------------------------

/// Application state managed by Tauri.
///
/// Decomposed into logical sub-structs for clarity and reduced lock contention.
/// Tauri manages this as shared state across all commands.
pub struct AppState {
    /// SQLite database connection.
    pub db: DbState,
    /// Claude sidecar process and tool approval state.
    pub sidecar: SidecarState,
    /// Semantic search engine for code and artifact indexing.
    pub search: SearchState,
    /// Startup task progress tracker.
    pub startup: StartupState,
    /// Enforcement rule engine for governance validation.
    pub enforcement: EnforcementState,
    /// Session-level process compliance and workflow state.
    pub session: SessionState,
    /// Artifact graph, file watcher, and knowledge injector.
    pub artifacts: ArtifactState,
    /// Plugin CLI tool runner.
    pub cli_tools: CliToolState,
}
