/**
 * MCP proxy command — connects to the running OrqaStudio app's IPC socket
 * and bridges stdin/stdout ↔ TCP for MCP protocol messages.
 *
 * If the app isn't running, falls back to spawning orqa-studio --mcp directly.
 *
 * orqa mcp [project-path]
 */
export declare function runMcpCommand(args: string[]): Promise<void>;
//# sourceMappingURL=mcp.d.ts.map