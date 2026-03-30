//! `orqa-mcp-server` — standalone MCP server library for OrqaStudio.
//!
//! Exposes the OrqaStudio artifact graph as MCP (Model Context Protocol) tools
//! over JSON-RPC. Supports two transports:
//!
//! - **stdio** (default): LLM clients spawn `orqa-mcp-server` directly and
//!   communicate over stdin/stdout. Backwards compatible with all MCP clients.
//! - **TCP** (`--tcp <port>`): the daemon manages a single persistent MCP process
//!   that LLM clients connect to over TCP on `127.0.0.1:<port>`.
//!
//! # Port allocation
//!
//! The daemon uses `ORQA_PORT_BASE` (default 10100) as the base port. The MCP
//! server listens at `ORQA_PORT_BASE + 2` (default 10102).
//!
//! # Public API
//!
//! ```no_run
//! use std::path::Path;
//!
//! // stdio mode (direct client use)
//! orqa_mcp_server::run(Path::new("/my/project")).unwrap();
//!
//! // TCP mode (daemon-managed)
//! orqa_mcp_server::run_tcp(Path::new("/my/project"), 10102, 10100).unwrap();
//! ```

use orqa_engine::ports;

/// Resolve the default MCP TCP port from the environment.
///
/// Returns `ORQA_PORT_BASE + MCP_PORT_OFFSET`. Defaults to 10102 when the
/// environment variable is absent or unparseable.
pub fn default_mcp_port() -> u16 {
    ports::resolve_mcp_port()
}

/// Daemon proxy: delegate tool calls to the daemon when it is running.
pub mod daemon;
/// Unified error type for MCP server operations.
pub mod error;
/// Artifact graph construction and querying over `.orqa/` files.
pub mod graph;
/// JSON-RPC server: reads requests and dispatches to tool implementations.
pub mod server;
/// Project settings types loaded from `project.json`.
pub mod settings;
/// MCP tool implementations for search, graph, and integrity operations.
pub mod tools;
/// JSON-RPC and MCP protocol envelope types.
pub mod types;

// Re-export the primary entry points and error type at the crate root.
pub use error::McpError;
pub use server::{run, run_tcp, run_with_daemon_port};
