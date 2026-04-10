// HTTP client for `/health-snapshots` endpoints.
//
// Mirrors HealthRepository from engine/storage/src/traits.rs.

use orqa_engine_types::types::health::{HealthSnapshot, NewHealthSnapshot};

use crate::{parse_response, DbError};

/// Sub-client for `/health-snapshots` daemon endpoints.
///
/// Obtained via `DbClient::health_snapshots()`.
pub struct HealthSnapshotsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl HealthSnapshotsClient<'_> {
    /// Store a new health snapshot for a project and return the inserted row.
    ///
    /// Calls POST /health-snapshots.
    pub async fn create(
        &self,
        project_id: i64,
        snapshot: &NewHealthSnapshot,
    ) -> Result<HealthSnapshot, DbError> {
        let mut body =
            serde_json::to_value(snapshot).map_err(|e| DbError::Deserialization(e.to_string()))?;
        // Inject project_id into the body alongside the snapshot fields.
        if let Some(obj) = body.as_object_mut() {
            obj.insert("project_id".to_owned(), serde_json::json!(project_id));
        }
        let resp = self
            .http
            .post(format!("{}/health-snapshots", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get a single snapshot by its ID.
    ///
    /// Calls GET /health-snapshots/:id.
    pub async fn get(&self, id: i64) -> Result<HealthSnapshot, DbError> {
        let resp = self
            .http
            .get(format!("{}/health-snapshots/{id}", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get the most recent N snapshots for a project, ordered newest first.
    ///
    /// Calls GET /health-snapshots?project_id=...&limit=...
    pub async fn get_recent(
        &self,
        project_id: i64,
        limit: i64,
    ) -> Result<Vec<HealthSnapshot>, DbError> {
        let resp = self
            .http
            .get(format!("{}/health-snapshots", self.base_url))
            .query(&[
                ("project_id", project_id.to_string()),
                ("limit", limit.to_string()),
            ])
            .send()
            .await?;
        parse_response(resp).await
    }
}
