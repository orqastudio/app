/**
 * MCP command — spawns the pre-built orqa-mcp-server binary over stdio.
 *
 * The binary handles the MCP protocol on stdin/stdout and connects to the
 * orqa-validation daemon (HTTP localhost:10100 by default) for graph/search/validation.
 * If the daemon isn't running, it is auto-started first.
 *
 * orqa mcp [project-path]
 * orqa mcp index [project-path]
 */
/**
 * Dispatch the mcp command: spawn the orqa-mcp-server binary or run the index subcommand.
 * @param args - CLI arguments after "mcp".
 */
export declare function runMcpCommand(args: string[]): Promise<void>;
//# sourceMappingURL=mcp.d.ts.map