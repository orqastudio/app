//! Probe binary: verify that LIVE SELECT works against an embedded SurrealDB 3.x instance.
//!
//! This is a one-shot investigation binary. It opens a temporary embedded SurrealDB,
//! subscribes to LIVE SELECT on the artifact table, triggers a CREATE and a DELETE,
//! then checks whether the live notifications arrive within a 5-second timeout.
//!
//! Example binaries are permitted to use stdout directly (unlike production code which
//! uses `tracing`) because their entire purpose is to print a human-readable report.
//! They are also permitted to be linear (over the 50-line function limit) — one-shot
//! investigation code benefits from reading top-to-bottom as a single narrative rather
//! than being fragmented into helpers.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::too_many_lines)]
//!
//! Exit codes:
//!   0 — both CREATE and DELETE notifications arrived (PASS)
//!   1 — one or both notifications did not arrive within the timeout (FAIL)
//!
//! Run with:
//!   cargo run -p orqa-graph-db --example live_probe

use std::time::{Duration, Instant};

use futures::StreamExt;
use surrealdb::engine::local::SurrealKv;
use surrealdb::{Notification, Surreal};
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let result = run_probe().await;
    match result {
        Ok(()) => {
            println!("VERDICT: PASS — both CREATE and DELETE notifications arrived");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("VERDICT: FAIL — {e}");
            std::process::exit(1);
        }
    }
}

/// Run the LIVE SELECT probe against a tempdir-backed embedded SurrealDB.
///
/// Returns Ok(()) if both CREATE and DELETE notifications arrive within timeout.
/// Returns Err with a descriptive message on any failure (including timeouts).
async fn run_probe() -> Result<(), String> {
    // Use a temporary directory so we never touch .state/surreal/
    let tmp = tempfile::tempdir().map_err(|e| format!("tempdir creation failed: {e}"))?;
    let db_path = tmp.path().join("probe.db");

    println!("SurrealDB crate version: 3.0.5");
    println!("DB path: {}", db_path.display());

    // Open an embedded SurrealKV instance at the tempdir path
    let db = Surreal::new::<SurrealKv>(db_path.to_string_lossy().as_ref())
        .await
        .map_err(|e| format!("failed to open embedded SurrealDB: {e}"))?;

    db.use_ns("probe")
        .use_db("test")
        .await
        .map_err(|e| format!("use_ns/use_db failed: {e}"))?;

    // Apply minimal schema for the artifact table
    db.query(
        "
        DEFINE TABLE IF NOT EXISTS artifact SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS artifact_type ON artifact TYPE string;
        DEFINE FIELD IF NOT EXISTS title ON artifact TYPE string;
        DEFINE FIELD IF NOT EXISTS path ON artifact TYPE string;
    ",
    )
    .await
    .map_err(|e| format!("schema definition failed: {e}"))?;

    println!("Schema initialized.");

    // Subscribe to LIVE SELECT — returns a live query stream
    let mut live_response = db
        .query("LIVE SELECT * FROM artifact")
        .await
        .map_err(|e| format!("LIVE SELECT query failed: {e}"))?;

    // In SurrealDB 3.x, stream() on the response returns a QueryStream.
    // When R = Value, items are Result<Notification<Value>>.
    let mut stream = live_response
        .stream::<Notification<serde_json::Value>>(0)
        .map_err(|e| format!("failed to obtain live stream: {e}"))?;

    println!("LIVE SELECT subscribed. Triggering CREATE...");
    let create_start = Instant::now();

    // Trigger a CREATE event
    db.query("CREATE artifact:probe SET artifact_type = 'test', title = 'Probe Artifact', path = '/probe/test.md'")
        .await
        .map_err(|e| format!("CREATE failed: {e}"))?;

    println!("CREATE executed. Waiting for notification (5s timeout)...");

    // Wait for the CREATE notification
    let create_event = timeout(Duration::from_secs(5), stream.next())
        .await
        .map_err(|_| "CREATE notification did not arrive within 5 seconds".to_owned())?;

    let create_latency = create_start.elapsed();

    match create_event {
        Some(Ok(notification)) => {
            println!(
                "CREATE notification received in {:.3}ms: action={:?} data={:?}",
                create_latency.as_secs_f64() * 1000.0,
                notification.action,
                notification.data
            );
        }
        Some(Err(e)) => {
            return Err(format!("CREATE notification stream error: {e}"));
        }
        None => {
            return Err("live stream closed before CREATE notification arrived".to_owned());
        }
    }

    println!("Triggering DELETE...");
    let delete_start = Instant::now();

    // Trigger a DELETE event
    db.query("DELETE artifact:probe")
        .await
        .map_err(|e| format!("DELETE failed: {e}"))?;

    println!("DELETE executed. Waiting for notification (5s timeout)...");

    // Wait for the DELETE notification
    let delete_event = timeout(Duration::from_secs(5), stream.next())
        .await
        .map_err(|_| "DELETE notification did not arrive within 5 seconds".to_owned())?;

    let delete_latency = delete_start.elapsed();

    match delete_event {
        Some(Ok(notification)) => {
            println!(
                "DELETE notification received in {:.3}ms: action={:?} data={:?}",
                delete_latency.as_secs_f64() * 1000.0,
                notification.action,
                notification.data
            );
        }
        Some(Err(e)) => {
            return Err(format!("DELETE notification stream error: {e}"));
        }
        None => {
            return Err("live stream closed before DELETE notification arrived".to_owned());
        }
    }

    Ok(())
}
