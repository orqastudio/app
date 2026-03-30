/**
 * Dev environment — manages Vite + Tauri + daemon + watch mode.
 *
 * `orqa dev` is the primary entry point for the development environment.
 * Run it in a separate terminal — it watches Rust sources and auto-rebuilds.
 *
 * MCP and LSP server lifecycle is owned exclusively by the daemon. The dev
 * controller starts the daemon and the daemon starts MCP/LSP. The controller
 * does NOT spawn orqa-mcp-server or orqa-lsp-server directly.
 *
 * orqa dev                Start the full dev environment (Vite + Tauri + daemon)
 * orqa dev stop           Stop gracefully
 * orqa dev kill           Force-kill all processes
 * orqa dev restart        Restart Vite + Tauri
 * orqa dev restart-tauri  Restart Tauri only
 * orqa dev restart-vite   Restart Vite only
 * orqa dev status         Show process status
 * orqa dev icons          Generate brand icons from SVG sources
 * orqa dev tool           Run the debug-tool submodule
 */
/**
 * Dispatch the dev command: start the dev environment or a subcommand.
 * @param args - CLI arguments after "dev".
 */
export declare function runDevCommand(args: string[]): Promise<void>;
//# sourceMappingURL=dev.d.ts.map