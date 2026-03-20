//! Standalone `orqa-mcp-server` binary.
//!
//! Accepts a project path, initialises the search engine, and serves the MCP
//! protocol over stdio until stdin is closed.
//!
//! Usage:
//!   orqa-mcp-server <project-path>

use std::path::PathBuf;
use std::process;

use tracing_subscriber::EnvFilter;

fn main() {
    // Initialise tracing — output to stderr so it doesn't pollute the JSON-RPC stdout stream.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("ORQA_LOG").unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .init();

    let project_root: PathBuf = match std::env::args().nth(1) {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("usage: orqa-mcp-server <project-path>");
            process::exit(1);
        }
    };

    if !project_root.exists() {
        eprintln!("error: project path does not exist: {}", project_root.display());
        process::exit(1);
    }

    if let Err(e) = orqa_mcp_server::run(&project_root) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
