/**
 * MCP proxy command — connects to the running OrqaStudio app's IPC socket
 * and bridges stdin/stdout ↔ TCP for MCP protocol messages.
 *
 * If the app isn't running, falls back to the standalone MCP server crate
 * (libs/mcp-server) via cargo run.
 *
 * orqa mcp [project-path]
 */

import { createConnection } from "node:net";
import { spawn, spawnSync } from "node:child_process";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const USAGE = `
Usage: orqa mcp [project-path|subcommand]

Start an MCP server bridge. Connects to the running OrqaStudio app
via IPC socket. Falls back to the standalone MCP server crate if the
app is not running.

Subcommands:
  index [project-path]   Download ONNX model, index codebase, generate embeddings
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

	const projectPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
	const port = readPort();

	if (port) {
		// Connect to running app via IPC socket
		await bridgeViaSocket(port, projectPath);
	} else {
		// Fallback: spawn orqa-studio --mcp directly
		await spawnDirect(projectPath);
	}
}

async function bridgeViaSocket(port: number, projectPath: string): Promise<void> {
	return new Promise((resolve, reject) => {
		const socket = createConnection({ host: "127.0.0.1", port }, () => {
			// Send protocol header
			socket.write(`MCP ${projectPath}\n`);

			// Bridge stdin → socket
			process.stdin.pipe(socket);

			// Bridge socket → stdout
			socket.pipe(process.stdout);
		});

		socket.on("error", (err) => {
			// Connection failed — fall back to direct spawn
			process.stderr.write(`IPC connection failed (${err.message}), falling back to direct mode\n`);
			spawnDirect(projectPath).then(resolve, reject);
		});

		socket.on("close", () => resolve());
	});
}

function findMcpServerManifest(projectPath: string): string | null {
	// Search for the MCP server crate relative to the project
	const candidates = [
		join(projectPath, "libs", "mcp-server", "Cargo.toml"),
		join(projectPath, "..", "libs", "mcp-server", "Cargo.toml"),
		// Dev repo structure: project might be app/ inside dev repo
		join(projectPath, "..", "..", "libs", "mcp-server", "Cargo.toml"),
	];
	for (const candidate of candidates) {
		if (existsSync(candidate)) return candidate;
	}
	return null;
}

async function spawnDirect(projectPath: string): Promise<void> {
	const manifest = findMcpServerManifest(projectPath);

	if (manifest) {
		// Use the standalone MCP server crate
		return new Promise((resolve) => {
			const child = spawn("cargo", [
				"run", "--manifest-path", manifest,
				"--bin", "orqa-mcp-server", "--",
				projectPath,
			], {
				stdio: ["pipe", "pipe", "inherit"],
				env: { ...process.env, RUST_LOG: process.env.RUST_LOG ?? "info" },
			});

			process.stdin.pipe(child.stdin);
			child.stdout.pipe(process.stdout);

			child.on("error", (err) => {
				process.stderr.write(`Failed to start MCP server: ${err.message}\n`);
				process.stderr.write("Ensure libs/mcp-server exists and Rust toolchain is available.\n");
				process.exit(1);
			});

			child.on("close", () => resolve());
		});
	}

	// Last resort: try orqa-studio --mcp (legacy)
	return new Promise((resolve) => {
		const child = spawn("orqa-studio", ["--mcp", projectPath], {
			stdio: ["pipe", "pipe", "inherit"],
		});

		process.stdin.pipe(child.stdin);
		child.stdout.pipe(process.stdout);

		child.on("error", (err) => {
			process.stderr.write(`Failed to start orqa-studio: ${err.message}\n`);
			process.stderr.write("Ensure the OrqaStudio app is running (make dev), or libs/mcp-server crate exists.\n");
			process.exit(1);
		});

		child.on("close", () => resolve());
	});
}
