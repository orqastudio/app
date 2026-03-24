#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Resolve the daemon port from the environment.
fn daemon_port() -> u16 {
    std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .map_or(10258, |base| base + 58)
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
            eprintln!("MCP server error: {e}");
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
            eprintln!("LSP server error: {e}");
            std::process::exit(1);
        }
        return;
    }

    orqa_studio_lib::run();
}
