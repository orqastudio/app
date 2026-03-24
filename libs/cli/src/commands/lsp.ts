/**
 * LSP command — spawns the standalone orqa-lsp-server binary over stdio.
 *
 * The binary handles the LSP protocol and connects to the validation daemon
 * (HTTP) for diagnostics. The daemon should already be running via `orqa dev`.
 *
 * Architecture: IDE → orqa lsp → orqa-lsp-server (stdio) → daemon (HTTP)
 *
 * orqa lsp [project-path]
 */

import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { getPort } from "../lib/ports.js";
import { getRoot } from "../lib/root.js";

const DAEMON_PORT = getPort("daemon");

const USAGE = `
Usage: orqa lsp [project-path]

Start an LSP server for .orqa/ artifact validation.
Spawns the standalone orqa-lsp-server binary over stdio.

Requires the dev environment to be running (\`orqa dev\` in a separate terminal).
`.trim();

/**
 * Find the pre-built orqa-lsp-server binary.
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

async function isDaemonHealthy(): Promise<boolean> {
	try {
		const res = await fetch(`http://127.0.0.1:${DAEMON_PORT}/health`, {
			signal: AbortSignal.timeout(500),
		});
		return res.ok;
	} catch {
		return false;
	}
}

export async function runLspCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const projectPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
	const projectRoot = getRoot();

	const lspBinary = findLspBinary(projectRoot);
	if (lspBinary === null) {
		process.stderr.write(
			"orqa-lsp-server binary not found.\n" +
			"Build it with: cargo build -p orqa-lsp-server\n",
		);
		process.exit(1);
	}

	if (!(await isDaemonHealthy())) {
		process.stderr.write(
			"Warning: daemon not running on port " + DAEMON_PORT + ". " +
			"Start dev environment with `orqa dev` in a separate terminal.\n",
		);
	}

	const child = spawn(lspBinary, [projectPath, "--daemon-port", String(DAEMON_PORT)], {
		stdio: ["pipe", "pipe", "inherit"],
	});

	process.stdin.pipe(child.stdin);
	child.stdout.pipe(process.stdout);

	child.on("error", (err) => {
		process.stderr.write(`Failed to start orqa-lsp-server: ${err.message}\n`);
		process.exit(1);
	});

	await new Promise<void>((resolve) => {
		child.on("close", () => resolve());
	});
}
