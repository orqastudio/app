// Library interface for orqa-daemon.
//
// Exposes the daemon's internal modules so integration tests can call real route
// handlers against fixture state. The binary entrypoint (`main.rs`) depends only
// on this library crate. Tests import this library directly to avoid duplicating
// route handler logic in test helpers.
//
// The missing_docs lint is suppressed here because the daemon's internal modules
// use `//` comments rather than `///` doc comments — they were written as a binary
// crate where documentation comments don't generate rustdoc output. Promoting them
// all to `///` is out of scope for this task.
#![allow(missing_docs)]

pub mod config;
pub mod correlation;
pub mod event_bus;
pub mod graph_state;
pub mod health;
pub mod middleware;
pub mod routes;

// Internal modules that route handlers depend on — re-exported so the binary
// can still access them via `crate::` paths after the refactor.
pub mod compact_context;
pub mod context;
pub mod knowledge;
pub mod logging;
pub mod lsp;
pub mod mcp;
pub mod parse;
pub mod process;
pub mod prompt;
pub mod session_start;
pub mod subprocess;
pub mod tray;
pub mod watcher;

// Build a testable axum Router mounted on the same paths as the real daemon,
// wired to real route handlers using the provided state.
//
// This mirrors the router construction in `health::start` without port binding.
// Tests call this with a `HealthState` built from fixture data, then dispatch
// requests via `tower::ServiceExt::oneshot`.
#[allow(clippy::too_many_lines)]
pub fn build_router(state: health::HealthState) -> axum::Router {
    use axum::routing::{get, post};

    let artifact_router = axum::Router::new()
        .route("/", get(routes::artifacts::list_artifacts))
        .route("/tree", get(routes::artifacts::get_artifact_tree))
        .route("/import", post(routes::import::import_artifacts))
        .route("/{id}", get(routes::artifacts::get_artifact))
        .route(
            "/{id}",
            axum::routing::put(routes::artifacts::update_artifact),
        )
        .route(
            "/{id}/content",
            get(routes::artifacts::get_artifact_content),
        )
        .route(
            "/{id}/traceability",
            get(routes::artifacts::get_artifact_traceability),
        )
        .route("/{id}/impact", get(routes::artifacts::get_artifact_impact))
        .with_state(state.graph_state.clone());

    let graph_router = axum::Router::new()
        .route("/stats", get(routes::graph::get_graph_stats))
        .route("/health", get(routes::graph::get_graph_health))
        .route(
            "/health/snapshots",
            get(routes::graph::list_health_snapshots).post(routes::graph::create_health_snapshot),
        )
        .route("/parity", get(routes::graph::get_graph_parity))
        .route("/trace/{id}", get(routes::graph::get_graph_trace))
        .route("/siblings/{id}", get(routes::graph::get_graph_siblings))
        .route("/orphans", get(routes::graph::get_graph_orphans))
        .with_state(state.graph_state.clone());

    // Watcher control routes — pause/resume event emission during migration.
    let watcher_router = axum::Router::new()
        .route("/pause", post(routes::watcher::pause_watcher))
        .route("/resume", post(routes::watcher::resume_watcher))
        .route("/status", get(routes::watcher::watcher_status))
        .with_state(state.watcher_control.clone());

    // Admin storage migration routes — ingest phase for `orqa migrate storage`.
    let admin_migrate_router = axum::Router::new()
        .route(
            "/storage/ingest",
            post(routes::admin_migrate::storage_ingest),
        )
        .with_state(state.graph_state.clone());

    axum::Router::new()
        .route("/reload", post(health_reload_handler))
        .nest("/artifacts", artifact_router)
        .nest("/graph", graph_router)
        .nest("/watcher", watcher_router)
        .nest("/admin/migrate", admin_migrate_router)
        .layer(axum::middleware::from_fn(
            middleware::correlation_id_middleware,
        ))
        .with_state(state)
}

/// POST /reload handler for test router — rebuilds graph and returns counts.
///
/// This is a minimal reload handler used only by the test router. The real
/// daemon exposes this via the full health server. Tests need it to verify that
/// reload reflects updated state.
async fn health_reload_handler(
    axum::extract::State(state): axum::extract::State<health::HealthState>,
) -> axum::Json<serde_json::Value> {
    let project_root = state
        .graph_state
        .0
        .read()
        .map(|g| g.project_root.clone())
        .unwrap_or_default();

    let artifact_count = state.graph_state.reload(&project_root);
    let rule_count = state.graph_state.rule_count();

    axum::Json(serde_json::json!({
        "status": "reloaded",
        "artifacts": artifact_count,
        "rules": rule_count,
    }))
}
