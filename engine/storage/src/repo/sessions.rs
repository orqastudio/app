// Sessions repository for orqa-storage.
//
// Provides CRUD and query operations over the `sessions` table. Sessions are
// the primary unit of user–agent interaction within a project. All SQL is
// ported directly from app/src-tauri/src/repo/session_repo.rs.

use rusqlite::params;

use orqa_engine_types::types::session::{Session, SessionStatus, SessionSummary};

use crate::error::StorageError;
use crate::Storage;

/// Zero-cost repository handle for the `sessions` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::sessions()`.
pub struct SessionRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl SessionRepo<'_> {
    /// Create a new session and return the full row.
    pub fn create(
        &self,
        project_id: i64,
        model: &str,
        system_prompt: Option<&str>,
    ) -> Result<Session, StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO sessions (project_id, model, system_prompt) VALUES (?1, ?2, ?3)",
            params![project_id, model, system_prompt],
        )?;
        let id = conn.last_insert_rowid();
        get_conn(&conn, id)
    }

    /// Get a session by its primary key.
    pub fn get(&self, id: i64) -> Result<Session, StorageError> {
        let conn = self.storage.conn()?;
        get_conn(&conn, id)
    }

    /// List sessions for a project with optional status filter and pagination.
    pub fn list(
        &self,
        project_id: i64,
        status_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SessionSummary>, StorageError> {
        let conn = self.storage.conn()?;

        let sql = if status_filter.is_some() {
            "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                    (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                    (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                     AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
             FROM sessions s \
             WHERE s.project_id = ?1 AND s.status = ?2 \
             ORDER BY s.updated_at DESC \
             LIMIT ?3 OFFSET ?4"
        } else {
            "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                    (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                    (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                     AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
             FROM sessions s \
             WHERE s.project_id = ?1 \
             ORDER BY s.updated_at DESC \
             LIMIT ?2 OFFSET ?3"
        };

        let mut stmt = conn.prepare(sql)?;
        let rows = if let Some(status) = status_filter {
            stmt.query_map(
                params![project_id, status, limit, offset],
                map_session_summary,
            )?
        } else {
            stmt.query_map(params![project_id, limit, offset], map_session_summary)?
        };

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// List all sessions across all projects with optional status filter.
    ///
    /// Used by the daemon when no project_id filter is requested — returns
    /// sessions ordered by most recently updated. Applies an implicit cap of
    /// 1000 rows to prevent unbounded result sets.
    pub fn list_all(
        &self,
        status_filter: Option<&str>,
    ) -> Result<Vec<SessionSummary>, StorageError> {
        let conn = self.storage.conn()?;
        let base = "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                    (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                    (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                     AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
             FROM sessions s";

        let mut stmt;
        let rows: Box<dyn Iterator<Item = rusqlite::Result<SessionSummary>>> = if let Some(status) =
            status_filter
        {
            let sql = format!("{base} WHERE s.status = ?1 ORDER BY s.updated_at DESC LIMIT 1000");
            stmt = conn.prepare(&sql)?;
            Box::new(stmt.query_map(params![status], map_session_summary)?)
        } else {
            let sql = format!("{base} ORDER BY s.updated_at DESC LIMIT 1000");
            stmt = conn.prepare(&sql)?;
            Box::new(stmt.query_map([], map_session_summary)?)
        };

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Update a session's status to an arbitrary value.
    ///
    /// Used by the daemon HTTP layer to apply status transitions received from
    /// HTTP clients. The CHECK constraint in SQLite enforces valid values.
    pub fn update_status(&self, id: i64, status: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET status = ?1, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?2",
            params![status, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Store the next turn index by inserting a user message and returning its ID.
    ///
    /// Calls the messages repo internally — provided as a convenience method so
    /// the streaming route only needs `Arc<Storage>`.
    pub fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError> {
        let conn = self.storage.conn()?;
        let max: Option<i32> = conn.query_row(
            "SELECT MAX(turn_index) FROM messages WHERE session_id = ?1",
            params![session_id],
            |row| row.get(0),
        )?;
        Ok(max.map_or(0, |m| m + 1))
    }

    /// Update the title of a session and mark it as manually set.
    ///
    /// Once marked, auto-naming will not overwrite this title.
    pub fn update_title(&self, id: i64, title: &str) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET title = ?1, title_manually_set = 1, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?2",
            params![title, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Auto-update session title only if not manually set.
    ///
    /// Returns `Ok(true)` if the title was updated, `Ok(false)` if skipped
    /// because the session has `title_manually_set = 1`.
    pub fn auto_update_title(&self, id: i64, title: &str) -> Result<bool, StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET title = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?2 AND (title_manually_set = 0 OR title_manually_set IS NULL)",
            params![title, id],
        )?;
        Ok(rows > 0)
    }

    /// Mark a session as completed.
    pub fn end_session(&self, id: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET status = 'completed', \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?1",
            params![id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Delete a session and its messages (cascade).
    pub fn delete(&self, id: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Update token usage counters for a session (additive).
    pub fn update_token_usage(
        &self,
        id: i64,
        input_tokens: i64,
        output_tokens: i64,
    ) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET \
             total_input_tokens = total_input_tokens + ?1, \
             total_output_tokens = total_output_tokens + ?2, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?3",
            params![input_tokens, output_tokens, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Store the provider session ID so conversation context survives app restarts.
    pub fn update_provider_session_id(
        &self,
        id: i64,
        provider_session_id: &str,
    ) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE sessions SET provider_session_id = ?1, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?2",
            params![provider_session_id, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }
}

/// Fetch a session by id from an existing open connection.
fn get_conn(conn: &rusqlite::Connection, id: i64) -> Result<Session, StorageError> {
    conn.query_row(
        "SELECT id, project_id, title, model, system_prompt, status, summary, \
                handoff_notes, total_input_tokens, total_output_tokens, total_cost_usd, \
                provider_session_id, created_at, updated_at, \
                COALESCE(title_manually_set, 0) \
         FROM sessions WHERE id = ?1",
        params![id],
        |row| {
            let status_str: String = row.get(5)?;
            Ok(Session {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                model: row.get(3)?,
                system_prompt: row.get(4)?,
                status: parse_status(&status_str),
                summary: row.get(6)?,
                handoff_notes: row.get(7)?,
                total_input_tokens: row.get(8)?,
                total_output_tokens: row.get(9)?,
                total_cost_usd: row.get(10)?,
                provider_session_id: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
                title_manually_set: row.get::<_, i64>(14)? != 0,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => StorageError::NotFound(format!("session {id}")),
        other => StorageError::Database(other.to_string()),
    })
}

fn parse_status(s: &str) -> SessionStatus {
    match s {
        "active" => SessionStatus::Active,
        "completed" => SessionStatus::Completed,
        "abandoned" => SessionStatus::Abandoned,
        _ => SessionStatus::Error,
    }
}

fn map_session_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionSummary> {
    let status_str: String = row.get(2)?;
    Ok(SessionSummary {
        id: row.get(0)?,
        title: row.get(1)?,
        status: parse_status(&status_str),
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        message_count: row.get(5)?,
        preview: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    fn setup() -> Storage {
        let storage = Storage::open_in_memory().expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .expect("create project");
        storage
    }

    #[test]
    fn create_and_get_session() {
        let storage = setup();
        let session = storage
            .sessions()
            .create(1, "claude-opus-4-6", Some("You are helpful"))
            .expect("create session");

        assert_eq!(session.project_id, 1);
        assert_eq!(session.model, "claude-opus-4-6");
        assert_eq!(session.system_prompt.as_deref(), Some("You are helpful"));
        assert_eq!(session.status, SessionStatus::Active);
        assert_eq!(session.total_input_tokens, 0);
        assert!(!session.title_manually_set);
    }

    #[test]
    fn get_nonexistent_session() {
        let storage = setup();
        let result = storage.sessions().get(999);
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn end_session_marks_completed() {
        let storage = setup();
        let s = storage.sessions().create(1, "auto", None).expect("create");
        storage.sessions().end_session(s.id).expect("end");
        let fetched = storage.sessions().get(s.id).expect("get");
        assert_eq!(fetched.status, SessionStatus::Completed);
    }

    #[test]
    fn update_token_usage_is_additive() {
        let storage = setup();
        let s = storage.sessions().create(1, "auto", None).expect("create");
        storage
            .sessions()
            .update_token_usage(s.id, 100, 50)
            .expect("first update");
        storage
            .sessions()
            .update_token_usage(s.id, 200, 100)
            .expect("second update");
        let fetched = storage.sessions().get(s.id).expect("get");
        assert_eq!(fetched.total_input_tokens, 300);
        assert_eq!(fetched.total_output_tokens, 150);
    }

    #[test]
    fn auto_update_title_skips_manually_set() {
        let storage = setup();
        let s = storage.sessions().create(1, "auto", None).expect("create");
        storage
            .sessions()
            .update_title(s.id, "User Title")
            .expect("manual title");
        let updated = storage
            .sessions()
            .auto_update_title(s.id, "Auto Title")
            .expect("auto");
        assert!(!updated, "auto title should not overwrite manual title");
        let fetched = storage.sessions().get(s.id).expect("get");
        assert_eq!(fetched.title.as_deref(), Some("User Title"));
    }

    #[test]
    fn list_sessions_pagination() {
        let storage = setup();
        for _ in 0..5 {
            storage.sessions().create(1, "auto", None).expect("create");
        }
        let page1 = storage.sessions().list(1, None, 2, 0).expect("page 1");
        assert_eq!(page1.len(), 2);
        let page3 = storage.sessions().list(1, None, 2, 4).expect("page 3");
        assert_eq!(page3.len(), 1);
    }
}
