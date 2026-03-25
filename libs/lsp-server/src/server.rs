//! LSP server implementation for OrqaStudio artifact validation.
//!
//! Implements the `LanguageServer` trait from `tower-lsp`. Handles:
//! - `initialize` / `initialized` / `shutdown`
//! - `textDocument/didOpen`, `didChange`, `didSave`, `didClose`
//! - `textDocument/codeAction` — offers quick-fixes for invalid statuses
//!
//! On each document event the server:
//! 1. Runs fast text-level checks (frontmatter structure, duplicate keys, ID
//!    format, JSON Schema validation) using local logic on the unsaved buffer.
//! 2. Calls the validation daemon `POST /validate` for graph-level checks
//!    (broken refs, missing inverses, type constraints, cardinality, cycles).
//!
//! Schema definitions are loaded from plugin manifests (`plugins/*/orqa-plugin.json`
//! and `connectors/*/orqa-plugin.json`) on startup. They are refreshed whenever a
//! plugin manifest is saved.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, CodeActionResponse, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
    ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url, WorkspaceEdit,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use orqa_validation::platform::{scan_plugin_manifests, ArtifactTypeDef};

use crate::validation::validate_file;

// ---------------------------------------------------------------------------
// Backend state
// ---------------------------------------------------------------------------

pub(crate) struct OrqaLspBackend {
    client: Client,
    project_root: PathBuf,
    /// Base URL of the validation daemon, e.g. `http://127.0.0.1:{port}`.
    daemon_url: String,
    http: HttpClient,
    /// Artifact type definitions loaded from plugin manifests.
    /// Refreshed when a plugin manifest (`orqa-plugin.json`) is saved.
    artifact_types: Arc<RwLock<Vec<ArtifactTypeDef>>>,
}

impl OrqaLspBackend {
    pub(crate) fn new(client: Client, project_root: PathBuf, daemon_port: u16) -> Self {
        Self {
            client,
            project_root,
            daemon_url: format!("http://127.0.0.1:{daemon_port}"),
            http: HttpClient::new(),
            artifact_types: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Load artifact type definitions from all plugin manifests under the project root.
    ///
    /// Called on `initialized` and whenever a plugin manifest is saved.
    async fn load_artifact_types(&self) {
        let contributions = scan_plugin_manifests(&self.project_root);
        let count = contributions.artifact_types.len();
        let mut types = self.artifact_types.write().await;
        *types = contributions.artifact_types;
        tracing::info!(count, "loaded artifact type schemas from plugin manifests");
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
        frontmatter.lines().find(|l| l.starts_with("id:")).map(|l| {
            l.trim_start_matches("id:")
                .trim()
                .trim_matches('"')
                .to_owned()
        })
    }

    /// Return the list of valid statuses for an artifact ID prefix, if known.
    ///
    /// Used by the code-action handler to suggest valid alternatives when the
    /// current status is invalid.
    async fn valid_statuses_for_prefix(&self, prefix: &str) -> Vec<String> {
        let types = self.artifact_types.read().await;
        types
            .iter()
            .find(|t| t.id_prefix == prefix)
            .map(|t| t.status_transitions.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    }

    /// Tell the daemon to reload its graph from disk.
    ///
    /// Called on `initialized` and on `didSave` of a plugin manifest.
    /// Failures are logged but not fatal — the daemon may not be running.
    async fn daemon_reload(&self) {
        let url = format!("{}/reload", self.daemon_url);
        match self
            .http
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await
        {
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
    /// ID format, JSON Schema validation) with graph-level checks from the daemon.
    async fn validate_and_publish(&self, uri: Url, content: &str) {
        let rel_path = self.relative_path(&uri);

        // Text-level checks: frontmatter syntax, ID format, duplicate keys,
        // and JSON Schema validation against plugin-defined type schemas.
        let types = self.artifact_types.read().await;
        let mut diagnostics = validate_file(&rel_path, content, None, &types);
        drop(types);

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
    let severity_str = check
        .get("severity")
        .and_then(|v| v.as_str())
        .unwrap_or("Error");
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
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
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

        // Load artifact type schemas from plugin manifests.
        self.load_artifact_types().await;

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
        let rel_path = self.relative_path(&params.text_document.uri);

        // Refresh artifact type schemas when a plugin manifest is saved.
        if rel_path.ends_with("orqa-plugin.json") {
            self.load_artifact_types().await;
        }

        // Reload the daemon whenever anything is saved — new artifacts become
        // visible as relationship targets, and plugin manifest changes take effect.
        self.daemon_reload().await;

        if let Some(text) = params.text {
            self.validate_and_publish(params.text_document.uri, &text)
                .await;
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> RpcResult<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let mut actions: Vec<CodeActionOrCommand> = Vec::new();

        // Scan existing diagnostics for [invalid-status] and [schema] status errors
        // and offer quick-fix replacements with valid alternatives.
        for diag in &params.context.diagnostics {
            if diag.source.as_deref() != Some("orqastudio") {
                continue;
            }

            // Quick-fix for invalid status values.
            // Matches diagnostics from the daemon ("[invalid-status]") and from
            // the JSON Schema validator ("[schema] ... status ...").
            let is_status_diag = diag.message.contains("[invalid-status]")
                || (diag.message.contains("[schema]") && diag.message.contains("status"));

            if !is_status_diag {
                continue;
            }

            // Read the document to find the current status line and artifact ID prefix.
            let Some(file_path) = uri.to_file_path().ok() else {
                continue;
            };
            let Ok(content) = std::fs::read_to_string(&file_path) else {
                continue;
            };

            let prefix = Self::extract_artifact_id(&content)
                .and_then(|id| id.split('-').next().map(String::from));

            let Some(prefix) = prefix else { continue };

            let valid_statuses = self.valid_statuses_for_prefix(&prefix).await;
            if valid_statuses.is_empty() {
                continue;
            }

            // Find the status line and its range in the document.
            let mut status_range = None;
            for (i, line) in content.lines().enumerate() {
                if line.starts_with("status:") {
                    let value_start = "status:".len();
                    let trimmed_value = line[value_start..].trim();
                    // Compute column range for the status value.
                    let col_start = line.find(trimmed_value).unwrap_or(value_start) as u32;
                    let col_end = col_start + trimmed_value.len() as u32;
                    status_range = Some(Range::new(
                        Position::new(i as u32, col_start),
                        Position::new(i as u32, col_end),
                    ));
                    break;
                }
            }

            let Some(range) = status_range else { continue };

            for status in &valid_statuses {
                let mut changes = std::collections::HashMap::new();
                changes.insert(
                    uri.clone(),
                    vec![TextEdit {
                        range,
                        new_text: status.clone(),
                    }],
                );
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: format!("Change status to \"{status}\""),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diag.clone()]),
                    edit: Some(WorkspaceEdit {
                        changes: Some(changes),
                        ..Default::default()
                    }),
                    ..Default::default()
                }));
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
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
