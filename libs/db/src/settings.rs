// HTTP client for `/settings` endpoints.
//
// Mirrors SettingsRepository from engine/storage/src/traits.rs.

use std::collections::HashMap;

use crate::{parse_empty_response, parse_response, DbError};

/// Sub-client for `/settings` daemon endpoints.
///
/// Obtained via `DbClient::settings()`.
pub struct SettingsClient<'a> {
    /// Shared HTTP client owned by `DbClient`.
    pub(crate) http: &'a reqwest::Client,
    /// Daemon base URL, e.g. `"http://127.0.0.1:10421"`.
    pub(crate) base_url: &'a str,
}

impl SettingsClient<'_> {
    /// Get a single setting value by key and scope.
    ///
    /// Returns `None` if the key does not exist in the given scope.
    /// Calls GET /settings?key=...&scope=...
    pub async fn get(&self, key: &str, scope: &str) -> Result<Option<serde_json::Value>, DbError> {
        let resp = self
            .http
            .get(format!("{}/settings", self.base_url))
            .query(&[("key", key), ("scope", scope)])
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let val: serde_json::Value = parse_response(resp).await?;
        Ok(Some(val["value"].clone()))
    }

    /// Set a setting value (upsert by key + scope).
    ///
    /// Calls PUT /settings/{key} with body {"value": X, "scope": "..."}.
    pub async fn set(
        &self,
        key: &str,
        value: &serde_json::Value,
        scope: &str,
    ) -> Result<(), DbError> {
        let body = serde_json::json!({ "value": value, "scope": scope });
        let key_encoded = key.replace('/', "%2F");
        let resp = self
            .http
            .put(format!("{}/settings/{key_encoded}", self.base_url))
            .json(&body)
            .send()
            .await?;
        parse_empty_response(resp).await
    }

    /// Get all settings regardless of scope, ordered by key.
    ///
    /// Calls GET /settings/all.
    pub async fn get_all_any_scope(&self) -> Result<HashMap<String, serde_json::Value>, DbError> {
        let resp = self
            .http
            .get(format!("{}/settings/all", self.base_url))
            .send()
            .await?;
        parse_response(resp).await
    }

    /// Get all settings for a given scope, ordered by key.
    ///
    /// Calls GET /settings/all?scope=...
    pub async fn get_all(
        &self,
        scope: &str,
    ) -> Result<HashMap<String, serde_json::Value>, DbError> {
        let resp = self
            .http
            .get(format!("{}/settings/all", self.base_url))
            .query(&[("scope", scope)])
            .send()
            .await?;
        parse_response(resp).await
    }
}
