//! OrqaStudio application binary entry point.
//!
//! Handles CLI mode flags (`--mcp`, `--lsp`) before launching the Tauri GUI.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Resolve the daemon port from the environment.
///
/// The daemon binds directly to ORQA_PORT_BASE (default 10100) with no offset.
fn daemon_port() -> u16 {
    std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(10100)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // CLI mode: --mcp [project-path] — spawn orqa-mcp-server over stdio
    if args.iter().any(|a| a == "--mcp") {
        let project_path = args
            .iter()
            .position(|a| a == "--mcp")
            .and_then(|i| args.get(i + 1))
            .map_or_else(
                || std::env::current_dir().expect("failed to get current dir"),
                std::path::PathBuf::from,
            );

        if let Err(e) = orqa_studio_lib::servers::mcp::run(&project_path, daemon_port()) {
            // eprintln! is necessary here — tracing is not yet initialized, and
            // this is a fatal startup error that must reach the terminal.
            #[allow(clippy::print_stderr)]
            {
                eprintln!("MCP server error: {e}");
            }
            std::process::exit(1);
        }
        return;
    }

    // CLI mode: --lsp [project-path] — spawn orqa-lsp-server over stdio
    if args.iter().any(|a| a == "--lsp") {
        let project_path = args
            .iter()
            .position(|a| a == "--lsp")
            .and_then(|i| args.get(i + 1))
            .map_or_else(
                || std::env::current_dir().expect("failed to get current dir"),
                std::path::PathBuf::from,
            );

        if let Err(e) = orqa_studio_lib::servers::lsp::run(&project_path, daemon_port()) {
            // eprintln! is necessary here — tracing is not yet initialized, and
            // this is a fatal startup error that must reach the terminal.
            #[allow(clippy::print_stderr)]
            {
                eprintln!("LSP server error: {e}");
            }
            std::process::exit(1);
        }
        return;
    }

    orqa_studio_lib::run();
}
