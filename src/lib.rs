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

pub mod error;
pub mod graph;
pub mod integrity;
pub mod platform;
pub mod server;
pub mod settings;
pub mod tools;
pub mod types;

// Re-export the primary entry point and error type at the crate root.
pub use error::McpError;
pub use server::run;
