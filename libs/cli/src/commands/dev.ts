/**
 * Dev environment — manages Vite + Tauri + daemon + watch mode.
 *
 * `orqa dev` is the primary entry point for the development environment.
 * Run it in a separate terminal — it watches Rust sources and auto-rebuilds.
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

import { spawn, execSync, type ChildProcess as NodeChildProcess } from "node:child_process";
import { createServer as createNetServer } from "node:net";
import * as path from "node:path";
import * as fs from "node:fs";
import { platform } from "node:os";
import { getRoot } from "../lib/root.js";
import { getPort } from "../lib/ports.js";

function isWindows(): boolean { return platform() === "win32"; }
function npm(): string { return isWindows() ? "npm.cmd" : "npm"; }
function npx(): string { return isWindows() ? "npx.cmd" : "npx"; }
function rustEnv(): NodeJS.ProcessEnv {
	return { ...process.env, RUST_LOG: process.env["RUST_LOG"] ?? "debug" };
}
const VITE_PORT = getPort("vite");
const PORT_TIMEOUT_MS = 15_000;
const POLL_INTERVAL_MS = 500;
const WATCH_DEBOUNCE_MS = 500;

const COLOURS = {
	reset: "\x1b[0m",
	dim: "\x1b[2m",
	red: "\x1b[31m",
	green: "\x1b[32m",
	yellow: "\x1b[33m",
	blue: "\x1b[34m",
	magenta: "\x1b[35m",
	cyan: "\x1b[36m",
	orange: "\x1b[38;5;208m",
	teal: "\x1b[38;5;37m",
	pink: "\x1b[38;5;213m",
};

const USAGE = `
Usage: orqa dev [subcommand]

Subcommands:
  (none)              Start the full dev environment
  stop                Stop all processes gracefully
  kill                Force-kill all processes
  restart             Restart everything (daemon + frontend + app + services)
  restart daemon      Restart the validation daemon
  restart frontend    Restart Vite dev server
  restart app         Restart Tauri app (rebuild + relaunch)
  restart search      Restart search server (+ MCP)
  restart mcp         Restart MCP server
  restart lsp         Restart LSP server
  status              Show process status
  icons [--deploy]    Generate brand icons from SVG sources
  tool [args...]      Run the debug-tool submodule
`.trim();

// ── Logging ─────────────────────────────────────────────────────────────────

function prefixLines(prefix: string, colour: string, data: string): void {
	const text = data.toString().trimEnd();
	if (!text) return;
	for (const line of text.split("\n")) {
		const ts = new Date().toLocaleTimeString("en-GB", { hour12: false });
		console.log(
			`${COLOURS.dim}${ts}${COLOURS.reset} ${colour}[${prefix}]${COLOURS.reset} ${line}`,
		);
	}
}

function logCtrl(msg: string): void {
	prefixLines("ctrl", COLOURS.yellow, msg);
}

function logError(prefix: string, msg: string): void {
	prefixLines(prefix, COLOURS.red, msg);
}

function logSuccess(msg: string): void {
	prefixLines("ctrl", COLOURS.green, msg);
}

// ── Process Utilities ───────────────────────────────────────────────────────

function exec(cmd: string): string {
	try {
		return execSync(cmd, {
			encoding: "utf-8",
			timeout: 10_000,
			windowsHide: true,
		}).trim();
	} catch {
		return "";
	}
}

function findPidsOnPort(port: number): number[] {
	if (isWindows()) {
		const out = exec("netstat -ano");
		const pids = new Set<number>();
		for (const line of out.split("\n")) {
			if (line.includes(`:${port}`) && line.includes("LISTENING")) {
				const parts = line.trim().split(/\s+/);
				const pid = parseInt(parts[parts.length - 1] ?? "", 10);
				if (pid > 0) pids.add(pid);
			}
		}
		return [...pids];
	}
	if (platform() === "darwin") {
		return exec(`lsof -ti:${port}`)
			.split("\n")
			.map((s) => parseInt(s, 10))
			.filter((n) => n > 0);
	}
	return exec(
		`ss -tlnp sport = :${port} 2>/dev/null | awk 'NR>1{match($0,/pid=([0-9]+)/,a); print a[1]}'`,
	)
		.split("\n")
		.map((s) => parseInt(s, 10))
		.filter((n) => n > 0);
}

function findPidsByName(name: string): number[] {
	if (isWindows()) {
		return exec(
			`powershell.exe -NoProfile -Command "Get-Process -Name '${name}' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id"`,
		)
			.split("\n")
			.map((s) => parseInt(s.trim(), 10))
			.filter((n) => n > 0);
	}
	return exec(`pgrep -f "${name}"`)
		.split("\n")
		.map((s) => parseInt(s, 10))
		.filter((n) => n > 0);
}

function killProcessTree(pid: number): void {
	if (pid === process.pid) return;

	if (isWindows()) {
		const childPids = exec(
			`powershell.exe -NoProfile -Command "function Get-Tree($id){Get-CimInstance Win32_Process|Where-Object{$_.ParentProcessId -eq $id}|ForEach-Object{Get-Tree $_.ProcessId;$_.ProcessId}};Get-Tree ${pid}"`,
		)
			.split("\n")
			.map((s) => parseInt(s.trim(), 10))
			.filter((n) => n > 0);

		for (const childPid of childPids) {
			try {
				process.kill(childPid, "SIGKILL");
			} catch {
				/* already dead */
			}
		}
	}

	try {
		process.kill(pid, "SIGKILL");
	} catch {
		/* already dead */
	}
}

function isPortFree(port: number): Promise<boolean> {
	return new Promise((resolve) => {
		const server = createNetServer();
		server.once("error", () => resolve(false));
		server.once("listening", () => server.close(() => resolve(true)));
		server.listen(port, "127.0.0.1");
	});
}

async function waitForPort(port: number, timeoutMs: number, wantFree: boolean): Promise<boolean> {
	const deadline = Date.now() + timeoutMs;
	while (Date.now() < deadline) {
		const free = await isPortFree(port);
		if (wantFree === free) return true;
		await sleep(POLL_INTERVAL_MS);
	}
	return false;
}

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

// ── Control File (IPC) ──────────────────────────────────────────────────────

interface ControlFileState {
	pid: number;
	state: string;
	vite: number | null;
	rust: number | null;
	search: number | null;
	mcp: number | null;
	lsp: number | null;
}

function getControlFilePath(root: string): string {
	return path.join(root, "tmp", "dev-controller.json");
}

function getSignalFilePath(root: string): string {
	return path.join(root, "tmp", "dev-signal");
}

function ensureTmpDir(root: string): void {
	const tmpDir = path.join(root, "tmp");
	if (!fs.existsSync(tmpDir)) {
		fs.mkdirSync(tmpDir, { recursive: true });
	}
}

function writeControlFile(root: string, state: Omit<ControlFileState, "pid">): void {
	ensureTmpDir(root);
	fs.writeFileSync(
		getControlFilePath(root),
		JSON.stringify({ pid: process.pid, ...state }, null, 2),
	);
}

function readControlFile(root: string): ControlFileState | null {
	try {
		return JSON.parse(fs.readFileSync(getControlFilePath(root), "utf-8")) as ControlFileState;
	} catch {
		return null;
	}
}

function removeControlFile(root: string): void {
	try {
		fs.unlinkSync(getControlFilePath(root));
	} catch {
		/* ignore */
	}
}

function processIsAlive(pid: number): boolean {
	try {
		process.kill(pid, 0);
		return true;
	} catch {
		return false;
	}
}

// ── Child Process Management ────────────────────────────────────────────────

interface ManagedProcess {
	name: string;
	colour: string;
	child: NodeChildProcess | null;
	running: boolean;
}

function createManagedProcess(name: string, colour: string): ManagedProcess {
	return { name, colour, child: null, running: false };
}

function spawnManaged(
	mp: ManagedProcess,
	cmd: string,
	args: string[],
	opts: {
		cwd?: string;
		env?: NodeJS.ProcessEnv;
		stdinMode?: "ignore" | "pipe";
	} = {},
): ManagedProcess {
	const stdinMode = opts.stdinMode ?? "ignore";
	mp.child = spawn(cmd, args, {
		cwd: opts.cwd,
		env: opts.env ?? { ...process.env },
		stdio: [stdinMode, "pipe", "pipe"],
		shell: isWindows(),
		windowsHide: true,
	});
	mp.running = true;

	mp.child.stdout?.on("data", (data: Buffer) =>
		prefixLines(mp.name, mp.colour, data.toString()),
	);
	mp.child.stderr?.on("data", (data: Buffer) =>
		prefixLines(mp.name, mp.colour, data.toString()),
	);

	mp.child.on("close", (code) => {
		mp.running = false;
		if (code !== 0 && code !== null) {
			logError(mp.name, `Exited with code ${code}`);
		}
	});

	return mp;
}

function killManaged(mp: ManagedProcess): void {
	if (mp.child && mp.running && mp.child.pid) {
		prefixLines(mp.name, mp.colour, "Stopping...");
		killProcessTree(mp.child.pid);
		mp.running = false;
	}
}

// ── Kill All (orphan cleanup) ───────────────────────────────────────────────

async function killAll(root: string): Promise<void> {
	logCtrl("Stopping OrqaStudio processes...");

	const pidsToKill = new Set<number>();

	for (const name of ["orqa-studio", "cargo-tauri"]) {
		for (const pid of findPidsByName(name)) {
			logCtrl(`Found ${name} (PID ${pid})`);
			pidsToKill.add(pid);
		}
	}
	for (const port of [VITE_PORT, 5173, getPort("dashboard")]) {
		for (const pid of findPidsOnPort(port)) {
			logCtrl(`Found process on port ${port} (PID ${pid})`);
			pidsToKill.add(pid);
		}
	}

	if (pidsToKill.size === 0) {
		logCtrl("No OrqaStudio processes found.");
	} else {
		for (const pid of pidsToKill) {
			logCtrl(`Killing PID ${pid}...`);
			killProcessTree(pid);
		}
	}

	logCtrl("Waiting for ports to release...");
	const freed = await waitForPort(VITE_PORT, PORT_TIMEOUT_MS, true);
	if (!freed) {
		for (const pid of findPidsOnPort(VITE_PORT)) {
			logCtrl(`Force killing PID ${pid} on port ${VITE_PORT}...`);
			killProcessTree(pid);
		}
		const retried = await waitForPort(VITE_PORT, 5_000, true);
		if (!retried) {
			logError("ctrl", `FAILED: Port ${VITE_PORT} still in use`);
			process.exit(1);
		}
	}

	removeControlFile(root);
	logSuccess("All processes stopped.");
}

// ── Controller (foreground long-running process) ────────────────────────────

async function startController(root: string, opts: { watch: boolean } = { watch: true }): Promise<void> {
	const existing = readControlFile(root);
	if (existing && processIsAlive(existing.pid)) {
		logError(
			"ctrl",
			`Controller already running (PID ${existing.pid}). Use 'orqa dev stop' first.`,
		);
		process.exit(1);
	}
	if (existing) removeControlFile(root);

	// Kill any orphaned processes from previous runs
	await killAll(root);

	console.log("");
	console.log(
		`${COLOURS.yellow}╔══════════════════════════════════════════════╗${COLOURS.reset}`,
	);
	console.log(
		`${COLOURS.yellow}║      OrqaStudio Dev Environment               ║${COLOURS.reset}`,
	);
	console.log(
		`${COLOURS.yellow}╚══════════════════════════════════════════════╝${COLOURS.reset}`,
	);
	console.log("");

	const appDir = path.join(root, "app");
	const uiDir = path.join(appDir, "ui");
	const libsDir = path.join(root, "libs");
	const npmCmd = npm();

	writeControlFile(root, {
		state: "starting",
		vite: null,
		rust: null,
		search: null,
		mcp: null,
		lsp: null,
	});

	// ── 1. Start Vite ────────────────────────────────────────────────────
	logCtrl("Starting Vite dev server...");
	const vite = createManagedProcess("vite", COLOURS.cyan);
	spawnManaged(vite, npmCmd, ["run", "dev"], { cwd: uiDir });

	logCtrl(`Waiting for Vite on port ${VITE_PORT}...`);
	const viteReady = await waitForPort(VITE_PORT, 30_000, false);
	if (!viteReady) {
		logError("ctrl", "Vite failed to start within 30s");
		killManaged(vite);
		process.exit(1);
	}
	logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);

	// ── 1b. Start TypeScript library watch builds ────────────────────────
	// These run tsc --watch so linked packages rebuild automatically.
	const tsLibs = ["libs/sdk", "libs/graph-visualiser", "libs/logger"];
	for (const lib of tsLibs) {
		const libDir = path.join(root, lib);
		const tsconfigPath = path.join(libDir, "tsconfig.json");
		if (!fs.existsSync(tsconfigPath)) continue;

		const proc = createManagedProcess(`tsc:${lib.split("/").pop()}`, COLOURS.dim);
		spawnManaged(proc, npx(), ["tsc", "--watch", "--preserveWatchOutput"], { cwd: libDir });
		logCtrl(`TypeScript watch: ${lib}`);
	}

	// ── 2. Start search, MCP, LSP servers ────────────────────────────────
	const searchProc = createManagedProcess("search", COLOURS.orange);
	const mcpProc = createManagedProcess("mcp", COLOURS.teal);
	const lspProc = createManagedProcess("lsp", COLOURS.pink);

	function startSearch(): void {
		logCtrl("Starting search server...");
		spawnManaged(searchProc, "cargo", [
			"run",
			"--manifest-path", path.join(libsDir, "search", "Cargo.toml"),
			"--bin", "orqa-search-server",
			"--", appDir,
		], {
			stdinMode: "pipe",
			env: rustEnv(),
		});
	}

	function startMcp(): void {
		logCtrl("Starting MCP server...");
		spawnManaged(mcpProc, "cargo", [
			"run",
			"--manifest-path", path.join(libsDir, "mcp-server", "Cargo.toml"),
			"--bin", "orqa-mcp-server",
			"--", appDir,
		], {
			stdinMode: "pipe",
			env: rustEnv(),
		});
	}

	function startLsp(): void {
		logCtrl("Starting LSP server...");
		spawnManaged(lspProc, "cargo", [
			"run",
			"--manifest-path", path.join(libsDir, "lsp-server", "Cargo.toml"),
			"--bin", "orqa-lsp-server",
			"--", appDir,
		], {
			stdinMode: "pipe",
			env: rustEnv(),
		});
	}

	// Start search first (MCP depends on it), then MCP + LSP
	startSearch();
	await sleep(2_000);
	startMcp();
	startLsp();
	logSuccess("Search, MCP, and LSP servers starting.");

	// ── 3. Build + Run Rust (cargo run, no --watch) ──────────────────────
	const rust = createManagedProcess("rust", COLOURS.magenta);

	async function startRust(): Promise<void> {
		logCtrl("Building Rust app (this may take a while on first run)...");

		writeControlFile(root, {
			state: "building",
			vite: vite.child?.pid ?? null,
			rust: null,
			search: searchProc.child?.pid ?? null,
			mcp: mcpProc.child?.pid ?? null,
			lsp: lspProc.child?.pid ?? null,
		});

		spawnManaged(rust, "cargo", [
			"run",
			"--manifest-path", path.join(appDir, "backend/src-tauri/Cargo.toml"),
			"--no-default-features",
			"--color", "always",
		], {
			env: rustEnv(),
		});

		// Wait for the app binary to actually start (cargo compiles first, then runs).
		// The Tauri app writes an IPC port file when it's ready.
		const ipcPortFile = path.join(
			process.env["LOCALAPPDATA"] ?? path.join(process.env["HOME"] ?? "~", ".local", "share"),
			"com.orqastudio.app",
			"ipc.port",
		);

		const buildDeadline = Date.now() + 300_000; // 5 minutes for compilation
		let appReady = false;

		// First wait for the process to be alive (cargo finished compiling)
		while (Date.now() < buildDeadline) {
			await sleep(2_000);

			if (!rust.child || rust.child.exitCode !== null) {
				// Process exited during build — don't wait forever
				logError("ctrl", "Rust app exited during build.");
				break;
			}

			// Check if the IPC port file was recently written (app is up)
			try {
				const stat = fs.statSync(ipcPortFile);
				// Consider "ready" if port file was written in the last 30 seconds
				if (Date.now() - stat.mtimeMs < 30_000) {
					appReady = true;
					break;
				}
			} catch {
				// Port file doesn't exist yet — still building/starting
			}
		}

		if (appReady) {
			logSuccess("App loaded and ready.");
		} else {
			logCtrl("App may still be loading — check the window.");
		}

		writeControlFile(root, {
			state: "running",
			vite: vite.child?.pid ?? null,
			rust: rust.child?.pid ?? null,
			search: searchProc.child?.pid ?? null,
			mcp: mcpProc.child?.pid ?? null,
			lsp: lspProc.child?.pid ?? null,
		});

		// When app exits cleanly (code 0 = user closed window), shut down everything.
		// Non-zero = crash → stay alive for restart-tauri.
		rust.child?.on("close", (code) => {
			if (code === 0 || code === null) {
				logCtrl("App window closed. Shutting down...");
				killManaged(searchProc);
				killManaged(mcpProc);
				killManaged(lspProc);
				killManaged(vite);
				removeControlFile(root);
				cleanupSignalFile(root);
				process.exit(0);
			} else {
				writeControlFile(root, {
					state: "app-crashed",
					vite: vite.child?.pid ?? null,
					rust: null,
					search: searchProc.child?.pid ?? null,
					mcp: mcpProc.child?.pid ?? null,
					lsp: lspProc.child?.pid ?? null,
				});
				logCtrl(
					`App crashed (code ${code}). Use 'orqa dev restart-tauri' to relaunch.`,
				);
			}
		});
	}

	await startRust();

	// ── 4. Rust Source File Watchers ────────────────────────────────────
	if (opts.watch) {
		logCtrl("Setting up Rust source file watchers...");
		setupRustWatchers([
			{
				label: "daemon (validation)",
				dir: path.join(libsDir, "validation", "src"),
				manifest: path.join(libsDir, "validation", "Cargo.toml"),
				restart: async () => {
					// Restart the daemon (separate process, managed via daemon.ts)
					try {
						const { runDaemonCommand } = await import("./daemon.js");
						await runDaemonCommand(["restart"]);
					} catch {
						logError("watch", "Daemon restart failed — may need manual restart");
					}
				},
			},
			{
				label: "search server",
				dir: path.join(libsDir, "search", "src"),
				manifest: path.join(libsDir, "search", "Cargo.toml"),
				restart: async () => {
					killManaged(searchProc);
					await sleep(500);
					startSearch();
					await sleep(2_000);
					// MCP depends on search, restart it too
					killManaged(mcpProc);
					await sleep(500);
					startMcp();
					writeFullControlFile("running");
				},
			},
			{
				label: "MCP server",
				dir: path.join(libsDir, "mcp-server", "src"),
				manifest: path.join(libsDir, "mcp-server", "Cargo.toml"),
				restart: async () => {
					killManaged(mcpProc);
					await sleep(500);
					startMcp();
					writeFullControlFile("running");
				},
			},
			{
				label: "LSP server",
				dir: path.join(libsDir, "lsp-server", "src"),
				manifest: path.join(libsDir, "lsp-server", "Cargo.toml"),
				restart: async () => {
					killManaged(lspProc);
					await sleep(500);
					startLsp();
					writeFullControlFile("running");
				},
			},
			{
				label: "Tauri app",
				dir: path.join(appDir, "backend", "src-tauri", "src"),
				manifest: path.join(appDir, "backend", "src-tauri", "Cargo.toml"),
				restart: async () => {
					killManaged(rust);
					await sleep(1_000);
					await startRust();
				},
			},
		]);
		logSuccess("Rust file watchers active.");

		logCtrl("Setting up plugin source watchers...");
		setupPluginWatchers(root);
		logSuccess("Plugin file watchers active.");
	} else {
		logCtrl("File watching disabled (--no-watch).");
	}

	// ── 5. Signal File Handling ──────────────────────────────────────────
	const signalFile = getSignalFilePath(root);
	let lastMtime = 0;

	function writeFullControlFile(state: string): void {
		writeControlFile(root, {
			state,
			vite: vite.child?.pid ?? null,
			rust: rust.child?.pid ?? null,
			search: searchProc.child?.pid ?? null,
			mcp: mcpProc.child?.pid ?? null,
			lsp: lspProc.child?.pid ?? null,
		});
	}

	async function processSignal(signal: string): Promise<void> {
		if (signal === "restart-vite") {
			logCtrl("Restart Vite signal received...");
			killManaged(vite);
			await waitForPort(VITE_PORT, PORT_TIMEOUT_MS, true);
			logCtrl("Restarting Vite...");
			spawnManaged(vite, npmCmd, ["run", "dev"], { cwd: uiDir });
			const ready = await waitForPort(VITE_PORT, 30_000, false);
			if (!ready) {
				logError("ctrl", "Vite failed to restart within 30s");
				return;
			}
			logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
		} else if (signal === "restart-tauri") {
			logCtrl("Restart Tauri signal received — rebuilding app...");
			killManaged(rust);
			await sleep(1_000);
			await startRust();
			logSuccess("App restarted. Vite was kept alive.");
		} else if (signal === "restart-search") {
			logCtrl("Restart search signal received — restarting search (and MCP)...");
			killManaged(mcpProc);
			killManaged(searchProc);
			await sleep(500);
			startSearch();
			await sleep(2_000);
			startMcp();
			writeFullControlFile("running");
			logSuccess("Search and MCP restarted.");
		} else if (signal === "restart-mcp") {
			logCtrl("Restart MCP signal received...");
			killManaged(mcpProc);
			await sleep(500);
			startMcp();
			writeFullControlFile("running");
			logSuccess("MCP server restarted.");
		} else if (signal === "restart-lsp") {
			logCtrl("Restart LSP signal received...");
			killManaged(lspProc);
			await sleep(500);
			startLsp();
			writeFullControlFile("running");
			logSuccess("LSP server restarted.");
		} else if (signal === "restart-daemon") {
			logCtrl("Restart daemon signal received...");
			try {
				const { runDaemonCommand } = await import("./daemon.js");
				await runDaemonCommand(["restart"]);
				logSuccess("Daemon restarted.");
			} catch {
				logError("ctrl", "Daemon restart failed — may need manual restart");
			}
		} else if (signal === "restart") {
			logCtrl("Full restart signal received — restarting everything...");
			// Restart daemon first
			try {
				const { runDaemonCommand } = await import("./daemon.js");
				await runDaemonCommand(["restart"]);
				logSuccess("Daemon restarted.");
			} catch {
				logError("ctrl", "Daemon restart failed");
			}
			killManaged(rust);
			killManaged(mcpProc);
			killManaged(lspProc);
			killManaged(searchProc);
			killManaged(vite);
			await waitForPort(VITE_PORT, PORT_TIMEOUT_MS, true);
			logCtrl("Restarting Vite...");
			spawnManaged(vite, npmCmd, ["run", "dev"], { cwd: uiDir });
			await waitForPort(VITE_PORT, 30_000, false);
			logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
			startSearch();
			await sleep(2_000);
			startMcp();
			startLsp();
			await startRust();
			logSuccess("Full restart complete.");
		} else if (signal === "stop") {
			logCtrl("Stop signal received — shutting down...");
			closeAllWatchers();
			killManaged(rust);
			killManaged(searchProc);
			killManaged(mcpProc);
			killManaged(lspProc);
			killManaged(vite);
			removeControlFile(root);
			cleanupSignalFile(root);
			fs.unwatchFile(signalFile);
			logSuccess("All processes stopped.");
			process.exit(0);
		}
	}

	// Watch for signal file changes (written by restart subcommands)
	fs.watchFile(signalFile, { interval: 500 }, async (curr) => {
		if (curr.mtimeMs <= lastMtime) return;
		lastMtime = curr.mtimeMs;
		try {
			const signal = fs.readFileSync(signalFile, "utf-8").trim();
			await processSignal(signal);
		} catch {
			// Signal file may not exist yet
		}
	});

	// Handle Ctrl+C / terminal close
	function shutdown(): void {
		logCtrl("Shutting down...");
		closeAllWatchers();
		fs.unwatchFile(signalFile);
		killManaged(rust);
		killManaged(searchProc);
		killManaged(mcpProc);
		killManaged(lspProc);
		killManaged(vite);
		removeControlFile(root);
		cleanupSignalFile(root);
		process.exit(0);
	}

	process.on("SIGINT", shutdown);
	process.on("SIGTERM", shutdown);
	if (isWindows()) {
		process.on("SIGHUP", shutdown);
	}

	logCtrl("Dev environment running. Press Ctrl+C to stop.");
	logCtrl("Use 'orqa dev restart-tauri' to rebuild the app (Vite stays alive).");
	logCtrl("Use 'orqa dev stop' to shut everything down.");
}

function cleanupSignalFile(root: string): void {
	try {
		fs.unlinkSync(getSignalFilePath(root));
	} catch {
		/* ignore */
	}
}

// ── File Watcher (Rust source → auto-rebuild) ───────────────────────────────

interface WatchTarget {
	/** Human-readable label for logs */
	label: string;
	/** Directory to watch (recursive) */
	dir: string;
	/** Cargo.toml manifest path for targeted rebuild */
	manifest: string;
	/** Function to restart the affected process after a successful rebuild */
	restart: () => Promise<void>;
}

/** Active fs.watch handles — closed on shutdown */
const activeWatchers: fs.FSWatcher[] = [];

function setupRustWatchers(targets: WatchTarget[]): void {
	for (const target of targets) {
		if (!fs.existsSync(target.dir)) {
			logCtrl(`Watch: skipping ${target.label} — ${target.dir} does not exist`);
			continue;
		}

		let debounceTimer: ReturnType<typeof setTimeout> | null = null;
		let rebuilding = false;

		const watcher = fs.watch(target.dir, { recursive: true }, (_event, filename) => {
			// Only care about Rust source files
			if (!filename || !filename.endsWith(".rs")) return;

			// Debounce: reset timer on each change
			if (debounceTimer) clearTimeout(debounceTimer);
			debounceTimer = setTimeout(() => {
				void handleChange(target, filename ?? "unknown");
			}, WATCH_DEBOUNCE_MS);
		});

		async function handleChange(t: WatchTarget, file: string): Promise<void> {
			if (rebuilding) return;
			rebuilding = true;

			const relPath = path.relative(t.dir, path.join(t.dir, file));
			logCtrl(`Detected change in ${t.label}: ${relPath} → rebuilding...`);

			try {
				await runCargoBuild(t.manifest);
				logSuccess(`${t.label} rebuilt successfully. Restarting...`);
				await t.restart();
				logSuccess(`${t.label} restarted.`);
			} catch (err: unknown) {
				const msg = err instanceof Error ? err.message : String(err);
				logError("watch", `Build failed for ${t.label}: ${msg}`);
				logCtrl("Keeping old binary running. Fix the error and save again.");
			} finally {
				rebuilding = false;
			}
		}

		watcher.on("error", (err) => {
			logError("watch", `Watcher error for ${target.label}: ${err.message}`);
		});

		activeWatchers.push(watcher);
		logCtrl(`Watching ${target.label}: ${target.dir}`);
	}
}

function runCargoBuild(manifestPath: string): Promise<void> {
	return new Promise((resolve, reject) => {
		const child = spawn("cargo", ["build", "--manifest-path", manifestPath, "--color", "always"], {
			stdio: ["ignore", "pipe", "pipe"],
			shell: isWindows(),
			windowsHide: true,
		});

		child.stdout?.on("data", (data: Buffer) => prefixLines("build", COLOURS.yellow, data.toString()));
		child.stderr?.on("data", (data: Buffer) => prefixLines("build", COLOURS.yellow, data.toString()));

		child.on("close", (code) => {
			if (code === 0) resolve();
			else reject(new Error(`cargo build exited with code ${code}`));
		});

		child.on("error", reject);
	});
}

function closeAllWatchers(): void {
	for (const w of activeWatchers) {
		try { w.close(); } catch { /* ignore */ }
	}
	activeWatchers.length = 0;
}

// ── Plugin File Watchers ────────────────────────────────────────────────────

/**
 * Watch plugin src/ directories for changes. When a plugin source file changes,
 * rebuild the plugin (`npm run build`) and refresh its content (`orqa plugin refresh`).
 */
function setupPluginWatchers(projectRoot: string): void {
	const pluginDirs = ["plugins", "connectors"];
	const npmCmd = npm();

	for (const baseDir of pluginDirs) {
		const absBase = path.join(projectRoot, baseDir);
		if (!fs.existsSync(absBase)) continue;

		for (const entry of fs.readdirSync(absBase, { withFileTypes: true })) {
			if (!entry.isDirectory()) continue;

			const pluginDir = path.join(absBase, entry.name);
			const pkgPath = path.join(pluginDir, "package.json");
			if (!fs.existsSync(pkgPath)) continue;

			let pkg: { name?: string; scripts?: Record<string, string> };
			try {
				pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
			} catch {
				continue;
			}

			if (!pkg.scripts?.["build"]) continue;

			const srcDir = path.join(pluginDir, "src");
			if (!fs.existsSync(srcDir)) continue;

			const label = pkg.name ?? entry.name;
			let debounceTimer: ReturnType<typeof setTimeout> | null = null;
			let building = false;

			const watcher = fs.watch(srcDir, { recursive: true }, (_event, filename) => {
				if (!filename) return;
				// Watch TS, Svelte, CSS, and JSON source files
				if (!/\.(ts|svelte|css|json|js)$/.test(filename)) return;

				if (debounceTimer) clearTimeout(debounceTimer);
				debounceTimer = setTimeout(() => {
					void rebuildPlugin(label, pluginDir, npmCmd, filename ?? "unknown");
				}, WATCH_DEBOUNCE_MS);
			});

			async function rebuildPlugin(
				pluginLabel: string,
				dir: string,
				npm: string,
				file: string,
			): Promise<void> {
				if (building) return;
				building = true;

				logCtrl(`Plugin change: ${pluginLabel} (${file}) → rebuilding...`);

				try {
					await runNpmBuild(npm, dir);
					logSuccess(`${pluginLabel} rebuilt.`);

					// Refresh content so .orqa/ gets the latest
					try {
						const { runPluginCommand } = await import("./plugin.js");
						await runPluginCommand(["refresh", "--plugin", pluginLabel]);
					} catch {
						logCtrl(`Content refresh skipped for ${pluginLabel}`);
					}
				} catch (err: unknown) {
					const msg = err instanceof Error ? err.message : String(err);
					logError("watch", `Plugin build failed for ${pluginLabel}: ${msg}`);
				} finally {
					building = false;
				}
			}

			watcher.on("error", (err) => {
				logError("watch", `Plugin watcher error for ${label}: ${err.message}`);
			});

			activeWatchers.push(watcher);
			logCtrl(`Watching plugin: ${label} (${srcDir})`);
		}
	}
}

function runNpmBuild(npmCmd: string, cwd: string): Promise<void> {
	return new Promise((resolve, reject) => {
		const child = spawn(npmCmd, ["run", "build"], {
			cwd,
			stdio: ["ignore", "pipe", "pipe"],
			shell: isWindows(),
			windowsHide: true,
		});

		child.stdout?.on("data", (data: Buffer) => prefixLines("plugin", COLOURS.pink, data.toString()));
		child.stderr?.on("data", (data: Buffer) => prefixLines("plugin", COLOURS.pink, data.toString()));

		child.on("close", (code) => {
			if (code === 0) resolve();
			else reject(new Error(`npm run build exited with code ${code}`));
		});

		child.on("error", reject);
	});
}

// ── Subcommand: dev (spawn controller in background) ────────────────────────

async function cmdDev(root: string): Promise<void> {
	const existing = readControlFile(root);
	if (existing && processIsAlive(existing.pid)) {
		logSuccess(
			`Dev environment already running (PID ${existing.pid}).`,
		);
		return;
	}
	if (existing) removeControlFile(root);

	// 1. Install all workspace dependencies from root
	const npmCmd = npm();
	logCtrl("Installing dependencies...");
	try {
		execSync(`${npmCmd} install`, { cwd: root, stdio: "inherit" });
	} catch {
		logCtrl("npm install failed — dependencies may be stale");
	}

	// 2. Build all Rust binaries (daemon, MCP, LSP, app backend)
	logCtrl("Building Rust binaries...");
	try {
		execSync("cargo build --workspace --color always", { cwd: root, stdio: "inherit" });
	} catch {
		logCtrl("Rust build failed — some binaries may be stale");
	}

	// 3. Initial TS library build (so dist/ exists for linked packages)
	logCtrl("Building TypeScript libraries...");
	for (const lib of ["libs/sdk", "libs/graph-visualiser", "libs/logger"]) {
		const libDir = path.join(root, lib);
		const distDir = path.join(libDir, "dist");
		// Only build if dist/ doesn't exist yet — watchers handle incremental
		if (!fs.existsSync(distDir)) {
			try {
				execSync(`${npmCmd} run build`, { cwd: libDir, stdio: "inherit" });
			} catch {
				logCtrl(`Initial build failed for ${lib}`);
			}
		}
	}

	// 4. Refresh plugin content (builds plugins, syncs to .orqa/)
	logCtrl("Building plugins and syncing content...");
	try {
		const { runPluginCommand } = await import("./plugin.js");
		await runPluginCommand(["refresh"]);
	} catch {
		// Non-fatal — content may be stale but dev can still start
	}

	// Start the validation daemon (skip if already running)
	const daemonPort = getPort("daemon");
	let daemonUp = false;
	try {
		const res = await fetch(`http://127.0.0.1:${daemonPort}/health`, {
			signal: AbortSignal.timeout(500),
		});
		daemonUp = res.ok;
	} catch { /* not running */ }

	if (daemonUp) {
		logCtrl("Daemon already running.");
	} else {
		try {
			const { runDaemonCommand } = await import("./daemon.js");
			await runDaemonCommand(["start"]);
		} catch {
			logCtrl("Daemon start failed — may need manual start: orqa daemon start");
		}
	}

	logCtrl("Starting dev environment...");

	// Spawn the controller as a detached process (this same script with __start-controller)
	const nodeCmd = process.execPath;
	const cliEntry = path.join(root, "libs/cli/dist/cli.js");
	const child = spawn(nodeCmd, [cliEntry, "dev", "__start-controller"], {
		cwd: root,
		detached: true,
		stdio: "ignore",
		windowsHide: true,
		env: { ...process.env },
	});
	child.unref();

	logCtrl(`Controller spawned (PID ${child.pid}). Waiting for ready...`);

	// Poll control file until state is "running"
	const READY_TIMEOUT_MS = 300_000; // 5 minutes — Rust compilation can be slow
	const deadline = Date.now() + READY_TIMEOUT_MS;
	let lastState = "";

	while (Date.now() < deadline) {
		await sleep(POLL_INTERVAL_MS);
		const ctrl = readControlFile(root);
		if (!ctrl) continue;

		if (!processIsAlive(ctrl.pid)) {
			logError("ctrl", "Controller process died during startup.");
			removeControlFile(root);
			process.exit(1);
		}

		// Show state transitions
		if (ctrl.state !== lastState) {
			lastState = ctrl.state;
			if (ctrl.state === "building") {
				logCtrl("Building Rust app — this may take a few minutes...");
			} else if (ctrl.state === "starting") {
				logCtrl("Services starting...");
			}
		}

		if (ctrl.state === "running") {
			logSuccess("Dev environment ready. App loaded.");
			return;
		}
	}

	logError("ctrl", "Timed out waiting for dev environment to become ready.");
	process.exit(1);
}

// ── Subcommand: stop ────────────────────────────────────────────────────────

async function cmdStop(root: string): Promise<void> {
	const ctrl = readControlFile(root);

	if (ctrl && processIsAlive(ctrl.pid)) {
		logCtrl("Signalling controller to stop...");
		ensureTmpDir(root);
		fs.writeFileSync(getSignalFilePath(root), "stop");

		// Wait for controller to exit
		const deadline = Date.now() + 10_000;
		while (Date.now() < deadline) {
			if (!processIsAlive(ctrl.pid)) {
				logSuccess("Controller stopped.");
				return;
			}
			await sleep(500);
		}
		logCtrl("Controller did not exit gracefully. Use 'orqa dev kill' to force.");
		return;
	}

	if (ctrl) removeControlFile(root);
	logCtrl("No controller running. Use 'orqa dev kill' to force-kill orphaned processes.");
}

// ── Subcommand: kill ────────────────────────────────────────────────────────

async function cmdKill(root: string): Promise<void> {
	await killAll(root);
}

// ── Subcommand: restart (send signal to running controller) ─────────────────

async function cmdSignal(root: string, signal: string, label: string): Promise<void> {
	const ctrl = readControlFile(root);
	if (ctrl && processIsAlive(ctrl.pid)) {
		logCtrl(`Signalling controller to ${label}...`);
		ensureTmpDir(root);
		fs.writeFileSync(getSignalFilePath(root), signal);
		logSuccess(`${label} signal sent. Watch the controller output for progress.`);
		return;
	}

	if (ctrl) removeControlFile(root);

	// For restart-tauri and restart (full), start the dev env if not running
	if (signal === "restart-tauri" || signal === "restart") {
		logCtrl("No controller running. Starting dev environment...");
		await cmdDev(root);
		return;
	}

	logError("ctrl", "No controller running. Use 'orqa dev' first.");
}

// ── Subcommand: status ──────────────────────────────────────────────────────

function cmdStatus(root: string): void {
	const ctrl = readControlFile(root);
	if (!ctrl) {
		console.log("No dev controller running.");
		return;
	}

	const alive = processIsAlive(ctrl.pid);
	console.log(`Controller PID: ${ctrl.pid} (${alive ? "alive" : "dead"})`);
	console.log(`State: ${ctrl.state}`);
	if (ctrl.vite) console.log(`Vite PID: ${ctrl.vite}`);
	if (ctrl.rust) console.log(`Rust PID: ${ctrl.rust}`);
	if (ctrl.search) console.log(`Search PID: ${ctrl.search}`);
	if (ctrl.mcp) console.log(`MCP PID: ${ctrl.mcp}`);
	if (ctrl.lsp) console.log(`LSP PID: ${ctrl.lsp}`);
}

// ── Subcommand: icons ───────────────────────────────────────────────────────

function cmdIcons(root: string, args: string[]): void {
	const brandScript = path.join(root, "libs/brand/scripts/generate-icons.mjs");
	if (!fs.existsSync(brandScript)) {
		console.error("Brand icon script not found. Are you in the dev repo root?");
		process.exit(1);
	}
	const iconArgs = args.join(" ");
	try {
		execSync(`node "${brandScript}" ${iconArgs}`, {
			cwd: path.join(root, "libs/brand"),
			stdio: "inherit",
		});
	} catch {
		process.exit(1);
	}
}

// ── Subcommand: tool ────────────────────────────────────────────────────────

function cmdDebugTool(root: string, args: string[]): void {
	const debugToolPaths = [
		path.join(root, "debug-tool", "debug-tool.sh"),
		path.join(root, "node_modules", ".bin", "orqa-debug"),
	];

	let debugToolPath: string | null = null;
	for (const p of debugToolPaths) {
		if (fs.existsSync(p)) {
			debugToolPath = p;
			break;
		}
	}

	if (!debugToolPath) {
		console.error("Debug tool not found. Ensure debug-tool submodule is initialized.");
		process.exit(1);
	}

	const cmd = `"${debugToolPath}" ${args.join(" ")}`;
	try {
		execSync(cmd, { encoding: "utf-8", stdio: "inherit" });
	} catch {
		process.exit(1);
	}
}

// ── Main entry point ────────────────────────────────────────────────────────

export async function runDevCommand(args: string[]): Promise<void> {
	if (args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return;
	}

	const root = getRoot();
	const sub = args[0] ?? "dev";

	switch (sub) {
		case "dev":
			await cmdDev(root);
			break;
		case "__start-controller":
			await startController(root);
			break;
		case "stop":
			await cmdStop(root);
			break;
		case "kill":
			await cmdKill(root);
			break;
		case "restart": {
			const target = args[1];
			if (!target) {
				await cmdSignal(root, "restart", "Full restart");
			} else {
				const signalMap: Record<string, [string, string]> = {
					daemon:   ["restart-daemon",  "Restart daemon"],
					frontend: ["restart-vite",    "Restart frontend (Vite)"],
					vite:     ["restart-vite",    "Restart Vite"],
					app:      ["restart-tauri",   "Restart app (Tauri)"],
					tauri:    ["restart-tauri",    "Restart Tauri"],
					search:   ["restart-search",  "Restart search (+ MCP)"],
					mcp:      ["restart-mcp",     "Restart MCP"],
					lsp:      ["restart-lsp",     "Restart LSP"],
				};
				const entry = signalMap[target];
				if (entry) {
					await cmdSignal(root, entry[0], entry[1]);
				} else {
					console.error(`Unknown restart target: ${target}`);
					console.error(`Available: ${Object.keys(signalMap).join(", ")}`);
					process.exit(1);
				}
			}
			break;
		}
		case "status":
			cmdStatus(root);
			break;
		case "icons":
			cmdIcons(root, args.slice(1));
			break;
		case "tool":
			cmdDebugTool(root, args.slice(1));
			break;
		default:
			console.error(`Unknown subcommand: ${sub}`);
			console.log(USAGE);
			process.exit(1);
	}
}
