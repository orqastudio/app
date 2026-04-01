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

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tracing::{error, info};

use orqa_engine::ports::resolve_daemon_port;
use orqa_engine_types::types::event::{EventLevel, EventSource, LogEvent};

use crate::config::DaemonConfig;
use crate::event_bus::{EventBus, EventBusStats};
use crate::event_store::{EventStore, EventQuery};

/// Shared state passed to all route handlers, containing startup metadata,
/// runtime configuration loaded from orqa.toml, the central event bus, and
/// the optional SQLite event store for historical event queries.
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
    /// Optional SQLite-backed event store for GET /events historical queries.
    /// Absent when the store failed to open at startup.
    pub event_store: Option<Arc<EventStore>>,
}

/// JSON response body for GET /health.
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    uptime_seconds: u64,
    pid: u32,
    /// Point-in-time snapshot of event bus statistics.
    event_bus: EventBusStats,
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
        let level = ingest.level.as_deref().map_or(EventLevel::Info, parse_level);
        let source = ingest.source.as_deref().map_or(EventSource::Frontend, parse_source);
        let category = ingest.category.unwrap_or_else(|| source.to_string());
        let timestamp = ingest.timestamp.unwrap_or(now_ms);

        let event = LogEvent {
            id,
            timestamp,
            level,
            source,
            category,
            message: ingest.message,
            metadata: serde_json::Value::Null,
            session_id: ingest.session_id,
        };

        state.event_bus.publish(event);
    }

    StatusCode::NO_CONTENT
}

/// Handle GET /events — return stored events matching the query parameters.
///
/// Delegates to `EventStore::query_sync` via `spawn_blocking`. Returns an
/// empty list when the store is absent (store failed to open at startup).
async fn events_query_handler(
    State(state): State<HealthState>,
    axum::extract::Query(query): axum::extract::Query<EventQuery>,
) -> Json<serde_json::Value> {
    let Some(store) = state.event_store else {
        return Json(serde_json::json!({ "events": [], "count": 0 }));
    };

    let filter = crate::event_store::EventFilter::from(query);
    let events = tokio::task::spawn_blocking(move || store.query_sync(&filter))
        .await
        .unwrap_or_default();

    let count = events.len();
    Json(serde_json::json!({ "events": events, "count": count }))
}

/// Handle GET /health by returning the current daemon status and bus stats.
///
/// Always returns `{"status": "ok", ...}` while the daemon is running. If the
/// daemon were unresponsive, the HTTP connection would simply time out on the
/// client side.
async fn health_handler(State(state): State<HealthState>) -> Json<HealthResponse> {
    let uptime = state.started_at.elapsed();
    Json(HealthResponse {
        status: "ok",
        uptime_seconds: uptime.as_secs(),
        pid: state.pid,
        event_bus: state.event_bus.stats(),
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
/// `event_store` is optional — when absent (store failed to open), GET /events
/// returns an empty list rather than an error.
pub async fn start(port: u16, config: DaemonConfig, event_bus: Arc<EventBus>, event_store: Option<Arc<EventStore>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = HealthState {
        started_at: Arc::new(Instant::now()),
        pid: std::process::id(),
        config,
        event_bus,
        event_store,
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/compact-context", post(crate::compact_context::compact_context_handler))
        .route("/context", post(crate::context::context_handler))
        .route("/events", get(events_query_handler).post(events_ingest_handler))
        .route("/events/stream", get(events_stream_handler))
        .route("/knowledge", post(crate::knowledge::knowledge_handler))
        .route("/parse", post(crate::parse::parse_handler))
        .route("/prompt", post(crate::prompt::prompt_handler))
        .route(
            "/session-start",
            post(crate::session_start::session_start_handler),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!(subsystem = "health", port, "starting health endpoint");
    info!(subsystem = "health", addr = %addr, "health endpoint listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.map_err(|e| {
        error!(subsystem = "health", error = %e, "health server error");
        e.into()
    })
}
