// Log events repository for orqa-storage.
//
// Provides batch insert, query, and retention-purge operations over the
// `log_events` table. Previously this data lived in the daemon's separate
// events.db. Now it's part of the unified orqa.db. The batch insert pattern
// is retained from daemon/src/event_store.rs for high-throughput event ingestion.

use rusqlite::params;

use orqa_engine_types::types::event::LogEvent;

use crate::error::StorageError;
use crate::Storage;

/// Milliseconds per day, used for retention cutoff arithmetic.
const MS_PER_DAY: i64 = 86_400_000;

/// Filter parameters for `EventRepo::query_sync`.
///
/// All fields are optional. Absent fields are not applied as constraints.
#[derive(Debug, Clone, Default)]
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

/// Zero-cost repository handle for the `log_events` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::events()`.
pub struct EventRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl EventRepo<'_> {
    /// Insert a batch of events in a single SQLite transaction.
    ///
    /// Duplicate IDs are silently ignored (`INSERT OR IGNORE`). This is safe
    /// because the daemon assigns monotonically increasing IDs via an atomic
    /// counter. Must be called from a blocking context.
    pub fn insert_batch(&self, events: Vec<LogEvent>) -> Result<(), StorageError> {
        if events.is_empty() {
            return Ok(());
        }
        let conn = self.storage.conn()?;
        let tx = conn.unchecked_transaction()?;
        {
            let mut stmt = tx
                .prepare_cached(
                    "INSERT OR IGNORE INTO log_events
                     (id, timestamp, level, source, category, message, metadata, session_id)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                )
                .map_err(|e| StorageError::Database(e.to_string()))?;
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
                ])
                .map_err(|e| StorageError::Database(e.to_string()))?;
            }
        }
        tx.commit()
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Return stored events matching `filter`, ordered by timestamp ascending.
    ///
    /// Result count is capped at `filter.limit` (default 500, max 5000).
    pub fn query(&self, filter: &EventFilter) -> Result<Vec<serde_json::Value>, StorageError> {
        let conn = self.storage.conn()?;
        let limit = filter.limit.unwrap_or(500).clamp(1, 5000);
        let after = filter.after.unwrap_or(0);

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

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| StorageError::Database(e.to_string()))?;
        let rows: Vec<serde_json::Value> = match (&filter.source, &filter.level) {
            (Some(src), Some(lvl)) => stmt
                .query_map(params![after, src, lvl], row_to_json)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (Some(src), None) => stmt
                .query_map(params![after, src], row_to_json)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (None, Some(lvl)) => stmt
                .query_map(params![after, lvl], row_to_json)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
            (None, None) => stmt
                .query_map(params![after], row_to_json)
                .map_err(|e| StorageError::Database(e.to_string()))?
                .filter_map(Result::ok)
                .collect(),
        };

        Ok(rows)
    }

    /// Delete all events with `timestamp < (now_ms - retention_days * MS_PER_DAY)`.
    ///
    /// Returns the number of rows deleted.
    pub fn purge(&self, retention_days: u32) -> Result<usize, StorageError> {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        let cutoff = now_ms - (retention_days as i64 * MS_PER_DAY);
        let conn = self.storage.conn()?;
        let deleted = conn.execute(
            "DELETE FROM log_events WHERE timestamp < ?1",
            params![cutoff],
        )?;
        Ok(deleted)
    }
}

/// Convert a SQLite row to a `serde_json::Value` for HTTP transport.
///
/// The metadata column is stored as a JSON string and is parsed back so the
/// HTTP response contains structured JSON rather than an escaped string.
fn row_to_json(row: &rusqlite::Row<'_>) -> rusqlite::Result<serde_json::Value> {
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

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::event::{EventLevel, EventSource};

    use super::*;
    use crate::Storage;

    fn open_storage() -> Storage {
        Storage::open_in_memory().expect("in-memory storage")
    }

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
            fingerprint: None,
            message_template: None,
            correlation_id: None,
            stack_frames: None,
        }
    }

    fn event(id: u64) -> LogEvent {
        make_event(
            id,
            EventSource::Daemon,
            EventLevel::Info,
            &format!("msg-{id}"),
        )
    }

    #[test]
    fn insert_batch_makes_events_queryable() {
        let storage = open_storage();
        storage
            .events()
            .insert_batch(vec![event(1), event(2), event(3)])
            .expect("insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .expect("query");
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn insert_batch_empty_is_noop() {
        let storage = open_storage();
        storage.events().insert_batch(vec![]).expect("empty insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .expect("query");
        assert!(rows.is_empty());
    }

    #[test]
    fn duplicate_id_is_silently_ignored() {
        let storage = open_storage();
        storage
            .events()
            .insert_batch(vec![event(99)])
            .expect("first insert");
        let duplicate = LogEvent {
            id: 99,
            timestamp: 9_999_999,
            level: EventLevel::Error,
            source: EventSource::MCP,
            category: "dupe".to_owned(),
            message: "should not appear".to_owned(),
            metadata: serde_json::Value::Null,
            session_id: None,
            fingerprint: None,
            message_template: None,
            correlation_id: None,
            stack_frames: None,
        };
        storage
            .events()
            .insert_batch(vec![duplicate])
            .expect("duplicate insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .expect("query");
        assert_eq!(rows.len(), 1, "duplicate must not create a second row");
        assert_eq!(rows[0]["message"].as_str().unwrap(), "msg-99");
    }

    #[test]
    fn query_source_filter() {
        let storage = open_storage();
        storage
            .events()
            .insert_batch(vec![
                make_event(1, EventSource::Daemon, EventLevel::Info, "daemon"),
                make_event(2, EventSource::MCP, EventLevel::Info, "mcp"),
                make_event(3, EventSource::Daemon, EventLevel::Info, "daemon2"),
            ])
            .expect("insert");

        let rows = storage
            .events()
            .query(&EventFilter {
                source: Some("daemon".to_owned()),
                ..Default::default()
            })
            .expect("query");
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn query_limit_caps_results() {
        let storage = open_storage();
        let events: Vec<LogEvent> = (1..=20).map(event).collect();
        storage.events().insert_batch(events).expect("insert");
        let rows = storage
            .events()
            .query(&EventFilter {
                limit: Some(5),
                ..Default::default()
            })
            .expect("query");
        assert_eq!(rows.len(), 5);
    }
}
