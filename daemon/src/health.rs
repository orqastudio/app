// HTTP health endpoint for the OrqaStudio daemon.
//
// Exposes GET /health (daemon liveness), POST /parse (artifact impact),
// POST /prompt, POST /knowledge (knowledge injection), POST /context (active
// rules and workflows for CLAUDE.md generation), POST /session-start
// (structured startup checks), POST /events (external event ingest from
// frontend SDK and dev-controller), and GET /events/stream (SSE stream of all
// live events from the central event bus). The endpoint runs on the tokio
// runtime and binds to 127.0.0.1:<ORQA_PORT_BASE> (default port 10100). This
// allows other tools (app, CLI, connector) to check whether the daemon is alive
// without reading the PID file directly.
//
// GET /health includes a `processes` array populated from a shared
// `Arc<Mutex<Vec<ProcessSnapshot>>>` that the daemon event loop updates every
// polling cycle. This allows OrqaDev to auto-discover managed subprocesses
// without any hardcoded process list — new processes appear automatically.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware as axum_mw;
use axum::response::sse::{Event, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::middleware::correlation_id_middleware;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tracing::{error, info};

use orqa_engine::ports::resolve_daemon_port;
use orqa_engine_types::fingerprint::{compute_fingerprint, extract_template};
use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent, StackFrame};
use orqa_storage::repo::issue_groups::IssueGroup;
use orqa_storage::traits::EventRepository as _;
use orqa_storage::Storage;

use crate::config::DaemonConfig;
use crate::event_bus::{EventBus, EventBusStats};
use crate::graph_state::GraphState;
use crate::routes::streaming::SessionStreamRegistry;
use crate::subprocess::ProcessSnapshot;

/// Query parameters accepted by `GET /events`.
///
/// All fields are optional. Deserialized from the URL query string by axum.
#[derive(Debug, Deserialize)]
pub struct EventQuery {
    /// Filter by source subsystem name (matches `EventSource` Display string).
    pub source: Option<String>,
    /// Filter by level name (matches `EventLevel` Debug string, e.g. "Error").
    pub level: Option<String>,
    /// Unix timestamp in milliseconds — return only events at or after this time.
    pub after: Option<i64>,
    /// Maximum number of events to return (capped at 5000).
    pub limit: Option<i64>,
}

/// Shared state passed to all route handlers, containing startup metadata,
/// runtime configuration loaded from orqa.toml, the central event bus, and
/// the unified SQLite storage for all persistence operations.
#[derive(Clone)]
pub struct HealthState {
    /// Instant the daemon started — used to compute uptime.
    started_at: Arc<Instant>,
    /// PID of this daemon process.
    pid: u32,
    /// Runtime configuration loaded from orqa.toml at startup.
    pub config: DaemonConfig,
    /// Shared event bus — exposes stats to the health endpoint.
    pub event_bus: Arc<EventBus>,
    /// Unified SQLite storage for sessions, projects, settings, and events.
    /// Absent when the database cannot be opened (typically a permissions issue).
    pub storage: Option<Arc<Storage>>,
    /// Snapshots of all managed subprocesses, updated by the event loop every
    /// 250 ms. The health handler reads this to populate the `processes` array
    /// without requiring access to the subprocess managers directly.
    pub process_snapshots: Arc<Mutex<Vec<ProcessSnapshot>>>,
    /// Shared cached artifact graph and validation context. All artifact,
    /// graph, and validation route handlers read from this.
    pub graph_state: GraphState,
    /// Per-session stream state for the daemon-side stream loop.
    ///
    /// Each active session has a broadcast channel for SSE delivery, a
    /// cancellation flag, and a pending-approval map for tool approval.
    pub stream_registry: SessionStreamRegistry,
    /// Broadcast sender for `IssueGroup` updates published by the daemon-side
    /// `issue_group_consumer`.  SSE handlers call `subscribe()` on this sender
    /// to stream live updates to connected clients.  `None` when storage is
    /// unavailable — in that case the consumer was never spawned and no
    /// updates are produced.
    pub issue_group_updates: Option<broadcast::Sender<IssueGroup>>,
}

impl HealthState {
    /// Construct a minimal `HealthState` for use in tests.
    ///
    /// Provides sensible defaults for fields that route handlers under test do
    /// not access. This avoids exposing private fields (`started_at`, `pid`)
    /// to route test modules and integration test helpers.
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn for_test(graph_state: GraphState, storage: Option<Arc<Storage>>) -> Self {
        Self {
            started_at: Arc::new(Instant::now()),
            pid: std::process::id(),
            config: DaemonConfig::default(),
            event_bus: EventBus::new(),
            storage,
            process_snapshots: Arc::new(Mutex::new(Vec::<ProcessSnapshot>::new())),
            graph_state,
            stream_registry: SessionStreamRegistry::new(),
            issue_group_updates: None,
        }
    }
}

/// JSON response body for GET /health.
///
/// The `processes` array is populated from the shared `process_snapshots`
/// registry maintained by the daemon event loop. OrqaDev uses this array to
/// auto-discover managed subprocesses without any hardcoded process names.
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    uptime_seconds: u64,
    pid: u32,
    /// Crate version of the daemon binary.
    version: &'static str,
    /// Absolute path to the project root the daemon is serving.
    project_root: String,
    /// Total number of artifacts in the cached graph.
    artifact_count: usize,
    /// Number of rule artifacts in the cached graph.
    rule_count: usize,
    /// Point-in-time snapshot of event bus statistics.
    event_bus: EventBusStats,
    /// Per-process detail for all managed subprocesses. Auto-populated from
    /// the subprocess registry — new processes appear automatically.
    processes: Vec<ProcessSnapshot>,
}

/// JSON body for a single event submitted via POST /events.
///
/// External sources (frontend SDK, dev-controller) do not assign bus IDs —
/// the daemon assigns them via `EventBus::next_ingest_id()`. The `source`
/// field is a string tag that maps to `EventSource`; unknown values default to
/// `Frontend` so stale callers don't break the bus.
#[derive(Deserialize)]
pub struct IngestEvent {
    /// Severity level string: "debug", "info", "warn", "error", "perf".
    pub level: Option<String>,
    /// Source subsystem string: "frontend", "dev-controller", "app", etc.
    pub source: Option<String>,
    /// Log category for filtering (e.g. "navigation", "build", "ctrl").
    pub category: Option<String>,
    /// Human-readable description of the event.
    pub message: String,
    /// Unix timestamp in milliseconds. If absent, current time is used.
    pub timestamp: Option<i64>,
    /// Optional agent session identifier.
    pub session_id: Option<String>,
    /// Resolved source stack frames from the frontend, if available.
    pub stack_frames: Option<Vec<StackFrame>>,
}

/// Map a string source tag from an external caller to the canonical `EventSource`.
///
/// Unknown tags default to `Frontend` so stale or custom callers don't break
/// the bus — they're all frontend-class sources anyway.
fn parse_source(s: &str) -> EventSource {
    match s.to_lowercase().as_str() {
        "daemon" => EventSource::Daemon,
        "app" => EventSource::App,
        "dev-controller" | "devcontroller" | "ctrl" => EventSource::DevController,
        "mcp" => EventSource::MCP,
        "lsp" => EventSource::LSP,
        "search" => EventSource::Search,
        "worker" => EventSource::Worker,
        // "frontend" and all unknown tags map to Frontend.
        _ => EventSource::Frontend,
    }
}

/// Map a string level tag to the canonical `EventLevel`. Defaults to `Info`.
fn parse_level(s: &str) -> EventLevel {
    match s.to_lowercase().as_str() {
        "debug" => EventLevel::Debug,
        "warn" | "warning" => EventLevel::Warn,
        "error" | "err" => EventLevel::Error,
        "perf" => EventLevel::Perf,
        _ => EventLevel::Info,
    }
}

/// Handle POST /events — accept a batch of events from external sources and
/// publish each to the central event bus.
///
/// Accepts `Content-Type: application/json` with a JSON array of `IngestEvent`
/// objects. Each event is stamped with a daemon-assigned ID and current
/// timestamp (if not provided) before publication. Returns 204 on success.
/// Returns 400 if the body is not a valid JSON array.
async fn events_ingest_handler(
    State(state): State<HealthState>,
    Json(events): Json<Vec<IngestEvent>>,
) -> StatusCode {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);

    for ingest in events {
        let id = state.event_bus.next_ingest_id();
        let level = ingest
            .level
            .as_deref()
            .map_or(EventLevel::Info, parse_level);
        let source = ingest
            .source
            .as_deref()
            .map_or(EventSource::Frontend, parse_source);
        let category = ingest.category.unwrap_or_else(|| source.to_string());
        let timestamp = ingest.timestamp.unwrap_or(now_ms);

        let template = extract_template(&ingest.message);
        let fp = compute_fingerprint(
            &source.to_string(),
            &level.to_string(),
            &template,
            "", // stack_top from frontend — empty for now, wired in Task 3.4
        );

        let event = LogEvent {
            id,
            timestamp,
            level,
            source,
            tier: EventTier::default(),
            category,
            message: ingest.message,
            metadata: serde_json::Value::Null,
            session_id: ingest.session_id,
            fingerprint: Some(fp),
            message_template: Some(template),
            correlation_id: None,
            stack_frames: ingest.stack_frames,
        };

        state.event_bus.publish(event);
    }

    StatusCode::NO_CONTENT
}

/// Handle GET /events — return stored events matching the query parameters.
///
/// Delegates to the events repo via `spawn_blocking`. Returns an empty list
/// when storage is absent (failed to open at startup).
async fn events_query_handler(
    State(state): State<HealthState>,
    axum::extract::Query(query): axum::extract::Query<EventQuery>,
) -> Json<serde_json::Value> {
    let Some(storage) = state.storage else {
        return Json(serde_json::json!({ "events": [], "count": 0 }));
    };

    let filter = orqa_storage::repo::events::EventFilter {
        source: query.source,
        level: query.level,
        after: query.after,
        limit: query.limit,
    };

    let events = storage.events().query(&filter).await.unwrap_or_default();

    let count = events.len();
    Json(serde_json::json!({ "events": events, "count": count }))
}

/// Handle GET /health by returning the current daemon status, bus stats, and
/// per-process snapshots from the subprocess registry.
///
/// Always returns `{"status": "ok", ...}` while the daemon is running. The
/// `processes` array is cloned from the shared snapshot registry — no blocking
/// required because the event loop holds the lock only briefly.
async fn health_handler(State(state): State<HealthState>) -> Json<HealthResponse> {
    let uptime = state.started_at.elapsed();
    let processes = state
        .process_snapshots
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default();

    let project_root = state
        .graph_state
        .0
        .read()
        .map(|g| g.project_root.to_string_lossy().to_string())
        .unwrap_or_default();

    Json(HealthResponse {
        status: "ok",
        uptime_seconds: uptime.as_secs(),
        pid: state.pid,
        version: env!("CARGO_PKG_VERSION"),
        project_root,
        artifact_count: state.graph_state.artifact_count(),
        rule_count: state.graph_state.rule_count(),
        event_bus: state.event_bus.stats(),
        processes,
    })
}

/// Handle GET /events/stream — subscribe to the event bus and stream all
/// events to the caller as Server-Sent Events.
///
/// Each event is serialized to JSON and delivered as a single SSE `data`
/// field. The stream runs until the client disconnects or the event bus shuts
/// down (sender dropped). Errors from the broadcast channel (e.g. lagged
/// receiver) are logged and the stream is terminated.
async fn events_stream_handler(
    State(state): State<HealthState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, axum::Error>>> {
    let receiver = state.event_bus.subscribe();
    let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
        Ok(event) => match serde_json::to_string(&event) {
            Ok(json) => Some(Ok(Event::default().data(json))),
            Err(e) => {
                error!(subsystem = "sse", error = %e, "failed to serialize event for SSE");
                None
            }
        },
        Err(e) => {
            error!(subsystem = "sse", error = %e, "SSE broadcast receiver error — stream ending");
            None
        }
    });
    Sse::new(stream)
}

/// Handle GET /issue-groups/stream — subscribe to the daemon-side issue group
/// consumer and stream updated `IssueGroup` values to the caller as SSE.
///
/// Each update is serialized to JSON and delivered as a single SSE `data`
/// field.  Returns an empty stream when storage is unavailable (no consumer
/// is running).  The stream runs until the client disconnects.
async fn issue_groups_stream_handler(
    State(state): State<HealthState>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, axum::Error>>> {
    // When storage is unavailable we create an empty broadcast so the stream
    // resolves immediately with no events and the client sees a clean close.
    let receiver = match state.issue_group_updates.as_ref() {
        Some(tx) => tx.subscribe(),
        None => {
            let (tx, rx) = broadcast::channel::<IssueGroup>(1);
            drop(tx);
            rx
        }
    };
    let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
        Ok(group) => match serde_json::to_string(&group) {
            Ok(json) => Some(Ok(Event::default().data(json))),
            Err(e) => {
                error!(subsystem = "sse", error = %e, "failed to serialize issue group for SSE");
                None
            }
        },
        Err(e) => {
            error!(subsystem = "sse", error = %e, "issue-group SSE broadcast error — stream ending");
            None
        }
    });
    Sse::new(stream)
}

/// Handle POST /reload — rebuild the cached artifact graph and validation context from disk.
///
/// Returns the new artifact and rule counts. The watcher already calls
/// `GraphState::reload()` on file changes, but this endpoint allows manual reload
/// without requiring a file change event.
async fn reload_handler(State(state): State<HealthState>) -> Json<serde_json::Value> {
    let project_root = state
        .graph_state
        .0
        .read()
        .map(|g| g.project_root.clone())
        .unwrap_or_default();

    let artifact_count = state.graph_state.reload(&project_root);
    let rule_count = state.graph_state.rule_count();

    Json(serde_json::json!({
        "status": "reloaded",
        "artifacts": artifact_count,
        "rules": rule_count,
    }))
}

/// Handle POST /shutdown — initiate graceful daemon shutdown.
///
/// Schedules process exit after a brief delay so the 204 response is
/// flushed to the caller before the process terminates. The same shutdown
/// path used by the ctrlc handler fires after the response completes.
async fn shutdown_handler() -> StatusCode {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        std::process::exit(0);
    });
    StatusCode::NO_CONTENT
}

/// Resolve the port to bind from the environment or use the default.
///
/// Delegates to `orqa_engine::ports::resolve_daemon_port()`. Falls back to
/// `DEFAULT_PORT_BASE` when ORQA_PORT_BASE is absent or unparseable.
pub fn resolve_port() -> u16 {
    resolve_daemon_port()
}

/// Start the health HTTP server on the tokio runtime.
///
/// Binds to `127.0.0.1:<port>` and serves all daemon HTTP endpoints until the
/// tokio runtime shuts down. Logs the bound address on startup. Returns an
/// error if the port is already in use.
///
/// `storage` is optional — when absent (failed to open), session/project/settings
/// endpoints return 503 and GET /events returns an empty list.
///
/// `process_snapshots` is the shared subprocess registry written by the event
/// loop. The health handler reads it on every GET /health request so OrqaDev
/// always receives current per-process detail.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub async fn start(
    port: u16,
    config: DaemonConfig,
    event_bus: Arc<EventBus>,
    storage: Option<Arc<Storage>>,
    process_snapshots: Arc<Mutex<Vec<ProcessSnapshot>>>,
    graph_state: GraphState,
    issue_group_updates: Option<broadcast::Sender<IssueGroup>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = HealthState {
        started_at: Arc::new(Instant::now()),
        pid: std::process::id(),
        config,
        event_bus,
        storage,
        process_snapshots,
        graph_state,
        stream_registry: SessionStreamRegistry::new(),
        issue_group_updates,
    };

    // Artifact routes — operate on the cached graph.
    let artifact_router = Router::new()
        .route("/", get(crate::routes::artifacts::list_artifacts))
        .route("/tree", get(crate::routes::artifacts::get_artifact_tree))
        .route("/{id}", get(crate::routes::artifacts::get_artifact))
        .route(
            "/{id}",
            axum::routing::put(crate::routes::artifacts::update_artifact),
        )
        .route(
            "/{id}/content",
            get(crate::routes::artifacts::get_artifact_content),
        )
        .route(
            "/{id}/traceability",
            get(crate::routes::artifacts::get_artifact_traceability),
        )
        .route(
            "/{id}/impact",
            get(crate::routes::artifacts::get_artifact_impact),
        )
        .with_state(state.graph_state.clone());

    // Graph analytics routes.
    let graph_router = Router::new()
        .route("/stats", get(crate::routes::graph::get_graph_stats))
        .route("/health", get(crate::routes::graph::get_graph_health))
        .route(
            "/health/snapshots",
            get(crate::routes::graph::list_health_snapshots)
                .post(crate::routes::graph::create_health_snapshot),
        )
        .with_state(state.graph_state.clone());

    // Validation routes.
    let validation_router = Router::new()
        .route("/scan", post(crate::routes::validation::validation_scan))
        .route("/fix", post(crate::routes::validation::validation_fix))
        .route("/hook", post(crate::routes::validation::validation_hook))
        .with_state(state.graph_state.clone());

    // Enforcement routes — governance rule management.
    let enforcement_router = Router::new()
        .route(
            "/rules",
            get(crate::routes::enforcement::list_enforcement_rules),
        )
        .route(
            "/rules/reload",
            post(crate::routes::enforcement::reload_enforcement_rules),
        )
        .route(
            "/violations",
            get(crate::routes::enforcement::list_enforcement_violations),
        )
        .route("/scan", post(crate::routes::enforcement::enforcement_scan))
        .with_state(state.graph_state.clone());

    // Search routes — codebase indexing and semantic/regex search.
    let search_router = Router::new()
        .route("/index", post(crate::routes::search::search_index))
        .route("/embed", post(crate::routes::search::search_embed))
        .route("/regex", post(crate::routes::search::search_regex))
        .route("/semantic", post(crate::routes::search::search_semantic))
        .route("/status", get(crate::routes::search::search_status))
        .with_state(state.graph_state.clone());

    // Workflow routes — status transition evaluation.
    let workflow_router = Router::new()
        .route(
            "/transitions",
            get(crate::routes::workflow::list_transitions),
        )
        .route(
            "/transitions/apply",
            post(crate::routes::workflow::apply_transition),
        )
        .with_state(state.graph_state.clone());

    // Prompt routes — system prompt generation and knowledge injection.
    let prompt_router = Router::new()
        .route("/generate", post(crate::routes::prompt::generate_prompt))
        .route("/knowledge", post(crate::routes::prompt::prompt_knowledge))
        .route("/context", post(crate::routes::prompt::prompt_context))
        .route(
            "/compact-context",
            post(crate::routes::prompt::prompt_compact_context),
        )
        .with_state(state.clone());

    // Plugin routes — plugin lifecycle management.
    let plugin_router = Router::new()
        .route("/", get(crate::routes::plugins::list_plugins))
        .route(
            "/registry",
            get(crate::routes::plugins::list_plugin_registry),
        )
        .route(
            "/updates",
            get(crate::routes::plugins::check_plugin_updates),
        )
        .route(
            "/install/local",
            post(crate::routes::plugins::install_plugin_local),
        )
        .route(
            "/install/github",
            post(crate::routes::plugins::install_plugin_github),
        )
        .route("/{name}", get(crate::routes::plugins::get_plugin))
        .route(
            "/{name}",
            axum::routing::delete(crate::routes::plugins::uninstall_plugin),
        )
        .route("/{name}/path", get(crate::routes::plugins::get_plugin_path))
        .with_state(state.graph_state.clone());

    // Agent routes — agent preamble and behavioral messages.
    let agent_router = Router::new()
        .route(
            "/behavioral-messages",
            get(crate::routes::agents::get_behavioral_messages),
        )
        .route("/{role}", get(crate::routes::agents::get_agent))
        .with_state(state.graph_state.clone());

    // Content routes — knowledge artifact loading.
    let content_router = Router::new()
        .route(
            "/knowledge/{key}",
            get(crate::routes::content::get_knowledge),
        )
        .with_state(state.graph_state.clone());

    // Lesson routes — lesson CRUD and recurrence.
    let lesson_router = Router::new()
        .route(
            "/",
            get(crate::routes::lessons::list_lessons).post(crate::routes::lessons::create_lesson),
        )
        .route(
            "/{id}/recurrence",
            axum::routing::put(crate::routes::lessons::increment_lesson_recurrence),
        )
        .with_state(state.graph_state.clone());

    // Message routes — create, tool-message create, and FTS5 search.
    let message_router = Router::new()
        .route("/", post(crate::routes::messages::create_message))
        .route("/tool", post(crate::routes::messages::create_tool_message))
        .route("/search", post(crate::routes::messages::search_messages))
        .route(
            "/{id}/content",
            axum::routing::put(crate::routes::messages::update_message_content),
        )
        .route(
            "/{id}/stream-status",
            axum::routing::put(crate::routes::messages::update_message_stream_status),
        )
        .with_state(state.clone());

    // Theme routes — project design tokens and user overrides.
    let theme_router = Router::new()
        .route("/", get(crate::routes::themes::get_themes))
        .route(
            "/overrides",
            get(crate::routes::themes::get_overrides)
                .delete(crate::routes::themes::clear_overrides),
        )
        .route(
            "/overrides/{token}",
            axum::routing::put(crate::routes::themes::set_override),
        )
        .with_state(state.clone());

    // Health snapshot routes — persist and retrieve graph health metrics.
    let health_snapshot_router = Router::new()
        .route(
            "/",
            get(crate::routes::health_snapshots::list_health_snapshots)
                .post(crate::routes::health_snapshots::create_health_snapshot),
        )
        .route(
            "/{id}",
            get(crate::routes::health_snapshots::get_health_snapshot),
        )
        .with_state(state.clone());

    // Violation routes — record and list enforcement violations in SQLite.
    let violation_router = Router::new()
        .route(
            "/",
            get(crate::routes::violations::list_violations)
                .post(crate::routes::violations::record_violation),
        )
        .with_state(state.clone());

    // Devtools session routes — session lifecycle and event query for OrqaDev.
    let devtools_session_router = Router::new()
        .route(
            "/",
            post(crate::routes::devtools_sessions::create_devtools_session)
                .get(crate::routes::devtools_sessions::list_devtools_sessions),
        )
        .route(
            "/mark-orphaned",
            post(crate::routes::devtools_sessions::mark_orphaned_sessions),
        )
        .route(
            "/purge",
            post(crate::routes::devtools_sessions::purge_devtools_sessions),
        )
        .route(
            "/{id}",
            get(crate::routes::devtools_sessions::get_devtools_session)
                .delete(crate::routes::devtools_sessions::delete_devtools_session),
        )
        .route(
            "/{id}/label",
            axum::routing::put(crate::routes::devtools_sessions::rename_devtools_session),
        )
        .route(
            "/{id}/end",
            post(crate::routes::devtools_sessions::end_devtools_session),
        )
        .route(
            "/{id}/events",
            post(crate::routes::devtools_sessions::insert_devtools_events),
        )
        .route(
            "/{id}/events/query",
            post(crate::routes::devtools_sessions::query_devtools_events),
        )
        .with_state(state.clone());

    // Issue group routes — deduplicated error clusters.
    //
    // Upsert is NOT exposed over HTTP: the daemon computes issue groups
    // internally from its own event bus via `issue_group_consumer`.  The SSE
    // stream at `/issue-groups/stream` broadcasts updates to subscribers so
    // devtools can render live changes without polling.
    let issue_group_router = Router::new()
        .route("/", get(crate::routes::issue_groups::list_issue_groups))
        .route("/stream", get(issue_groups_stream_handler))
        .route(
            "/{fingerprint}",
            get(crate::routes::issue_groups::get_issue_group),
        )
        .with_state(state.clone());

    // Session routes — chat session and message management, plus daemon stream loop.
    let session_router = Router::new()
        .route(
            "/",
            post(crate::routes::sessions::create_session)
                .get(crate::routes::sessions::list_sessions),
        )
        .route(
            "/{id}",
            get(crate::routes::sessions::get_session)
                .put(crate::routes::sessions::update_session)
                .delete(crate::routes::sessions::delete_session),
        )
        .route("/{id}/end", post(crate::routes::sessions::end_session))
        .route(
            "/{id}/messages",
            get(crate::routes::sessions::list_session_messages)
                .post(crate::routes::streaming::send_message),
        )
        .route(
            "/{id}/stream",
            get(crate::routes::streaming::session_stream),
        )
        .route("/{id}/stop", post(crate::routes::streaming::stop_stream))
        .route(
            "/{id}/tool-approval",
            post(crate::routes::streaming::tool_approval),
        )
        .with_state(state.clone());

    // Project routes — project management, settings, scan, icon.
    let project_router = Router::new()
        .route("/", get(crate::routes::projects::list_projects))
        .route("/active", get(crate::routes::projects::get_active_project))
        .route("/open", post(crate::routes::projects::open_project))
        .route(
            "/settings",
            get(crate::routes::projects::get_project_settings)
                .put(crate::routes::projects::update_project_settings),
        )
        .route("/scan", post(crate::routes::projects::scan_project_handler))
        .route(
            "/icon",
            post(crate::routes::projects::upload_project_icon)
                .get(crate::routes::projects::get_project_icon),
        )
        .with_state(state.clone());

    // Settings routes — app key/value settings store.
    let settings_router = Router::new()
        .route("/", get(crate::routes::settings::get_settings))
        .route(
            "/{key}",
            axum::routing::put(crate::routes::settings::set_setting),
        )
        .with_state(state.clone());

    // Sidecar routes — subprocess status and restart.
    let sidecar_router = Router::new()
        .route("/status", get(crate::routes::sidecar::get_sidecar_status))
        .route("/restart", post(crate::routes::sidecar::restart_sidecar))
        .with_state(state.clone());

    // CLI tool routes — plugin-registered tool execution.
    let cli_tools_router = Router::new()
        .route("/", get(crate::routes::cli_tools::list_cli_tools))
        .route(
            "/status",
            get(crate::routes::cli_tools::get_cli_tool_status),
        )
        .route(
            "/{plugin}/{key}/run",
            post(crate::routes::cli_tools::run_cli_tool),
        )
        .with_state(state.graph_state.clone());

    // Hook routes — plugin hook registry and dispatcher generation.
    let hooks_router = Router::new()
        .route("/", get(crate::routes::hooks::list_hooks))
        .route(
            "/generate",
            post(crate::routes::hooks::generate_hook_dispatchers),
        )
        .with_state(state.graph_state.clone());

    // Setup routes — wizard status and prerequisite checks.
    let setup_router = Router::new()
        .route("/status", get(crate::routes::setup::get_setup_status))
        .route("/claude-cli", get(crate::routes::setup::check_claude_cli))
        .route("/claude-auth", get(crate::routes::setup::check_claude_auth))
        .route(
            "/claude-reauth",
            post(crate::routes::setup::reauthenticate_claude),
        )
        .route(
            "/embedding-model",
            get(crate::routes::setup::check_embedding_model),
        )
        .route("/complete", post(crate::routes::setup::complete_setup))
        .with_state(state.clone());

    // DevTools routes — OrqaDev window launch and status.
    let devtools_router = Router::new()
        .route("/launch", post(crate::routes::devtools::launch_devtools))
        .route("/status", get(crate::routes::devtools::get_devtools_status))
        .with_state(state.clone());

    // Git routes — stash and uncommitted status.
    let git_router = Router::new()
        .route("/stashes", get(crate::routes::git::get_git_stashes))
        .route("/status", get(crate::routes::git::get_git_status))
        .with_state(state.graph_state.clone());

    // Startup routes — daemon initialization task status.
    let startup_router = Router::new()
        .route("/status", get(crate::routes::startup::get_startup_status))
        .with_state(state.clone());

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/reload", post(reload_handler))
        .route("/shutdown", post(shutdown_handler))
        .route(
            "/events",
            get(events_query_handler).post(events_ingest_handler),
        )
        .route("/events/stream", get(events_stream_handler))
        .route(
            "/session-start",
            post(crate::session_start::session_start_handler),
        )
        .nest("/artifacts", artifact_router)
        .nest("/graph", graph_router)
        .nest("/validation", validation_router)
        .nest("/enforcement", enforcement_router)
        .nest("/search", search_router)
        .nest("/workflow", workflow_router)
        .nest("/prompt", prompt_router)
        .nest("/plugins", plugin_router)
        .nest("/agents", agent_router)
        .nest("/content", content_router)
        .nest("/lessons", lesson_router)
        .nest("/messages", message_router)
        .nest("/sessions", session_router)
        .nest("/projects", project_router)
        .nest("/themes", theme_router)
        .nest("/health-snapshots", health_snapshot_router)
        .nest("/violations", violation_router)
        .nest("/devtools-sessions", devtools_session_router)
        .nest("/issue-groups", issue_group_router)
        .nest("/settings", settings_router)
        .nest("/sidecar", sidecar_router)
        .nest("/cli-tools", cli_tools_router)
        .nest("/hooks", hooks_router)
        .nest("/setup", setup_router)
        .nest("/devtools", devtools_router)
        .nest("/git", git_router)
        .nest("/startup", startup_router)
        .layer(axum_mw::from_fn(correlation_id_middleware))
        .with_state(state);

    // In headless/container mode, bind to 0.0.0.0 so Docker port publishing
    // works. In native mode, bind to 127.0.0.1 for security (don't expose to
    // the network).
    let headless = std::env::var("ORQA_HEADLESS").is_ok_and(|v| v == "1" || v == "true");
    let bind_addr = if headless {
        Ipv4Addr::UNSPECIFIED // 0.0.0.0
    } else {
        Ipv4Addr::LOCALHOST // 127.0.0.1
    };
    let addr = SocketAddr::from((bind_addr, port));
    info!(subsystem = "health", %addr, "starting health endpoint");

    // Use SO_REUSEADDR so the daemon can rebind immediately after a restart.
    // Without this, a killed daemon leaves the port in TIME_WAIT and the new
    // instance fails with EADDRINUSE (OS error 10048 on Windows).
    let socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&SocketAddrV4::new(bind_addr, port).into())?;
    socket.listen(128)?;
    let listener = tokio::net::TcpListener::from_std(socket.into())?;

    info!(subsystem = "health", addr = %addr, "health endpoint listening");

    // Signal readiness by writing .state/daemon.ready. The CLI watches for
    // this file instead of polling /health, giving instant startup notification.
    if let Ok(cwd) = std::env::current_dir() {
        let ready_path = cwd.join(".state/daemon.ready");
        let _ = std::fs::write(&ready_path, format!("{}", std::process::id()));
        info!(subsystem = "health", path = %ready_path.display(), "ready signal written");
    }

    axum::serve(listener, app).await.map_err(|e| {
        error!(subsystem = "health", error = %e, "health server error");
        e.into()
    })
}
