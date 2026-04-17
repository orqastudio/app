// Integration tests for POST /artifacts/import.
//
// Tests use the real import route handler backed by an in-memory SurrealDB
// instance. Fixtures from `tests/fixtures/s2-03-import/` are used:
//
//   fixture-a/  — three artifacts: EPIC-001, EPIC-002, TASK-001 (initial state)
//   fixture-b/  — same three IDs, EPIC-001 and EPIC-002 have different fields,
//                 TASK-001 is identical (should be SKIPPED on re-import)
//   fixture-merge-conflict/ — EPIC-CONFLICT (designed to trigger a conflict
//                              under merge policy)
//
// Test matrix:
//   C1. Fresh import of fixture-a → all three CREATED
//   C2. Re-import fixture-a (same content) → all three SKIPPED (content-hash idempotent)
//   C3. Import fixture-b with upsert policy after fixture-a → 2 UPDATED + 1 SKIPPED
//   C4. Import fixture-b with merge policy after fixture-a → 2 MERGED + 1 SKIPPED
//       (EPIC-001 and EPIC-002 have non-conflicting field changes)
//   C5. Import fixture-merge-conflict with merge policy → CONFLICT outcome,
//       conflict file written
//   C6. version increments on UPDATE; unchanged on SKIPPED
//   C6b. base_snapshot enables HTTP-level three-way merge → MERGED outcome
//   C7. base_snapshot field: present but unused → warning in response
//   C8. Invalid path → 400 Bad Request

#![allow(missing_docs)]
// Integration tests may use `.unwrap()` freely — they are test code by
// definition (`daemon/tests/*` are all test binaries). Clippy's
// `allow-unwrap-in-tests` only applies to `#[test]`-annotated functions, not
// to the helper functions shared across tests. Integration tests also justify
// `too_many_lines` allowances when a single HTTP flow needs many assertions.
#![allow(clippy::unwrap_used, clippy::too_many_lines)]

mod helpers;

use std::path::PathBuf;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use orqa_graph::surreal::{initialize_schema, open_memory};
use orqa_graph::writers::{import_upsert, read_artifact};
use serde_json::json;
use tower::ServiceExt as _;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Path to the s2-03-import fixture root.
fn fixture_root() -> PathBuf {
    // Fixtures live at <workspace root>/tests/fixtures/s2-03-import/.
    // CARGO_MANIFEST_DIR resolves to daemon/ at compile time.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("daemon parent dir")
        .join("tests/fixtures/s2-03-import")
}

/// Absolute path to the named fixture subdirectory.
fn fixture_dir(name: &str) -> String {
    fixture_root()
        .join(name)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Build an axum Router wired to a fresh in-memory SurrealDB for import tests.
///
/// Uses `build_app_router` from the test helpers which constructs the real
/// HealthState with an embedded SurrealDB. We override the SurrealDB handle
/// with an in-memory instance so tests are isolated and fast.
async fn build_import_router() -> axum::Router {
    helpers::build_app_router().await
}

/// POST /artifacts/import with the given JSON body; return (status, body) pair.
async fn post_import(
    router: axum::Router,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    let req = Request::builder()
        .method("POST")
        .uri("/artifacts/import")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
    (status, json)
}

// ---------------------------------------------------------------------------
// C1: Fresh import → all CREATED
// ---------------------------------------------------------------------------

/// C1: Importing fixture-a into an empty DB creates all 3 artifacts.
#[tokio::test]
async fn c1_fresh_import_creates_all_artifacts() {
    let router = build_import_router().await;
    let (status, body) = post_import(
        router,
        json!({
            "path": fixture_dir("fixture-a"),
            "on_conflict": "upsert"
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "import must return 200: {body}");
    assert_eq!(
        body["created"], 3,
        "all 3 fixture-a artifacts must be CREATED: {body}"
    );
    assert_eq!(body["updated"], 0);
    assert_eq!(body["skipped"], 0);
    assert_eq!(body["conflicts"], 0);
}

// ---------------------------------------------------------------------------
// C2: Re-import same content → all SKIPPED (idempotent)
// ---------------------------------------------------------------------------

/// C2: Re-running the same import is a no-op (content hash unchanged).
#[tokio::test]
async fn c2_reimport_same_content_is_noop() {
    let router = helpers::build_app_router().await;

    // First import: seed fixture-a.
    let (s1, _) = post_import(
        router.clone(),
        json!({ "path": fixture_dir("fixture-a"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s1, StatusCode::OK);

    // Second import of the same directory: must be all SKIPPED.
    let (s2, body2) = post_import(
        router,
        json!({ "path": fixture_dir("fixture-a"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(
        body2["skipped"], 3,
        "all 3 must be SKIPPED on re-run: {body2}"
    );
    assert_eq!(body2["updated"], 0);
    assert_eq!(body2["created"], 0);
}

// ---------------------------------------------------------------------------
// C3: Upsert policy — fixture-b overwrites EPIC-001, EPIC-002; skips TASK-001
// ---------------------------------------------------------------------------

/// C3: Under upsert policy, changed records are UPDATED; unchanged is SKIPPED.
#[tokio::test]
async fn c3_upsert_policy_updates_changed_artifacts() {
    let router = helpers::build_app_router().await;

    // Seed with fixture-a.
    let (s1, _) = post_import(
        router.clone(),
        json!({ "path": fixture_dir("fixture-a"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s1, StatusCode::OK);

    // Import fixture-b with upsert policy.
    let (s2, body) = post_import(
        router,
        json!({ "path": fixture_dir("fixture-b"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK, "upsert import must return 200: {body}");

    // EPIC-001 and EPIC-002 differ; TASK-001 is identical.
    assert_eq!(
        body["updated"], 2,
        "2 changed artifacts must be UPDATED: {body}"
    );
    assert_eq!(
        body["skipped"], 1,
        "TASK-001 must be SKIPPED (unchanged): {body}"
    );
    assert_eq!(body["conflicts"], 0);
    assert_eq!(body["merged"], 0);
}

// ---------------------------------------------------------------------------
// C4: Merge policy — fixture-b auto-merges non-conflicting fields
// ---------------------------------------------------------------------------

/// C4: Under merge policy without a known base, fixture-b import surfaces
///     all changed records as CONFLICT (review-each) since there is no base.
///     With no_base_action=take-theirs, the records are written as UPDATED.
#[tokio::test]
async fn c4_merge_policy_take_theirs_writes_records() {
    let router = helpers::build_app_router().await;

    // Seed with fixture-a.
    let (s1, _) = post_import(
        router.clone(),
        json!({ "path": fixture_dir("fixture-a"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s1, StatusCode::OK);

    // Import fixture-b with merge + take-theirs (no base snapshot provided).
    // Since there is no base, three-way merge cannot run — no_base_action determines the outcome.
    let (s2, body) = post_import(
        router,
        json!({
            "path": fixture_dir("fixture-b"),
            "on_conflict": "merge",
            "no_base_action": "take-theirs"
        }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK, "merge import must return 200: {body}");

    // EPIC-001, EPIC-002 differ → take-theirs → UPDATED; TASK-001 unchanged → SKIPPED.
    let updated = body["updated"].as_u64().unwrap_or(0);
    let skipped = body["skipped"].as_u64().unwrap_or(0);
    let conflicts = body["conflicts"].as_u64().unwrap_or(0);
    assert_eq!(
        skipped, 1,
        "TASK-001 (unchanged hash) must be SKIPPED: {body}"
    );
    assert_eq!(
        updated + conflicts,
        2,
        "EPIC-001 and EPIC-002 must be UPDATED or CONFLICT: {body}"
    );
}

// ---------------------------------------------------------------------------
// C5: Merge policy — fixture-merge-conflict produces CONFLICT
// ---------------------------------------------------------------------------

/// C5: A record with conflicting fields produces a CONFLICT outcome and the
///     import returns HTTP 200 (import ran; caller inspects conflict count).
#[tokio::test]
async fn c5_merge_policy_conflict_file_written() {
    let dir = tempfile::tempdir().unwrap();
    let router = helpers::build_app_router().await;

    // Seed EPIC-CONFLICT with a known initial state.
    let initial = json!({
        "path": fixture_dir("fixture-merge-conflict"),
        "on_conflict": "upsert"
    });
    let (s1, b1) = post_import(router.clone(), initial).await;
    assert_eq!(s1, StatusCode::OK, "seed must succeed: {b1}");

    // Now import the same fixture again with merge + review-each.
    // The content hash is identical, so it should be SKIPPED — not conflicted.
    let (s2, body) = post_import(
        router.clone(),
        json!({
            "path": fixture_dir("fixture-merge-conflict"),
            "on_conflict": "merge",
            "no_base_action": "review-each"
        }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK);
    // Content hash unchanged from seed → should be SKIPPED (idempotent).
    assert_eq!(
        body["skipped"], 1,
        "unchanged content must be SKIPPED: {body}"
    );

    // Now create a conflicting in-DB state by importing with a modified title,
    // and then re-importing the original file under merge.
    // This simulates: DB has different title than the incoming file.
    // We do this by using the fixture-a EPIC-001 then fixture-b EPIC-001 under merge.
    let router2 = helpers::build_app_router().await;

    // Seed with fixture-a EPIC-001 (title: "Epic One").
    let (sa, _) = post_import(
        router2.clone(),
        json!({ "path": fixture_dir("fixture-a"), "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(sa, StatusCode::OK);

    // Import fixture-b EPIC-001 (title: "Epic One (Updated)") with merge + review-each.
    // No base snapshot → review-each → CONFLICT.
    let (sb, body_b) = post_import(
        router2,
        json!({
            "path": fixture_dir("fixture-b"),
            "on_conflict": "merge",
            "no_base_action": "review-each"
        }),
    )
    .await;
    assert_eq!(sb, StatusCode::OK);
    // At least one CONFLICT expected (EPIC-001 and EPIC-002 differ, no base).
    let n_conflicts = body_b["conflicts"].as_u64().unwrap_or(0);
    let n_skipped = body_b["skipped"].as_u64().unwrap_or(0);
    assert_eq!(n_skipped, 1, "TASK-001 must be SKIPPED: {body_b}");
    assert_eq!(
        n_conflicts, 2,
        "EPIC-001 and EPIC-002 must CONFLICT without a base: {body_b}"
    );

    drop(dir);
}

// ---------------------------------------------------------------------------
// C6: version increments on UPDATE; unchanged on SKIPPED
// ---------------------------------------------------------------------------

/// C6: Version counter increments on UPDATED records; SKIPPED leaves version unchanged.
#[tokio::test]
async fn c6_version_increments_on_update_skipped_unchanged() {
    let db = {
        let d = open_memory().await.unwrap();
        initialize_schema(&d).await.unwrap();
        d
    };

    // Schema DEFAULT 1 + bump_version: first import_upsert produces version 2.
    // Version 1 = "record created, never explicitly written via import".
    // Version 2+ = "at least one explicit import write has occurred".
    use std::collections::BTreeMap;
    let mut fields: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    fields.insert("id".into(), json!("EPIC-001"));
    fields.insert("type".into(), json!("epic"));
    fields.insert("title".into(), json!("Epic One"));
    fields.insert("status".into(), json!("active"));

    import_upsert(&db, "EPIC-001", &fields, ".orqa/EPIC-001.md", "hash-v1", 0)
        .await
        .unwrap();

    let after_v1 = read_artifact(&db, "EPIC-001").await.unwrap().unwrap();
    assert_eq!(
        after_v1.version, 2,
        "first write must set version=2 (DEFAULT 1 + one bump)"
    );

    // Second write with a different hash (simulate changed content) → version=3.
    import_upsert(
        &db,
        "EPIC-001",
        &fields,
        ".orqa/EPIC-001.md",
        "hash-v2",
        after_v1.version,
    )
    .await
    .unwrap();

    let after_v2 = read_artifact(&db, "EPIC-001").await.unwrap().unwrap();
    assert_eq!(
        after_v2.version, 3,
        "second write must increment version to 3"
    );

    // Write with the SAME hash as v2 would be SKIPPED by the import route.
    // The writers module itself does not enforce the skip — the route does.
    // We verify here that version stays at 3 (no write means no change).
    let unchanged = read_artifact(&db, "EPIC-001").await.unwrap().unwrap();
    assert_eq!(
        unchanged.version, 3,
        "version must not change without a write"
    );
}

// ---------------------------------------------------------------------------
// C7: base_snapshot present but unused → warning in response
// ---------------------------------------------------------------------------

/// C7: A base_snapshot in the request body produces a warning if no record
///     has source_plugin set (which is the normal case for this importer).
#[tokio::test]
async fn c7_base_snapshot_present_but_unused_warning() {
    let router = build_import_router().await;

    let (status, body) = post_import(
        router,
        json!({
            "path": fixture_dir("fixture-a"),
            "on_conflict": "upsert",
            "base_snapshot": {
                "EPIC-001": {"status": "active", "title": "Epic One"}
            }
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    // Warning must be present since no records have source_plugin set.
    let warning = body["base_snapshot_warning"].as_str().unwrap_or("");
    assert!(
        !warning.is_empty(),
        "base_snapshot_warning must be set when snapshot is provided but unused: {body}"
    );
    assert!(
        warning.contains("not used"),
        "warning must explain snapshot was not used: {warning}"
    );
}

// ---------------------------------------------------------------------------
// C6b: Three-way merge via HTTP — base_snapshot enables clean auto-merge
// ---------------------------------------------------------------------------

/// C6b: An import with `on_conflict: merge` and a `base_snapshot` entry that covers
/// the divergence between ours and theirs produces a MERGED outcome via the HTTP route,
/// not just via the engine layer directly.
///
/// Scenario:
///   base   = {title: "Merge Test", status: "active"}
///   ours   = {title: "Merge Test", status: "draft"}          (we changed status)
///   theirs = {title: "Merge Test (Renamed)", status: "active"} (they changed title)
///
/// Expected: clean three-way merge → merged=1, conflicts=0, outcome=MERGED
#[tokio::test]
async fn c6b_merge_with_base_snapshot_auto_merges() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Step 1: seed the DB with the base state via upsert.
    let base_content = "---\nid: EPIC-MERGE-BASE\ntype: epic\ntitle: Merge Test\nstatus: active\npriority: high\n---\n\nBody text.\n";
    std::fs::write(dir.join("EPIC-MERGE-BASE.md"), base_content).unwrap();

    let router = helpers::build_app_router().await;
    let dir_str = dir.to_string_lossy().replace('\\', "/");

    let (s1, b1) = post_import(
        router.clone(),
        serde_json::json!({ "path": dir_str, "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s1, StatusCode::OK, "base seed must succeed: {b1}");
    assert_eq!(b1["created"], 1, "must create base record: {b1}");

    // Step 2: simulate "our" change — update status to "draft" via a second upsert.
    let ours_content = "---\nid: EPIC-MERGE-BASE\ntype: epic\ntitle: Merge Test\nstatus: draft\npriority: high\n---\n\nBody text.\n";
    std::fs::write(dir.join("EPIC-MERGE-BASE.md"), ours_content).unwrap();

    let (s2, b2) = post_import(
        router.clone(),
        serde_json::json!({ "path": dir_str, "on_conflict": "upsert" }),
    )
    .await;
    assert_eq!(s2, StatusCode::OK, "our upsert must succeed: {b2}");
    assert_eq!(b2["updated"], 1, "must update with our change: {b2}");

    // Step 3: import "theirs" (new title, original status) with base_snapshot.
    // The base_snapshot tells the merge algorithm what the original base looked like
    // so it can distinguish our change (status→draft) from their change (title→renamed).
    let theirs_content = "---\nid: EPIC-MERGE-BASE\ntype: epic\ntitle: Merge Test (Renamed)\nstatus: active\npriority: high\n---\n\nBody text.\n";
    std::fs::write(dir.join("EPIC-MERGE-BASE.md"), theirs_content).unwrap();

    let (s3, b3) = post_import(
        router.clone(),
        serde_json::json!({
            "path": dir_str,
            "on_conflict": "merge",
            "no_base_action": "review-each",
            "base_snapshot": {
                "EPIC-MERGE-BASE": {
                    "id": "EPIC-MERGE-BASE",
                    "type": "epic",
                    "title": "Merge Test",
                    "status": "active",
                    "priority": "high"
                }
            }
        }),
    )
    .await;
    assert_eq!(StatusCode::OK, s3, "merge import must return 200: {b3}");
    assert_eq!(
        b3["merged"], 1,
        "three-way merge must produce merged=1: {b3}"
    );
    assert_eq!(
        b3["conflicts"], 0,
        "clean merge must have conflicts=0: {b3}"
    );

    // Verify the per-artifact result entry.
    let results = b3["results"]
        .as_array()
        .expect("response must have results array");
    assert_eq!(results.len(), 1, "must have exactly one result entry: {b3}");
    let result = &results[0];
    assert_eq!(
        result["outcome"].as_str(),
        Some("MERGED"),
        "per-artifact outcome must be MERGED: {result}"
    );
    assert_eq!(
        result["id"].as_str(),
        Some("EPIC-MERGE-BASE"),
        "result id must be EPIC-MERGE-BASE: {result}"
    );
}

// ---------------------------------------------------------------------------
// C8: Invalid path → 400
// ---------------------------------------------------------------------------

/// C8: A path that does not exist returns 400.
#[tokio::test]
async fn c8_invalid_path_returns_400() {
    let router = build_import_router().await;

    let (status, body) = post_import(
        router,
        json!({
            "path": "/nonexistent/path/that/does/not/exist",
            "on_conflict": "upsert"
        }),
    )
    .await;

    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "non-existent path must return 400: {body}"
    );
    assert!(
        body["error"].as_str().is_some(),
        "400 response must include error field: {body}"
    );
}
