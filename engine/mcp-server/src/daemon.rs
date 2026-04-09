//! Blocking HTTP client for the OrqaStudio daemon.
//!
//! Wraps the daemon REST API with typed helper methods. All calls are
//! synchronous (blocking) because the MCP server's JSON-RPC loop is
//! single-threaded.
//!
//! # Endpoints used
//!
//! | Method | Path                          | Used by tool          |
//! |--------|-------------------------------|-----------------------|
//! | GET    | `/health`                     | `graph_stats`         |
//! | GET    | `/artifacts`                  | `graph_query`         |
//! | GET    | `/artifacts/:id`              | `graph_resolve`       |
//! | POST   | `/validation/scan`            | `graph_validate`      |
//! | GET    | `/artifacts/:id/traceability` | `graph_traceability`  |
//! | POST   | `/reload`                     | `graph_refresh`       |
//! | POST   | `/search/regex`               | `search_regex`        |
//! | POST   | `/search/semantic`            | `search_semantic`     |
//! | GET    | `/search/status`              | `search_status`       |
//! | POST   | `/search/index`               | `search_index`        |
//! | POST   | `/search/embed`               | `search_embed`        |

use serde_json::Value;

use orqa_engine::ports::resolve_daemon_port;

use crate::error::McpError;

/// Default port for the OrqaStudio daemon.
///
/// Delegates to `orqa_engine::ports::resolve_daemon_port()` which reads
/// `ORQA_PORT_BASE` (default 10100).
pub fn default_daemon_port() -> u16 {
    resolve_daemon_port()
}

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

    fn get_with_query(&self, path: &str, params: &[(&str, &str)]) -> Result<Value, McpError> {
        let url = format!("{}{path}", self.base_url);
        self.client
            .get(&url)
            .query(params)
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

    /// `GET /artifacts` — query the artifact graph.
    ///
    /// Accepts any subset of query parameters: `type`, `status`, `id`, `search`.
    pub fn query(&self, params: &Value) -> Result<Value, McpError> {
        let query_pairs: Vec<(String, String)> = params
            .as_object()
            .into_iter()
            .flat_map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
            })
            .collect();
        let pairs_ref: Vec<(&str, &str)> = query_pairs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        self.get_with_query("/artifacts", &pairs_ref)
    }

    /// `GET /artifacts/:id` — read and parse a single artifact by ID.
    pub fn parse(&self, artifact_id: &str) -> Result<Value, McpError> {
        self.get(&format!("/artifacts/{artifact_id}"))
    }

    /// `POST /validation/scan` — full graph validation report.
    pub fn validate(&self) -> Result<Value, McpError> {
        self.post("/validation/scan", &serde_json::json!({}))
    }

    /// `GET /artifacts/:id/traceability` — full traceability chain for an artifact.
    ///
    /// Uses the daemon's cached graph for low-latency responses.
    pub fn traceability(&self, artifact_id: &str) -> Result<Value, McpError> {
        self.get(&format!("/artifacts/{artifact_id}/traceability"))
    }

    /// `POST /reload` — rebuild all graph state from disk.
    pub fn reload(&self) -> Result<Value, McpError> {
        self.post("/reload", &serde_json::json!({}))
    }

    /// `POST /search/regex` — regex search over the indexed codebase.
    ///
    /// Returns an array of `SearchResult` objects from the daemon's search index.
    pub fn search_regex(
        &self,
        pattern: &str,
        path_filter: Option<&str>,
        max_results: u32,
    ) -> Result<Value, McpError> {
        let mut body = serde_json::json!({
            "pattern": pattern,
            "max_results": max_results
        });
        if let Some(filter) = path_filter {
            body["path_filter"] = serde_json::json!(filter);
        }
        self.post("/search/regex", &body)
    }

    /// `POST /search/semantic` — semantic search using ONNX embeddings.
    ///
    /// Returns an array of `SearchResult` objects ranked by cosine similarity.
    pub fn search_semantic(&self, query: &str, max_results: u32) -> Result<Value, McpError> {
        self.post(
            "/search/semantic",
            &serde_json::json!({
                "query": query,
                "max_results": max_results
            }),
        )
    }

    /// `GET /search/status` — get the current search index status.
    ///
    /// Returns an `IndexStatus` object with chunk count and embedding state.
    pub fn search_status(&self) -> Result<Value, McpError> {
        self.get("/search/status")
    }

    /// `POST /search/index` — trigger re-indexing of the codebase.
    ///
    /// Returns an `IndexResponse` with the resulting chunk count.
    pub fn search_index(&self, exclude: &[String]) -> Result<Value, McpError> {
        self.post("/search/index", &serde_json::json!({ "exclude": exclude }))
    }

    /// `POST /search/embed` — generate ONNX embeddings for un-embedded chunks.
    ///
    /// Returns an `IndexResponse` with the count of newly embedded chunks.
    pub fn search_embed(&self) -> Result<Value, McpError> {
        self.post("/search/embed", &serde_json::json!({}))
    }
}
