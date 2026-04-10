// HTTP client for `/projects` endpoints.
//
// Mirrors ProjectRepository from engine/storage/src/traits.rs. Every method
// issues an HTTP request to the daemon and returns the typed result.

use orqa_engine_types::types::project::{DetectedStack, Project, ProjectSummary};

use crate::{parse_empty_response, parse_response, DbError};

/// Sub-client for `/projects` daemon endpoints.
///
/// Obtained via `DbClient::projects()`. Borrows the shared `reqwest::Client`
/// and base URL for the duration of each call.
pub struct ProjectsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl ProjectsClient<'_> {
    /// Create a new project record and return the full row.
    ///
    /// Calls POST /projects/open with a name and path.
    pub async fn create(
        &self,
        name: &str,
        path: &str,
        description: Option<&str>,
    ) -> Result<Project, DbError> {
        let body = serde_json::json!({
            "name": name,
            "path": path,
            "description": description,
        });
        let resp = self
            .http
            .post(format!("{}/projects/open", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get a project by its primary key.
    ///
    /// Calls GET /projects/:id.
    pub async fn get(&self, id: i64) -> Result<Project, DbError> {
        let resp = self
            .http
            .get(format!("{}/projects/{id}", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get a project by its filesystem path.
    ///
    /// Calls GET /projects/by-path?path=...
    pub async fn get_by_path(&self, path: &str) -> Result<Project, DbError> {
        let resp = self
            .http
            .get(format!("{}/projects/by-path", self.base_url))
            .query(&[("path", path)])
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get the most recently updated project, or `None` if none exist.
    ///
    /// Calls GET /projects/active.
    pub async fn get_active(&self) -> Result<Option<Project>, DbError> {
        let resp = self
            .http
            .get(format!("{}/projects/active", self.base_url))
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        parse_response(resp).await.map(Some)
    }

    /// List all projects with summary info.
    ///
    /// Calls GET /projects.
    pub async fn list(&self) -> Result<Vec<ProjectSummary>, DbError> {
        let resp = self
            .http
            .get(format!("{}/projects", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Touch the `updated_at` timestamp for a project.
    ///
    /// Calls POST /projects/:id/touch.
    pub async fn touch_updated_at(&self, id: i64) -> Result<(), DbError> {
        let resp = self
            .http
            .post(format!("{}/projects/{id}/touch", self.base_url))
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Update the detected technology stack for a project.
    ///
    /// Calls PUT /projects/:id/stack.
    pub async fn update_detected_stack(
        &self,
        id: i64,
        stack: &DetectedStack,
    ) -> Result<(), DbError> {
        let resp = self
            .http
            .put(format!("{}/projects/{id}/stack", self.base_url))
            .json(stack)
            .send()
            .await?;
        parse_empty_response(resp).await
    }
}
