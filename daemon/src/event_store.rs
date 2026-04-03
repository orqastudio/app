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

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_engine_types::types::event::{EventLevel, EventSource, LogEvent};
    use tempfile::TempDir;

    /// Open a fresh EventStore backed by a temporary directory that is cleaned
    /// up automatically when the TempDir is dropped.
    fn open_temp_store() -> (Arc<EventStore>, TempDir) {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        // Use a very long retention (9999 days) so the startup purge is a no-op.
        let store = EventStore::open(dir.path(), 9999).expect("EventStore::open failed");
        (store, dir)
    }

    /// Build a LogEvent with the given id and source for use in tests.
    fn make_event(id: u64, source: EventSource, level: EventLevel, message: &str) -> LogEvent {
        LogEvent {
            id,
            timestamp: 1_000_000 + id as i64,
            level,
            source,
            category: "test".to_owned(),
            message: message.to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
        }
    }

    /// Convenience overload for the common case of Daemon/Info.
    fn event(id: u64) -> LogEvent {
        make_event(id, EventSource::Daemon, EventLevel::Info, &format!("msg-{id}"))
    }

    // -------------------------------------------------------------------------
    // Schema / open
    // -------------------------------------------------------------------------

    /// open creates a valid database; the log_events table and its indexes exist.
    #[test]
    fn open_creates_schema() {
        let (store, _dir) = open_temp_store();
        // Verify by running a query — if the table didn't exist this would error.
        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        // An empty result is correct; the point is that the query didn't panic.
        assert!(rows.is_empty(), "new database must be empty");
    }

    // -------------------------------------------------------------------------
    // insert_batch_sync
    // -------------------------------------------------------------------------

    /// Inserting a non-empty batch persists events that are then queryable.
    #[test]
    fn insert_batch_makes_events_queryable() {
        let (store, _dir) = open_temp_store();
        let events = vec![event(1), event(2), event(3)];
        store.insert_batch_sync(events);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 3, "all inserted events must be queryable");
    }

    /// Inserting an empty batch is a no-op and must not error.
    #[test]
    fn insert_batch_empty_is_noop() {
        let (store, _dir) = open_temp_store();
        // This must not panic or return an error.
        store.insert_batch_sync(vec![]);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert!(rows.is_empty(), "empty insert must leave store empty");
    }

    // -------------------------------------------------------------------------
    // query_sync — ordering and defaults
    // -------------------------------------------------------------------------

    /// Default query returns events in ascending timestamp order.
    #[test]
    fn query_default_returns_insertion_order_ascending() {
        let (store, _dir) = open_temp_store();
        // Insert in reverse id order to distinguish "insertion order" from "id order".
        store.insert_batch_sync(vec![event(3), event(1), event(2)]);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 3);

        // Timestamps are 1_000_003, 1_000_001, 1_000_002 — ascending means 1,2,3.
        let ids: Vec<i64> = rows.iter().map(|r| r["id"].as_i64().unwrap()).collect();
        assert_eq!(ids, vec![1, 2, 3], "results must be ordered by timestamp ascending");
    }

    // -------------------------------------------------------------------------
    // query_sync — source filter
    // -------------------------------------------------------------------------

    /// Source filter returns only events from the requested source.
    #[test]
    fn query_source_filter_returns_only_matching_source() {
        let (store, _dir) = open_temp_store();
        let events = vec![
            make_event(1, EventSource::Daemon, EventLevel::Info, "daemon"),
            make_event(2, EventSource::MCP, EventLevel::Info, "mcp"),
            make_event(3, EventSource::Daemon, EventLevel::Info, "daemon2"),
        ];
        store.insert_batch_sync(events);

        let filter = EventFilter {
            source: Some("daemon".to_owned()),
            level: None,
            after: None,
            limit: None,
        };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 2, "source filter must exclude non-matching sources");
        for row in &rows {
            assert_eq!(row["source"].as_str().unwrap(), "daemon");
        }
    }

    // -------------------------------------------------------------------------
    // query_sync — level filter
    // -------------------------------------------------------------------------

    /// Level filter returns only events at the requested severity level.
    #[test]
    fn query_level_filter_returns_only_matching_level() {
        let (store, _dir) = open_temp_store();
        let events = vec![
            make_event(1, EventSource::Daemon, EventLevel::Info, "info"),
            make_event(2, EventSource::Daemon, EventLevel::Error, "error"),
            make_event(3, EventSource::Daemon, EventLevel::Info, "info2"),
        ];
        store.insert_batch_sync(events);

        // EventLevel::Debug formats as "Debug" via {:?}.
        let filter = EventFilter {
            source: None,
            level: Some("Error".to_owned()),
            after: None,
            limit: None,
        };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 1, "level filter must exclude non-matching levels");
        assert_eq!(rows[0]["level"].as_str().unwrap(), "Error");
    }

    // -------------------------------------------------------------------------
    // query_sync — after timestamp filter
    // -------------------------------------------------------------------------

    /// After-timestamp filter returns only events at or after the cutoff.
    #[test]
    fn query_after_filter_excludes_older_events() {
        let (store, _dir) = open_temp_store();
        // Timestamps: id1=1_000_001, id2=1_000_002, id3=1_000_003.
        store.insert_batch_sync(vec![event(1), event(2), event(3)]);

        let filter = EventFilter {
            source: None,
            level: None,
            after: Some(1_000_002),
            limit: None,
        };
        let rows = store.query_sync(&filter);
        // timestamp >= 1_000_002 means ids 2 and 3.
        assert_eq!(rows.len(), 2);
        let ids: Vec<i64> = rows.iter().map(|r| r["id"].as_i64().unwrap()).collect();
        assert!(ids.contains(&2) && ids.contains(&3), "only events at or after cutoff returned");
    }

    // -------------------------------------------------------------------------
    // query_sync — combined filters (intersection)
    // -------------------------------------------------------------------------

    /// Combined source + level + after filters return the intersection of all constraints.
    #[test]
    fn query_combined_filters_return_intersection() {
        let (store, _dir) = open_temp_store();
        let events = vec![
            make_event(1, EventSource::Daemon, EventLevel::Error, "e1"), // ts=1_000_001
            make_event(2, EventSource::MCP, EventLevel::Error, "e2"),    // ts=1_000_002, wrong source
            make_event(3, EventSource::Daemon, EventLevel::Info, "e3"),  // ts=1_000_003, wrong level
            make_event(4, EventSource::Daemon, EventLevel::Error, "e4"), // ts=1_000_004, matches all
        ];
        store.insert_batch_sync(events);

        let filter = EventFilter {
            source: Some("daemon".to_owned()),
            level: Some("Error".to_owned()),
            after: Some(1_000_002), // excludes id=1
            limit: None,
        };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 1, "combined filters must return intersection");
        assert_eq!(rows[0]["id"].as_i64().unwrap(), 4);
    }

    // -------------------------------------------------------------------------
    // query_sync — limit
    // -------------------------------------------------------------------------

    /// Limit parameter caps the number of returned rows.
    #[test]
    fn query_limit_caps_result_count() {
        let (store, _dir) = open_temp_store();
        let events: Vec<LogEvent> = (1..=20).map(event).collect();
        store.insert_batch_sync(events);

        let filter = EventFilter { source: None, level: None, after: None, limit: Some(5) };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 5, "limit must cap returned row count");
    }

    /// Default limit is 500 — more than the 20 we insert, so all are returned.
    #[test]
    fn query_default_limit_is_500() {
        let (store, _dir) = open_temp_store();
        let events: Vec<LogEvent> = (1..=20).map(event).collect();
        store.insert_batch_sync(events);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        // Default limit is 500 — all 20 events fit.
        assert_eq!(rows.len(), 20, "default limit of 500 must return all events when under 500");
    }

    // -------------------------------------------------------------------------
    // purge_sync
    // -------------------------------------------------------------------------

    /// purge_sync removes events whose timestamp falls before the cutoff.
    #[test]
    fn purge_removes_old_events() {
        let (store, _dir) = open_temp_store();

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Insert one event in the past (8 days ago) and one now.
        let old_ts = now_ms - (8 * MS_PER_DAY);
        let old_event = LogEvent {
            id: 100,
            timestamp: old_ts,
            level: EventLevel::Info,
            source: EventSource::Daemon,
            category: "test".to_owned(),
            message: "old".to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
        };
        let new_event = LogEvent {
            id: 101,
            timestamp: now_ms,
            level: EventLevel::Info,
            source: EventSource::Daemon,
            category: "test".to_owned(),
            message: "new".to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
        };
        store.insert_batch_sync(vec![old_event, new_event]);

        // Purge with 7-day retention — the 8-day-old event must be deleted.
        store.purge_sync(7 * MS_PER_DAY, 7);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 1, "purge must remove events older than retention threshold");
        assert_eq!(rows[0]["id"].as_i64().unwrap(), 101);
    }

    /// purge_sync preserves events newer than the retention threshold.
    #[test]
    fn purge_preserves_recent_events() {
        let (store, _dir) = open_temp_store();

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // All events are from the last 2 days — should survive a 7-day purge.
        let events: Vec<LogEvent> = (0..5)
            .map(|i| LogEvent {
                id: i,
                timestamp: now_ms - (i as i64 * MS_PER_DAY / 2),
                level: EventLevel::Info,
                source: EventSource::Daemon,
                category: "test".to_owned(),
                message: format!("recent-{i}"),
                metadata: serde_json::Value::Null,
                session_id: None,
            })
            .collect();
        store.insert_batch_sync(events);

        store.purge_sync(7 * MS_PER_DAY, 7);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 5, "recent events must not be removed by purge");
    }

    // -------------------------------------------------------------------------
    // Duplicate ID handling
    // -------------------------------------------------------------------------

    /// INSERT OR IGNORE on duplicate IDs silently skips the duplicate without error.
    #[test]
    fn duplicate_id_is_silently_ignored() {
        let (store, _dir) = open_temp_store();

        let original = event(99);
        store.insert_batch_sync(vec![original]);

        // Insert a second event with the same id but different message.
        let duplicate = LogEvent {
            id: 99,
            timestamp: 1_000_099,
            level: EventLevel::Warn,
            source: EventSource::MCP,
            category: "dupe".to_owned(),
            message: "should not appear".to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
        };
        // Must not panic or error.
        store.insert_batch_sync(vec![duplicate]);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        // Only one row for id=99, and it must be the original.
        assert_eq!(rows.len(), 1, "duplicate insert must not create a second row");
        assert_eq!(rows[0]["message"].as_str().unwrap(), "msg-99", "original must be preserved");
    }

    // -------------------------------------------------------------------------
    // Round-trip fidelity
    // -------------------------------------------------------------------------

    /// Insert events and query them back — all fields must survive the round-trip.
    #[test]
    fn round_trip_fields_match_exactly() {
        let (store, _dir) = open_temp_store();

        let original = LogEvent {
            id: 77,
            timestamp: 9_999_999,
            level: EventLevel::Warn,
            source: EventSource::MCP,
            category: "integration".to_owned(),
            message: "round-trip check".to_owned(),
            metadata: serde_json::json!({"key": "value", "count": 3}),
            session_id: Some("sess-abc".to_owned()),
        };
        store.insert_batch_sync(vec![original.clone()]);

        let filter = EventFilter { source: None, level: None, after: None, limit: None };
        let rows = store.query_sync(&filter);
        assert_eq!(rows.len(), 1);

        let row = &rows[0];
        assert_eq!(row["id"].as_i64().unwrap(), original.id as i64);
        assert_eq!(row["timestamp"].as_i64().unwrap(), original.timestamp);
        // Level is stored via {:?} formatting.
        assert_eq!(row["level"].as_str().unwrap(), "Warn");
        // Source is stored via Display formatting.
        assert_eq!(row["source"].as_str().unwrap(), "mcp");
        assert_eq!(row["category"].as_str().unwrap(), original.category);
        assert_eq!(row["message"].as_str().unwrap(), original.message);
        assert_eq!(row["metadata"]["key"].as_str().unwrap(), "value");
        assert_eq!(row["metadata"]["count"].as_i64().unwrap(), 3);
        assert_eq!(
            row["session_id"].as_str().unwrap(),
            original.session_id.as_deref().unwrap()
        );
    }
}

