// HTTP health endpoint for the OrqaStudio daemon.
//
// Exposes GET /health (daemon liveness), POST /parse (artifact impact),
// POST /prompt, POST /knowledge (knowledge injection), POST /context (active
// rules and workflows for CLAUDE.md generation), and POST /session-start
// (structured startup checks). The endpoint runs on the tokio runtime and binds
// to 127.0.0.1:<ORQA_PORT_BASE> (default port 10100). This allows other tools
// (app, CLI, connector) to check whether the daemon is alive without reading
// the PID file directly.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use tracing::{error, info};

use orqa_engine::ports::resolve_daemon_port;

use crate::config::DaemonConfig;

/// Shared state passed to all route handlers, containing startup metadata and
/// runtime configuration loaded from orqa.toml.
#[derive(Clone)]
pub struct HealthState {
    /// Instant the daemon started — used to compute uptime.
    started_at: Arc<Instant>,
    /// PID of this daemon process.
    pid: u32,
    /// Runtime configuration loaded from orqa.toml at startup.
    pub config: DaemonConfig,
}

/// JSON response body for GET /health.
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    uptime_seconds: u64,
    pid: u32,
}

/// Handle GET /health by returning the current daemon status.
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
    })
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
/// Binds to `127.0.0.1:<port>` and serves GET /health until the tokio runtime
/// shuts down. Logs the bound address on startup. Returns an error if the port
/// is already in use.
pub async fn start(port: u16, config: DaemonConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = HealthState {
        started_at: Arc::new(Instant::now()),
        pid: std::process::id(),
        config,
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/compact-context", post(crate::compact_context::compact_context_handler))
        .route("/context", post(crate::context::context_handler))
        .route("/knowledge", post(crate::knowledge::knowledge_handler))
        .route("/parse", post(crate::parse::parse_handler))
        .route("/prompt", post(crate::prompt::prompt_handler))
        .route(
            "/session-start",
            post(crate::session_start::session_start_handler),
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!(addr = %addr, "health endpoint listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "health server error");
        e.into()
    })
}
