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
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::RwLock;

/// Process-global counter for generating unique in-memory SurrealDB database names.
///
/// SurrealDB's `kv-mem` backend shares one in-process datastore across all instances
/// using the same namespace+database pair. Tests must use unique names to avoid
/// cross-test data contamination when running concurrently.
static TEST_DB_COUNTER: AtomicU64 = AtomicU64::new(0);

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

/// Build a full axum Router with SurrealDB absent — `db: None` in GraphState.
///
/// Used to test that routes requiring SurrealDB return 503 rather than silently
/// degrading to a disk-write fallback. The fixture HashMap and validation context
/// are populated (so schema validation and artifact lookups work), but no SurrealDB
/// handle is injected, so routes that gate on `surreal_db()` return 503.
pub async fn build_app_router_without_surrealdb() -> Router {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = fixture_dir();
    // build() loads the HashMap + ctx from fixture files and opens an embedded SurrealDB.
    // We then clear the db handle so routes see db: None and return 503.
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));
    if let Ok(mut guard) = graph_state.0.write() {
        guard.db = None;
    }

    let state = HealthState::for_test(graph_state, None);
    orqa_daemon_lib::build_router(state)
}

/// Build a `HealthState` backed by a fresh in-memory SurrealDB and the minimal fixture project.
///
/// Useful when a test needs two routers that share the same `GraphState` — e.g. POST followed
/// by GET to verify read-your-writes. Both routers are built via `build_router(state.clone())`.
///
/// Calls `GraphState::build` to populate the HashMap and validation context from fixture files,
/// then replaces the embedded SurrealDB handle with an isolated in-memory instance so tests
/// do not contend on the shared `.state/surreal/` file lock.
pub async fn build_app_state() -> orqa_daemon_lib::health::HealthState {
    use orqa_graph::surreal::{initialize_schema, open_memory_isolated};

    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = fixture_dir();
    // build() populates HashMap + validation ctx from fixture files.
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    // Replace the embedded SurrealDB with an isolated in-memory instance.
    let id = TEST_DB_COUNTER.fetch_add(1, Ordering::SeqCst);
    let db_name = format!("test_appstate_{id}");
    let db = open_memory_isolated(&db_name)
        .await
        .expect("open isolated in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");
    orqa_graph::sync::bulk_sync(&db, &root)
        .await
        .expect("bulk_sync fixture artifacts");
    graph_state.inject_db(db);

    HealthState::for_test(graph_state, None)
}

/// Build a `HealthState` with a fresh on-disk SurrealDB rooted at `project_root`.
///
/// Uses `open_embedded` at `<project_root>/.state/surreal/` to create a truly
/// isolated SurrealDB store per test. SurrealDB's `kv-mem` backend shares a
/// single in-process store across ALL in-memory instances regardless of database
/// name — concurrent writes from multiple tests corrupt each other's state and
/// cause queries to hang. On-disk embedded stores are isolated at the OS level.
///
/// `project_root` must be a unique temp directory per test (callers use
/// `tempfile::tempdir()` to guarantee this).
///
/// The embedded SurrealDB is initialized with the schema but NOT pre-populated;
/// callers drive population via the route under test (e.g. POST /admin/migrate/storage/ingest).
pub async fn build_state_for_project(
    project_root: &std::path::Path,
) -> orqa_daemon_lib::health::HealthState {
    use orqa_graph::surreal::{initialize_schema, open_embedded};

    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let graph_state = GraphState::build_empty(project_root);

    let surreal_path = project_root.join(".state").join("surreal");
    std::fs::create_dir_all(&surreal_path).expect("create .state/surreal dir");
    let db = open_embedded(&surreal_path)
        .await
        .expect("open embedded SurrealDB for test");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");
    graph_state.inject_db(db);

    HealthState::for_test(graph_state, None)
}

/// Build a full axum Router using real route handlers from `orqa_daemon_lib`,
/// backed by a minimal `HealthState` constructed from fixture data.
///
/// Uses `orqa_daemon_lib::build_router` which mounts the same artifact and
/// graph routes as the production daemon. Tests dispatch requests via
/// `tower::ServiceExt::oneshot` without binding a real port.
///
/// Calls `GraphState::build` to populate the HashMap and validation context
/// from fixture files (required for routes that read the HashMap and for schema
/// validation). Then replaces the embedded SurrealDB handle with a fresh isolated
/// in-memory instance so concurrent tests do not contend on the shared
/// `.state/surreal/` embedded DB file lock.
///
/// This is the entry point for all daemon integration tests.
pub async fn build_app_router() -> Router {
    use orqa_graph::surreal::{initialize_schema, open_memory_isolated};

    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;

    let root = fixture_dir();
    // build() populates the HashMap and validation ctx (incl. plugin schema) from
    // fixture files — required for GET /artifacts/:id, schema validation, etc.
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    // Replace the embedded SurrealDB with a per-call isolated in-memory instance.
    // Each call gets a unique DB name so kv-mem does not share state across tests.
    let id = TEST_DB_COUNTER.fetch_add(1, Ordering::SeqCst);
    let db_name = format!("test_approuter_{id}");
    let db = open_memory_isolated(&db_name)
        .await
        .expect("open isolated in-memory SurrealDB");
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
    // build() populates the HashMap and validation ctx from fixture files — required
    // for routes_misc tests that assert artifact counts and agent messages.
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
    use axum::routing::post;
    Router::new()
        .route("/", get(orqa_daemon_lib::routes::plugins::list_plugins))
        .route("/{name}", get(orqa_daemon_lib::routes::plugins::get_plugin))
        .route(
            "/{name}/path",
            get(orqa_daemon_lib::routes::plugins::get_plugin_path),
        )
        .route(
            "/install/local",
            post(orqa_daemon_lib::routes::plugins::install_plugin_local),
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
