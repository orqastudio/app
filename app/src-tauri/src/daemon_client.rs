// HTTP client for the OrqaStudio daemon REST API.
//
// The Tauri app is a thin client over the daemon. All artifact graph
// operations, validation, and reload requests are delegated to the daemon
// via HTTP. This module owns the `reqwest::Client` and exposes typed methods
// for each endpoint the app needs.
//
// Port resolution uses `orqa_engine::ports::resolve_daemon_port()` so the
// same `ORQA_PORT_BASE` environment variable controls both the daemon and all
// its clients.

use std::time::Duration;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::Value;

use orqa_engine_types::ports::resolve_daemon_port;
use orqa_engine_types::types::artifact::NavTree;
use orqa_engine_types::types::enforcement::{EnforcementRule, EnforcementViolation};
use orqa_engine_types::types::governance::GovernanceScanResult;
use orqa_engine_types::types::lesson::Lesson;
use orqa_engine_types::types::settings::SidecarStatus;
use orqa_engine_types::{AppliedFix, ArtifactNode, IntegrityCheck, TraceabilityResult};

use crate::error::OrqaError;

// ---------------------------------------------------------------------------
// Daemon response shapes
//
// These types mirror the exact JSON the daemon sends. They are intentionally
// separate from the engine type definitions so that daemon API changes do not
// silently break deserialization.
// ---------------------------------------------------------------------------

/// Response body for POST /reload.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReloadResponse {
    /// Fixed string "reloaded".
    pub status: String,
    /// Number of artifact nodes after reload.
    pub artifacts: u64,
    /// Number of rule artifacts after reload.
    pub rules: u64,
}

/// Graph health metrics as returned by GET /graph/health.
///
/// Mirrors the `GraphHealth` struct returned by `orqa_validation::metrics::compute_health`.
/// Fields match the outlier-based health model: delivery and learning pipeline connectivity,
/// outlier counts, and graph-theoretic metrics.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct DaemonGraphHealth {
    /// Number of active pipeline outliers past their type-specific grace period.
    pub outlier_count: usize,
    /// Percentage of outliers relative to total active nodes (0.0–100.0).
    pub outlier_percentage: f64,
    /// Fraction of delivery artifacts connected to the main delivery component (0.0–1.0).
    pub delivery_connectivity: f64,
    /// Fraction of learning artifacts connected to each other or to decisions (0.0–1.0).
    pub learning_connectivity: f64,
    /// Average number of edges per node (in + out combined).
    pub avg_degree: f64,
    /// Fraction of nodes in the largest connected component (0.0–1.0).
    pub largest_component_ratio: f64,
    /// Total number of primary nodes.
    pub total_nodes: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Percentage of non-doc nodes that trace to a pillar artifact (0.0–100.0).
    pub pillar_traceability: f64,
    /// Number of broken references (target not in graph).
    pub broken_ref_count: usize,
}

/// Response body for GET /health.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DaemonHealthResponse {
    /// Health status string (e.g., "ok").
    pub status: String,
    /// Seconds since daemon startup.
    #[serde(default)]
    pub uptime_seconds: u64,
    /// Daemon process ID.
    #[serde(default)]
    pub pid: u32,
    /// Crate version string.
    #[serde(default)]
    pub version: String,
    /// Total number of governance artifacts indexed.
    #[serde(default)]
    pub artifact_count: u64,
    /// Number of rule artifacts indexed.
    #[serde(default)]
    pub rule_count: u64,
}

/// Response body for GET /graph/stats.
///
/// Mirrors `GraphStats` but uses the daemon's field naming (broken_refs vs broken_ref_count).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GraphStatsResponse {
    /// Total artifact node count.
    pub node_count: usize,
    /// Total directed edge count.
    pub edge_count: usize,
    /// Nodes with no incoming references.
    pub orphan_count: usize,
    /// Number of references to non-existent artifacts.
    pub broken_refs: usize,
}

/// Response body for GET /artifacts/:id/content.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ContentResponse {
    /// Raw markdown content of the artifact file.
    pub content: String,
}

/// Response body for PUT /artifacts/:id.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UpdateArtifactResponse {
    /// Artifact identifier.
    pub id: String,
    /// Name of the updated frontmatter field.
    pub field: String,
    /// New value written to the field.
    pub new_value: Value,
}

/// Response body for POST /validation/scan.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ValidationScanResponse {
    /// All integrity check results from the scan.
    pub checks: Vec<IntegrityCheck>,
    /// Current graph health snapshot at scan time.
    pub health: DaemonGraphHealth,
}

/// Response body for POST /validation/fix.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ValidationFixResponse {
    /// All integrity check results after applying fixes.
    pub checks: Vec<IntegrityCheck>,
    /// Current graph health snapshot after fixes.
    pub health: DaemonGraphHealth,
    /// List of fixes that were automatically applied.
    pub fixes_applied: Vec<AppliedFix>,
}

/// Response body for POST /enforcement/scan.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EnforcementScanResponse {
    /// Parsed rules from the learning/rules area.
    pub rules: Vec<EnforcementRule>,
    /// Governance scan result across all artifact areas.
    pub governance: GovernanceScanResult,
    /// Total artifact count across scanned areas.
    pub total_artifacts: usize,
}

/// A registered CLI tool from plugin manifests.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RegisteredCliTool {
    /// Plugin that registered this tool.
    pub plugin: String,
    /// Tool key within the plugin.
    pub key: String,
    /// Human-readable display name.
    pub name: String,
    /// Command to execute.
    pub command: String,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Human-readable description.
    pub description: Option<String>,
}

/// Result from running a CLI tool.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CliToolResult {
    /// Plugin that owns this tool.
    pub plugin: String,
    /// Tool key within the plugin.
    pub key: String,
    /// Standard output from the process.
    pub stdout: String,
    /// Standard error from the process.
    pub stderr: String,
    /// Process exit code.
    pub exit_code: i32,
    /// Execution duration in milliseconds.
    pub duration_ms: u64,
}

/// Status of a CLI tool (last run info).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CliToolStatus {
    /// Plugin that owns this tool.
    pub plugin: String,
    /// Tool key.
    pub key: String,
    /// Last run result, or None if never run.
    pub last_run: Option<CliToolResult>,
}

/// A registered git hook from plugin manifests.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RegisteredHook {
    /// Git hook event (e.g. "pre-commit").
    pub event: String,
    /// Plugin that registered this hook.
    pub plugin: String,
    /// Command to run.
    pub command: String,
}

/// Result from generating hook dispatcher scripts.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HookGenerationResult {
    /// Number of dispatcher scripts written.
    pub scripts_written: usize,
    /// Number of legacy hooks preserved.
    pub legacy_preserved: usize,
}

/// Request body for POST /lessons.
#[derive(Debug, serde::Serialize)]
pub struct NewLessonRequest {
    /// Short title describing the lesson.
    pub title: String,
    /// Category key.
    pub category: String,
    /// Full markdown body.
    pub body: String,
}

/// Response for GET /projects/settings.
pub type ProjectSettingsResponse = Value;

/// A scan result from POST /projects/scan.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectScanResult {
    /// Detected stack information.
    pub stack: Value,
    /// Governance coverage metrics.
    pub governance: Value,
}

// ---------------------------------------------------------------------------
// DaemonClient
// ---------------------------------------------------------------------------

/// HTTP client for the OrqaStudio daemon.
///
/// Wraps a `reqwest::Client` with a base URL derived from the daemon port. All
/// methods are async and return `Result<T, OrqaError>`. HTTP errors (non-2xx
/// responses) are mapped to `OrqaError::Sidecar`.
#[derive(Clone, Debug)]
pub struct DaemonClient {
    client: Client,
    base_url: String,
}

impl DaemonClient {
    /// Create a new `DaemonClient` with a 10-second timeout.
    ///
    /// Resolves the daemon port via `orqa_engine::ports::resolve_daemon_port()`.
    pub fn new() -> Result<Self, OrqaError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| OrqaError::Sidecar(format!("failed to build HTTP client: {e}")))?;

        let port = resolve_daemon_port();
        let base_url = format!("http://127.0.0.1:{port}");

        Ok(Self { client, base_url })
    }

    // ---------------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------------

    /// GET `{base_url}{path}` and deserialize the JSON response.
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, OrqaError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable at {url}: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OrqaError::Sidecar(format!(
                "daemon returned HTTP {status}: {body}"
            )));
        }

        response.json::<T>().await.map_err(|e| {
            OrqaError::Sidecar(format!("failed to parse daemon response from {url}: {e}"))
        })
    }

    /// GET `{base_url}{path}` with query parameters and deserialize the JSON response.
    async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, OrqaError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .query(query)
            .send()
            .await
            .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable at {url}: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OrqaError::Sidecar(format!(
                "daemon returned HTTP {status}: {body}"
            )));
        }

        response.json::<T>().await.map_err(|e| {
            OrqaError::Sidecar(format!("failed to parse daemon response from {url}: {e}"))
        })
    }

    /// POST `{base_url}{path}` with a JSON body and deserialize the JSON response.
    async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, OrqaError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable at {url}: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OrqaError::Sidecar(format!(
                "daemon returned HTTP {status}: {body}"
            )));
        }

        response.json::<T>().await.map_err(|e| {
            OrqaError::Sidecar(format!("failed to parse daemon response from {url}: {e}"))
        })
    }

    /// PUT `{base_url}{path}` with a JSON body and deserialize the JSON response.
    async fn put<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, OrqaError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .put(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable at {url}: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OrqaError::Sidecar(format!(
                "daemon returned HTTP {status}: {body}"
            )));
        }

        response.json::<T>().await.map_err(|e| {
            OrqaError::Sidecar(format!("failed to parse daemon response from {url}: {e}"))
        })
    }

    // ---------------------------------------------------------------------------
    // Accessors for raw HTTP primitives
    // ---------------------------------------------------------------------------

    /// Return the base URL for the daemon (e.g. "http://127.0.0.1:10100").
    ///
    /// Used by callers that need to build custom requests (SSE streaming).
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Return a clone of the underlying reqwest client.
    ///
    /// Used by callers that need to send raw HTTP requests (SSE streaming).
    pub fn reqwest_client(&self) -> &Client {
        &self.client
    }

    // ---------------------------------------------------------------------------
    // Lifecycle
    // ---------------------------------------------------------------------------

    /// GET /health — check daemon liveness.
    pub async fn health(&self) -> Result<DaemonHealthResponse, OrqaError> {
        self.get("/health").await
    }

    /// POST /reload — rebuild all cached state from disk.
    pub async fn reload(&self) -> Result<ReloadResponse, OrqaError> {
        self.post("/reload", &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Artifacts
    // ---------------------------------------------------------------------------

    /// GET /artifacts — query artifacts with optional type and status filters.
    pub async fn query_artifacts(
        &self,
        type_filter: Option<&str>,
        status_filter: Option<&str>,
    ) -> Result<Vec<ArtifactNode>, OrqaError> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(t) = type_filter {
            query.push(("type", t));
        }
        if let Some(s) = status_filter {
            query.push(("status", s));
        }
        self.get_with_query("/artifacts", &query).await
    }

    /// GET /artifacts/tree — build the navigation tree from schema and disk scan.
    ///
    /// Returns the full nav tree structure (groups → types → nodes) for the
    /// active project. Returns an empty tree when no project is open.
    pub async fn get_artifact_tree(&self) -> Result<NavTree, OrqaError> {
        self.get("/artifacts/tree").await
    }

    /// GET /artifacts/:id — get a single artifact node by ID.
    pub async fn get_artifact(&self, id: &str) -> Result<ArtifactNode, OrqaError> {
        let path = format!("/artifacts/{id}");
        self.get(&path).await
    }

    /// GET /artifacts/:id/content — read the raw markdown content of an artifact file.
    pub async fn get_artifact_content(&self, id: &str) -> Result<String, OrqaError> {
        let path = format!("/artifacts/{id}/content");
        let resp: ContentResponse = self.get(&path).await?;
        Ok(resp.content)
    }

    /// PUT /artifacts/:id — update a single frontmatter field in an artifact.
    pub async fn update_artifact_field(
        &self,
        id: &str,
        field: &str,
        value: &str,
    ) -> Result<(), OrqaError> {
        let path = format!("/artifacts/{id}");
        let body = serde_json::json!({ "field": field, "value": value });
        let _: UpdateArtifactResponse = self.put(&path, &body).await?;
        Ok(())
    }

    /// GET /artifacts/:id/traceability — compute the traceability chain for an artifact.
    pub async fn get_traceability(&self, id: &str) -> Result<TraceabilityResult, OrqaError> {
        let path = format!("/artifacts/{id}/traceability");
        self.get(&path).await
    }

    // ---------------------------------------------------------------------------
    // Graph analytics
    // ---------------------------------------------------------------------------

    /// GET /graph/stats — return summary statistics about the artifact graph.
    pub async fn get_graph_stats(&self) -> Result<GraphStatsResponse, OrqaError> {
        self.get("/graph/stats").await
    }

    /// GET /graph/health — return extended structural health metrics.
    pub async fn get_graph_health(&self) -> Result<DaemonGraphHealth, OrqaError> {
        self.get("/graph/health").await
    }

    // ---------------------------------------------------------------------------
    // Validation
    // ---------------------------------------------------------------------------

    /// POST /validation/scan — run all integrity checks on the artifact graph.
    pub async fn run_validation_scan(&self) -> Result<ValidationScanResponse, OrqaError> {
        self.post("/validation/scan", &serde_json::json!({})).await
    }

    /// POST /validation/fix — run integrity checks and apply auto-fixes.
    pub async fn run_validation_fix(&self) -> Result<ValidationFixResponse, OrqaError> {
        self.post("/validation/fix", &serde_json::json!({ "fix": true }))
            .await
    }

    // ---------------------------------------------------------------------------
    // Enforcement
    // ---------------------------------------------------------------------------

    /// GET /enforcement/rules — list all enforcement rules from .orqa/.
    pub async fn list_enforcement_rules(&self) -> Result<Vec<EnforcementRule>, OrqaError> {
        self.get("/enforcement/rules").await
    }

    /// POST /enforcement/rules/reload — reload enforcement rules from disk.
    pub async fn reload_enforcement_rules(&self) -> Result<Vec<EnforcementRule>, OrqaError> {
        self.post("/enforcement/rules/reload", &serde_json::json!({}))
            .await
    }

    /// GET /enforcement/violations — list recorded enforcement violations.
    pub async fn list_enforcement_violations(
        &self,
    ) -> Result<Vec<EnforcementViolation>, OrqaError> {
        self.get("/enforcement/violations").await
    }

    /// POST /enforcement/scan — run a full governance scan.
    pub async fn enforcement_scan(&self) -> Result<EnforcementScanResponse, OrqaError> {
        self.post("/enforcement/scan", &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Lessons
    // ---------------------------------------------------------------------------

    /// GET /lessons — list all lessons.
    pub async fn list_lessons(&self) -> Result<Vec<Lesson>, OrqaError> {
        self.get("/lessons").await
    }

    /// POST /lessons — create a new lesson.
    pub async fn create_lesson(&self, req: &NewLessonRequest) -> Result<Lesson, OrqaError> {
        self.post("/lessons", req).await
    }

    /// PUT /lessons/:id/recurrence — increment lesson recurrence count.
    pub async fn increment_lesson_recurrence(&self, id: &str) -> Result<Lesson, OrqaError> {
        let path = format!("/lessons/{id}/recurrence");
        self.put(&path, &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Plugins
    // ---------------------------------------------------------------------------

    /// GET /plugins — list all installed plugins.
    pub async fn list_plugins(&self) -> Result<Value, OrqaError> {
        self.get("/plugins").await
    }

    /// GET /plugins/registry — list plugin registry catalog.
    pub async fn list_plugin_registry(&self) -> Result<Value, OrqaError> {
        self.get("/plugins/registry").await
    }

    /// GET /plugins/updates — check for available plugin updates.
    pub async fn check_plugin_updates(&self) -> Result<Value, OrqaError> {
        self.get("/plugins/updates").await
    }

    /// POST /plugins/install/local — install a plugin from a local path.
    pub async fn install_plugin_local(&self, path: &str) -> Result<Value, OrqaError> {
        self.post(
            "/plugins/install/local",
            &serde_json::json!({ "path": path }),
        )
        .await
    }

    /// POST /plugins/install/github — install a plugin from GitHub.
    pub async fn install_plugin_github(
        &self,
        repo: &str,
        version: Option<&str>,
    ) -> Result<Value, OrqaError> {
        self.post(
            "/plugins/install/github",
            &serde_json::json!({ "repo": repo, "version": version }),
        )
        .await
    }

    /// DELETE /plugins/:name — uninstall a plugin.
    pub async fn uninstall_plugin(&self, name: &str) -> Result<(), OrqaError> {
        let url = format!("{}/plugins/{name}", self.base_url);
        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable at {url}: {e}")))?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OrqaError::Sidecar(format!(
                "daemon returned HTTP {status}: {body}"
            )));
        }
        Ok(())
    }

    /// GET /plugins/:name/path — get the filesystem path for an installed plugin.
    pub async fn get_plugin_path(&self, name: &str) -> Result<Value, OrqaError> {
        let path = format!("/plugins/{name}/path");
        self.get(&path).await
    }

    /// GET /plugins/:name — get a specific plugin manifest.
    pub async fn get_plugin(&self, name: &str) -> Result<Value, OrqaError> {
        let path = format!("/plugins/{name}");
        self.get(&path).await
    }

    // ---------------------------------------------------------------------------
    // CLI tools
    // ---------------------------------------------------------------------------

    /// GET /cli-tools — list all registered CLI tools.
    pub async fn list_cli_tools(&self) -> Result<Vec<RegisteredCliTool>, OrqaError> {
        self.get("/cli-tools").await
    }

    /// GET /cli-tools/status — get status of all CLI tools.
    pub async fn get_cli_tool_status(&self) -> Result<Vec<CliToolStatus>, OrqaError> {
        self.get("/cli-tools/status").await
    }

    /// POST /cli-tools/:plugin/:key/run — run a CLI tool.
    pub async fn run_cli_tool(&self, plugin: &str, key: &str) -> Result<CliToolResult, OrqaError> {
        let path = format!("/cli-tools/{plugin}/{key}/run");
        self.post(&path, &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Hooks
    // ---------------------------------------------------------------------------

    /// GET /hooks — list all registered hooks.
    pub async fn list_hooks(&self) -> Result<Vec<RegisteredHook>, OrqaError> {
        self.get("/hooks").await
    }

    /// POST /hooks/generate — generate hook dispatcher scripts.
    pub async fn generate_hook_dispatchers(&self) -> Result<HookGenerationResult, OrqaError> {
        self.post("/hooks/generate", &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Sidecar (claude subprocess status)
    // ---------------------------------------------------------------------------

    /// GET /sidecar/status — get the status of the claude subprocess.
    pub async fn get_sidecar_status(&self) -> Result<SidecarStatus, OrqaError> {
        self.get("/sidecar/status").await
    }

    /// POST /sidecar/restart — restart the claude subprocess.
    pub async fn restart_sidecar(&self) -> Result<SidecarStatus, OrqaError> {
        self.post("/sidecar/restart", &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Workflow transitions
    // ---------------------------------------------------------------------------

    /// GET /workflow/transitions — evaluate all proposed status transitions.
    pub async fn list_workflow_transitions(
        &self,
    ) -> Result<Vec<orqa_engine_types::types::workflow::ProposedTransition>, OrqaError> {
        self.get("/workflow/transitions").await
    }

    /// POST /workflow/transitions/apply — apply a single status transition.
    pub async fn apply_workflow_transition(
        &self,
        artifact_id: &str,
        target_status: &str,
    ) -> Result<Value, OrqaError> {
        self.post(
            "/workflow/transitions/apply",
            &serde_json::json!({ "artifact_id": artifact_id, "target_status": target_status }),
        )
        .await
    }

    // ---------------------------------------------------------------------------
    // Project settings and scan
    // ---------------------------------------------------------------------------

    /// GET /projects/settings — read project.json for the active project.
    ///
    /// Returns the full project settings as a JSON value. Returns an empty object
    /// when no project.json exists.
    pub async fn get_project_settings(&self) -> Result<Value, OrqaError> {
        self.get("/projects/settings").await
    }

    /// PUT /projects/settings — write project.json for the active project.
    ///
    /// Overwrites the full project.json with the provided value. Returns the
    /// written value on success.
    pub async fn write_project_settings(&self, settings: &Value) -> Result<Value, OrqaError> {
        self.put(
            "/projects/settings",
            &serde_json::json!({ "settings": settings }),
        )
        .await
    }

    /// POST /projects/scan — scan the project filesystem for stack and governance info.
    ///
    /// Returns detected languages, frameworks, and governance artifact counts as
    /// a JSON value.
    pub async fn scan_project(&self) -> Result<Value, OrqaError> {
        self.post("/projects/scan", &serde_json::json!({})).await
    }

    // ---------------------------------------------------------------------------
    // Setup
    // ---------------------------------------------------------------------------

    /// GET /setup/embedding-model — check embedding model availability.
    ///
    /// Returns whether the ONNX model files are present and ready for use.
    pub async fn get_embedding_model_status(&self) -> Result<Value, OrqaError> {
        self.get("/setup/embedding-model").await
    }
}

impl Default for DaemonClient {
    fn default() -> Self {
        // new() only fails if reqwest::Client::builder().build() fails, which is
        // effectively infallible on standard platforms (TLS initialization always
        // succeeds with rustls/native-tls). The primary construction path uses
        // DaemonClient::new()? to propagate errors. This impl exists for contexts
        // that require Default (e.g. struct initialization without Result).
        Self::new().unwrap_or_else(|e| {
            // Fall back to a client with no timeout. This preserves the base_url
            // so callers get predictable behaviour rather than a panic.
            let client = Client::new();
            let port = resolve_daemon_port();
            tracing::warn!(error = %e, "DaemonClient::default() fell back to timeout-less client");
            Self {
                client,
                base_url: format!("http://127.0.0.1:{port}"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ---------------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------------

    /// Build a DaemonClient pointed at the given base URL.
    ///
    /// Private fields are accessible inside this test module because the test
    /// module is defined in the same file as DaemonClient. This avoids exposing
    /// a test-only constructor in the production interface.
    fn client_for(base_url: &str) -> DaemonClient {
        DaemonClient {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("test client must build"),
            base_url: base_url.to_owned(),
        }
    }

    // ---------------------------------------------------------------------------
    // Construction tests (no network)
    // ---------------------------------------------------------------------------

    #[test]
    fn daemon_client_creates_without_error() {
        // Only tests construction — no network call.
        let client = DaemonClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn base_url_uses_daemon_port() {
        let client = DaemonClient::new().expect("should create");
        let port = resolve_daemon_port();
        assert!(client.base_url.contains(&port.to_string()));
    }

    #[test]
    fn default_construction_succeeds() {
        // Default::default() must not panic.
        let client = DaemonClient::default();
        assert!(!client.base_url.is_empty());
    }

    #[test]
    fn base_url_returns_expected_scheme_and_host() {
        let client = DaemonClient::new().expect("should create");
        assert!(
            client.base_url().starts_with("http://127.0.0.1:"),
            "base_url must be a localhost HTTP URL, got: {}",
            client.base_url()
        );
    }

    // ---------------------------------------------------------------------------
    // Unreachable daemon — no mock server needed
    // ---------------------------------------------------------------------------

    #[tokio::test]
    async fn health_returns_sidecar_error_when_daemon_unreachable() {
        // Port 1 is typically reserved/closed — always produces a connection error.
        let client = client_for("http://127.0.0.1:1");
        let result = client.health().await;
        assert!(result.is_err(), "health() on unreachable port must fail");
        let err = result.unwrap_err();
        assert!(
            matches!(err, OrqaError::Sidecar(_)),
            "unreachable daemon must produce OrqaError::Sidecar, got: {err:?}"
        );
    }

    #[tokio::test]
    async fn reload_returns_sidecar_error_when_daemon_unreachable() {
        let client = client_for("http://127.0.0.1:1");
        let result = client.reload().await;
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "reload() on unreachable port must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn query_artifacts_returns_sidecar_error_when_unreachable() {
        let client = client_for("http://127.0.0.1:1");
        let result = client.query_artifacts(None, None).await;
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "query_artifacts() on unreachable port must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn get_artifact_returns_sidecar_error_when_unreachable() {
        let client = client_for("http://127.0.0.1:1");
        let result = client.get_artifact("EPIC-test001").await;
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "get_artifact() on unreachable port must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn get_graph_stats_returns_sidecar_error_when_unreachable() {
        let client = client_for("http://127.0.0.1:1");
        let result = client.get_graph_stats().await;
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "get_graph_stats() on unreachable port must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn run_validation_scan_returns_sidecar_error_when_unreachable() {
        let client = client_for("http://127.0.0.1:1");
        let result = client.run_validation_scan().await;
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "run_validation_scan() on unreachable port must produce OrqaError::Sidecar"
        );
    }

    // ---------------------------------------------------------------------------
    // Mock server — success paths
    // ---------------------------------------------------------------------------

    #[tokio::test]
    async fn health_returns_ok_when_daemon_responds() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "uptime_seconds": 42,
                "pid": 1234,
                "version": "0.1.4-dev",
                "artifact_count": 3,
                "rule_count": 1,
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client.health().await.expect("health() must succeed");

        assert_eq!(result.status, "ok");
        assert_eq!(result.artifact_count, 3);
        assert_eq!(result.rule_count, 1);
    }

    #[tokio::test]
    async fn reload_returns_reload_response_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/reload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "reloaded",
                "artifacts": 5,
                "rules": 2,
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client.reload().await.expect("reload() must succeed");

        assert_eq!(result.status, "reloaded");
        assert_eq!(result.artifacts, 5);
        assert_eq!(result.rules, 2);
    }

    #[tokio::test]
    async fn query_artifacts_no_filter_returns_all() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/artifacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": "EPIC-001", "artifact_type": "epic", "title": "Test Epic",
                    "status": null, "path": ".orqa/epics/EPIC-001.md",
                    "description": null, "priority": null, "frontmatter": {},
                    "references_out": [], "references_in": [],
                }
            ])))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .query_artifacts(None, None)
            .await
            .expect("query_artifacts() must succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "EPIC-001");
    }

    #[tokio::test]
    async fn query_artifacts_with_type_filter_sends_query_param() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/artifacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        // query_artifacts with type filter must not panic and must return an array.
        let result = client
            .query_artifacts(Some("epic"), None)
            .await
            .expect("query_artifacts() with type filter must succeed");
        assert!(
            result.is_empty(),
            "mock returns empty array for filtered query"
        );
    }

    #[tokio::test]
    async fn get_artifact_returns_node_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/artifacts/EPIC-001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "EPIC-001",
                "artifact_type": "epic",
                "title": "Test Epic",
                "status": null,
                "path": ".orqa/epics/EPIC-001.md",
                "description": null,
                "priority": null,
                "frontmatter": {},
                "references_out": [],
                "references_in": [],
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .get_artifact("EPIC-001")
            .await
            .expect("get_artifact() must succeed for known ID");

        assert_eq!(result.id, "EPIC-001");
        assert_eq!(result.artifact_type, "epic");
    }

    #[tokio::test]
    async fn get_artifact_returns_sidecar_error_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/artifacts/EPIC-notexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "artifact 'EPIC-notexist' not found",
                "code": "NOT_FOUND"
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client.get_artifact("EPIC-notexist").await;

        assert!(result.is_err(), "get_artifact() on 404 must fail");
        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "404 from daemon must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn get_graph_stats_returns_stats_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/graph/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "node_count": 10,
                "edge_count": 5,
                "orphan_count": 2,
                "broken_refs": 0,
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .get_graph_stats()
            .await
            .expect("get_graph_stats() must succeed");

        assert_eq!(result.node_count, 10);
        assert_eq!(result.edge_count, 5);
        assert_eq!(result.broken_refs, 0);
    }

    #[tokio::test]
    async fn health_returns_sidecar_error_on_500() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client.health().await;

        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "HTTP 500 from daemon must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn health_returns_sidecar_error_on_invalid_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("this is not json")
                    .insert_header("content-type", "application/json"),
            )
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client.health().await;

        assert!(
            matches!(result.unwrap_err(), OrqaError::Sidecar(_)),
            "invalid JSON from daemon must produce OrqaError::Sidecar"
        );
    }

    #[tokio::test]
    async fn run_validation_scan_returns_scan_response_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/validation/scan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "checks": [],
                "health": {
                    "outlier_count": 0,
                    "outlier_percentage": 0.0,
                    "delivery_connectivity": 1.0,
                    "learning_connectivity": 1.0,
                    "avg_degree": 0.0,
                    "largest_component_ratio": 1.0,
                    "total_nodes": 3,
                    "total_edges": 0,
                    "pillar_traceability": 0.0,
                    "broken_ref_count": 0,
                }
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .run_validation_scan()
            .await
            .expect("run_validation_scan() must succeed");

        assert!(result.checks.is_empty(), "no checks in mock response");
        assert_eq!(result.health.total_nodes, 3);
    }

    #[tokio::test]
    async fn list_hooks_returns_empty_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hooks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .list_hooks()
            .await
            .expect("list_hooks() must succeed");
        assert!(result.is_empty(), "empty hooks list from mock");
    }

    #[tokio::test]
    async fn query_artifacts_with_status_filter_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/artifacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .query_artifacts(None, Some("active"))
            .await
            .expect("query with status filter must succeed");
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_graph_health_returns_health_metrics() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/graph/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "outlier_count": 1,
                "outlier_percentage": 33.3,
                "delivery_connectivity": 0.5,
                "learning_connectivity": 0.8,
                "avg_degree": 1.2,
                "largest_component_ratio": 0.9,
                "total_nodes": 8,
                "total_edges": 6,
                "pillar_traceability": 75.0,
                "broken_ref_count": 0,
            })))
            .mount(&server)
            .await;

        let client = client_for(&server.uri());
        let result = client
            .get_graph_health()
            .await
            .expect("get_graph_health() must succeed");

        assert_eq!(result.total_nodes, 8);
        assert_eq!(result.broken_ref_count, 0);
        assert!((result.delivery_connectivity - 0.5).abs() < 1e-9);
    }
}
