// Log events repository for orqa-storage.
//
// Provides batch insert, query, and retention-purge operations over the
// `log_events` table. Previously this data lived in the daemon's separate
// events.db. Now it's part of the unified orqa.db. The batch insert pattern
// is retained from daemon/src/event_store.rs for high-throughput event ingestion.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, TransactionTrait};

use orqa_engine_types::types::event::LogEvent;

use crate::error::StorageError;
use crate::traits::EventRepository;

/// Milliseconds per day, used for retention cutoff arithmetic.
const MS_PER_DAY: i64 = 86_400_000;

/// Filter parameters for `EventRepo::query`.
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

/// Async repository handle for the `log_events` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::events()`.
pub struct EventRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Convert a SeaORM `QueryResult` row to a `serde_json::Value` for HTTP transport.
///
/// The metadata column is stored as a JSON string and is parsed back so the
/// HTTP response contains structured JSON rather than an escaped string.
fn row_to_json(row: &sea_orm::QueryResult) -> Result<serde_json::Value, StorageError> {
    let id: i64 = row
        .try_get("", "id")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let timestamp: i64 = row
        .try_get("", "timestamp")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let level: String = row
        .try_get("", "level")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let source: String = row
        .try_get("", "source")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let category: String = row
        .try_get("", "category")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let message: String = row
        .try_get("", "message")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let metadata_str: String = row
        .try_get("", "metadata")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let session_id: Option<String> = row
        .try_get("", "session_id")
        .map_err(|e| StorageError::Database(e.to_string()))?;

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

#[async_trait::async_trait]
impl EventRepository for EventRepo {
    /// Insert a batch of events in a single SQLite transaction.
    ///
    /// Duplicate IDs are silently ignored (`INSERT OR IGNORE`).
    async fn insert_batch(&self, events: Vec<LogEvent>) -> Result<(), StorageError> {
        if events.is_empty() {
            return Ok(());
        }

        let txn = self
            .db
            .begin()
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        for event in &events {
            let session_val: sea_orm::Value = match &event.session_id {
                Some(s) => s.as_str().into(),
                None => sea_orm::Value::String(None),
            };
            txn.execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT OR IGNORE INTO log_events \
                 (id, timestamp, level, source, category, message, metadata, session_id) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    (event.id as i64).into(),
                    event.timestamp.into(),
                    format!("{:?}", event.level).into(),
                    format!("{}", event.source).into(),
                    event.category.clone().into(),
                    event.message.clone().into(),
                    event.metadata.to_string().into(),
                    session_val,
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        }

        txn.commit()
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(())
    }

    /// Return stored events matching `filter`, ordered by timestamp ascending.
    ///
    /// Result count is capped at `filter.limit` (default 500, max 5000).
    async fn query(&self, filter: &EventFilter) -> Result<Vec<serde_json::Value>, StorageError> {
        let limit = filter.limit.unwrap_or(500).clamp(1, 5000);
        let after = filter.after.unwrap_or(0);

        // Build WHERE clause and value list from optional filter fields.
        // Use positional `?` placeholders — SeaORM will bind them in order.
        let mut conditions = vec!["timestamp >= ?".to_owned()];
        let mut values: Vec<sea_orm::Value> = vec![after.into()];

        if let Some(ref src) = filter.source {
            conditions.push("source = ?".to_owned());
            values.push(src.as_str().into());
        }
        if let Some(ref lvl) = filter.level {
            conditions.push("level = ?".to_owned());
            values.push(lvl.as_str().into());
        }

        let where_clause = conditions.join(" AND ");
        let sql = format!(
            "SELECT id, timestamp, level, source, category, message, metadata, session_id \
             FROM log_events WHERE {where_clause} \
             ORDER BY timestamp ASC LIMIT {limit}"
        );

        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &sql,
                values,
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter().map(row_to_json).collect()
    }

    /// Delete all events with `timestamp < (now_ms - retention_days * MS_PER_DAY)`.
    ///
    /// Returns the number of rows deleted.
    async fn purge(&self, retention_days: u32) -> Result<usize, StorageError> {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        let cutoff = now_ms - (retention_days as i64 * MS_PER_DAY);

        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM log_events WHERE timestamp < ?",
                [cutoff.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }
}

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier};

    use super::*;
    use crate::traits::EventRepository;
    use crate::Storage;

    async fn open_storage() -> Storage {
        Storage::open_in_memory().await.expect("in-memory storage")
    }

    fn make_event(id: u64, source: EventSource, level: EventLevel, message: &str) -> LogEvent {
        LogEvent {
            id,
            timestamp: 1_000_000 + id as i64,
            level,
            source,
            tier: EventTier::default(),
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

    #[tokio::test]
    async fn insert_batch_makes_events_queryable() {
        let storage = open_storage().await;
        storage
            .events()
            .insert_batch(vec![event(1), event(2), event(3)])
            .await
            .expect("insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .await
            .expect("query");
        assert_eq!(rows.len(), 3);
    }

    #[tokio::test]
    async fn insert_batch_empty_is_noop() {
        let storage = open_storage().await;
        storage
            .events()
            .insert_batch(vec![])
            .await
            .expect("empty insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .await
            .expect("query");
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn duplicate_id_is_silently_ignored() {
        let storage = open_storage().await;
        storage
            .events()
            .insert_batch(vec![event(99)])
            .await
            .expect("first insert");
        let duplicate = LogEvent {
            id: 99,
            timestamp: 9_999_999,
            level: EventLevel::Error,
            source: EventSource::MCP,
            tier: EventTier::default(),
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
            .await
            .expect("duplicate insert");
        let rows = storage
            .events()
            .query(&EventFilter::default())
            .await
            .expect("query");
        assert_eq!(rows.len(), 1, "duplicate must not create a second row");
        assert_eq!(rows[0]["message"].as_str().unwrap(), "msg-99");
    }

    #[tokio::test]
    async fn query_source_filter() {
        let storage = open_storage().await;
        storage
            .events()
            .insert_batch(vec![
                make_event(1, EventSource::Daemon, EventLevel::Info, "daemon"),
                make_event(2, EventSource::MCP, EventLevel::Info, "mcp"),
                make_event(3, EventSource::Daemon, EventLevel::Info, "daemon2"),
            ])
            .await
            .expect("insert");

        let rows = storage
            .events()
            .query(&EventFilter {
                source: Some("daemon".to_owned()),
                ..Default::default()
            })
            .await
            .expect("query");
        assert_eq!(rows.len(), 2);
    }

    #[tokio::test]
    async fn query_limit_caps_results() {
        let storage = open_storage().await;
        let events: Vec<LogEvent> = (1..=20).map(event).collect();
        storage.events().insert_batch(events).await.expect("insert");
        let rows = storage
            .events()
            .query(&EventFilter {
                limit: Some(5),
                ..Default::default()
            })
            .await
            .expect("query");
        assert_eq!(rows.len(), 5);
    }
}
