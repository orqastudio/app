// Devtools repository for orqa-storage.
//
// Provides session lifecycle and event query operations over the
// `devtools_sessions` and `devtools_events` tables. Previously this data lived
// in the devtools-sessions.db. Now it's part of the unified orqa.db. Logic is
// ported from devtools/src-tauri/src/session_db.rs.

use rusqlite::params;
use serde::{Deserialize, Serialize};

use orqa_engine_types::types::event::LogEvent;

use crate::error::StorageError;
use crate::Storage;

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

/// Zero-cost repository handle for devtools tables.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::devtools()`.
pub struct DevtoolsRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl DevtoolsRepo<'_> {
    /// Create a new devtools session with the given UUID and start timestamp.
    ///
    /// The caller (DevtoolsApp startup) generates the UUID and records the
    /// current time so it owns the session lifecycle.
    pub fn create_session(&self, session_id: &str, started_at: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO devtools_sessions (id, started_at) VALUES (?1, ?2)",
            params![session_id, started_at],
        )?;
        Ok(())
    }

    /// Mark any sessions with `ended_at IS NULL` as interrupted (set ended_at = started_at).
    ///
    /// Called on startup before creating the new session so crashed sessions
    /// display as interrupted in the session list.
    pub fn mark_orphaned_sessions_interrupted(&self) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "UPDATE devtools_sessions SET ended_at = started_at WHERE ended_at IS NULL",
            [],
        )?;
        Ok(())
    }

    /// Mark a session as ended by setting its `ended_at` timestamp.
    pub fn end_session(&self, session_id: &str, ended_at: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "UPDATE devtools_sessions SET ended_at = ?1 WHERE id = ?2",
            params![ended_at, session_id],
        )?;
        Ok(())
    }

    /// Insert a batch of events for the given session in a single transaction.
    ///
    /// Also increments the denormalized `event_count` on the session row.
    /// Must be called from a blocking context.
    pub fn insert_events(
        &self,
        session_id: &str,
        events: Vec<LogEvent>,
    ) -> Result<(), StorageError> {
        if events.is_empty() {
            return Ok(());
        }
        let count = events.len() as i64;
        let conn = self.storage.conn()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| StorageError::Database(e.to_string()))?;
        {
            let mut stmt = tx
                .prepare_cached(
                    "INSERT INTO devtools_events \
                     (original_id, session_id, timestamp, level, source, tier, \
                      category, message, metadata, daemon_event_id) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                )
                .map_err(|e| StorageError::Database(e.to_string()))?;
            for event in &events {
                stmt.execute(params![
                    event.id as i64,
                    session_id,
                    event.timestamp,
                    format!("{:?}", event.level),
                    format!("{}", event.source),
                    format!("{}", event.tier),
                    event.category,
                    event.message,
                    event.metadata.to_string(),
                    event.id as i64,
                ])
                .map_err(|e| StorageError::Database(e.to_string()))?;
            }
        }
        tx.execute(
            "UPDATE devtools_sessions SET event_count = event_count + ?1 WHERE id = ?2",
            params![count, session_id],
        )
        .map_err(|e| StorageError::Database(e.to_string()))?;
        tx.commit()
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(())
    }

    /// List all devtools sessions ordered by `started_at DESC`.
    ///
    /// `current_session_id` is compared against each session id to set
    /// `is_current` on the result.
    pub fn list_sessions(
        &self,
        current_session_id: &str,
    ) -> Result<Vec<DevtoolsSessionSummary>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, started_at, ended_at, label, event_count \
             FROM devtools_sessions \
             ORDER BY started_at DESC",
        )?;

        let current = current_session_id.to_owned();
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let is_current = id == current;
            Ok(DevtoolsSessionSummary {
                id,
                started_at: row.get(1)?,
                ended_at: row.get(2)?,
                label: row.get(3)?,
                event_count: row.get::<_, i64>(4)? as u64,
                is_current,
            })
        })?;

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Get metadata for a specific session by id.
    pub fn get_session(&self, session_id: &str) -> Result<DevtoolsSessionInfo, StorageError> {
        let conn = self.storage.conn()?;
        conn.query_row(
            "SELECT id, started_at, label, event_count \
             FROM devtools_sessions WHERE id = ?1",
            params![session_id],
            |row| {
                Ok(DevtoolsSessionInfo {
                    id: row.get(0)?,
                    started_at: row.get(1)?,
                    label: row.get(2)?,
                    event_count: row.get::<_, i64>(3)? as u64,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                StorageError::NotFound(format!("devtools session {session_id}"))
            }
            other => StorageError::Database(other.to_string()),
        })
    }

    /// Update the user-editable label for a session.
    pub fn rename_session(&self, session_id: &str, label: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "UPDATE devtools_sessions SET label = ?1 WHERE id = ?2",
            params![label, session_id],
        )?;
        Ok(())
    }

    /// Delete a session and cascade its events.
    pub fn delete_session(&self, session_id: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "DELETE FROM devtools_sessions WHERE id = ?1",
            params![session_id],
        )?;
        Ok(())
    }

    /// Delete sessions (and their cascaded events) older than `retention_days`.
    ///
    /// Returns the number of sessions deleted.
    pub fn purge_old_sessions(&self, retention_days: u32) -> Result<usize, StorageError> {
        let ms_per_day: i64 = 86_400_000;
        let cutoff = now_ms() - (retention_days as i64 * ms_per_day);
        let conn = self.storage.conn()?;
        let deleted = conn.execute(
            "DELETE FROM devtools_sessions WHERE started_at < ?1",
            params![cutoff],
        )?;
        Ok(deleted)
    }

    /// Return paginated and filtered events for a session.
    #[allow(clippy::too_many_lines)]
    pub fn query_events(
        &self,
        query: &DevtoolsEventQuery,
    ) -> Result<DevtoolsEventQueryResponse, StorageError> {
        let conn = self.storage.conn()?;

        // Build WHERE clause dynamically from optional filters.
        let mut conditions = vec!["session_id = ?1".to_owned()];
        let mut bind_idx = 2usize;
        let mut bind_values: Vec<String> = Vec::new();

        if let Some(ref source) = query.source {
            conditions.push(format!("source = ?{bind_idx}"));
            bind_values.push(source.clone());
            bind_idx += 1;
        }
        if let Some(ref level) = query.level {
            conditions.push(format!("level = ?{bind_idx}"));
            bind_values.push(level.clone());
            bind_idx += 1;
        }
        if let Some(ref category) = query.category {
            conditions.push(format!("category LIKE ?{bind_idx}"));
            bind_values.push(format!("%{category}%"));
            bind_idx += 1;
        }
        if let Some(ref search) = query.search_text {
            conditions.push(format!("message LIKE ?{bind_idx}"));
            bind_values.push(format!("%{search}%"));
            bind_idx += 1;
        }
        // bind_idx is only needed for dynamic query construction above.
        let _ = bind_idx;

        let where_clause = conditions.join(" AND ");

        // Count total matching rows.
        let count_sql = format!("SELECT COUNT(*) FROM devtools_events WHERE {where_clause}");
        let total: usize = {
            let mut count_stmt = conn
                .prepare(&count_sql)
                .map_err(|e| StorageError::Database(e.to_string()))?;
            let mut total_params = vec![query.session_id.clone()];
            total_params.extend_from_slice(&bind_values);
            count_stmt
                .query_row(rusqlite::params_from_iter(total_params.iter()), |row| {
                    row.get::<_, i64>(0)
                })
                .map_err(|e| StorageError::Database(e.to_string()))? as usize
        };

        // Fetch the page.
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(1000).min(5000);
        let data_sql = format!(
            "SELECT rowid, original_id, session_id, timestamp, level, source, tier,
                    category, message, metadata, daemon_event_id
             FROM devtools_events
             WHERE {where_clause}
             ORDER BY timestamp ASC
             LIMIT {limit} OFFSET {offset}"
        );

        let mut data_params = vec![query.session_id.clone()];
        data_params.extend_from_slice(&bind_values);

        let mut data_stmt = conn
            .prepare(&data_sql)
            .map_err(|e| StorageError::Database(e.to_string()))?;
        let events: Vec<serde_json::Value> = data_stmt
            .query_map(rusqlite::params_from_iter(data_params.iter()), |row| {
                let metadata_str: String = row.get(9)?;
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, String>(8)?,
                    metadata_str,
                    row.get::<_, Option<i64>>(10)?,
                ))
            })
            .map_err(|e| StorageError::Database(e.to_string()))?
            .filter_map(Result::ok)
            .map(
                |(
                    rowid,
                    original_id,
                    session_id,
                    timestamp,
                    level,
                    source,
                    tier,
                    category,
                    message,
                    metadata_str,
                    daemon_event_id,
                )| {
                    let metadata =
                        serde_json::from_str(&metadata_str).unwrap_or(serde_json::Value::Null);
                    serde_json::json!({
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
                    })
                },
            )
            .collect();

        Ok(DevtoolsEventQueryResponse { events, total })
    }
}

/// Return the current Unix timestamp in milliseconds.
fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use orqa_engine_types::types::event::{EventLevel, EventSource, EventTier, LogEvent};

    use super::*;
    use crate::Storage;

    fn open_storage() -> Storage {
        Storage::open_in_memory().expect("in-memory storage")
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

    #[test]
    fn create_session_and_list() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000).expect("create");

        let sessions = repo.list_sessions("sess-001").expect("list");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "sess-001");
        assert!(sessions[0].is_current);
    }

    #[test]
    fn insert_events_increments_count() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000).expect("create");
        repo.insert_events("sess-001", vec![make_event(1), make_event(2)])
            .expect("insert");

        let info = repo.get_session("sess-001").expect("get");
        assert_eq!(info.event_count, 2);
    }

    #[test]
    fn end_session_sets_ended_at() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000).expect("create");
        repo.end_session("sess-001", 2_000_000).expect("end");

        let sessions = repo.list_sessions("other").expect("list");
        assert_eq!(sessions[0].ended_at, Some(2_000_000));
    }

    #[test]
    fn mark_orphaned_sessions() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("orphan", 1_000_000).expect("create");
        repo.mark_orphaned_sessions_interrupted().expect("mark");

        let sessions = repo.list_sessions("other").expect("list");
        assert_eq!(
            sessions[0].ended_at,
            Some(1_000_000),
            "orphaned session should have ended_at = started_at"
        );
    }

    #[test]
    fn delete_session_cascades_events() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000).expect("create");
        repo.insert_events("sess-001", vec![make_event(1)])
            .expect("insert");
        repo.delete_session("sess-001").expect("delete");

        let sessions = repo.list_sessions("other").expect("list");
        assert!(sessions.is_empty());
    }

    #[test]
    fn query_events_pagination() {
        let storage = open_storage();
        let repo = storage.devtools();
        repo.create_session("sess-001", 1_000_000).expect("create");
        let events: Vec<LogEvent> = (1..=5).map(make_event).collect();
        repo.insert_events("sess-001", events).expect("insert");

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
            .expect("query");
        assert_eq!(resp.events.len(), 2);
        assert_eq!(resp.total, 5);
    }
}
