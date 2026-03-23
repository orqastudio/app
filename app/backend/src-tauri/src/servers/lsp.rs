//! LSP server entry point for the OrqaStudio Tauri application.
//!
//! This module is a thin wrapper around the `orqa-lsp-server` library crate.
//! All validation logic, graph building, and LSP protocol handling live in
//! `libs/lsp-server`.
//!
//! Launched via `orqa-studio --lsp <project-path>`.

use std::path::Path;

/// Run the LSP server over stdio.
///
/// Delegates entirely to `orqa_lsp_server::run_stdio`.
pub async fn run(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    orqa_lsp_server::run_stdio(project_root, 3002).await
}
