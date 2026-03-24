/**
 * LSP command — spawns the standalone orqa-lsp-server binary over stdio.
 *
 * The binary handles the LSP protocol and connects to the validation daemon
 * (HTTP localhost:10258) for diagnostics. If the daemon is not running,
 * it is auto-started before the LSP server is launched.
 *
 * Architecture: IDE → orqa lsp → orqa-lsp-server (stdio) → daemon (HTTP)
 *
 * orqa lsp [project-path]
 */

import { spawn } from "node:child_process";
import { existsSync, readFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { getPort } from "../lib/ports.js";
import { getRoot } from "../lib/root.js";
import { findBinary as findValidationBinary } from "../lib/validation-engine.js";

const DEFAULT_DAEMON_PORT = getPort("daemon");

const USAGE = `
Usage: orqa lsp [project-path]

Start an LSP server for .orqa/ artifact validation.
Spawns the standalone orqa-lsp-server binary over stdio.
Auto-starts the validation daemon if it is not running.

Provides real-time diagnostics for:
- Frontmatter schema validation
- Hex ID format validation
- Knowledge documentation constraints
- Relationship target existence
- Status validation
- Duplicate frontmatter key detection
`.trim();

/**
 * Find the pre-built orqa-lsp-server binary.
 * Checks workspace target dirs (release first, then debug).
 */
function findLspBinary(projectRoot: string): string | null {
	const ext = process.platform === "win32" ? ".exe" : "";
	const bin = `orqa-lsp-server${ext}`;
	const candidates = [
		join(projectRoot, "target", "release", bin),
		join(projectRoot, "target", "debug", bin),
		join(projectRoot, "libs", "lsp-server", "target", "release", bin),
		join(projectRoot, "libs", "lsp-server", "target", "debug", bin),
	];
	for (const c of candidates) {
		if (existsSync(c)) return c;
	}
	return null;
}

// ---------------------------------------------------------------------------
// Daemon helpers (adapted from daemon.ts)
// ---------------------------------------------------------------------------

function getPidPath(projectRoot: string): string {
	return join(projectRoot, "tmp", "daemon.pid");
}

function readPid(pidPath: string): number | null {
	if (!existsSync(pidPath)) return null;
	const raw = readFileSync(pidPath, "utf-8").trim();
	const n = parseInt(raw, 10);
	return Number.isNaN(n) ? null : n;
}

function processIsAlive(pid: number): boolean {
	try {
		process.kill(pid, 0);
		return true;
	} catch {
		return false;
	}
}

async function fetchHealth(port: number): Promise<Record<string, unknown> | null> {
	try {
		const controller = new AbortController();
		const timeout = setTimeout(() => controller.abort(), 500);
		try {
			const response = await fetch(`http://127.0.0.1:${port}/health`, {
				signal: controller.signal,
			});
			if (!response.ok) return null;
			return (await response.json()) as Record<string, unknown>;
		} finally {
			clearTimeout(timeout);
		}
	} catch {
		return null;
	}
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Ensure the daemon is running. If not, auto-start it.
 * Returns true if the daemon is healthy, false if it could not be started.
 */
async function ensureDaemon(projectRoot: string, port: number): Promise<boolean> {
	// Check if daemon is already running.
	const health = await fetchHealth(port);
	if (health !== null) return true;

	// Check PID file — if alive but health failed, it may be starting up.
	const pidPath = getPidPath(projectRoot);
	const pid = readPid(pidPath);
	if (pid !== null && processIsAlive(pid)) {
		// PID is alive but health didn't respond — wait briefly.
		for (let i = 0; i < 6; i++) {
			await sleep(250);
			const h = await fetchHealth(port);
			if (h !== null) return true;
		}
	}

	// Daemon not running — start it.
	const validationBinary = findValidationBinary(projectRoot);
	if (validationBinary === null) {
		process.stderr.write(
			"orqa-validation binary not found. Cannot auto-start daemon.\n" +
			"Build it with: cargo build --manifest-path libs/validation/Cargo.toml\n" +
			"Or start the daemon manually: orqa daemon start\n",
		);
		return false;
	}

	const tmpDir = join(projectRoot, "tmp");
	if (!existsSync(tmpDir)) {
		mkdirSync(tmpDir, { recursive: true });
	}

	process.stderr.write("Auto-starting validation daemon...\n");

	const child = spawn(validationBinary, ["daemon", projectRoot, "--port", String(port)], {
		detached: true,
		stdio: "ignore",
		windowsHide: true,
	});
	child.unref();

	// Wait up to 3 seconds for /health to respond.
	const startedAt = Date.now();
	while (Date.now() - startedAt < 3000) {
		await sleep(150);
		const h = await fetchHealth(port);
		if (h !== null) {
			process.stderr.write("Daemon started.\n");
			return true;
		}
	}

	process.stderr.write("Daemon did not respond within 3 seconds. Proceeding anyway.\n");
	return false;
}

// ---------------------------------------------------------------------------
// Main command
// ---------------------------------------------------------------------------

export async function runLspCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const projectPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
	const projectRoot = getRoot();

	// Find the pre-built LSP server binary.
	const lspBinary = findLspBinary(projectRoot);
	if (lspBinary === null) {
		process.stderr.write(
			"orqa-lsp-server binary not found.\n" +
			"Build it with: cargo build -p orqa-lsp-server\n",
		);
		process.exit(1);
	}

	// Auto-start daemon if not running.
	await ensureDaemon(projectRoot, DEFAULT_DAEMON_PORT);

	// Spawn the LSP server binary with stdio bridging.
	process.stderr.write(`Starting LSP server: ${lspBinary}\n`);

	const child = spawn(lspBinary, [projectPath, "--daemon-port", String(DEFAULT_DAEMON_PORT)], {
		stdio: ["pipe", "pipe", "inherit"],
	});

	process.stdin.pipe(child.stdin);
	child.stdout.pipe(process.stdout);

	child.on("error", (err) => {
		process.stderr.write(`Failed to start orqa-lsp-server: ${err.message}\n`);
		process.stderr.write("Build it with: cargo build -p orqa-lsp-server\n");
		process.exit(1);
	});

	await new Promise<void>((resolve) => {
		child.on("close", () => resolve());
	});
}
