// Devtools repository for orqa-storage.
//
// Provides session lifecycle and event query operations over the
// `devtools_sessions` and `devtools_events` tables. Previously this data lived
// in the devtools-sessions.db. Now it's part of the unified orqa.db. Logic is
// ported from devtools/src-tauri/src/session_db.rs.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, TransactionTrait};
use serde::{Deserialize, Serialize};

use orqa_engine_types::types::event::LogEvent;

use crate::error::StorageError;
use crate::traits::DevtoolsRepository;

/// Summary of a devtools session returned by `DevtoolsRepo::list_sessions`.
#[derive(Debug, Clone, Serialize)]
pub struct DevtoolsSessionSummary {
    /// UUID v4 session identifier.
    pub id: String,
    /// Unix timestamp in milliseconds when the session started.
    pub started_at: i64,
    /// Unix timestamp in milliseconds when the session ended, or `None` if active.
    pub ended_at: Option<i64>,
    /// User-editable label; `None` means show an auto-generated name.
    pub label: Option<String>,
    /// Denormalized count of events in this session.
    pub event_count: u64,
    /// True when this is the currently active session.
    pub is_current: bool,
}

/// Metadata for the currently active devtools session.
#[derive(Debug, Clone, Serialize)]
pub struct DevtoolsSessionInfo {
    /// UUID v4 session identifier.
    pub id: String,
    /// Unix timestamp in milliseconds when the session started.
    pub started_at: i64,
    /// User-editable label; `None` means show an auto-generated name.
    pub label: Option<String>,
    /// Denormalized count of events in this session.
    pub event_count: u64,
}

/// Query parameters for `DevtoolsRepo::query_events`.
#[derive(Debug, Clone, Deserialize)]
pub struct DevtoolsEventQuery {
    /// UUID of the session whose events to query.
    pub session_id: String,
    /// Zero-based row offset for pagination.
    pub offset: Option<usize>,
    /// Maximum rows to return (capped at 5000).
    pub limit: Option<usize>,
    /// Optional exact-match filter on the `source` column.
    pub source: Option<String>,
    /// Optional exact-match filter on the `level` column.
    pub level: Option<String>,
    /// Optional substring filter on the `category` column.
    pub category: Option<String>,
    /// Optional substring filter on the `message` column.
    pub search_text: Option<String>,
}

/// Paginated response from `DevtoolsRepo::query_events`.
#[derive(Debug, Clone, Serialize)]
pub struct DevtoolsEventQueryResponse {
    /// Matching events as JSON values.
    pub events: Vec<serde_json::Value>,
    /// Total number of matching events (before paging).
    pub total: usize,
}

/// Async repository handle for devtools tables.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::devtools()`.
pub struct DevtoolsRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Return the current Unix timestamp in milliseconds.
fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Map a SeaORM `QueryResult` row from `devtools_sessions` to a `DevtoolsSessionSummary`.
fn map_session_summary(
    row: &sea_orm::QueryResult,
    current_session_id: &str,
) -> Result<DevtoolsSessionSummary, StorageError> {
    let id: String = row
        .try_get("", "id")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let is_current = id == current_session_id;
    let event_count_raw: i64 = row
        .try_get("", "event_count")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(DevtoolsSessionSummary {
        is_current,
        id,
        started_at: row
            .try_get("", "started_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        ended_at: row
            .try_get("", "ended_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        label: row
            .try_get("", "label")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        event_count: event_count_raw as u64,
    })
}

/// Map a SeaORM `QueryResult` row from `devtools_sessions` to a `DevtoolsSessionInfo`.
fn map_session_info(row: &sea_orm::QueryResult) -> Result<DevtoolsSessionInfo, StorageError> {
    let event_count_raw: i64 = row
        .try_get("", "event_count")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(DevtoolsSessionInfo {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        started_at: row
            .try_get("", "started_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        label: row
            .try_get("", "label")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        event_count: event_count_raw as u64,
    })
}

/// Map a SeaORM `QueryResult` row from `devtools_events` to a `serde_json::Value`.
fn map_event_row(row: &sea_orm::QueryResult) -> Result<serde_json::Value, StorageError> {
    let rowid: i64 = row
        .try_get("", "rowid")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let original_id: i64 = row
        .try_get("", "original_id")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let session_id: String = row
        .try_get("", "session_id")
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
    let tier: String = row
        .try_get("", "tier")
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
    let daemon_event_id: Option<i64> = row
        .try_get("", "daemon_event_id")
        .map_err(|e| StorageError::Database(e.to_string()))?;

    let metadata: serde_json::Value =
        serde_json::from_str(&metadata_str).unwrap_or(serde_json::Value::Null);

    Ok(serde_json::json!({
        "rowid":           rowid,
        "id":              original_id,
        "session_id":      session_id,
        "timestamp":       timestamp,
        "level":           level,
        "source":          source,
        "tier":            tier,
        "category":        category,
        "message":         message,
        "metadata":        metadata,
        "daemon_event_id": daemon_event_id,
    }))
}

#[async_trait::async_trait]
#[allow(clippy::too_many_lines)]
impl DevtoolsRepository for DevtoolsRepo {
    /// Create a new devtools session with the given UUID and start timestamp.
    async fn create_session(&self, session_id: &str, started_at: i64) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO devtools_sessions (id, started_at) VALUES (?, ?)",
                [session_id.into(), started_at.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Mark any sessions with `ended_at IS NULL` as interrupted (set ended_at = started_at).
    ///
    /// Called on startup before creating the new session so crashed sessions
    /// display as interrupted in the session list.
    async fn mark_orphaned_sessions_interrupted(&self) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_string(
                DbBackend::Sqlite,
                "UPDATE devtools_sessions SET ended_at = started_at WHERE ended_at IS NULL"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Mark a session as ended by setting its `ended_at` timestamp.
    async fn end_session(&self, session_id: &str, ended_at: i64) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE devtools_sessions SET ended_at = ? WHERE id = ?",
                [ended_at.into(), session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Insert a batch of events for the given session in a single transaction.
    ///
    /// Also increments the denormalized `event_count` on the session row.
    async fn insert_events(
        &self,
        session_id: &str,
        events: Vec<LogEvent>,
    ) -> Result<(), StorageError> {
        if events.is_empty() {
            return Ok(());
        }

        let count = events.len() as i64;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        for event in &events {
            txn.execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO devtools_events \
                 (original_id, session_id, timestamp, level, source, tier, \
                  category, message, metadata, daemon_event_id) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    (event.id as i64).into(),
                    session_id.into(),
                    event.timestamp.into(),
                    format!("{:?}", event.level).into(),
                    format!("{}", event.source).into(),
                    format!("{}", event.tier).into(),
                    event.category.clone().into(),
                    event.message.clone().into(),
                    event.metadata.to_string().into(),
                    (event.id as i64).into(),
                ],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        }

        // Increment the denormalized event counter on the session row.
        txn.execute_raw(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "UPDATE devtools_sessions SET event_count = event_count + ? WHERE id = ?",
            [count.into(), session_id.into()],
        ))
        .await
        .map_err(|e| StorageError::Database(e.to_string()))?;

        txn.commit()
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(())
    }

    /// List all devtools sessions ordered by `started_at DESC`.
    ///
    /// `current_session_id` is compared against each session id to set
    /// `is_current` on the result.
    async fn list_sessions(
        &self,
        current_session_id: &str,
    ) -> Result<Vec<DevtoolsSessionSummary>, StorageError> {
        let rows = self
            .db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, started_at, ended_at, label, event_count \
                 FROM devtools_sessions \
                 ORDER BY started_at DESC"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter()
            .map(|row| map_session_summary(row, current_session_id))
            .collect()
    }

    /// Get metadata for a specific session by id.
    async fn get_session(&self, session_id: &str) -> Result<DevtoolsSessionInfo, StorageError> {
        let row = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, started_at, label, event_count \
                 FROM devtools_sessions WHERE id = ?",
                [session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| StorageError::NotFound(format!("devtools session {session_id}")))?;
        map_session_info(&row)
    }

    /// Update the user-editable label for a session.
    async fn rename_session(&self, session_id: &str, label: &str) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE devtools_sessions SET label = ? WHERE id = ?",
                [label.into(), session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Delete a session and cascade its events.
    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError> {
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM devtools_sessions WHERE id = ?",
                [session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// Delete sessions (and their cascaded events) older than `retention_days`.
    ///
    /// Returns the number of sessions deleted.
    async fn purge_old_sessions(&self, retention_days: u32) -> Result<usize, StorageError> {
        let cutoff = now_ms() - (retention_days as i64 * 86_400_000);
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM devtools_sessions WHERE started_at < ?",
                [cutoff.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(result.rows_affected() as usize)
    }

    /// Return paginated and filtered events for a session.
    async fn query_events(
        &self,
        query: &DevtoolsEventQuery,
    ) -> Result<DevtoolsEventQueryResponse, StorageError> {
        // Build WHERE clause dynamically from optional filters.
        // Bind values are collected in order. session_id is always first.
        let mut conditions = vec!["session_id = ?".to_owned()];
        let mut bind_values: Vec<sea_orm::Value> = vec![query.session_id.as_str().into()];

        if let Some(ref source) = query.source {
            conditions.push("source = ?".to_owned());
            bind_values.push(source.as_str().into());
        }
        if let Some(ref level) = query.level {
            conditions.push("level = ?".to_owned());
            bind_values.push(level.as_str().into());
        }
        if let Some(ref category) = query.category {
            conditions.push("category LIKE ?".to_owned());
            bind_values.push(format!("%{category}%").into());
        }
        if let Some(ref search) = query.search_text {
            conditions.push("message LIKE ?".to_owned());
            bind_values.push(format!("%{search}%").into());
        }

        let where_clause = conditions.join(" AND ");

        // Count total matching rows for pagination metadata.
        let count_sql = format!("SELECT COUNT(*) AS cnt FROM devtools_events WHERE {where_clause}");
        let count_row = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &count_sql,
                bind_values.clone(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| StorageError::Database("COUNT query returned no row".to_owned()))?;
        let total: usize = {
            let n: i64 = count_row
                .try_get("", "cnt")
                .map_err(|e| StorageError::Database(e.to_string()))?;
            n as usize
        };

        // Fetch the page.
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(1000).min(5000);
        let data_sql = format!(
            "SELECT rowid, original_id, session_id, timestamp, level, source, tier, \
                    category, message, metadata, daemon_event_id \
             FROM devtools_events \
             WHERE {where_clause} \
             ORDER BY timestamp ASC \
             LIMIT {limit} OFFSET {offset}"
        );

        let rows = self
            .db
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &data_sql,
                bind_values,
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let events: Vec<serde_json::Value> = rows
            .iter()
            .map(map_event_row)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DevtoolsEventQueryResponse { events, total })
    }
}

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent};

    use super::*;
    use crate::traits::DevtoolsRepository;
    use crate::Storage;

    async fn open_storage() -> Storage {
        Storage::open_in_memory().await.expect("in-memory storage")
    }

    fn make_event(id: u64) -> LogEvent {
        LogEvent {
            id,
            timestamp: 1_000_000 + id as i64,
            level: EventLevel::Info,
            source: EventSource::Daemon,
            tier: EventTier::default(),
            category: "test".to_owned(),
            message: format!("msg-{id}"),
            metadata: serde_json::Value::Null,
            session_id: None,
            fingerprint: None,
            message_template: None,
            correlation_id: None,
            stack_frames: None,
        }
    }

    #[tokio::test]
    async fn create_session_and_list() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000)
            .await
            .expect("create");

        let sessions = repo.list_sessions("sess-001").await.expect("list");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "sess-001");
        assert!(sessions[0].is_current);
    }

    #[tokio::test]
    async fn insert_events_increments_count() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000)
            .await
            .expect("create");
        repo.insert_events("sess-001", vec![make_event(1), make_event(2)])
            .await
            .expect("insert");

        let info = repo.get_session("sess-001").await.expect("get");
        assert_eq!(info.event_count, 2);
    }

    #[tokio::test]
    async fn end_session_sets_ended_at() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000)
            .await
            .expect("create");
        repo.end_session("sess-001", 2_000_000).await.expect("end");

        let sessions = repo.list_sessions("other").await.expect("list");
        assert_eq!(sessions[0].ended_at, Some(2_000_000));
    }

    #[tokio::test]
    async fn mark_orphaned_sessions() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("orphan", 1_000_000)
            .await
            .expect("create");
        repo.mark_orphaned_sessions_interrupted()
            .await
            .expect("mark");

        let sessions = repo.list_sessions("other").await.expect("list");
        assert_eq!(
            sessions[0].ended_at,
            Some(1_000_000),
            "orphaned session should have ended_at = started_at"
        );
    }

    #[tokio::test]
    async fn delete_session_cascades_events() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000)
            .await
            .expect("create");
        repo.insert_events("sess-001", vec![make_event(1)])
            .await
            .expect("insert");
        repo.delete_session("sess-001").await.expect("delete");

        let sessions = repo.list_sessions("other").await.expect("list");
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn query_events_pagination() {
        let storage = open_storage().await;
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000)
            .await
            .expect("create");
        let events: Vec<LogEvent> = (1..=5).map(make_event).collect();
        repo.insert_events("sess-001", events)
            .await
            .expect("insert");

        let resp = repo
            .query_events(&DevtoolsEventQuery {
                session_id: "sess-001".to_owned(),
                offset: Some(0),
                limit: Some(2),
                source: None,
                level: None,
                category: None,
                search_text: None,
            })
            .await
            .expect("query");
        assert_eq!(resp.events.len(), 2);
        assert_eq!(resp.total, 5);
    }
}
