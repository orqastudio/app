//! OrqaStudio LSP server — standalone binary.
//!
//! # Usage
//!
//! **Stdio mode** (standard LSP transport, used by editors):
//! ```
//! orqa-lsp-server /path/to/project
//! ```
//!
//! **TCP mode** (for debugging with editors that support TCP):
//! ```
//! orqa-lsp-server /path/to/project --tcp 9257
//! ```
//!
//! **Daemon port** (defaults to 9120):
//! ```
//! orqa-lsp-server /path/to/project --daemon-port 9120
//! ```
//!
//! The project path is the root of the repository containing the `.orqa/`
//! directory. If omitted, the current working directory is used.

use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

/// Default port for the validation daemon HTTP API.
///
/// Reads `ORQA_PORT_BASE` directly as the daemon port (default 9120). This
/// matches `daemon/src/health.rs resolve_port()` — no offset is applied.
fn default_daemon_port() -> u16 {
    std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9120)
}

#[tokio::main]
async fn main() {
    // Initialise tracing — output goes to stderr so it doesn't interfere with
    // the LSP JSON-RPC stdio transport on stdout.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let (project_root, tcp_port, daemon_port) = parse_args(&args);

    tracing::info!(
        project_root = %project_root.display(),
        daemon_port,
        "OrqaStudio LSP server starting"
    );

    let result = if let Some(port) = tcp_port {
        tracing::info!(port, "using TCP transport");
        orqa_lsp_server::run_tcp(&project_root, port, daemon_port).await
    } else {
        tracing::info!("using stdio transport");
        orqa_lsp_server::run_stdio(&project_root, daemon_port).await
    };

    if let Err(e) = result {
        tracing::error!(error = %e, "LSP server terminated with error");
        std::process::exit(1);
    }
}

/// Parse command-line arguments into `(project_root, tcp_port, daemon_port)`.
///
/// Supported argument forms:
/// - `orqa-lsp-server`                                   → cwd, stdio, default daemon port
/// - `orqa-lsp-server /path/to/project`                  → given path, stdio, default daemon port
/// - `orqa-lsp-server --tcp 9257`                        → cwd, TCP 9257, default daemon port
/// - `orqa-lsp-server /path/to/project --tcp 9257`
/// - `orqa-lsp-server /path/to/project --daemon-port 9120`
#[allow(clippy::too_many_lines)]
fn parse_args(args: &[String]) -> (PathBuf, Option<u16>, u16) {
    let mut project_root: Option<PathBuf> = None;
    let mut tcp_port: Option<u16> = None;
    let mut daemon_port: u16 = default_daemon_port();
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--tcp" => {
                i += 1;
                if i < args.len() {
                    tcp_port = args[i].parse::<u16>().ok();
                    if tcp_port.is_none() {
                        tracing::error!(port = args[i].as_str(), "invalid port, expected a number 1-65535");
                        std::process::exit(2);
                    }
                } else {
                    tracing::error!("--tcp requires a port number");
                    std::process::exit(2);
                }
            }
            "--daemon-port" => {
                i += 1;
                if i < args.len() {
                    if let Ok(p) = args[i].parse::<u16>() {
                        daemon_port = p;
                    } else {
                        tracing::error!(port = args[i].as_str(), "invalid daemon port, expected a number 1-65535");
                        std::process::exit(2);
                    }
                } else {
                    tracing::error!("--daemon-port requires a port number");
                    std::process::exit(2);
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                project_root = Some(PathBuf::from(arg));
            }
            other => {
                tracing::error!(arg = other, "unknown argument, run with --help for usage");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let root = project_root
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    (root, tcp_port, daemon_port)
}

/// Print usage information via tracing.
fn print_usage() {
    tracing::info!("OrqaStudio LSP Server");
    tracing::info!("USAGE: orqa-lsp-server [PROJECT_PATH] [--tcp PORT] [--daemon-port PORT]");
    tracing::info!("ARGS:  PROJECT_PATH  Path to the project root (default: current directory)");
    tracing::info!("OPTIONS: --tcp PORT, --daemon-port PORT (default: 9120), --help");
    tracing::info!("ENVIRONMENT: RUST_LOG  Tracing filter (default: info)");
}
