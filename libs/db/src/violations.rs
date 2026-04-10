// HTTP client for `/violations` endpoints.
//
// Mirrors ViolationRepository from engine/storage/src/traits.rs.

use orqa_engine_types::types::enforcement::EnforcementViolation;

use crate::{parse_empty_response, parse_response, DbError};

/// Sub-client for `/violations` daemon endpoints.
///
/// Obtained via `DbClient::violations()`.
pub struct ViolationsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl ViolationsClient<'_> {
    /// Record a new enforcement violation.
    ///
    /// Calls POST /violations.
    pub async fn record(
        &self,
        project_id: i64,
        rule_name: &str,
        action: &str,
        tool_name: &str,
        detail: Option<&str>,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({
            "project_id": project_id,
            "rule_name": rule_name,
            "action": action,
            "tool_name": tool_name,
            "detail": detail,
        });
        let resp = self
            .http
            .post(format!("{}/violations", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Query enforcement violation history for a project, most recent first.
    ///
    /// Calls GET /violations?project_id=...&limit=...
    pub async fn list_for_project(
        &self,
        project_id: i64,
        limit: Option<u32>,
    ) -> Result<Vec<EnforcementViolation>, DbError> {
        let mut req = self
            .http
            .get(format!("{}/violations", self.base_url))
            .query(&[("project_id", project_id.to_string())]);
        if let Some(l) = limit {
            req = req.query(&[("limit", l.to_string())]);
        }
        let resp = req.send().await?;
        parse_response(resp).await
    }
}
