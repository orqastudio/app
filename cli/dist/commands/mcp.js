/**
 * MCP command — spawns the pre-built orqa-mcp-server binary over stdio.
 *
 * The binary handles the MCP protocol on stdin/stdout and connects to the
 * orqa-validation daemon (HTTP localhost:9120 by default) for graph/search/validation.
 * If the daemon isn't running, it is auto-started first.
 *
 * orqa mcp [project-path]
 * orqa mcp index [project-path]
 */
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { getRoot } from "../lib/root.js";
const USAGE = `
Usage: orqa mcp [project-path|subcommand]

Start the OrqaStudio MCP server. Spawns the pre-built orqa-mcp-server binary
which serves MCP protocol over stdio and connects to the daemon for graph,
search, and validation operations.

Subcommands:
  index [project-path]   Download ONNX model, index codebase, generate embeddings
`.trim();
/**
 * Locate the pre-built orqa-mcp-server binary.
 * Checks common build locations relative to the project root.
 * @param projectRoot - Absolute path to the project root.
 * @returns Full path to the binary, or null if not found.
 */
function findMcpBinary(projectRoot) {
    const name = process.platform === "win32" ? "orqa-mcp-server.exe" : "orqa-mcp-server";
    const candidates = [
        join(projectRoot, "engine", "mcp-server", "target", "release", name),
        join(projectRoot, "engine", "mcp-server", "target", "debug", name),
        join(projectRoot, "target", "release", name),
        join(projectRoot, "target", "debug", name),
        join(projectRoot, "app", "backend", "target", "release", name),
        join(projectRoot, "app", "backend", "target", "debug", name),
    ];
    for (const c of candidates) {
        if (existsSync(c))
            return c;
    }
    return null;
}
/**
 * Dispatch the mcp command: spawn the orqa-mcp-server binary or run the index subcommand.
 * @param args - CLI arguments after "mcp".
 */
export async function runMcpCommand(args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(USAGE);
        return;
    }
    // Handle 'index' subcommand
    if (args[0] === "index") {
        const { runIndexCommand } = await import("./index.js");
        await runIndexCommand(args.slice(1));
        return;
    }
    const projectPath = args.find((a) => !a.startsWith("--")) ?? getRoot();
    // Find the pre-built MCP server binary
    const projectRoot = getRoot();
    const binary = findMcpBinary(projectRoot);
    if (binary === null) {
        process.stderr.write("orqa-mcp-server binary not found. Build it with:\n" +
            "  cargo build --manifest-path engine/mcp-server/Cargo.toml\n");
        process.exit(1);
    }
    // Check if daemon is running — warn if not, but don't auto-start.
    // The daemon is managed by `orqa dev` in a separate terminal.
    if (!(await isDaemonHealthy())) {
        process.stderr.write("Warning: daemon not running on port " + DAEMON_PORT + ". " +
            "Start dev environment with `orqa dev` in a separate terminal.\n");
    }
    // Spawn the MCP server binary with stdio bridging
    return new Promise((resolve) => {
        const child = spawn(binary, [projectPath], {
            stdio: ["pipe", "pipe", "inherit"],
            env: { ...process.env, RUST_LOG: process.env.RUST_LOG ?? "info" },
        });
        process.stdin.pipe(child.stdin);
        child.stdout.pipe(process.stdout);
        child.on("error", (err) => {
            process.stderr.write(`Failed to start orqa-mcp-server: ${err.message}\n`);
            process.exit(1);
        });
        child.on("close", (code) => {
            if (code !== null && code !== 0) {
                process.exitCode = code;
            }
            resolve();
        });
    });
}
// ---------------------------------------------------------------------------
// Daemon health check
// ---------------------------------------------------------------------------
import { getPort } from "../lib/ports.js";
const DAEMON_PORT = getPort("daemon");
async function isDaemonHealthy() {
    try {
        const res = await fetch(`http://127.0.0.1:${DAEMON_PORT}/health`, {
            signal: AbortSignal.timeout(500),
        });
        return res.ok;
    }
    catch {
        return false;
    }
}
//# sourceMappingURL=mcp.js.map