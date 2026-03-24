/**
 * MCP command — spawns the pre-built orqa-mcp-server binary over stdio.
 *
 * The binary handles the MCP protocol on stdin/stdout and connects to the
 * orqa-validation daemon (HTTP localhost:10258) for graph/search/validation.
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
 */
function findMcpBinary(projectRoot: string): string | null {
	const name = process.platform === "win32" ? "orqa-mcp-server.exe" : "orqa-mcp-server";
	const candidates = [
		join(projectRoot, "libs", "mcp-server", "target", "release", name),
		join(projectRoot, "libs", "mcp-server", "target", "debug", name),
		join(projectRoot, "target", "release", name),
		join(projectRoot, "target", "debug", name),
		join(projectRoot, "app", "backend", "target", "release", name),
		join(projectRoot, "app", "backend", "target", "debug", name),
	];
	for (const c of candidates) {
		if (existsSync(c)) return c;
	}
	return null;
}

export async function runMcpCommand(args: string[]): Promise<void> {
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
		process.stderr.write(
			"orqa-mcp-server binary not found. Build it with:\n" +
			"  cargo build --manifest-path libs/mcp-server/Cargo.toml\n",
		);
		process.exit(1);
	}

	// Auto-start daemon if not running (best-effort, non-fatal).
	// We check health directly rather than calling runDaemonCommand because
	// daemonStart calls process.exit(1) when the daemon is already running.
	await ensureDaemonRunning();

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
// Daemon auto-start
// ---------------------------------------------------------------------------

import { getPort } from "../lib/ports.js";

const DAEMON_PORT = getPort("daemon");

/**
 * Check if the daemon is reachable; if not, attempt to start it.
 * Best-effort — failures are logged to stderr but do not prevent the MCP
 * server from launching (it handles daemon unavailability gracefully).
 */
async function ensureDaemonRunning(): Promise<void> {
	// Quick health check
	if (await isDaemonHealthy()) return;

	// Daemon not responding — try to start it directly (we avoid
	// runDaemonCommand because daemonStart calls process.exit on failure,
	// which would kill this process before we can spawn the MCP binary).
	const projectRoot = getRoot();
	const { findBinary } = await import("../lib/validation-engine.js");
	const daemonBin = findBinary(projectRoot);
	if (daemonBin === null) {
		process.stderr.write("Warning: daemon binary not found, MCP server will run without daemon.\n");
		return;
	}

	process.stderr.write("Daemon not running, starting...\n");
	try {
		const child = spawn(daemonBin, ["daemon", projectRoot, "--port", String(DAEMON_PORT)], {
			detached: true,
			stdio: "ignore",
			windowsHide: true,
		});
		child.unref();

		// Wait up to 3 seconds for health to respond
		const deadline = Date.now() + 3000;
		while (Date.now() < deadline) {
			await new Promise((r) => setTimeout(r, 150));
			if (await isDaemonHealthy()) {
				process.stderr.write("Daemon started.\n");
				return;
			}
		}
		process.stderr.write("Warning: daemon did not start within 3s. MCP server will run without it.\n");
	} catch {
		process.stderr.write("Warning: failed to auto-start daemon. MCP server will run without it.\n");
	}
}

async function isDaemonHealthy(): Promise<boolean> {
	try {
		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 500);
		try {
			const response = await fetch(`http://127.0.0.1:${DAEMON_PORT}/health`, {
				signal: controller.signal,
			});
			return response.ok;
		} finally {
			clearTimeout(timeout);
		}
	} catch {
		return false;
	}
}
