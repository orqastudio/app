// Test harness helpers for orqa-daemon integration tests.
// Each function is used by at least one test binary; unused-in-binary warnings are suppressed.
#![allow(dead_code)]
//
// Provides factory functions for building the full axum Router and daemon state
// objects from the minimal fixture project at `tests/fixtures/minimal-project/`.
// All helpers work with real engine crates — no mocking. The fixture project is
// a genuine .orqa/ directory that produces a real graph with real nodes.
//
// Two router builders are available:
//   - `build_test_router`  — smoke-test router with 3 inline routes (no real handlers)
//   - `build_app_router`   — full router using real route handlers and HealthState

use std::path::PathBuf;
use std::sync::RwLock;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use orqa_validation::context::build_validation_context_complete;
use orqa_validation::graph::{build_artifact_graph, graph_stats, load_project_config};
use orqa_validation::platform::scan_plugin_manifests;

/// Absolute path to the minimal fixture project root.
///
/// Resolved at compile time relative to the cargo manifest dir so it works
/// regardless of which directory tests are invoked from.
pub fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-project")
}

/// Build a full axum Router using real route handlers from `orqa_daemon_lib`,
/// backed by a minimal `HealthState` constructed from fixture data.
///
/// Uses `orqa_daemon_lib::build_router` which mounts the same artifact and
/// graph routes as the production daemon. Tests dispatch requests via
/// `tower::ServiceExt::oneshot` without binding a real port.
///
/// The GraphState is loaded from the minimal fixture project (so artifact
/// routes see real fixture nodes) and is also backed by a fresh in-memory
/// SurrealDB (so import and graph-DB routes get an isolated, clean database
/// per call without writing to disk).
///
/// This is the entry point for all daemon integration tests.
pub async fn build_app_router() -> Router {
    use orqa_graph::surreal::{initialize_schema, open_memory};

    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = fixture_dir();
    // Load fixture artifacts into the in-memory HashMap graph so that
    // artifact-route tests (routes_artifacts.rs) see pre-populated nodes.
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    // Replace/add the SurrealDB handle with a fresh in-memory instance so
    // tests do not write to the on-disk .state/surreal/ path under the fixture
    // directory and each call gets an isolated database.
    //
    // bulk_sync populates the in-memory DB from fixture files so that
    // SurrealDB-backed list/filter/get routes (routes_artifacts.rs) see the
    // same fixture artifacts that the HashMap graph contains.
    // Import tests (routes_import.rs) call build_app_router() for a fresh
    // empty router per test — bulk_sync is fast on the small fixture set.
    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");
    orqa_graph::sync::bulk_sync(&db, &root)
        .await
        .expect("bulk_sync fixture artifacts into in-memory SurrealDB");
    graph_state.inject_db(db);

    let state = HealthState::for_test(graph_state, None);
    orqa_daemon_lib::build_router(state)
}

/// Build a router covering all routes needed for C-9+C-10 misc route tests.
///
/// Extends `build_app_router` with plugin, workflow, agent, and hook routes
/// backed by `GraphState`. These routes do not require `HealthState`, so they
/// are mounted directly with `GraphState` extracted from the fixture project.
///
/// Returns `(Router, tempfile::NamedTempFile)` — the NamedTempFile is unused
/// but returned so the caller signature is consistent with `build_store_router`.
/// Drop it at end of test scope.
pub async fn build_full_router() -> (Router, tempfile::NamedTempFile) {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let state = HealthState::for_test(graph_state.clone(), None);
    let base = orqa_daemon_lib::build_router(state);

    let router = base
        .route("/health", get(test_health_handler(graph_state.clone())))
        .nest("/plugins", plugin_router(graph_state.clone()))
        .nest("/workflow", workflow_router(graph_state.clone()))
        .nest("/agents", agent_router(graph_state.clone()))
        .nest("/hooks", hook_router(graph_state));

    let db_file = tempfile::NamedTempFile::new().expect("tempfile must create");
    (router, db_file)
}

fn plugin_router(state: orqa_daemon_lib::graph_state::GraphState) -> Router {
    Router::new()
        .route("/", get(orqa_daemon_lib::routes::plugins::list_plugins))
        .route("/{name}", get(orqa_daemon_lib::routes::plugins::get_plugin))
        .route(
            "/{name}/path",
            get(orqa_daemon_lib::routes::plugins::get_plugin_path),
        )
        .with_state(state)
}

fn workflow_router(state: orqa_daemon_lib::graph_state::GraphState) -> Router {
    Router::new()
        .route(
            "/transitions",
            get(orqa_daemon_lib::routes::workflow::list_transitions),
        )
        .with_state(state)
}

fn agent_router(state: orqa_daemon_lib::graph_state::GraphState) -> Router {
    Router::new()
        .route(
            "/behavioral-messages",
            get(orqa_daemon_lib::routes::agents::get_behavioral_messages),
        )
        .route("/{role}", get(orqa_daemon_lib::routes::agents::get_agent))
        .with_state(state)
}

fn hook_router(state: orqa_daemon_lib::graph_state::GraphState) -> Router {
    Router::new()
        .route("/", get(orqa_daemon_lib::routes::hooks::list_hooks))
        .with_state(state)
}

fn test_health_handler(
    gs: orqa_daemon_lib::graph_state::GraphState,
) -> impl FnOnce() -> std::pin::Pin<
    Box<dyn std::future::Future<Output = axum::Json<serde_json::Value>> + Send>,
> + Clone {
    move || {
        let gs = gs.clone();
        Box::pin(async move {
            axum::Json(serde_json::json!({
                "status": "ok",
                "uptime_seconds": 0u64,
                "pid": std::process::id(),
                "version": env!("CARGO_PKG_VERSION"),
                "artifact_count": gs.artifact_count(),
                "rule_count": gs.rule_count(),
                "processes": [],
            }))
        })
    }
}

/// Build and return the full axum `Router` with all smoke-tested daemon routes,
/// using state built from the minimal fixture project.
///
/// Mirrors the router construction in `health::start` but binds to no port.
/// Callers use `tower::ServiceExt::oneshot` to dispatch test requests directly
/// without occupying a real socket. All state is built from the minimal fixture
/// project, producing a real graph with real artifacts.
#[allow(clippy::too_many_lines)]
pub fn build_test_router() -> Router {
    let root = fixture_dir();

    // Build the artifact graph and validation context from fixture artifacts.
    // Both use the same construction path as the real daemon so fixture artifacts
    // appear exactly as they would in production.
    let graph = build_artifact_graph(&root).expect("fixture graph must build cleanly");
    let (valid_statuses, delivery, project_relationships) = load_project_config(&root);
    let plugin_contributions = scan_plugin_manifests(&root);
    let ctx = build_validation_context_complete(
        &valid_statuses,
        &delivery,
        &project_relationships,
        &plugin_contributions.relationships,
        &plugin_contributions.artifact_types,
        &plugin_contributions.schema_extensions,
        &plugin_contributions.enforcement_mechanisms,
    );

    // Shared state tuple: (graph, validation_context, project_root).
    // Wrapped in Arc<RwLock<>> for concurrent read access from route handlers.
    let state = Arc::new(RwLock::new((graph, ctx, root)));

    let health_state = Arc::clone(&state);
    let artifacts_state = Arc::clone(&state);
    let stats_state = Arc::clone(&state);

    Router::new()
        .route(
            "/health",
            get(move || {
                let s = Arc::clone(&health_state);
                async move {
                    let guard = s.read().expect("RwLock not poisoned");
                    let artifact_count = guard.0.nodes.len();
                    axum::Json(serde_json::json!({
                        "status": "ok",
                        "artifact_count": artifact_count,
                        "pid": std::process::id(),
                        "version": env!("CARGO_PKG_VERSION"),
                    }))
                }
            }),
        )
        .route(
            "/artifacts",
            get(move || {
                let s = Arc::clone(&artifacts_state);
                async move {
                    let guard = s.read().expect("RwLock not poisoned");
                    let nodes: Vec<serde_json::Value> = guard
                        .0
                        .nodes
                        .values()
                        .map(|n| {
                            serde_json::json!({
                                "id": n.id,
                                "type": n.artifact_type,
                                "title": n.title,
                                "status": n.status,
                            })
                        })
                        .collect();
                    axum::Json(nodes)
                }
            }),
        )
        .route(
            "/graph/stats",
            get(move || {
                let s = Arc::clone(&stats_state);
                async move {
                    let guard = s.read().expect("RwLock not poisoned");
                    let stats = graph_stats(&guard.0);
                    axum::Json(serde_json::json!({
                        "node_count": stats.node_count,
                        "edge_count": stats.edge_count,
                        "orphan_count": stats.orphan_count,
                        "broken_refs": stats.broken_ref_count,
                    }))
                }
            }),
        )
}
