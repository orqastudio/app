//! Criterion benchmarks for the SurrealDB artifact graph POC.
//!
//! Benchmarks run against the REAL project data at `../../.orqa/`.
//! They measure cold start, full ingest, single upsert, and query latencies.

#![allow(clippy::unwrap_used, clippy::print_stderr, missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use orqa_graph_db::ingest::{ingest_directory, ingest_single_file};
use orqa_graph_db::queries;
use orqa_graph_db::GraphDb;
use std::path::PathBuf;
use surrealdb::types::SurrealValue;

/// Path to the project's `.orqa/` directory relative to the bench binary.
fn orqa_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("../../.orqa")
}

/// Find a leaf task artifact path for single-file benchmarks.
fn find_leaf_artifact() -> Option<PathBuf> {
    let dir = orqa_dir().join("implementation/tasks");
    if !dir.exists() {
        return None;
    }
    for entry in walkdir::WalkDir::new(&dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.path().extension().and_then(|e| e.to_str()) == Some("md") {
            return Some(entry.path().to_owned());
        }
    }
    None
}

/// Benchmark: full ingest of all `.orqa/` files from scratch.
fn bench_full_ingest(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        eprintln!("SKIP: .orqa/ directory not found at {}", orqa.display());
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("full_ingest", |b| {
        b.iter(|| {
            rt.block_on(async {
                let db = GraphDb::open_memory().await.unwrap();
                ingest_directory(&db, &orqa).await.unwrap()
            })
        });
    });

    // Print a summary from a single run for the findings report.
    let summary = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap()
    });
    eprintln!(
        "\n=== INGEST SUMMARY ===\n\
         Files scanned:      {}\n\
         Artifacts inserted:  {}\n\
         Edges created:       {}\n\
         Errors skipped:      {}\n",
        summary.files_scanned,
        summary.artifacts_inserted,
        summary.edges_created,
        summary.errors_skipped,
    );
}

/// Benchmark: single file upsert.
fn bench_single_upsert(c: &mut Criterion) {
    let orqa = orqa_dir();
    let Some(leaf) = find_leaf_artifact() else {
        eprintln!("SKIP: no leaf artifact found for single_upsert benchmark");
        return;
    };

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Pre-ingest the full graph so the upsert has context.
    let db = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap();
        db
    });

    c.bench_function("single_upsert", |b| {
        b.iter(|| rt.block_on(async { ingest_single_file(&db, &leaf, &orqa).await.unwrap() }));
    });
}

/// Benchmark: traceability query (trace from a leaf task to pillars).
fn bench_traceability(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    let db = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap();
        db
    });

    #[derive(Clone, SurrealValue)]
    struct Aid {
        aid: String,
    }

    // Find a task artifact ID to trace from.
    let task_id = rt.block_on(async {
        let mut response = db
            .db
            .query("SELECT meta::id(id) AS aid FROM artifact WHERE artifact_type = 'task' LIMIT 1;")
            .await
            .unwrap();

        let results: Vec<Aid> = response.take(0).unwrap_or_default();
        results.into_iter().next().map(|a| a.aid)
    });

    let Some(task_id) = task_id else {
        eprintln!("SKIP: no task artifact found for traceability benchmark");
        return;
    };

    eprintln!("Tracing from: {task_id}");

    c.bench_function("traceability", |b| {
        b.iter(|| {
            rt.block_on(async {
                queries::trace_to_pillars(&db, &task_id, &["pillar", "vision"])
                    .await
                    .unwrap()
            })
        });
    });
}

/// Benchmark: descendants query.
fn bench_descendants(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    let db = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap();
        db
    });

    #[derive(Clone, SurrealValue)]
    struct Aid {
        aid: String,
    }

    // Find an epic to query descendants of.
    let epic_id = rt.block_on(async {
        let mut response = db
            .db
            .query("SELECT meta::id(id) AS aid FROM artifact WHERE artifact_type = 'epic' LIMIT 1;")
            .await
            .unwrap();

        let results: Vec<Aid> = response.take(0).unwrap_or_default();
        results.into_iter().next().map(|a| a.aid)
    });

    let Some(epic_id) = epic_id else {
        eprintln!("SKIP: no epic found for descendants benchmark");
        return;
    };

    eprintln!("Descendants of: {epic_id}");

    c.bench_function("descendants", |b| {
        b.iter(|| rt.block_on(async { queries::trace_descendants(&db, &epic_id).await.unwrap() }));
    });
}

/// Benchmark: find all orphan artifacts.
fn bench_orphans(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    let db = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap();
        db
    });

    c.bench_function("orphans", |b| {
        b.iter(|| rt.block_on(async { queries::find_orphans(&db).await.unwrap() }));
    });
}

/// Benchmark: combined health metrics (avg degree + orphans + type counts).
fn bench_health_metrics(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    let db = rt.block_on(async {
        let db = GraphDb::open_memory().await.unwrap();
        ingest_directory(&db, &orqa).await.unwrap();
        db
    });

    c.bench_function("health_metrics", |b| {
        b.iter(|| {
            rt.block_on(async {
                let avg = queries::avg_degree(&db).await.unwrap();
                let orphans = queries::find_orphans(&db).await.unwrap();
                let types = queries::count_by_type(&db).await.unwrap();
                let statuses = queries::count_by_status(&db).await.unwrap();
                (avg, orphans.len(), types.len(), statuses.len())
            })
        });
    });

    // Print the actual metric values for findings.
    let (avg, orphan_count, type_count, status_count, total_artifacts, total_edges) =
        rt.block_on(async {
            let avg = queries::avg_degree(&db).await.unwrap();
            let orphans = queries::find_orphans(&db).await.unwrap();
            let types = queries::count_by_type(&db).await.unwrap();
            let statuses = queries::count_by_status(&db).await.unwrap();
            let total_a = queries::total_artifacts(&db).await.unwrap();
            let total_e = queries::total_edges(&db).await.unwrap();
            (
                avg,
                orphans.len(),
                types.len(),
                statuses.len(),
                total_a,
                total_e,
            )
        });

    eprintln!(
        "\n=== HEALTH METRICS ===\n\
         Total artifacts:  {total_artifacts}\n\
         Total edges:      {total_edges}\n\
         Avg degree:       {avg:.2}\n\
         Orphan count:     {orphan_count}\n\
         Type groups:      {type_count}\n\
         Status groups:    {status_count}\n"
    );
}

/// Benchmark: cold start — open embedded DB and count artifacts.
///
/// SurrealKV holds a file lock, so we cannot re-open the same path across
/// iterations. Instead, we measure a single cold-start cycle (open + schema
/// + ingest + query) to get a representative timing.
fn bench_cold_start(c: &mut Criterion) {
    let orqa = orqa_dir();
    if !orqa.exists() {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("cold_start_embedded", |b| {
        b.iter(|| {
            rt.block_on(async {
                let tmp = tempfile::tempdir().unwrap();
                let db_path = tmp.path().join("graph.surrealkv");
                let db = GraphDb::open_embedded(&db_path).await.unwrap();
                let summary = ingest_directory(&db, &orqa).await.unwrap();
                let count = queries::total_artifacts(&db).await.unwrap();
                (summary.artifacts_inserted, count)
            })
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_cold_start,
        bench_full_ingest,
        bench_single_upsert,
        bench_traceability,
        bench_descendants,
        bench_orphans,
        bench_health_metrics
}

criterion_main!(benches);
