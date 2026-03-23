//! LSP server implementation for OrqaStudio artifact validation.
//!
//! Implements the `LanguageServer` trait from `tower-lsp`. Handles:
//! - `initialize` / `initialized` / `shutdown`
//! - `textDocument/didOpen`, `didChange`, `didSave`, `didClose`
//!
//! On each document event the server:
//! 1. Runs fast text-level checks (frontmatter structure, duplicate keys, ID
//!    format) using local logic that works on the unsaved editor buffer.
//! 2. Calls the validation daemon `POST /validate` for graph-level checks
//!    (broken refs, missing inverses, type constraints, cardinality, cycles).
//!
//! The daemon owns all graph state. The LSP never builds the graph directly.

use std::path::{Path, PathBuf};

use reqwest::Client as HttpClient;
use serde_json::Value;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializeParams, InitializeResult, InitializedParams, MessageType,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::validation::validate_file;

// ---------------------------------------------------------------------------
// Backend state
// ---------------------------------------------------------------------------

pub(crate) struct OrqaLspBackend {
    client: Client,
    project_root: PathBuf,
    /// Base URL of the validation daemon, e.g. `http://127.0.0.1:9258`.
    daemon_url: String,
    http: HttpClient,
}

impl OrqaLspBackend {
    pub(crate) fn new(client: Client, project_root: PathBuf, daemon_port: u16) -> Self {
        Self {
            client,
            project_root,
            daemon_url: format!("http://127.0.0.1:{daemon_port}"),
            http: HttpClient::new(),
        }
    }

    /// Compute the relative path of `uri` from the project root, with forward slashes.
    fn relative_path(&self, uri: &Url) -> String {
        let path = uri.to_file_path().unwrap_or_default();
        path.strip_prefix(&self.project_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    /// Extract the `id:` value from YAML frontmatter, if present.
    fn extract_artifact_id(content: &str) -> Option<String> {
        if !content.starts_with("---\n") {
            return None;
        }
        let end = content[4..].find("\n---")?;
        let frontmatter = &content[4..end + 4];
        frontmatter
            .lines()
            .find(|l| l.starts_with("id:"))
            .map(|l| l.trim_start_matches("id:").trim().trim_matches('"').to_owned())
    }

    /// Tell the daemon to reload its graph from disk.
    ///
    /// Called on `initialized` and on `didSave` of a plugin manifest.
    /// Failures are logged but not fatal — the daemon may not be running.
    async fn daemon_reload(&self) {
        let url = format!("{}/reload", self.daemon_url);
        match self.http.post(&url).json(&serde_json::json!({})).send().await {
            Ok(resp) if resp.status().is_success() => {
                tracing::debug!("daemon reloaded");
            }
            Ok(resp) => {
                tracing::warn!(status = %resp.status(), "daemon reload returned non-success");
            }
            Err(e) => {
                tracing::debug!(error = %e, "daemon not reachable — skipping reload");
            }
        }
    }

    /// Fetch graph-level diagnostics from the daemon for `artifact_id`.
    ///
    /// Calls `POST /validate` and filters the resulting `checks` to those
    /// whose `artifact_id` matches. Returns an empty vec if the daemon is
    /// unavailable or the artifact has no ID yet.
    async fn daemon_validate(&self, artifact_id: Option<&str>) -> Vec<Diagnostic> {
        let Some(artifact_id) = artifact_id else {
            return Vec::new();
        };

        let url = format!("{}/validate", self.daemon_url);
        let resp = match self
            .http
            .post(&url)
            .json(&serde_json::json!({ "fix": false }))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(error = %e, "daemon not reachable — skipping graph checks");
                return Vec::new();
            }
        };

        if !resp.status().is_success() {
            tracing::warn!(status = %resp.status(), "daemon /validate returned non-success");
            return Vec::new();
        }

        let body: Value = match resp.json().await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(error = %e, "failed to parse daemon /validate response");
                return Vec::new();
            }
        };

        let Some(checks) = body.get("checks").and_then(|v| v.as_array()) else {
            return Vec::new();
        };

        checks
            .iter()
            .filter(|c| c.get("artifact_id").and_then(|v| v.as_str()) == Some(artifact_id))
            .filter_map(integrity_check_value_to_diagnostic)
            .collect()
    }

    /// Validate `content` at `uri` and publish diagnostics to the client.
    ///
    /// Combines fast text-level checks (structural frontmatter, duplicate keys,
    /// ID format) with graph-level checks from the validation daemon.
    async fn validate_and_publish(&self, uri: Url, content: &str) {
        let rel_path = self.relative_path(&uri);

        // Text-level checks: frontmatter syntax, ID format, duplicate keys.
        // These run on the unsaved buffer and never need the graph.
        let mut diagnostics = validate_file(&rel_path, content, None, &[]);

        // Graph-level checks: delegated to the daemon which has a live graph.
        let artifact_id = Self::extract_artifact_id(content);
        let graph_diagnostics = self.daemon_validate(artifact_id.as_deref()).await;
        diagnostics.extend(graph_diagnostics);

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

// ---------------------------------------------------------------------------
// Convert a daemon `/validate` check JSON value to an LSP Diagnostic
// ---------------------------------------------------------------------------

fn integrity_check_value_to_diagnostic(check: &Value) -> Option<Diagnostic> {
    let message = check.get("message").and_then(|v| v.as_str())?;
    let severity_str = check.get("severity").and_then(|v| v.as_str()).unwrap_or("Error");
    let category_str = check.get("category").and_then(|v| v.as_str()).unwrap_or("");
    let fix_desc = check.get("fix_description").and_then(|v| v.as_str());

    let severity = match severity_str {
        "Warning" => DiagnosticSeverity::WARNING,
        "Info" => DiagnosticSeverity::INFORMATION,
        _ => DiagnosticSeverity::ERROR,
    };

    let category_label = category_label_from_str(category_str);
    let mut full_message = format!("{category_label} {message}");
    if let Some(fix) = fix_desc {
        use std::fmt::Write as _;
        let _ = write!(full_message, " (auto-fix: {fix})");
    }

    Some(Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 3)),
        severity: Some(severity),
        source: Some("orqastudio".into()),
        message: full_message,
        ..Default::default()
    })
}

fn category_label_from_str(category: &str) -> &'static str {
    match category {
        "BrokenLink" => "[broken-link]",
        "MissingInverse" => "[missing-inverse]",
        "TypeConstraintViolation" => "[type-constraint]",
        "RequiredRelationshipMissing" => "[required-relationship]",
        "CardinalityViolation" => "[cardinality]",
        "CircularDependency" => "[circular-dep]",
        "InvalidStatus" => "[invalid-status]",
        "BodyTextRefWithoutRelationship" => "[body-ref]",
        "ParentChildInconsistency" => "[parent-child]",
        "DeliveryPathMismatch" => "[delivery-path]",
        "MissingType" => "[missing-type]",
        "MissingStatus" => "[missing-status]",
        "DuplicateRelationship" => "[duplicate-relationship]",
        "FilenameMismatch" => "[filename-mismatch]",
        "SchemaViolation" => "[schema-violation]",
        _ => "[check]",
    }
}

// ---------------------------------------------------------------------------
// LanguageServer implementation
// ---------------------------------------------------------------------------

#[tower_lsp::async_trait]
impl LanguageServer for OrqaLspBackend {
    async fn initialize(&self, _params: InitializeParams) -> RpcResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "orqastudio-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "OrqaStudio LSP server initialized")
            .await;
        // Ask the daemon to load its initial state.
        self.daemon_reload().await;
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate_and_publish(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.last() {
            self.validate_and_publish(params.text_document.uri, &change.text)
                .await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Reload the daemon whenever anything is saved — new artifacts become
        // visible as relationship targets, and plugin manifest changes take effect.
        self.daemon_reload().await;

        if let Some(text) = params.text {
            self.validate_and_publish(params.text_document.uri, &text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Clear diagnostics when the file is closed.
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

/// Run the LSP server over stdio.
///
/// This is the standard LSP transport. The editor launches this binary and
/// communicates over stdin/stdout.
pub async fn run_stdio(
    project_root: &Path,
    daemon_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let project_root = project_root.to_path_buf();
    let (service, socket) =
        LspService::new(|client| OrqaLspBackend::new(client, project_root, daemon_port));
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}

/// Run the LSP server over a TCP connection.
///
/// Useful for debugging with editors that support TCP LSP connections.
pub async fn run_tcp(
    project_root: &Path,
    port: u16,
    daemon_port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::net::TcpListener;

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("OrqaStudio LSP server listening on {addr}");

    let (stream, _) = listener.accept().await?;
    let (read, write) = tokio::io::split(stream);

    let project_root = project_root.to_path_buf();
    let (service, socket) =
        LspService::new(|client| OrqaLspBackend::new(client, project_root, daemon_port));
    Server::new(read, write, socket).serve(service).await;

    Ok(())
}
