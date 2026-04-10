// HTTP client for `/themes` endpoints.
//
// Mirrors ThemeRepository from engine/storage/src/traits.rs.
// ThemeRow and ThemeOverrideRow are redefined here with serde derives since
// libs/db does not depend on engine/storage.

use serde::{Deserialize, Serialize};

use crate::{parse_empty_response, parse_response, DbError};

/// A raw theme row from the `project_themes` table.
///
/// Mirrors `engine/storage/src/repo/themes.rs::ThemeRow`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeRow {
    /// Primary key.
    pub id: i64,
    /// Foreign key to the `projects` table.
    pub project_id: i64,
    /// Path to the source file this theme was extracted from.
    pub source_file: String,
    /// Hash of the source file at extraction time, for change detection.
    pub source_hash: String,
    /// ISO-8601 timestamp when this theme was extracted.
    pub extracted_at: String,
    /// JSON-encoded light-mode design token map.
    pub tokens_light: String,
    /// JSON-encoded dark-mode design token map, if present.
    pub tokens_dark: Option<String>,
    /// JSON-encoded list of unmapped token names, if any.
    pub unmapped: Option<String>,
    /// Whether this is the currently active theme for the project.
    pub is_active: bool,
}

/// A raw override row from the `project_theme_overrides` table.
///
/// Mirrors `engine/storage/src/repo/themes.rs::ThemeOverrideRow`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeOverrideRow {
    /// Primary key.
    pub id: i64,
    /// Foreign key to the `projects` table.
    pub project_id: i64,
    /// Design token name being overridden (e.g., "primary").
    pub token_name: String,
    /// Override value for light mode.
    pub value_light: String,
    /// Override value for dark mode, if set.
    pub value_dark: Option<String>,
}

/// Sub-client for `/themes` daemon endpoints.
///
/// Obtained via `DbClient::themes()`.
pub struct ThemesClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl ThemesClient<'_> {
    /// Get all active themes for a project, ordered by source file path.
    ///
    /// Calls GET /themes?project_id=...
    pub async fn get_themes(&self, project_id: i64) -> Result<Vec<ThemeRow>, DbError> {
        let resp = self
            .http
            .get(format!("{}/themes", self.base_url))
            .query(&[("project_id", project_id)])
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get all theme overrides for a project, ordered by token name.
    ///
    /// Calls GET /themes/overrides?project_id=...
    pub async fn get_overrides(&self, project_id: i64) -> Result<Vec<ThemeOverrideRow>, DbError> {
        let resp = self
            .http
            .get(format!("{}/themes/overrides", self.base_url))
            .query(&[("project_id", project_id)])
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Set (upsert) a theme override for a specific token.
    ///
    /// Calls PUT /themes/overrides/:token.
    pub async fn set_override(
        &self,
        project_id: i64,
        token_name: &str,
        value_light: &str,
        value_dark: Option<&str>,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({
            "project_id": project_id,
            "value_light": value_light,
            "value_dark": value_dark,
        });
        let resp = self
            .http
            .put(format!("{}/themes/overrides/{token_name}", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Clear all theme overrides for a project.
    ///
    /// Calls DELETE /themes/overrides?project_id=...
    pub async fn clear_overrides(&self, project_id: i64) -> Result<(), DbError> {
        let resp = self
            .http
            .delete(format!("{}/themes/overrides", self.base_url))
            .query(&[("project_id", project_id)])
            .send()
            .await?;
        parse_empty_response(resp).await
    }
}
