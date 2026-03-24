//! Blocking HTTP client for the `orqa-validation` daemon.
//!
//! Wraps the daemon REST API with typed helper methods. All calls are
//! synchronous (blocking) because the MCP server's JSON-RPC loop is
//! single-threaded.
//!
//! # Endpoints used
//!
//! | Method | Path        | Used by tool       |
//! |--------|-------------|--------------------|
//! | GET    | `/health`   | `graph_stats`      |
//! | POST   | `/query`    | `graph_query`      |
//! | POST   | `/parse`    | `graph_resolve`    |
//! | POST   | `/validate` | `graph_validate`   |
//! | POST   | `/reload`   | `graph_refresh`    |

use serde_json::Value;

use crate::error::McpError;

/// Default port for the validation daemon.
pub const DEFAULT_DAEMON_PORT: u16 = 9258;

/// Blocking HTTP client bound to a specific daemon instance.
pub struct DaemonClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl DaemonClient {
    /// Create a new client for `http://127.0.0.1:<port>`.
    pub fn new(port: u16) -> Self {
        Self {
            base_url: format!("http://127.0.0.1:{port}"),
            client: reqwest::blocking::Client::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Raw HTTP helpers
    // -----------------------------------------------------------------------

    fn get(&self, path: &str) -> Result<Value, McpError> {
        let url = format!("{}{path}", self.base_url);
        self.client
            .get(&url)
            .send()
            .map_err(|e| McpError::DaemonUnreachable(e.to_string()))?
            .json::<Value>()
            .map_err(|e| McpError::Protocol(e.to_string()))
    }

    fn post(&self, path: &str, body: &Value) -> Result<Value, McpError> {
        let url = format!("{}{path}", self.base_url);
        self.client
            .post(&url)
            .json(body)
            .send()
            .map_err(|e| McpError::DaemonUnreachable(e.to_string()))?
            .json::<Value>()
            .map_err(|e| McpError::Protocol(e.to_string()))
    }

    // -----------------------------------------------------------------------
    // Typed API
    // -----------------------------------------------------------------------

    /// `GET /health` — returns artifact count, rule count, and status.
    pub fn health(&self) -> Result<Value, McpError> {
        self.get("/health")
    }

    /// `POST /query` — query the artifact graph.
    ///
    /// Accepts any subset of: `{ "type": "...", "status": "...", "id": "..." }`.
    pub fn query(&self, params: &Value) -> Result<Value, McpError> {
        self.post("/query", params)
    }

    /// `POST /parse` — read and parse a single artifact file.
    ///
    /// `file` must be an absolute path.
    pub fn parse(&self, absolute_path: &str) -> Result<Value, McpError> {
        self.post("/parse", &serde_json::json!({ "file": absolute_path }))
    }

    /// `POST /validate` — full graph validation report.
    pub fn validate(&self) -> Result<Value, McpError> {
        self.post("/validate", &serde_json::json!({ "fix": false }))
    }

    /// `POST /traceability` — compute traceability from the daemon's cached graph.
    ///
    /// Uses the daemon's in-memory graph instead of rebuilding from disk.
    pub fn traceability(&self, artifact_id: &str) -> Result<Value, McpError> {
        self.post(
            "/traceability",
            &serde_json::json!({ "artifact_id": artifact_id }),
        )
    }

    /// `POST /reload` — rebuild all graph state from disk.
    pub fn reload(&self) -> Result<Value, McpError> {
        self.post("/reload", &serde_json::json!({}))
    }
}
