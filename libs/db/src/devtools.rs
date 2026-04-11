// HTTP client for `/devtools-sessions` endpoints.
//
// Mirrors DevtoolsRepository from engine/storage/src/traits.rs.
// DevtoolsSessionSummary, DevtoolsSessionInfo, DevtoolsEventQuery, and
// DevtoolsEventQueryResponse are redefined here with serde derives since
// libs/db does not depend on engine/storage.

use serde::{Deserialize, Serialize};

use orqa_engine_types::types::event::LogEvent;

use crate::{parse_empty_response, parse_response, DbError};

/// Summary of a devtools session returned by `list_sessions`.
///
/// Mirrors `engine/storage/src/repo/devtools.rs::DevtoolsSessionSummary`.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Metadata for a single devtools session.
///
/// Mirrors `engine/storage/src/repo/devtools.rs::DevtoolsSessionInfo`.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Query parameters for `query_events`.
///
/// Mirrors `engine/storage/src/repo/devtools.rs::DevtoolsEventQuery`.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Paginated response from `query_events`.
///
/// Mirrors `engine/storage/src/repo/devtools.rs::DevtoolsEventQueryResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevtoolsEventQueryResponse {
    /// Matching events as JSON values.
    pub events: Vec<serde_json::Value>,
    /// Total number of matching events (before paging).
    pub total: usize,
}

/// Sub-client for `/devtools-sessions` daemon endpoints.
///
/// Obtained via `DbClient::devtools()`.
pub struct DevtoolsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl DevtoolsClient<'_> {
    /// Create a new devtools session with the given UUID and start timestamp.
    ///
    /// Calls POST /devtools-sessions.
    pub async fn create_session(&self, session_id: &str, started_at: i64) -> Result<(), DbError> {
        let body = serde_json::json!({
            "session_id": session_id,
            "started_at": started_at,
        });
        let resp = self
            .http
            .post(format!("{}/devtools-sessions", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Mark all sessions with `ended_at IS NULL` as interrupted.
    ///
    /// Calls POST /devtools-sessions/mark-orphaned.
    pub async fn mark_orphaned_sessions_interrupted(&self) -> Result<(), DbError> {
        let resp = self
            .http
            .post(format!("{}/devtools-sessions/mark-orphaned", self.base_url))
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Mark a session as ended.
    ///
    /// Calls POST /devtools-sessions/:id/end.
    pub async fn end_session(&self, session_id: &str, ended_at: i64) -> Result<(), DbError> {
        let body = serde_json::json!({ "ended_at": ended_at });
        let resp = self
            .http
            .post(format!(
                "{}/devtools-sessions/{session_id}/end",
                self.base_url
            ))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Insert a batch of events for the given session.
    ///
    /// Calls POST /devtools-sessions/:id/events with a bare JSON array body.
    /// The daemon handler deserializes the body as `Vec<LogEvent>` directly.
    pub async fn insert_events(
        &self,
        session_id: &str,
        events: Vec<LogEvent>,
    ) -> Result<(), DbError> {
        let resp = self
            .http
            .post(format!(
                "{}/devtools-sessions/{session_id}/events",
                self.base_url
            ))
            .json(&events)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// List all devtools sessions ordered by `started_at DESC`.
    ///
    /// Calls GET /devtools-sessions?current_session_id=...
    pub async fn list_sessions(
        &self,
        current_session_id: &str,
    ) -> Result<Vec<DevtoolsSessionSummary>, DbError> {
        let resp = self
            .http
            .get(format!("{}/devtools-sessions", self.base_url))
            .query(&[("current_session_id", current_session_id)])
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get metadata for a specific session by ID.
    ///
    /// Calls GET /devtools-sessions/:id.
    pub async fn get_session(&self, session_id: &str) -> Result<DevtoolsSessionInfo, DbError> {
        let resp = self
            .http
            .get(format!("{}/devtools-sessions/{session_id}", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Update the user-editable label for a session.
    ///
    /// Calls PUT /devtools-sessions/:id/label.
    pub async fn rename_session(&self, session_id: &str, label: &str) -> Result<(), DbError> {
        let body = serde_json::json!({ "label": label });
        let resp = self
            .http
            .put(format!(
                "{}/devtools-sessions/{session_id}/label",
                self.base_url
            ))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Delete a session and cascade its events.
    ///
    /// Calls DELETE /devtools-sessions/:id.
    pub async fn delete_session(&self, session_id: &str) -> Result<(), DbError> {
        let resp = self
            .http
            .delete(format!("{}/devtools-sessions/{session_id}", self.base_url))
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Delete sessions older than the given retention window.
    ///
    /// Returns the number of sessions deleted.
    /// Calls POST /devtools-sessions/purge.
    pub async fn purge_old_sessions(&self, retention_days: u32) -> Result<usize, DbError> {
        let body = serde_json::json!({ "retention_days": retention_days });
        let resp = self
            .http
            .post(format!("{}/devtools-sessions/purge", self.base_url))
            .json(&body)
            .send()
            .await?;
        let val: serde_json::Value = parse_response(resp).await?;
        val["deleted"]
            .as_u64()
            .map(|n| n as usize)
            .ok_or_else(|| DbError::Deserialization("missing deleted field".to_owned()))
    }

    /// Return paginated and filtered events for a session.
    ///
    /// Calls POST /devtools-sessions/:id/events/query.
    pub async fn query_events(
        &self,
        query: &DevtoolsEventQuery,
    ) -> Result<DevtoolsEventQueryResponse, DbError> {
        let resp = self
            .http
            .post(format!(
                "{}/devtools-sessions/{}/events/query",
                self.base_url, query.session_id
            ))
            .json(query)
            .send()
            .await?;
        parse_response(resp).await
    }
}
