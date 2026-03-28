//! `orqa-mcp-server` — standalone MCP server library for OrqaStudio.
//!
//! Exposes the OrqaStudio artifact graph as MCP (Model Context Protocol) tools
//! over JSON-RPC stdio. Can be embedded in the Tauri app or run as a standalone
//! binary (`orqa-mcp-server`).
//!
//! # Public API
//!
//! The primary entry point is [`server::run`], which reads from stdin and writes
//! to stdout until the stream is closed.
//!
//! ```no_run
//! use std::path::Path;
//!
//! orqa_mcp_server::server::run(Path::new("/my/project")).unwrap();
//! ```

/// Daemon proxy: delegate tool calls to the daemon when it is running.
pub mod daemon;
/// Unified error type for MCP server operations.
pub mod error;
/// Artifact graph construction and querying over `.orqa/` files.
pub mod graph;
/// JSON-RPC stdio server: reads requests and dispatches to tool implementations.
pub mod server;
/// Project settings types loaded from `project.json`.
pub mod settings;
/// MCP tool implementations for search, graph, and integrity operations.
pub mod tools;
/// JSON-RPC and MCP protocol envelope types.
pub mod types;

// Re-export the primary entry points and error type at the crate root.
pub use error::McpError;
pub use server::{run, run_with_daemon_port};
