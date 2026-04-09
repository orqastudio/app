// Sessions repository for orqa-storage.
//
// Provides async CRUD and query operations over the `sessions` table. Sessions
// are the primary unit of user–agent interaction within a project. All SQL is
// expressed as raw statements via SeaORM's ConnectionTrait.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use orqa_engine_types::types::session::{Session, SessionStatus, SessionSummary};

use crate::error::StorageError;
use crate::traits::SessionRepository;

/// Async repository handle for the `sessions` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::sessions()`.
pub struct SessionRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to a `Session` domain value.
///
/// Column positions must match the SELECT order used in every get-by-id query.
fn map_session(row: &sea_orm::QueryResult) -> Result<Session, StorageError> {
    let status_str: String = row
        .try_get("", "status")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    let title_manually_set: i64 = row
        .try_get("", "title_manually_set")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(Session {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        project_id: row
            .try_get("", "project_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        title: row
            .try_get("", "title")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        model: row
            .try_get("", "model")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        system_prompt: row
            .try_get("", "system_prompt")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        status: parse_status(&status_str),
        summary: row
            .try_get("", "summary")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        handoff_notes: row
            .try_get("", "handoff_notes")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        total_input_tokens: row
            .try_get("", "total_input_tokens")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        total_output_tokens: row
            .try_get("", "total_output_tokens")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        total_cost_usd: row
            .try_get("", "total_cost_usd")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        provider_session_id: row
            .try_get("", "provider_session_id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        updated_at: row
            .try_get("", "updated_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        title_manually_set: title_manually_set != 0,
    })
}

/// Map a SeaORM `QueryResult` row to a `SessionSummary` domain value.
fn map_session_summary(row: &sea_orm::QueryResult) -> Result<SessionSummary, StorageError> {
    let status_str: String = row
        .try_get("", "status")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(SessionSummary {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        title: row
            .try_get("", "title")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        status: parse_status(&status_str),
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        updated_at: row
            .try_get("", "updated_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        message_count: row
            .try_get("", "message_count")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        preview: row
            .try_get("", "preview")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

/// Parse a status string into the `SessionStatus` enum.
fn parse_status(s: &str) -> SessionStatus {
    match s {
        "active" => SessionStatus::Active,
        "completed" => SessionStatus::Completed,
        "abandoned" => SessionStatus::Abandoned,
        _ => SessionStatus::Error,
    }
}

/// Serialize a `SessionStatus` to its SQL string representation.
fn status_to_str(status: SessionStatus) -> &'static str {
    match status {
        SessionStatus::Active => "active",
        SessionStatus::Completed => "completed",
        SessionStatus::Abandoned => "abandoned",
        SessionStatus::Error => "error",
    }
}

/// Fetch a session by its integer primary key from the shared connection.
async fn fetch_by_id(db: &DatabaseConnection, id: i64) -> Result<Session, StorageError> {
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "SELECT id, project_id, title, model, system_prompt, status, summary, \
                    handoff_notes, total_input_tokens, total_output_tokens, total_cost_usd, \
                    provider_session_id, created_at, updated_at, \
                    COALESCE(title_manually_set, 0) AS title_manually_set \
             FROM sessions WHERE id = ?",
            [id.into()],
        ))
        .await
        .map_err(|e| StorageError::Database(e.to_string()))?
        .ok_or_else(|| StorageError::NotFound(format!("session {id}")))?;
    map_session(&row)
}

#[async_trait::async_trait]
impl SessionRepository for SessionRepo {
    /// Create a new session and return the full row.
    async fn create(
        &self,
        project_id: i64,
        model: &str,
        system_prompt: Option<&str>,
    ) -> Result<Session, StorageError> {
        let prompt_val: sea_orm::Value = match system_prompt {
            Some(p) => p.into(),
            None => sea_orm::Value::String(None),
        };
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO sessions (project_id, model, system_prompt) VALUES (?, ?, ?)",
                [project_id.into(), model.into(), prompt_val],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        // Retrieve the most recently inserted row for this project and model.
        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, project_id, title, model, system_prompt, status, summary, \
                        handoff_notes, total_input_tokens, total_output_tokens, total_cost_usd, \
                        provider_session_id, created_at, updated_at, \
                        COALESCE(title_manually_set, 0) AS title_manually_set \
                 FROM sessions ORDER BY id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| {
                StorageError::NotFound("sessions table is empty after insert".to_owned())
            })?;
        map_session(&row)
    }

    /// Get a session by its primary key.
    async fn get(&self, id: i64) -> Result<Session, StorageError> {
        fetch_by_id(&self.db, id).await
    }

    /// List sessions for a project with optional status filter and pagination.
    async fn list(
        &self,
        project_id: i64,
        status_filter: Option<SessionStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SessionSummary>, StorageError> {
        let summary_select = "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                    (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                    (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                     AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
             FROM sessions s";

        let rows = match status_filter {
            Some(status) => {
                let status_str = status_to_str(status);
                self.db
                    .query_all_raw(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        format!("{summary_select} WHERE s.project_id = ? AND s.status = ? ORDER BY s.updated_at DESC LIMIT ? OFFSET ?"),
                        [project_id.into(), status_str.into(), limit.into(), offset.into()],
                    ))
                    .await
                    .map_err(|e| StorageError::Database(e.to_string()))?
            }
            None => {
                self.db
                    .query_all_raw(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        format!("{summary_select} WHERE s.project_id = ? ORDER BY s.updated_at DESC LIMIT ? OFFSET ?"),
                        [project_id.into(), limit.into(), offset.into()],
                    ))
                    .await
                    .map_err(|e| StorageError::Database(e.to_string()))?
            }
        };

        rows.iter().map(map_session_summary).collect()
    }

    /// List all sessions across all projects with optional status filter.
    ///
    /// Applies an implicit cap of 1000 rows to prevent unbounded result sets.
    async fn list_all(
        &self,
        status_filter: Option<SessionStatus>,
    ) -> Result<Vec<SessionSummary>, StorageError> {
        let summary_select = "SELECT s.id, s.title, s.status, s.created_at, s.updated_at, \
                    (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id) AS message_count, \
                    (SELECT m2.content FROM messages m2 WHERE m2.session_id = s.id \
                     AND m2.role = 'user' ORDER BY m2.turn_index ASC LIMIT 1) AS preview \
             FROM sessions s";

        let rows = match status_filter {
            Some(status) => {
                let status_str = status_to_str(status);
                self.db
                    .query_all_raw(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        format!("{summary_select} WHERE s.status = ? ORDER BY s.updated_at DESC LIMIT 1000"),
                        [status_str.into()],
                    ))
                    .await
                    .map_err(|e| StorageError::Database(e.to_string()))?
            }
            None => self
                .db
                .query_all_raw(Statement::from_string(
                    DbBackend::Sqlite,
                    format!("{summary_select} ORDER BY s.updated_at DESC LIMIT 1000"),
                ))
                .await
                .map_err(|e| StorageError::Database(e.to_string()))?,
        };

        rows.iter().map(map_session_summary).collect()
    }

    /// Update a session's status.
    async fn update_status(&self, id: i64, status: SessionStatus) -> Result<(), StorageError> {
        let status_str = status_to_str(status);
        let result = self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET status = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [status_str.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Return the next turn index for a session (max existing + 1, or 0).
    async fn next_turn_index(&self, session_id: i64) -> Result<i32, StorageError> {
        let row = self.db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT COALESCE(MAX(turn_index), -1) AS max_turn FROM messages WHERE session_id = ?",
                [session_id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| StorageError::Database("next_turn_index query returned no row".to_owned()))?;
        let max: i64 = row
            .try_get("", "max_turn")
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok((max + 1) as i32)
    }

    /// Update the session title and mark it as manually set.
    async fn update_title(&self, id: i64, title: &str) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET title = ?, title_manually_set = 1, \
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [title.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Auto-update session title only if not manually set.
    ///
    /// Returns `true` if updated, `false` if skipped.
    async fn auto_update_title(&self, id: i64, title: &str) -> Result<bool, StorageError> {
        let result = self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET title = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
                 WHERE id = ? AND (title_manually_set = 0 OR title_manually_set IS NULL)",
                [title.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    /// Mark a session as completed.
    async fn end_session(&self, id: i64) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET status = 'completed', \
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Delete a session and its messages (cascade).
    async fn delete(&self, id: i64) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "DELETE FROM sessions WHERE id = ?",
                [id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Increment token usage counters for a session (additive).
    async fn update_token_usage(
        &self,
        id: i64,
        input_tokens: i64,
        output_tokens: i64,
    ) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET \
                 total_input_tokens = total_input_tokens + ?, \
                 total_output_tokens = total_output_tokens + ?, \
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
                 WHERE id = ?",
                [input_tokens.into(), output_tokens.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }

    /// Store the provider session ID for context continuity across restarts.
    async fn update_provider_session_id(
        &self,
        id: i64,
        provider_session_id: &str,
    ) -> Result<(), StorageError> {
        let result = self
            .db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE sessions SET provider_session_id = ?, \
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [provider_session_id.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("session {id}")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{ProjectRepository, SessionRepository};
    use crate::Storage;

    async fn setup() -> Storage {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");
        storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create project");
        storage
    }

    #[tokio::test]
    async fn create_and_get_session() {
        let storage = setup().await;
        let session = storage
            .sessions()
            .create(1, "claude-opus-4-6", Some("You are helpful"))
            .await
            .expect("create session");

        assert_eq!(session.project_id, 1);
        assert_eq!(session.model, "claude-opus-4-6");
        assert_eq!(session.system_prompt.as_deref(), Some("You are helpful"));
        assert_eq!(session.status, SessionStatus::Active);
        assert_eq!(session.total_input_tokens, 0);
        assert!(!session.title_manually_set);
    }

    #[tokio::test]
    async fn get_nonexistent_session() {
        let storage = setup().await;
        let result = storage.sessions().get(999).await;
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[tokio::test]
    async fn end_session_marks_completed() {
        let storage = setup().await;
        let s = storage
            .sessions()
            .create(1, "auto", None)
            .await
            .expect("create");
        storage.sessions().end_session(s.id).await.expect("end");
        let fetched = storage.sessions().get(s.id).await.expect("get");
        assert_eq!(fetched.status, SessionStatus::Completed);
    }

    #[tokio::test]
    async fn update_token_usage_is_additive() {
        let storage = setup().await;
        let s = storage
            .sessions()
            .create(1, "auto", None)
            .await
            .expect("create");
        storage
            .sessions()
            .update_token_usage(s.id, 100, 50)
            .await
            .expect("first update");
        storage
            .sessions()
            .update_token_usage(s.id, 200, 100)
            .await
            .expect("second update");
        let fetched = storage.sessions().get(s.id).await.expect("get");
        assert_eq!(fetched.total_input_tokens, 300);
        assert_eq!(fetched.total_output_tokens, 150);
    }

    #[tokio::test]
    async fn auto_update_title_skips_manually_set() {
        let storage = setup().await;
        let s = storage
            .sessions()
            .create(1, "auto", None)
            .await
            .expect("create");
        storage
            .sessions()
            .update_title(s.id, "User Title")
            .await
            .expect("manual title");
        let updated = storage
            .sessions()
            .auto_update_title(s.id, "Auto Title")
            .await
            .expect("auto");
        assert!(!updated, "auto title should not overwrite manual title");
        let fetched = storage.sessions().get(s.id).await.expect("get");
        assert_eq!(fetched.title.as_deref(), Some("User Title"));
    }

    #[tokio::test]
    async fn list_sessions_pagination() {
        let storage = setup().await;
        for _ in 0..5 {
            storage
                .sessions()
                .create(1, "auto", None)
                .await
                .expect("create");
        }
        let page1 = storage
            .sessions()
            .list(1, None, 2, 0)
            .await
            .expect("page 1");
        assert_eq!(page1.len(), 2);
        let page3 = storage
            .sessions()
            .list(1, None, 2, 4)
            .await
            .expect("page 3");
        assert_eq!(page3.len(), 1);
    }
}
