//! LSP server implementation for OrqaStudio artifact validation.
//!
//! Implements the `LanguageServer` trait from `tower-lsp`. Handles:
//! - `initialize` / `initialized` / `shutdown`
//! - `textDocument/didOpen`, `didChange`, `didSave`, `didClose`
//!
//! On each document event, the server validates the file and publishes
//! diagnostics. The artifact graph is rebuilt on `didSave` to pick up new
//! artifacts referenced by other files.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, InitializeParams, InitializeResult, InitializedParams, MessageType,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use orqa_validation::platform::{scan_plugin_manifests, ArtifactTypeDef};

use crate::graph::build_artifact_graph;
use crate::types::ArtifactGraph;
use crate::validation::{validate_file, validate_graph_checks};

// ---------------------------------------------------------------------------
// Backend state
// ---------------------------------------------------------------------------

pub(crate) struct OrqaLspBackend {
    client: Client,
    project_root: PathBuf,
    graph: Mutex<Option<ArtifactGraph>>,
    /// Artifact type definitions loaded from plugin manifests.
    /// Used for JSON Schema frontmatter validation.
    artifact_types: Mutex<Vec<ArtifactTypeDef>>,
}

impl OrqaLspBackend {
    pub(crate) fn new(client: Client, project_root: PathBuf) -> Self {
        Self {
            client,
            project_root,
            graph: Mutex::new(None),
            artifact_types: Mutex::new(Vec::new()),
        }
    }

    /// Get the cached graph, building it on first access.
    fn get_graph(&self) -> Option<ArtifactGraph> {
        let mut guard = self.graph.lock().ok()?;
        if guard.is_none() {
            *guard = build_artifact_graph(&self.project_root).ok();
        }
        guard.clone()
    }

    /// Rebuild the graph from disk (called on `didSave`).
    fn refresh_graph(&self) {
        if let Ok(mut guard) = self.graph.lock() {
            *guard = build_artifact_graph(&self.project_root).ok();
        }
    }

    /// Load artifact type definitions from plugin manifests.
    fn load_artifact_types(&self) {
        let contributions = scan_plugin_manifests(&self.project_root);
        if let Ok(mut guard) = self.artifact_types.lock() {
            *guard = contributions.artifact_types;
        }
    }

    /// Get the cached artifact types.
    fn get_artifact_types(&self) -> Vec<ArtifactTypeDef> {
        self.artifact_types
            .lock()
            .ok()
            .map(|guard| guard.clone())
            .unwrap_or_default()
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

    /// Validate `content` at `uri` and publish diagnostics to the client.
    ///
    /// Combines file-level checks (fast, single-file) with graph-level checks
    /// (comprehensive, full graph scan via `orqa_validation`).
    async fn validate_and_publish(&self, uri: Url, content: &str) {
        let rel_path = self.relative_path(&uri);
        let graph = self.get_graph();
        let artifact_types = self.get_artifact_types();

        // File-level checks: frontmatter syntax, JSON Schema, IDs.
        let mut diagnostics = validate_file(&rel_path, content, graph.as_ref(), &artifact_types);

        // Graph-level checks: broken refs, missing inverses, type constraints,
        // cardinality, cycles — delegated to orqa_validation.
        let artifact_id = Self::extract_artifact_id(content);
        let graph_diagnostics =
            validate_graph_checks(&self.project_root, artifact_id.as_deref());
        diagnostics.extend(graph_diagnostics);

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
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
        self.load_artifact_types();
        self.refresh_graph();
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
        // Reload schemas if a plugin manifest was saved.
        let path = params.text_document.uri.to_file_path().unwrap_or_default();
        if path.file_name().is_some_and(|n| n == "orqa-plugin.json") {
            self.load_artifact_types();
        }

        // Refresh the graph so new artifacts become visible as relationship targets.
        self.refresh_graph();

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
pub async fn run_stdio(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let project_root = project_root.to_path_buf();
    let (service, socket) = LspService::new(|client| OrqaLspBackend::new(client, project_root));
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}

/// Run the LSP server over a TCP connection.
///
/// Useful for debugging with editors that support TCP LSP connections.
pub async fn run_tcp(project_root: &Path, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::net::TcpListener;

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("OrqaStudio LSP server listening on {addr}");

    let (stream, _) = listener.accept().await?;
    let (read, write) = tokio::io::split(stream);

    let project_root = project_root.to_path_buf();
    let (service, socket) = LspService::new(|client| OrqaLspBackend::new(client, project_root));
    Server::new(read, write, socket).serve(service).await;

    Ok(())
}
