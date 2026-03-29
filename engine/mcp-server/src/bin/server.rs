//! Standalone `orqa-mcp-server` binary.
//!
//! Serves the MCP (Model Context Protocol) over JSON-RPC.
//!
//! # Transport modes
//!
//! **stdio mode** (default — backwards compatible with all LLM clients):
//! ```
//! orqa-mcp-server <project-path>
//! orqa-mcp-server <project-path> --daemon-port 9120
//! ```
//!
//! **TCP mode** (daemon-managed — single persistent process for multiple clients):
//! ```
//! orqa-mcp-server <project-path> --tcp 9178
//! orqa-mcp-server <project-path> --tcp 9178 --daemon-port 9120
//! ```
//!
//! The MCP protocol is identical in both modes — only the transport differs.

use std::path::PathBuf;
use std::process;

use tracing_subscriber::EnvFilter;

use orqa_mcp_server::daemon::default_daemon_port;

fn main() {
    // Initialise tracing — output to stderr so it doesn't pollute the JSON-RPC
    // stdout stream in stdio mode, and is visible in both modes for debugging.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let (project_root, tcp_port, daemon_port) = parse_args(&args);

    if !project_root.exists() {
        tracing::error!(
            path = %project_root.display(),
            "project path does not exist"
        );
        process::exit(1);
    }

    tracing::info!(
        project_root = %project_root.display(),
        daemon_port,
        "OrqaStudio MCP server starting"
    );

    let result = if let Some(port) = tcp_port {
        tracing::info!(port, "using TCP transport");
        orqa_mcp_server::run_tcp(&project_root, port, daemon_port)
    } else {
        tracing::info!("using stdio transport");
        orqa_mcp_server::run_with_daemon_port(&project_root, daemon_port)
    };

    if let Err(e) = result {
        tracing::error!(error = %e, "MCP server failed");
        process::exit(1);
    }
}

/// Parse command-line arguments into `(project_root, tcp_port, daemon_port)`.
///
/// Supported forms:
/// - `orqa-mcp-server <project-path>`
/// - `orqa-mcp-server <project-path> --tcp <port>`
/// - `orqa-mcp-server <project-path> --daemon-port <port>`
/// - `orqa-mcp-server <project-path> --tcp <port> --daemon-port <port>`
fn require_port_arg(args: &[String], i: usize, flag: &str) -> u16 {
    if i < args.len() {
        if let Ok(p) = args[i].parse::<u16>() {
            return p;
        }
        tracing::error!(port = args[i].as_str(), "invalid port, expected 1-65535");
    } else {
        tracing::error!("{flag} requires a port number");
    }
    process::exit(2);
}

fn parse_args(args: &[String]) -> (PathBuf, Option<u16>, u16) {
    let mut project_root: Option<PathBuf> = None;
    let mut tcp_port: Option<u16> = None;
    let mut daemon_port: u16 = default_daemon_port();
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--tcp" => {
                i += 1;
                tcp_port = Some(require_port_arg(args, i, "--tcp"));
            }
            "--daemon-port" => {
                i += 1;
                daemon_port = require_port_arg(args, i, "--daemon-port");
            }
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                project_root = Some(PathBuf::from(arg));
            }
            other => {
                tracing::error!(arg = other, "unknown argument, run with --help for usage");
                process::exit(2);
            }
        }
        i += 1;
    }

    let root = project_root.unwrap_or_else(|| {
        tracing::error!("usage: orqa-mcp-server <project-path> [--tcp PORT] [--daemon-port PORT]");
        process::exit(1);
    });

    (root, tcp_port, daemon_port)
}

/// Print usage information to stderr via tracing.
fn print_usage() {
    tracing::info!("OrqaStudio MCP Server");
    tracing::info!("USAGE: orqa-mcp-server <PROJECT_PATH> [--tcp PORT] [--daemon-port PORT]");
    tracing::info!("ARGS:  PROJECT_PATH  Path to the project root (required)");
    tracing::info!("OPTIONS:");
    tracing::info!("  --tcp PORT         Listen on TCP 127.0.0.1:PORT instead of stdio");
    tracing::info!("  --daemon-port PORT Connect to daemon on PORT (default: 9120)");
    tracing::info!("  --help             Show this help");
    tracing::info!("ENVIRONMENT: RUST_LOG  Tracing filter (default: info)");
}
