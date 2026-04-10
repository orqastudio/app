//! orqa-db: typed HTTP client for the OrqaStudio daemon storage API.
//!
//! The daemon exposes all storage operations over HTTP. This crate provides
//! typed async methods for every operation, mirroring the repository traits in
//! `engine/storage/src/traits.rs`. The app and devtools import this crate
//! instead of `engine/storage` — they never open the SQLite database directly.
//!
//! # Usage
//! ```ignore
//! let db = DbClient::new("http://127.0.0.1:10421");
//! let projects = db.projects().list().await?;
//! ```

#![warn(missing_docs)]

/// HTTP client for the `/devtools-sessions` endpoints.
pub mod devtools;
/// Typed error variants for daemon HTTP API failures.
pub mod error;
/// HTTP client for the `/health-snapshots` endpoints.
pub mod health_snapshots;
/// HTTP client for the `/issue-groups` endpoints.
pub mod issue_groups;
/// HTTP client for the `/messages` endpoints.
pub mod messages;
/// HTTP client for the `/projects` endpoints.
pub mod projects;
/// HTTP client for the `/sessions` endpoints.
pub mod sessions;
/// HTTP client for the `/settings` endpoints.
pub mod settings;
/// HTTP client for the `/themes` endpoints.
pub mod themes;
/// HTTP client for the `/violations` endpoints.
pub mod violations;

pub use error::DbError;

/// Typed HTTP client for the OrqaStudio daemon storage API.
///
/// Owns the underlying `reqwest::Client` and the daemon base URL. Sub-clients
/// are accessed via method calls and borrow both for the duration of the call.
/// `Clone` is cheap — `reqwest::Client` is `Arc`-backed internally.
#[derive(Clone)]
pub struct DbClient {
    http: reqwest::Client,
    base_url: String,
}

impl DbClient {
    /// Create a new `DbClient` with a default `reqwest::Client`.
    ///
    /// `base_url` must be the full daemon origin, e.g. `"http://127.0.0.1:10421"`.
    pub fn new(base_url: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.to_owned(),
        }
    }

    /// Create a `DbClient` with a pre-configured `reqwest::Client`.
    ///
    /// Use this when the caller needs custom timeouts, TLS configuration, or
    /// connection pooling settings.
    pub fn with_client(client: reqwest::Client, base_url: &str) -> Self {
        Self {
            http: client,
            base_url: base_url.to_owned(),
        }
    }

    /// Return a sub-client for `/projects` operations.
    pub fn projects(&self) -> projects::ProjectsClient<'_> {
        projects::ProjectsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/sessions` operations.
    pub fn sessions(&self) -> sessions::SessionsClient<'_> {
        sessions::SessionsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/messages` operations.
    pub fn messages(&self) -> messages::MessagesClient<'_> {
        messages::MessagesClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/settings` operations.
    pub fn settings(&self) -> settings::SettingsClient<'_> {
        settings::SettingsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/themes` operations.
    pub fn themes(&self) -> themes::ThemesClient<'_> {
        themes::ThemesClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/violations` operations.
    pub fn violations(&self) -> violations::ViolationsClient<'_> {
        violations::ViolationsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/health-snapshots` operations.
    pub fn health_snapshots(&self) -> health_snapshots::HealthSnapshotsClient<'_> {
        health_snapshots::HealthSnapshotsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/devtools-sessions` operations.
    pub fn devtools(&self) -> devtools::DevtoolsClient<'_> {
        devtools::DevtoolsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }

    /// Return a sub-client for `/issue-groups` operations.
    pub fn issue_groups(&self) -> issue_groups::IssueGroupsClient<'_> {
        issue_groups::IssueGroupsClient {
            http: &self.http,
            base_url: &self.base_url,
        }
    }
}

// ---------------------------------------------------------------------------
// Shared response-handling helper
//
// Extracted here so every sub-client can use the same error-extraction logic
// without duplicating the reqwest::Response handling pattern.
// ---------------------------------------------------------------------------

/// Parse a reqwest `Response` into `T` on 2xx, or extract a `DbError` on failure.
///
/// On non-2xx responses, attempts to deserialize `{ "error": "...", "code": "..." }`
/// from the body. Falls back to a generic `DbError::Http` with an empty code if
/// the body cannot be parsed.
pub(crate) async fn parse_response<T>(response: reqwest::Response) -> Result<T, DbError>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if status.is_success() {
        let text = response.text().await.map_err(DbError::Network)?;
        serde_json::from_str::<T>(&text).map_err(|e| DbError::Deserialization(e.to_string()))
    } else {
        let status_u16 = status.as_u16();
        let text = response.text().await.unwrap_or_default();
        let body: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        Err(DbError::Http {
            status: status_u16,
            code: body["code"].as_str().unwrap_or("UNKNOWN").to_owned(),
            error: body["error"].as_str().unwrap_or(&text).to_owned(),
        })
    }
}

/// Parse a 2xx response with an empty body (unit return).
///
/// On non-2xx, extracts the structured error body identically to `parse_response`.
pub(crate) async fn parse_empty_response(response: reqwest::Response) -> Result<(), DbError> {
    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        let status_u16 = status.as_u16();
        let text = response.text().await.unwrap_or_default();
        let body: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        Err(DbError::Http {
            status: status_u16,
            code: body["code"].as_str().unwrap_or("UNKNOWN").to_owned(),
            error: body["error"].as_str().unwrap_or(&text).to_owned(),
        })
    }
}
