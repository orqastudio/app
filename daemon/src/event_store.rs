//! SQLite-backed persistent store for daemon log events.
//!
//! Subscribes to the central `EventBus` and batches incoming `LogEvent`
//! values into `.state/events.db`. Batches flush when they reach 100 events
//! or after 500 ms, whichever comes first. On startup, events older than 7
//! days are purged automatically. The store also exposes query support so the
//! HTTP `GET /events` endpoint can replay stored events with optional filters.
//!
//! Thread-safety: `rusqlite::Connection` is `!Send`. This module stores only
//! the database path and opens a fresh connection per blocking operation via
//! `tokio::task::spawn_blocking`. SQLite WAL mode allows safe concurrent
//! readers and a serialised writer. `EventStore` itself is `Send + Sync`.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use rusqlite::{Connection, OpenFlags, params};
use serde::Deserialize;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use orqa_engine_types::types::event::LogEvent;

use crate::event_bus::EventBus;

/// Maximum events to accumulate before flushing to SQLite.
const BATCH_SIZE: usize = 100;

/// Maximum time to wait before flushing a non-empty batch.
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);

/// Milliseconds per day, used for retention cutoff arithmetic.
const MS_PER_DAY: i64 = 86_400_000;

/// SQLite-backed event store.
///
/// Stores only the database path. All connections are opened on demand
/// inside `spawn_blocking` closures so the `!Send` `Connection` type
/// never crosses an `await` point or thread boundary. `EventStore` is
/// `Send + Sync` and can be shared safely via `Arc`.
#[derive(Debug, Clone)]
pub struct EventStore {
    /// Absolute path to the SQLite database file.
    db_path: PathBuf,
}

/// Filter parameters for `EventStore::query_sync`.
///
/// All fields are optional. Absent fields are not applied as constraints.
#[derive(Debug, Clone, Deserialize)]
pub struct EventFilter {
    /// Return only events from this source (e.g. "daemon", "mcp").
    pub source: Option<String>,
    /// Return only events at this level (e.g. "Error", "Warn").
    pub level: Option<String>,
    /// Return only events with `timestamp >= after` (Unix ms).
    pub after: Option<i64>,
    /// Maximum number of rows to return (default 500, max 5000).
    pub limit: Option<i64>,
}

/// Open a connection to `db_path` with WAL mode for concurrent access.
fn open_conn(db_path: &Path) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
    Ok(conn)
}

/// Create the `log_events` table and indexes if they do not already exist.
fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS log_events (
            id          INTEGER PRIMARY KEY,
            timestamp   INTEGER NOT NULL,
            level       TEXT    NOT NULL,
            source      TEXT    NOT NULL,
            category    TEXT    NOT NULL,
            message     TEXT    NOT NULL,
            metadata    TEXT    NOT NULL,
            session_id  TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_timestamp ON log_events(timestamp);
        CREATE INDEX IF NOT EXISTS idx_source    ON log_events(source);
        CREATE INDEX IF NOT EXISTS idx_level     ON log_events(level);",
    )
}

impl EventStore {
    /// Open (or create) `.state/events.db` inside `project_root`, initialise
    /// the schema, purge events older than `retention_days` days, and return the store.
    ///
    /// `retention_days` controls how long events are kept. Events older than this
    /// threshold are deleted immediately on open and again every 6 hours by the
    /// periodic purge task spawned in `main.rs`.
    ///
    /// Returns an error if the database cannot be opened or schema migration
    /// fails — the caller should degrade gracefully.
    pub fn open(
        project_root: &Path,
        retention_days: u32,
    ) -> Result<Arc<Self>, Box<dyn std::error::Error + Send + Sync>> {
        let state_dir = project_root.join(".state");
        std::fs::create_dir_all(&state_dir)?;

        let db_path = state_dir.join("events.db");

        // Initialise schema synchronously at startup on the calling thread.
        let conn = open_conn(&db_path)?;
        init_schema(&conn)?;
        drop(conn);

        let store = Arc::new(Self {
            db_path: db_path.clone(),
        });

        // Purge stale events immediately so we start with a clean window.
        store.purge_sync(retention_days as i64 * MS_PER_DAY, retention_days);

        info!(
            subsystem = "event-store",
            path = %db_path.display(),
            retention_days,
            "[event-store] opened"
        );

        Ok(store)
    }

    /// Insert a batch of events in a single SQLite transaction.
    ///
    /// Opens a fresh connection, wraps all inserts in one transaction to
    /// amortise fsync overhead, then commits. Must be called from a blocking
    /// context — use `spawn_blocking` from async callers.
    pub fn insert_batch_sync(&self, events: Vec<LogEvent>) {
        if events.is_empty() {
            return;
        }
        let result = (|| -> Result<(), rusqlite::Error> {
            let conn = open_conn(&self.db_path)?;
            let tx = conn.unchecked_transaction()?;
            {
                let mut stmt = tx.prepare_cached(
                    "INSERT OR IGNORE INTO log_events
                     (id, timestamp, level, source, category, message, metadata, session_id)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                )?;
                for event in &events {
                    stmt.execute(params![
                        event.id as i64,
                        event.timestamp,
                        format!("{:?}", event.level),
                        format!("{}", event.source),
                        event.category,
                        event.message,
                        event.metadata.to_string(),
                        event.session_id,
                    ])?;
                }
            }
            tx.commit()?;
            Ok(())
        })();

        if let Err(e) = result {
            error!(
                subsystem = "event-store",
                error = %e,
                count = events.len(),
                "[event-store] batch insert failed"
            );
        }
    }

    /// Return stored events matching `filter`, ordered by timestamp ascending.
    ///
    /// Caps results at `filter.limit` (default 500, max 5000). Must be called
    /// from a blocking context — use `spawn_blocking` from async callers.
    pub fn query_sync(&self, filter: &EventFilter) -> Vec<serde_json::Value> {
        let limit = filter.limit.unwrap_or(500).clamp(1, 5000);
        let after = filter.after.unwrap_or(0);

        let result = (|| -> Result<Vec<serde_json::Value>, rusqlite::Error> {
            let conn = open_conn(&self.db_path)?;

            // Build WHERE clause from optional filter fields.
            let mut conditions = vec!["timestamp >= ?1".to_owned()];
            if filter.source.is_some() {
                conditions.push("source = ?2".to_owned());
            }
            if filter.level.is_some() {
                let idx = if filter.source.is_some() { 3 } else { 2 };
                conditions.push(format!("level = ?{idx}"));
            }

            let where_clause = conditions.join(" AND ");
            let sql = format!(
                "SELECT id, timestamp, level, source, category, message, metadata, session_id
                 FROM log_events WHERE {where_clause}
                 ORDER BY timestamp ASC LIMIT {limit}"
            );

            let mut stmt = conn.prepare(&sql)?;
            let rows: Vec<serde_json::Value> = match (&filter.source, &filter.level) {
                (Some(src), Some(lvl)) => stmt
                    .query_map(params![after, src, lvl], row_to_json)?
                    .filter_map(Result::ok)
                    .collect(),
                (Some(src), None) => stmt
                    .query_map(params![after, src], row_to_json)?
                    .filter_map(Result::ok)
                    .collect(),
                (None, Some(lvl)) => stmt
                    .query_map(params![after, lvl], row_to_json)?
                    .filter_map(Result::ok)
                    .collect(),
                (None, None) => stmt
                    .query_map(params![after], row_to_json)?
                    .filter_map(Result::ok)
                    .collect(),
            };

            Ok(rows)
        })();

        match result {
            Ok(rows) => rows,
            Err(e) => {
                error!(subsystem = "event-store", error = %e, "[event-store] query failed");
                Vec::new()
            }
        }
    }

    /// Delete all events with `timestamp < (now - age_ms)` (Unix ms).
    ///
    /// `retention_days` is used only for the log message — it must equal `age_ms / MS_PER_DAY`.
    /// Must be called from a blocking context — use `purge` from async callers.
    pub fn purge_sync(&self, age_ms: i64, retention_days: u32) {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        let cutoff = now_ms - age_ms;

        let result = (|| -> Result<usize, rusqlite::Error> {
            let conn = open_conn(&self.db_path)?;
            conn.execute("DELETE FROM log_events WHERE timestamp < ?1", params![cutoff])
        })();

        match result {
            Ok(n) if n > 0 => {
                info!(
                    subsystem = "event-store",
                    deleted = n,
                    retention_days,
                    "[event-store] purged {n} events older than {retention_days} days"
                );
            }
            Ok(_) => {}
            Err(e) => {
                error!(subsystem = "event-store", error = %e, "[event-store] purge failed");
            }
        }
    }

    /// Async wrapper around `purge_sync` for use from tokio task contexts.
    ///
    /// Offloads the blocking SQLite delete to `spawn_blocking` so it does not
    /// stall the async runtime. Used by the 6-hour periodic purge task.
    pub async fn purge(store: Arc<Self>, retention_days: u32) {
        let age_ms = retention_days as i64 * MS_PER_DAY;
        let result = tokio::task::spawn_blocking(move || {
            store.purge_sync(age_ms, retention_days);
        })
        .await;
        if let Err(e) = result {
            error!(
                subsystem = "event-store",
                error = ?e,
                "[event-store] spawn_blocking panicked during periodic purge"
            );
        }
    }
}

/// Convert a SQLite row into a `serde_json::Value` for HTTP transport.
///
/// Metadata is stored as a JSON string; it is parsed back so the HTTP
/// response contains structured JSON rather than an escaped string.
fn row_to_json(row: &rusqlite::Row<'_>) -> Result<serde_json::Value, rusqlite::Error> {
    let id: i64 = row.get(0)?;
    let timestamp: i64 = row.get(1)?;
    let level: String = row.get(2)?;
    let source: String = row.get(3)?;
    let category: String = row.get(4)?;
    let message: String = row.get(5)?;
    let metadata_str: String = row.get(6)?;
    let session_id: Option<String> = row.get(7)?;

    let metadata: serde_json::Value =
        serde_json::from_str(&metadata_str).unwrap_or(serde_json::Value::Null);

    Ok(serde_json::json!({
        "id":         id,
        "timestamp":  timestamp,
        "level":      level,
        "source":     source,
        "category":   category,
        "message":    message,
        "metadata":   metadata,
        "session_id": session_id,
    }))
}

/// Spawn a background tokio task that drains the event bus into the store.
///
/// Subscribes to `bus`, accumulates events into a local batch, and flushes
/// to SQLite via `spawn_blocking` when the batch reaches `BATCH_SIZE` or
/// `FLUSH_INTERVAL` elapses — whichever comes first.
///
/// Exits automatically when the bus sender is dropped (daemon shutdown),
/// after flushing any remaining events.
pub fn spawn_batch_writer(bus: Arc<EventBus>, store: Arc<EventStore>) {
    tokio::spawn(async move {
        let mut rx: broadcast::Receiver<LogEvent> = bus.subscribe();
        let mut batch: Vec<LogEvent> = Vec::with_capacity(BATCH_SIZE);
        let flush_interval = tokio::time::interval(FLUSH_INTERVAL);
        tokio::pin!(flush_interval);

        loop {
            tokio::select! {
                biased;

                result = rx.recv() => {
                    match result {
                        Ok(event) => {
                            batch.push(event);
                            if batch.len() >= BATCH_SIZE {
                                flush_batch(&store, &mut batch).await;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!(
                                subsystem = "event-store",
                                dropped = n,
                                "[event-store] lagged — {n} events lost from store"
                            );
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            if !batch.is_empty() {
                                flush_batch(&store, &mut batch).await;
                            }
                            info!(
                                subsystem = "event-store",
                                "[event-store] bus closed, writer exiting"
                            );
                            break;
                        }
                    }
                }

                _ = flush_interval.tick() => {
                    if !batch.is_empty() {
                        flush_batch(&store, &mut batch).await;
                    }
                }
            }
        }
    });
}

/// Move `batch` into `spawn_blocking` and flush it to SQLite.
///
/// Clears `batch` after dispatching. The `Arc<EventStore>` clone ensures the
/// store outlives the blocking closure.
async fn flush_batch(store: &Arc<EventStore>, batch: &mut Vec<LogEvent>) {
    let store_clone = Arc::clone(store);
    let events = std::mem::take(batch);
    if let Err(e) = tokio::task::spawn_blocking(move || {
        store_clone.insert_batch_sync(events);
    })
    .await
    {
        error!(
            subsystem = "event-store",
            error = ?e,
            "[event-store] spawn_blocking panicked during flush"
        );
    }
}

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

impl From<EventQuery> for EventFilter {
    fn from(q: EventQuery) -> Self {
        Self {
            source: q.source,
            level: q.level,
            after: q.after,
            limit: q.limit,
        }
    }
}

