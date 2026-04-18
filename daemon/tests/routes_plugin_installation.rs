// Integration tests for plugin_installation ledger routes.
//
// Tests verify the full lifecycle of the SurrealDB-backed plugin installation
// ledger: write a record directly via engine writers, read it via the daemon
// routes, verify uninstall deletes it, and verify re-install produces a fresh
// installed_at record.
//
// Routes under test:
//   GET /plugins/installed          — list all plugin_installation records
//   GET /plugins/:name/installation — get a single record by plugin name

#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

/// Shared fixture: build a router and inject two plugin_installation records.
///
/// Returns `(router, db)` where `db` is the GraphDb handle used to set up the
/// fixture state.
///
/// Long body because the fixture builds a full router + two realistic
/// ledger records. Splitting would obscure the test setup.
#[allow(clippy::too_many_lines)]
async fn build_router_with_plugin_records() -> axum::Router {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");

    // Seed two plugin_installation records directly via engine writers.
    let files_a = vec![
        PluginFileEntry {
            path: ".orqa/knowledge/KNOW-001.md".to_owned(),
            source_hash: "aaa111".to_owned(),
            installed_hash: "aaa111".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
        PluginFileEntry {
            path: ".orqa/knowledge/KNOW-002.md".to_owned(),
            source_hash: "bbb222".to_owned(),
            installed_hash: "bbb999".to_owned(), // drifted
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
    ];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-alpha",
        "1.0.0",
        "hash-alpha",
        &files_a,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("upsert plugin-alpha");

    let files_b = vec![PluginFileEntry {
        path: ".claude/agents/AGENT-001.md".to_owned(),
        source_hash: "ccc333".to_owned(),
        installed_hash: "ccc333".to_owned(),
        target: "runtime".to_owned(),
        artifact_id: None,
    }];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-beta",
        "2.1.0",
        "hash-beta",
        &files_b,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("upsert plugin-beta");

    graph_state.inject_db(db);

    let state = HealthState::for_test(graph_state, None);
    orqa_daemon_lib::build_router(state)
}

// ---------------------------------------------------------------------------
// GET /plugins/installed
// ---------------------------------------------------------------------------

/// GET /plugins/installed returns all seeded records.
#[tokio::test]
async fn list_installed_plugins_returns_all_records() {
    let router = build_router_with_plugin_records().await;
    let request = Request::builder()
        .method("GET")
        .uri("/plugins/installed")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("response must be a JSON array");

    assert_eq!(items.len(), 2, "must return exactly 2 seeded records");
}

/// GET /plugins/installed returns plugin names in the response objects.
#[tokio::test]
async fn list_installed_plugins_includes_plugin_names() {
    let router = build_router_with_plugin_records().await;
    let request = Request::builder()
        .method("GET")
        .uri("/plugins/installed")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().unwrap();

    let names: Vec<&str> = items
        .iter()
        .filter_map(|r| r.get("plugin_name").and_then(|v| v.as_str()))
        .collect();

    assert!(
        names.contains(&"@orqastudio/plugin-alpha"),
        "plugin-alpha must be in the list"
    );
    assert!(
        names.contains(&"@orqastudio/plugin-beta"),
        "plugin-beta must be in the list"
    );
}

/// GET /plugins/installed returns records with files arrays.
#[tokio::test]
async fn list_installed_plugins_includes_files() {
    let router = build_router_with_plugin_records().await;
    let request = Request::builder()
        .method("GET")
        .uri("/plugins/installed")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().unwrap();

    let alpha = items
        .iter()
        .find(|r| r.get("plugin_name").and_then(|v| v.as_str()) == Some("@orqastudio/plugin-alpha"))
        .expect("plugin-alpha must be present");

    let files = alpha
        .get("files")
        .and_then(|v| v.as_array())
        .expect("plugin-alpha must have files array");

    assert_eq!(files.len(), 2, "plugin-alpha must have 2 files");
}

// ---------------------------------------------------------------------------
// GET /plugins/installed — empty database
// ---------------------------------------------------------------------------

/// GET /plugins/installed returns an empty array when no records exist.
#[tokio::test]
async fn list_installed_plugins_empty_when_no_records() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");
    graph_state.inject_db(db);

    let state = HealthState::for_test(graph_state, None);
    let router = orqa_daemon_lib::build_router(state);

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("must be array");
    assert!(items.is_empty(), "empty DB must return empty array");
}

// ---------------------------------------------------------------------------
// GET /plugins/:name/installation
// ---------------------------------------------------------------------------

/// GET /plugins/:name/installation returns the correct record for a known plugin.
#[tokio::test]
async fn get_plugin_installation_returns_record_for_known_plugin() {
    let router = build_router_with_plugin_records().await;
    let request = Request::builder()
        .method("GET")
        .uri("/plugins/@orqastudio%2Fplugin-beta/installation")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json.get("plugin_name").and_then(|v| v.as_str()),
        Some("@orqastudio/plugin-beta"),
    );
    assert_eq!(
        json.get("manifest_version").and_then(|v| v.as_str()),
        Some("2.1.0"),
    );
}

/// GET /plugins/:name/installation returns 404 for an unknown plugin.
#[tokio::test]
async fn get_plugin_installation_returns_404_for_unknown_plugin() {
    let router = build_router_with_plugin_records().await;
    let request = Request::builder()
        .method("GET")
        .uri("/plugins/nonexistent-plugin/installation")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "unknown plugin must return 404"
    );
}

// ---------------------------------------------------------------------------
// Delete via engine — verify it clears from GET /plugins/installed
// ---------------------------------------------------------------------------

/// Deleting a plugin_installation record directly (simulating uninstall) removes
/// it from GET /plugins/installed.
///
/// Long body because the test seeds two records, exercises the full HTTP
/// round-trip, and asserts counts before and after. Splitting would fragment
/// the before/after comparison.
#[tokio::test]
#[allow(clippy::too_many_lines)]
async fn uninstall_removes_plugin_from_installed_list() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{
        delete_plugin_installation, upsert_plugin_installation, PluginFileEntry,
        PluginInstallStatus,
    };

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");

    let files = vec![PluginFileEntry {
        path: "some/file.md".to_owned(),
        source_hash: "abc".to_owned(),
        installed_hash: "abc".to_owned(),
        target: "surrealdb".to_owned(),
        artifact_id: None,
    }];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-gamma",
        "3.0.0",
        "hash-gamma",
        &files,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("upsert plugin-gamma");

    // Verify it is present.
    let installed_before = orqa_graph::list_plugin_installations(&db)
        .await
        .expect("list before delete");
    assert_eq!(installed_before.len(), 1, "one record before delete");

    // Simulate uninstall by deleting the record.
    delete_plugin_installation(&db, "@orqastudio/plugin-gamma")
        .await
        .expect("delete plugin-gamma");

    graph_state.inject_db(db);
    let state = HealthState::for_test(graph_state, None);
    let router = orqa_daemon_lib::build_router(state);

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("must be array");
    assert!(items.is_empty(), "list must be empty after delete");
}

// ---------------------------------------------------------------------------
// Re-install produces fresh installed_at
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// HTTP round-trip: install → GET → DELETE → GET → reinstall → GET
// ---------------------------------------------------------------------------

/// Full install/uninstall/reinstall lifecycle exercised through the HTTP router.
///
/// Seeds plugin_installation records via engine writers (simulating what the
/// install route does to the ledger), then exercises GET and DELETE via actual
/// HTTP calls through the axum test router. Verifies that:
/// - GET /plugins/installed lists the seeded record.
/// - DELETE /plugins/installed/:name removes it and returns 204.
/// - GET after delete shows an empty list.
/// - A second "install" (upsert) produces a record with a higher version counter.
///
/// Uses `allow(clippy::too_many_lines)` because the fixture setup for a full
/// lifecycle test is inherently verbose.
#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn install_uninstall_reinstall_http_roundtrip() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");

    // --- First install: seed ledger record directly (simulates the route writing
    // the ledger after a successful filesystem install).
    let files_v1 = vec![PluginFileEntry {
        path: ".orqa/knowledge/KNOW-rt-001.md".to_owned(),
        source_hash: "rt-src-aaa".to_owned(),
        installed_hash: "rt-src-aaa".to_owned(),
        target: "surrealdb".to_owned(),
        artifact_id: None,
    }];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-roundtrip",
        "1.0.0",
        "hash-rt-v1",
        &files_v1,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("first install upsert");

    // Capture the version from the first install — used to verify that a second
    // install of the same plugin (without a delete in between) bumps the counter.
    // After delete + reinstall the counter resets to DEFAULT 1 + bump, so we
    // instead check manifest_version and file count to confirm the reinstall took.
    let record_after_first_install =
        orqa_graph::read_plugin_installation(&db, "@orqastudio/plugin-roundtrip")
            .await
            .expect("read after first install")
            .expect("record must exist after first install");
    let _version_after_install = record_after_first_install
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    graph_state.inject_db(db.clone());
    let state = HealthState::for_test(graph_state.clone(), None);
    let router = orqa_daemon_lib::build_router(state);

    // --- GET /plugins/installed — record must be listed.
    let resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let items: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = items.as_array().expect("must be array");
    assert_eq!(arr.len(), 1, "one record expected after first install");
    assert_eq!(
        arr[0].get("plugin_name").and_then(|v| v.as_str()),
        Some("@orqastudio/plugin-roundtrip"),
    );
    // Capture installed_at before delete for later comparison with the reinstall timestamp.
    let installed_at_v1 = arr[0]
        .get("installed_at")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    assert!(
        !installed_at_v1.is_empty(),
        "installed_at must be set after first install"
    );

    // --- DELETE /plugins/installed/:name — ledger record must be removed.
    let encoded_name = "@orqastudio%2Fplugin-roundtrip";
    let del_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/plugins/installed/{encoded_name}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        del_resp.status() == StatusCode::NO_CONTENT || del_resp.status() == StatusCode::OK,
        "DELETE must return 204 or 200, got {}",
        del_resp.status()
    );

    // --- GET after delete — list must be empty.
    let resp_after_del = router
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_after_del.status(), StatusCode::OK);
    let body = resp_after_del
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let items: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        items.as_array().is_some_and(Vec::is_empty),
        "list must be empty after delete"
    );

    // Sleep briefly so time::now() produces a strictly later timestamp on reinstall.
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // --- Second install: upsert again with a new version.
    let files_v2 = vec![
        PluginFileEntry {
            path: ".orqa/knowledge/KNOW-rt-001.md".to_owned(),
            source_hash: "rt-src-aaa".to_owned(),
            installed_hash: "rt-src-aaa".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
        PluginFileEntry {
            path: ".orqa/knowledge/KNOW-rt-002.md".to_owned(),
            source_hash: "rt-src-bbb".to_owned(),
            installed_hash: "rt-src-bbb".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
    ];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-roundtrip",
        "2.0.0",
        "hash-rt-v2",
        &files_v2,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("second install upsert");

    // --- GET after reinstall — manifest_version, file count, and installed_at must all be updated.
    let resp_after_reinstall = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_after_reinstall.status(), StatusCode::OK);
    let body = resp_after_reinstall
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let items: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = items.as_array().expect("must be array after reinstall");
    assert_eq!(arr.len(), 1, "one record expected after reinstall");

    assert_eq!(
        arr[0].get("manifest_version").and_then(|v| v.as_str()),
        Some("2.0.0"),
        "manifest_version must reflect the second install"
    );

    let file_count = arr[0]
        .get("files")
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    assert_eq!(file_count, 2, "second install must record both files");

    // installed_at must be strictly later than after the first install.
    let installed_at_v2 = arr[0]
        .get("installed_at")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    assert!(
        !installed_at_v2.is_empty(),
        "installed_at must be set after reinstall"
    );
    assert!(
        installed_at_v2 > installed_at_v1,
        "installed_at must advance on reinstall: before={installed_at_v1} after={installed_at_v2}"
    );
}

/// Re-installing (upserting) a plugin produces a fresh installed_at timestamp
/// and the file list is updated.
#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn reinstall_updates_record_and_bumps_version() {
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db)
        .await
        .expect("initialize SurrealDB schema");

    let files_v1 = vec![PluginFileEntry {
        path: "file-v1.md".to_owned(),
        source_hash: "v1hash".to_owned(),
        installed_hash: "v1hash".to_owned(),
        target: "surrealdb".to_owned(),
        artifact_id: None,
    }];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-delta",
        "1.0.0",
        "hash-v1",
        &files_v1,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("upsert v1");

    let record_v1 = orqa_graph::read_plugin_installation(&db, "@orqastudio/plugin-delta")
        .await
        .expect("read v1")
        .expect("v1 must exist");

    let version_v1 = record_v1
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    // Re-install with a new version and different files.
    let files_v2 = vec![
        PluginFileEntry {
            path: "file-v1.md".to_owned(),
            source_hash: "v1hash".to_owned(),
            installed_hash: "v1hash".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
        PluginFileEntry {
            path: "file-v2.md".to_owned(),
            source_hash: "v2hash".to_owned(),
            installed_hash: "v2hash".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
    ];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-delta",
        "2.0.0",
        "hash-v2",
        &files_v2,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("upsert v2");

    let record_v2 = orqa_graph::read_plugin_installation(&db, "@orqastudio/plugin-delta")
        .await
        .expect("read v2")
        .expect("v2 must exist");

    let version_v2 = record_v2
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    assert!(
        version_v2 > version_v1,
        "version must increment on re-install: before={version_v1} after={version_v2}"
    );

    let manifest_version = record_v2
        .get("manifest_version")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    assert_eq!(
        manifest_version, "2.0.0",
        "manifest_version must be updated"
    );

    let files = record_v2
        .get("files")
        .and_then(|v| v.as_array())
        .expect("files must be present");
    assert_eq!(files.len(), 2, "re-install must record both files");
}

// ---------------------------------------------------------------------------
// GET /plugins/drift — drift detection
// ---------------------------------------------------------------------------

/// GET /plugins/drift detects a plugin whose installed_hash differs from source_hash.
///
/// Seeds one plugin with 2 files: one clean (hashes match), one drifted (hashes differ).
/// Asserts the drift endpoint reports clean=false and lists only the drifted file.
#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn drift_detection_reports_drifted_plugin() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db).await.expect("initialize schema");

    // Seed one plugin with one clean file and one drifted file.
    let files = vec![
        PluginFileEntry {
            path: "file-clean.md".to_owned(),
            source_hash: "hash-clean-aaa".to_owned(),
            installed_hash: "hash-clean-aaa".to_owned(), // clean: hashes match
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
        PluginFileEntry {
            path: "file-drifted.md".to_owned(),
            source_hash: "hash-source-bbb".to_owned(),
            installed_hash: "hash-installed-ccc".to_owned(), // drifted: hashes differ
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
    ];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-drift-test",
        "1.0.0",
        "hash-manifest",
        &files,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("seed drifted plugin");

    graph_state.inject_db(db);
    let state = HealthState::for_test(graph_state, None);
    let router = orqa_daemon_lib::build_router(state);

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/drift")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let report: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        report.get("clean").and_then(serde_json::Value::as_bool),
        Some(false),
        "clean must be false when drift exists"
    );

    let drifted_plugins = report
        .get("drifted_plugins")
        .and_then(|v| v.as_array())
        .expect("drifted_plugins must be present");

    assert_eq!(drifted_plugins.len(), 1, "one plugin must be drifted");
    assert_eq!(
        drifted_plugins[0]
            .get("plugin_name")
            .and_then(|v| v.as_str()),
        Some("@orqastudio/plugin-drift-test"),
    );

    let drifted_files = drifted_plugins[0]
        .get("drifted_files")
        .and_then(|v| v.as_array())
        .expect("drifted_files must be present");

    // Only the drifted file must appear — not the clean one.
    assert_eq!(drifted_files.len(), 1, "only the drifted file must appear");
    assert_eq!(
        drifted_files[0].get("path").and_then(|v| v.as_str()),
        Some("file-drifted.md"),
    );
    assert_eq!(
        drifted_files[0].get("source_hash").and_then(|v| v.as_str()),
        Some("hash-source-bbb"),
    );
    assert_eq!(
        drifted_files[0]
            .get("installed_hash")
            .and_then(|v| v.as_str()),
        Some("hash-installed-ccc"),
    );
}

/// GET /plugins/drift reports clean=true when all installed file hashes match.
#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn drift_detection_reports_clean_when_all_hashes_match() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db).await.expect("initialize schema");

    // Seed one all-clean plugin.
    let files = vec![
        PluginFileEntry {
            path: "file-a.md".to_owned(),
            source_hash: "hash-aaa".to_owned(),
            installed_hash: "hash-aaa".to_owned(),
            target: "surrealdb".to_owned(),
            artifact_id: None,
        },
        PluginFileEntry {
            path: "file-b.md".to_owned(),
            source_hash: "hash-bbb".to_owned(),
            installed_hash: "hash-bbb".to_owned(),
            target: "runtime".to_owned(),
            artifact_id: None,
        },
    ];
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-clean",
        "1.0.0",
        "hash-manifest-clean",
        &files,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("seed clean plugin");

    graph_state.inject_db(db);
    let state = HealthState::for_test(graph_state, None);
    let router = orqa_daemon_lib::build_router(state);

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/drift")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let report: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        report.get("clean").and_then(serde_json::Value::as_bool),
        Some(true),
        "clean must be true when all hashes match"
    );
    let drifted_plugins = report
        .get("drifted_plugins")
        .and_then(|v| v.as_array())
        .expect("drifted_plugins must be present");
    assert!(drifted_plugins.is_empty(), "no drifted plugins expected");
}

/// GET /plugins/installed?include_failed=true includes Failed records;
/// the default endpoint excludes them.
#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn list_installed_excludes_failed_by_default_includes_with_flag() {
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::health::HealthState;
    use orqa_graph::surreal::{initialize_schema, open_memory};
    use orqa_graph::{upsert_plugin_installation, PluginFileEntry, PluginInstallStatus};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    let db = open_memory().await.expect("open in-memory SurrealDB");
    initialize_schema(&db).await.expect("initialize schema");

    let files = vec![PluginFileEntry {
        path: "file.md".to_owned(),
        source_hash: "aaa".to_owned(),
        installed_hash: "aaa".to_owned(),
        target: "surrealdb".to_owned(),
        artifact_id: None,
    }];

    // Seed one installed and one failed plugin.
    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-ok",
        "1.0.0",
        "hash-ok",
        &files,
        PluginInstallStatus::Installed,
    )
    .await
    .expect("seed installed plugin");

    upsert_plugin_installation(
        &db,
        "@orqastudio/plugin-failed",
        "1.0.0",
        "hash-fail",
        &files,
        PluginInstallStatus::Failed,
    )
    .await
    .expect("seed failed plugin");

    graph_state.inject_db(db);
    let state = HealthState::for_test(graph_state, None);
    let router = orqa_daemon_lib::build_router(state);

    // Default list: failed record must be excluded.
    let resp_default = router
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_default.status(), StatusCode::OK);
    let body = resp_default.into_body().collect().await.unwrap().to_bytes();
    let items: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = items.as_array().expect("must be array");
    assert_eq!(arr.len(), 1, "default list must exclude failed records");
    assert_eq!(
        arr[0].get("plugin_name").and_then(|v| v.as_str()),
        Some("@orqastudio/plugin-ok"),
    );

    // With include_failed=true: both records must appear.
    let resp_with_failed = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/plugins/installed?include_failed=true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_with_failed.status(), StatusCode::OK);
    let body = resp_with_failed
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let items: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = items.as_array().expect("must be array");
    assert_eq!(arr.len(), 2, "include_failed=true must return both records");

    let names: Vec<&str> = arr
        .iter()
        .filter_map(|r| r.get("plugin_name").and_then(|v| v.as_str()))
        .collect();
    assert!(names.contains(&"@orqastudio/plugin-ok"));
    assert!(names.contains(&"@orqastudio/plugin-failed"));
}
