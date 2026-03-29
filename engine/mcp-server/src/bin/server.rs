//! Standalone `orqa-mcp-server` binary.
//!
//! Accepts a project path and an optional daemon port, then serves the MCP
//! protocol over stdio until stdin is closed.
//!
//! Usage:
//!   orqa-mcp-server <project-path> [--daemon-port <port>]

use std::path::PathBuf;
use std::process;

use tracing_subscriber::EnvFilter;

use orqa_mcp_server::daemon::default_daemon_port;

fn main() {
    // Initialise tracing — output to stderr so it doesn't pollute the JSON-RPC stdout stream.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let (project_root, daemon_port) = parse_args(&args);

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

    if let Err(e) = orqa_mcp_server::run_with_daemon_port(&project_root, daemon_port) {
        tracing::error!(error = %e, "MCP server failed");
        process::exit(1);
    }
}

fn parse_args(args: &[String]) -> (PathBuf, u16) {
    let mut project_root: Option<PathBuf> = None;
    let mut daemon_port: u16 = default_daemon_port();
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--daemon-port" => {
                i += 1;
                if i < args.len() {
                    if let Ok(p) = args[i].parse::<u16>() {
                        daemon_port = p;
                    } else {
                        tracing::error!(port = args[i].as_str(), "invalid daemon port, expected 1-65535");
                        process::exit(2);
                    }
                } else {
                    tracing::error!("--daemon-port requires a port number");
                    process::exit(2);
                }
            }
            "--help" | "-h" => {
                tracing::info!("USAGE: orqa-mcp-server <project-path> [--daemon-port <port>]");
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                project_root = Some(PathBuf::from(arg));
            }
            other => {
                tracing::error!(arg = other, "unknown argument");
                process::exit(2);
            }
        }
        i += 1;
    }

    let root = project_root.unwrap_or_else(|| {
        tracing::error!("usage: orqa-mcp-server <project-path> [--daemon-port <port>]");
        process::exit(1);
    });

    (root, daemon_port)
}
