// HTTP client for `/sessions` endpoints.
//
// Mirrors SessionRepository from engine/storage/src/traits.rs.

use orqa_engine_types::types::session::{Session, SessionStatus, SessionSummary};

use crate::{parse_empty_response, parse_response, DbError};

/// Sub-client for `/sessions` daemon endpoints.
///
/// Obtained via `DbClient::sessions()`.
pub struct SessionsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl SessionsClient<'_> {
    /// Create a new session and return the full row.
    ///
    /// Calls POST /sessions.
    pub async fn create(
        &self,
        project_id: i64,
        model: &str,
        system_prompt: Option<&str>,
    ) -> Result<Session, DbError> {
        let body = serde_json::json!({
            "project_id": project_id,
            "model": model,
            "system_prompt": system_prompt,
        });
        let resp = self
            .http
            .post(format!("{}/sessions", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get a session by its primary key.
    ///
    /// Calls GET /sessions/:id.
    pub async fn get(&self, id: i64) -> Result<Session, DbError> {
        let resp = self
            .http
            .get(format!("{}/sessions/{id}", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// List sessions for a project with optional status filter and pagination.
    ///
    /// Calls GET /sessions?project_id=...&status=...&limit=...&offset=...
    pub async fn list(
        &self,
        project_id: i64,
        status_filter: Option<SessionStatus>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SessionSummary>, DbError> {
        let mut req = self
            .http
            .get(format!("{}/sessions", self.base_url))
            .query(&[
                ("project_id", project_id.to_string()),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ]);
        if let Some(status) = status_filter {
            req = req.query(&[("status", session_status_str(status))]);
        }
        let resp = req.send().await?;
        parse_response(resp).await
    }

    /// List all sessions across all projects with optional status filter.
    ///
    /// Calls GET /sessions/all?status=...
    pub async fn list_all(
        &self,
        status_filter: Option<SessionStatus>,
    ) -> Result<Vec<SessionSummary>, DbError> {
        let mut req = self.http.get(format!("{}/sessions/all", self.base_url));
        if let Some(status) = status_filter {
            req = req.query(&[("status", session_status_str(status))]);
        }
        let resp = req.send().await?;
        parse_response(resp).await
    }

    /// Update a session's status.
    ///
    /// Calls PUT /sessions/:id/status.
    pub async fn update_status(&self, id: i64, status: SessionStatus) -> Result<(), DbError> {
        let body = serde_json::json!({ "status": session_status_str(status) });
        let resp = self
            .http
            .put(format!("{}/sessions/{id}/status", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Return the next turn index for a session.
    ///
    /// Calls GET /sessions/:id/next-turn-index.
    pub async fn next_turn_index(&self, session_id: i64) -> Result<i32, DbError> {
        let resp = self
            .http
            .get(format!(
                "{}/sessions/{session_id}/next-turn-index",
                self.base_url
            ))
            .send()
            .await?;
        let val: serde_json::Value = parse_response(resp).await?;
        val["next_turn_index"]
            .as_i64()
            .map(|n| n as i32)
            .ok_or_else(|| DbError::Deserialization("missing next_turn_index".to_owned()))
    }

    /// Update the session title and mark it as manually set.
    ///
    /// Calls PUT /sessions/:id/title.
    pub async fn update_title(&self, id: i64, title: &str) -> Result<(), DbError> {
        let body = serde_json::json!({ "title": title });
        let resp = self
            .http
            .put(format!("{}/sessions/{id}/title", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Auto-update session title only if not manually set.
    ///
    /// Calls PUT /sessions/:id/auto-title.
    /// Returns `true` if the title was updated, `false` if skipped.
    pub async fn auto_update_title(&self, id: i64, title: &str) -> Result<bool, DbError> {
        let body = serde_json::json!({ "title": title });
        let resp = self
            .http
            .put(format!("{}/sessions/{id}/auto-title", self.base_url))
            .json(&body)
            .send()
            .await?;
        let val: serde_json::Value = parse_response(resp).await?;
        val["updated"]
            .as_bool()
            .ok_or_else(|| DbError::Deserialization("missing updated field".to_owned()))
    }

    /// Mark a session as completed.
    ///
    /// Calls POST /sessions/:id/end.
    pub async fn end_session(&self, id: i64) -> Result<(), DbError> {
        let resp = self
            .http
            .post(format!("{}/sessions/{id}/end", self.base_url))
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Delete a session and its messages (cascade).
    ///
    /// Calls DELETE /sessions/:id.
    pub async fn delete(&self, id: i64) -> Result<(), DbError> {
        let resp = self
            .http
            .delete(format!("{}/sessions/{id}", self.base_url))
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Increment token usage counters for a session.
    ///
    /// Calls POST /sessions/:id/token-usage.
    pub async fn update_token_usage(
        &self,
        id: i64,
        input_tokens: i64,
        output_tokens: i64,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({
            "input_tokens": input_tokens,
            "output_tokens": output_tokens,
        });
        let resp = self
            .http
            .post(format!("{}/sessions/{id}/token-usage", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Store the provider session ID for context continuity.
    ///
    /// Calls PUT /sessions/:id/provider-session-id.
    pub async fn update_provider_session_id(
        &self,
        id: i64,
        provider_session_id: &str,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({ "provider_session_id": provider_session_id });
        let resp = self
            .http
            .put(format!(
                "{}/sessions/{id}/provider-session-id",
                self.base_url
            ))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }
}

/// Serialize a `SessionStatus` to its wire string representation.
fn session_status_str(status: SessionStatus) -> &'static str {
    match status {
        SessionStatus::Active => "active",
        SessionStatus::Completed => "completed",
        SessionStatus::Abandoned => "abandoned",
        SessionStatus::Error => "error",
    }
}
