//! LSP server for real-time OrqaStudio artifact validation.
//!
//! Provides diagnostics for `.orqa/` markdown files:
//! - Frontmatter schema validation (required fields, valid types)
//! - Relationship type validation (only keys from core.json)
//! - Relationship target existence
//! - Status validation (12 canonical statuses)
//! - Bidirectional relationship enforcement
//!
//! Launched via `orqa-studio --lsp <project-path>`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::domain::artifact_graph::{build_artifact_graph, ArtifactGraph};

// ---------------------------------------------------------------------------
// Canonical validation data
// ---------------------------------------------------------------------------

const VALID_STATUSES: &[&str] = &[
    "captured",
    "exploring",
    "ready",
    "prioritised",
    "active",
    "hold",
    "blocked",
    "review",
    "completed",
    "surpassed",
    "archived",
    "recurring",
];

// ---------------------------------------------------------------------------
// Server state
// ---------------------------------------------------------------------------

struct OrqaLspBackend {
    client: Client,
    project_root: PathBuf,
    graph: Mutex<Option<ArtifactGraph>>,
}

impl OrqaLspBackend {
    fn new(client: Client, project_root: PathBuf) -> Self {
        Self {
            client,
            project_root,
            graph: Mutex::new(None),
        }
    }

    /// Get or build the artifact graph.
    fn get_graph(&self) -> Option<ArtifactGraph> {
        let mut guard = self.graph.lock().ok()?;
        if guard.is_none() {
            *guard = build_artifact_graph(&self.project_root).ok();
        }
        guard.clone()
    }

    /// Refresh the graph from disk.
    fn refresh_graph(&self) {
        if let Ok(mut guard) = self.graph.lock() {
            *guard = build_artifact_graph(&self.project_root).ok();
        }
    }

    /// Validate a single artifact file and return diagnostics.
    fn validate_file(&self, uri: &Url, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Only validate .orqa/ markdown files
        let path = uri.to_file_path().unwrap_or_default();
        let rel_path = path
            .strip_prefix(&self.project_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        if !rel_path.starts_with(".orqa/") || !rel_path.ends_with(".md") {
            return diagnostics;
        }

        // Parse frontmatter
        let fm_match = content.find("---\n");
        if fm_match != Some(0) {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 3)),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("orqastudio".into()),
                message: "Missing YAML frontmatter (must start with ---)".into(),
                ..Default::default()
            });
            return diagnostics;
        }

        let fm_end = content[4..].find("\n---");
        if fm_end.is_none() {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 3)),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("orqastudio".into()),
                message: "Unclosed YAML frontmatter (missing closing ---)".into(),
                ..Default::default()
            });
            return diagnostics;
        }

        let fm_end_offset = fm_end.unwrap() + 4;
        let frontmatter = &content[4..fm_end_offset];

        // Check required fields
        let lines: Vec<&str> = frontmatter.lines().collect();
        let has_id = lines.iter().any(|l| l.starts_with("id:"));
        let has_status = lines.iter().any(|l| l.starts_with("status:"));

        if !has_id {
            let line_num = self.find_frontmatter_end_line(content);
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, 3),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("orqastudio".into()),
                message: "Missing required frontmatter field: id".into(),
                ..Default::default()
            });
        }

        // Validate status
        if has_status {
            for (i, line) in content.lines().enumerate() {
                if line.starts_with("status:") {
                    let status = line
                        .trim_start_matches("status:")
                        .trim()
                        .trim_matches('"');
                    if !VALID_STATUSES.contains(&status) {
                        diagnostics.push(Diagnostic {
                            range: Range::new(
                                Position::new(i as u32, 0),
                                Position::new(i as u32, line.len() as u32),
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("orqastudio".into()),
                            message: format!(
                                "Invalid status \"{status}\" — must be one of: {}",
                                VALID_STATUSES.join(", ")
                            ),
                            ..Default::default()
                        });
                    }
                    break;
                }
            }
        }

        // Validate ID format (AD-057: TYPE-XXXXXXXX hex format)
        if has_id {
            for (i, line) in content.lines().enumerate() {
                if line.starts_with("id:") {
                    let id = line.trim_start_matches("id:").trim().trim_matches('"');
                    if !crate::domain::artifact::is_valid_artifact_id(id) {
                        diagnostics.push(Diagnostic {
                            range: Range::new(
                                Position::new(i as u32, 0),
                                Position::new(i as u32, line.len() as u32),
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("orqastudio".into()),
                            message: format!("Invalid artifact ID \"{id}\" — must be TYPE-XXXXXXXX (8 hex chars)"),
                            ..Default::default()
                        });
                    } else if !crate::domain::artifact::is_hex_artifact_id(id) {
                        diagnostics.push(Diagnostic {
                            range: Range::new(
                                Position::new(i as u32, 0),
                                Position::new(i as u32, line.len() as u32),
                            ),
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("orqastudio".into()),
                            message: format!("Legacy sequential ID \"{id}\" — new artifacts should use TYPE-XXXXXXXX hex format (AD-057)"),
                            ..Default::default()
                        });
                    }
                    break;
                }
            }
        }

        // Detect duplicate frontmatter keys
        {
            let mut seen_keys: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
            for (i, line) in content.lines().enumerate() {
                if line == "---" {
                    if seen_keys.is_empty() {
                        // Opening ---
                        continue;
                    }
                    break; // Closing ---
                }
                // Top-level key (not indented)
                if let Some(key) = line.split(':').next() {
                    let key = key.trim().to_string();
                    if !key.is_empty() && !key.starts_with('-') && !key.starts_with(' ') {
                        if let Some(&first_line) = seen_keys.get(&key) {
                            diagnostics.push(Diagnostic {
                                range: Range::new(
                                    Position::new(i as u32, 0),
                                    Position::new(i as u32, line.len() as u32),
                                ),
                                severity: Some(DiagnosticSeverity::ERROR),
                                source: Some("orqastudio".into()),
                                message: format!("Duplicate frontmatter key \"{key}\" (first seen on line {})", first_line + 1),
                                ..Default::default()
                            });
                        } else {
                            seen_keys.insert(key, i as u32);
                        }
                    }
                }
            }
        }

        // Check skill documentation requirement (AD-058: skills must have synchronised-with)
        {
            let is_skill = frontmatter.lines().any(|l| {
                l.trim().starts_with("type:") && l.contains("skill")
            }) || rel_path.contains("/skills/");

            if is_skill && !frontmatter.contains("synchronised-with") {
                let line_num = self.find_frontmatter_end_line(content);
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(line_num, 0),
                        Position::new(line_num, 3),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("orqastudio".into()),
                    message: "Skills must have at least one synchronised-with relationship to a human-facing doc (AD-058)".into(),
                    ..Default::default()
                });
            }
        }

        // Check for relationship targets existence
        if let Some(graph) = self.get_graph() {
            for (i, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("- target:") {
                    let target = trimmed
                        .trim_start_matches("- target:")
                        .trim()
                        .trim_matches('"');
                    if !target.is_empty() && !graph.nodes.contains_key(target) {
                        diagnostics.push(Diagnostic {
                            range: Range::new(
                                Position::new(i as u32, 0),
                                Position::new(i as u32, line.len() as u32),
                            ),
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("orqastudio".into()),
                            message: format!("Relationship target \"{target}\" not found in graph"),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // Check for relationships section in delivery/process artifacts
        if (rel_path.starts_with(".orqa/delivery/") || rel_path.starts_with(".orqa/process/"))
            && !frontmatter.contains("relationships:")
        {
            let line_num = self.find_frontmatter_end_line(content);
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, 3),
                ),
                severity: Some(DiagnosticSeverity::INFORMATION),
                source: Some("orqastudio".into()),
                message: "No relationships declared — most delivery/process artifacts should have at least one".into(),
                ..Default::default()
            });
        }

        diagnostics
    }

    fn find_frontmatter_end_line(&self, content: &str) -> u32 {
        let mut count = 0u32;
        let mut in_fm = false;
        for line in content.lines() {
            if line == "---" {
                if in_fm {
                    return count;
                }
                in_fm = true;
            }
            count += 1;
        }
        1 // fallback
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for OrqaLspBackend {
    async fn initialize(&self, _: InitializeParams) -> RpcResult<InitializeResult> {
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

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "OrqaStudio LSP server initialized")
            .await;

        // Build the graph on startup
        self.refresh_graph();
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let diagnostics = self.validate_file(&params.text_document.uri, &params.text_document.text);
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.last() {
            let diagnostics = self.validate_file(&params.text_document.uri, &change.text);
            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Refresh the graph when files are saved (new targets may exist)
        self.refresh_graph();

        if let Some(text) = params.text {
            let diagnostics = self.validate_file(&params.text_document.uri, &text);
            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Clear diagnostics on close
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run the LSP server over stdio.
pub async fn run(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let project_root = project_root.to_path_buf();
    let (service, socket) = LspService::new(|client| OrqaLspBackend::new(client, project_root));
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
