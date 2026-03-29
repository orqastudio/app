/**
 * Daemon command — manage the orqa-daemon background process.
 *
 * The orqa-daemon is a persistent Rust process that provides file watching,
 * health monitoring, system tray, and MCP/LSP server lifecycle management.
 * This command is the CLI interface for starting, stopping, restarting, and
 * checking the status of that process.
 *
 * Subcommands:
 *   orqa daemon start    — spawn orqa-daemon, verify via health endpoint
 *   orqa daemon stop     — send SIGTERM (Unix) or taskkill (Windows) via PID file
 *   orqa daemon restart  — stop then start
 *   orqa daemon status   — show PID, uptime, and health endpoint response
 */
/**
 * Dispatch orqa daemon subcommands.
 * @param args - CLI arguments after "daemon".
 */
export declare function runDaemonCommand(args: string[]): Promise<void>;
//# sourceMappingURL=daemon.d.ts.map