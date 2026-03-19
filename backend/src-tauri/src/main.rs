#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // CLI mode: --mcp [project-path] — run MCP server over stdio
    if args.iter().any(|a| a == "--mcp") {
        let project_path = args
            .iter()
            .position(|a| a == "--mcp")
            .and_then(|i| args.get(i + 1))
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().expect("failed to get current dir"));

        if let Err(e) = orqa_studio_lib::servers::mcp::run(&project_path) {
            eprintln!("MCP server error: {e}");
            std::process::exit(1);
        }
        return;
    }

    // CLI mode: --lsp [project-path] — run LSP server over stdio
    if args.iter().any(|a| a == "--lsp") {
        let project_path = args
            .iter()
            .position(|a| a == "--lsp")
            .and_then(|i| args.get(i + 1))
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().expect("failed to get current dir"));

        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        if let Err(e) = rt.block_on(orqa_studio_lib::servers::lsp::run(&project_path)) {
            eprintln!("LSP server error: {e}");
            std::process::exit(1);
        }
        return;
    }

    orqa_studio_lib::run();
}
