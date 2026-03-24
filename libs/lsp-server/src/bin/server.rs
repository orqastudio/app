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
//! **Daemon port** (defaults to 10258):
//! ```
//! orqa-lsp-server /path/to/project --daemon-port 10258
//! ```
//!
//! The project path is the root of the repository containing the `.orqa/`
//! directory. If omitted, the current working directory is used.

use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

/// Default port for the validation daemon HTTP API.
/// Reads `ORQA_PORT_BASE` from the environment (default 10200) and adds offset 58.
fn default_daemon_port() -> u16 {
    let base: u16 = std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10200);
    base + 58
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
/// - `orqa-lsp-server /path/to/project --daemon-port 10258`
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
                        eprintln!(
                            "orqa-lsp-server: invalid port '{}', expected a number 1–65535",
                            args[i]
                        );
                        std::process::exit(2);
                    }
                } else {
                    eprintln!("orqa-lsp-server: --tcp requires a port number");
                    std::process::exit(2);
                }
            }
            "--daemon-port" => {
                i += 1;
                if i < args.len() {
                    if let Ok(p) = args[i].parse::<u16>() {
                        daemon_port = p;
                    } else {
                        eprintln!(
                            "orqa-lsp-server: invalid daemon port '{}', expected a number 1–65535",
                            args[i]
                        );
                        std::process::exit(2);
                    }
                } else {
                    eprintln!("orqa-lsp-server: --daemon-port requires a port number");
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
                eprintln!("orqa-lsp-server: unknown argument '{other}'");
                eprintln!("Run with --help for usage.");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let root = project_root
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    (root, tcp_port, daemon_port)
}

fn print_usage() {
    eprintln!("OrqaStudio LSP Server");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    orqa-lsp-server [PROJECT_PATH] [--tcp PORT] [--daemon-port PORT]");
    eprintln!();
    eprintln!("ARGS:");
    eprintln!("    PROJECT_PATH    Path to the project root (default: current directory)");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --tcp PORT          Listen on TCP instead of stdio");
    eprintln!("    --daemon-port PORT  Validation daemon port (default: 10258)");
    eprintln!("    --help              Show this help message");
    eprintln!();
    eprintln!("ENVIRONMENT:");
    eprintln!("    RUST_LOG        Tracing filter (default: info)");
}
