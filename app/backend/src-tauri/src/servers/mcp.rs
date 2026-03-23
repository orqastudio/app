//! MCP (Model Context Protocol) server — thin wrapper around `orqa-mcp-server`.
//!
//! The standalone library at `libs/mcp-server` contains the full implementation.
//! This module delegates to it so the Tauri app and the standalone binary share
//! exactly the same protocol behaviour.

use std::path::Path;

use orqa_mcp_server::McpError;

/// Run the MCP server over stdio for the given project root.
///
/// Reads JSON-RPC messages from stdin (one per line), dispatches them to the
/// artifact graph and search tools, and writes responses to stdout. Runs until
/// stdin is closed.
///
/// # Errors
///
/// Returns `McpError::Io` if stdin/stdout I/O fails.
pub fn run(project_root: &Path) -> Result<(), McpError> {
    orqa_mcp_server::run(project_root)
}
