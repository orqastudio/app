/**
 * LSP proxy command — connects to the running OrqaStudio app's IPC socket
 * and bridges stdin/stdout ↔ TCP for LSP protocol messages.
 *
 * If the app isn't running, falls back to spawning orqa-studio --lsp directly.
 *
 * orqa lsp [project-path]
 */

import { createConnection } from "node:net";
import { spawn } from "node:child_process";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { getRoot } from "../lib/root.js";

const USAGE = `
Usage: orqa lsp [project-path]

Start an LSP server bridge for .orqa/ artifact validation.
Connects to the running OrqaStudio app via IPC socket.
Falls back to spawning orqa-studio --lsp if the app is not running.

Provides real-time diagnostics for:
- Frontmatter schema validation
- Hex ID format validation (AD-057)
- Knowledge documentation constraint (AD-058)
- Relationship target existence
- Status validation
- Duplicate frontmatter key detection
`.trim();

function getPortFilePath(): string {
	const dataDir = process.env.LOCALAPPDATA
		? join(process.env.LOCALAPPDATA, "com.orqastudio.app")
		: join(process.env.HOME ?? "~", ".local", "share", "com.orqastudio.app");
	return join(dataDir, "ipc.port");
}

function readPort(): number | null {
	const portFile = getPortFilePath();
	if (!existsSync(portFile)) return null;
	try {
		const content = readFileSync(portFile, "utf-8").trim();
		const port = parseInt(content, 10);
		return Number.isNaN(port) ? null : port;
	} catch {
		return null;
	}
}

export async function runLspCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const projectPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
	const port = readPort();

	if (port) {
		await bridgeViaSocket(port, projectPath);
	} else {
		await spawnDirect(projectPath);
	}
}

async function bridgeViaSocket(port: number, projectPath: string): Promise<void> {
	return new Promise((resolve, reject) => {
		const socket = createConnection({ host: "127.0.0.1", port }, () => {
			socket.write(`LSP ${projectPath}\n`);
			process.stdin.pipe(socket);
			socket.pipe(process.stdout);
		});

		socket.on("error", (err) => {
			process.stderr.write(`IPC connection failed (${err.message}), falling back to direct mode\n`);
			spawnDirect(projectPath).then(resolve, reject);
		});

		socket.on("close", () => resolve());
	});
}

/**
 * Find the standalone orqa-lsp-server binary.
 * Searches workspace target dirs (debug first, then release), plus the old
 * libs/lsp-server path for backwards compatibility.
 */
function findLspBinary(projectRoot: string): string | null {
	const ext = process.platform === "win32" ? ".exe" : "";
	const bin = `orqa-lsp-server${ext}`;
	const candidates = [
		join(projectRoot, "target", "debug", bin),
		join(projectRoot, "target", "release", bin),
		join(projectRoot, "libs", "lsp-server", "target", "debug", bin),
		join(projectRoot, "libs", "lsp-server", "target", "release", bin),
	];
	for (const c of candidates) {
		if (existsSync(c)) return c;
	}
	return null;
}

async function spawnDirect(projectPath: string): Promise<void> {
	const projectRoot = getRoot();
	const lspBinary = findLspBinary(projectRoot);

	if (lspBinary) {
		process.stderr.write(`Using standalone LSP server: ${lspBinary}\n`);
		return spawnBinary(lspBinary, [projectPath]);
	}

	// Last resort: try orqa-studio --lsp (requires the app binary on PATH).
	process.stderr.write("Standalone orqa-lsp-server not found, trying orqa-studio --lsp\n");
	return spawnBinary("orqa-studio", ["--lsp", projectPath]);
}

function spawnBinary(binary: string, args: string[]): Promise<void> {
	return new Promise((resolve) => {
		const child = spawn(binary, args, {
			stdio: ["pipe", "pipe", "inherit"],
		});

		process.stdin.pipe(child.stdin);
		child.stdout.pipe(process.stdout);

		child.on("error", (err) => {
			process.stderr.write(`Failed to start ${binary}: ${err.message}\n`);
			process.stderr.write(
				"Build the LSP server with: cargo build -p orqa-lsp-server\n" +
				"Or ensure the OrqaStudio app is running (make dev).\n",
			);
			process.exit(1);
		});

		child.on("close", () => resolve());
	});
}
