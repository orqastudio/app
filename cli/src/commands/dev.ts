/**
 * Dev environment — manages Vite + Tauri + daemon + watch mode.
 *
 * `orqa dev` is the primary entry point for the development environment.
 * Run it in a separate terminal — it watches Rust sources and auto-rebuilds.
 *
 * MCP and LSP server lifecycle is owned exclusively by the daemon. The dev
 * controller starts the daemon and the daemon starts MCP/LSP. The controller
 * does NOT spawn orqa-mcp-server or orqa-lsp-server directly.
 *
 * orqa dev                        Start the full dev environment (Vite + Tauri + daemon)
 * orqa dev --legacy-dashboard     Start with the legacy dev.mjs dashboard instead of OrqaDev
 * orqa dev stop                   Stop gracefully
 * orqa dev kill                   Force-kill all processes
 * orqa dev restart                Restart Vite + Tauri
 * orqa dev restart-tauri          Restart Tauri only
 * orqa dev restart-vite           Restart Vite only
 * orqa dev status                 Show process status
 * orqa dev icons                  Generate brand icons from SVG sources
 * orqa dev tool                   Run the debug-tool submodule
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
function rustEnv(projectRoot?: string): NodeJS.ProcessEnv {
	return {
		...process.env,
		RUST_LOG: process.env["RUST_LOG"] ?? "debug",
		...(projectRoot ? { ORQA_PROJECT_ROOT: projectRoot } : {}),
	};
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
Usage: orqa dev [subcommand] [flags]

Subcommands:
  (none)              Launch OrqaDev (start processes from inside OrqaDev)
  start-processes     Start all dev processes in the foreground (used by OrqaDev)
  stop                Stop all processes gracefully
  kill                Force-kill all processes
  restart             Restart everything (daemon + frontend + app + services)
  restart daemon      Restart daemon (+ MCP and LSP, which the daemon owns)
  restart frontend    Restart Vite dev server
  restart app         Restart Tauri app (rebuild + relaunch)
  restart search      Restart search server
  status              Show process status
  icons [--deploy]    Generate brand icons from SVG sources
  tool [args...]      Run the debug-tool submodule

Flags:
  --legacy-dashboard  Use the legacy dev.mjs web dashboard instead of OrqaDev (deprecated)
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

/**
 * Poll the daemon health endpoint until it responds 200 or the timeout elapses.
 * Returns true if the daemon is ready, false if it timed out.
 * @param port - Daemon HTTP port.
 * @param timeoutMs - Maximum time to wait in milliseconds.
 */
async function waitForDaemon(port: number, timeoutMs: number): Promise<boolean> {
	const deadline = Date.now() + timeoutMs;
	const url = `http://127.0.0.1:${port}/health`;

	while (Date.now() < deadline) {
		try {
			const res = await fetch(url);
			if (res.ok) return true;
		} catch {
			// Daemon not yet listening — keep polling
		}
		await sleep(250);
	}

	return false;
}

// ── Control File (IPC) ──────────────────────────────────────────────────────

// MCP and LSP are owned by the daemon — their PIDs are tracked by the daemon,
// not by the dev controller.
interface ControlFileState {
	pid: number;
	state: string;
	app: number | null;
	search: number | null;
	dashboard: number | null;
	devtools: number | null;
}

function getControlFilePath(root: string): string {
	return path.join(root, ".state", "dev-controller.json");
}

function getSignalFilePath(root: string): string {
	return path.join(root, ".state", "dev-signal");
}

function ensureStateDir(root: string): void {
	const stateDir = path.join(root, ".state");
	if (!fs.existsSync(stateDir)) {
		fs.mkdirSync(stateDir, { recursive: true });
	}
}

function writeControlFile(root: string, state: Omit<ControlFileState, "pid">): void {
	ensureStateDir(root);
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

async function killAll(root: string, opts: { preserveDevtools?: boolean } = {}): Promise<void> {
	logCtrl("Stopping OrqaStudio processes...");

	// Stop daemon gracefully first (it manages its own PID file)
	try {
		const { runDaemonCommand } = await import("./daemon.js");
		await runDaemonCommand(["stop"]);
		logCtrl("Daemon stopped.");
	} catch { /* not running */ }

	const pidsToKill = new Set<number>();

	// Find all OrqaStudio processes by name.
	// When preserveDevtools is true (called from devtools via start-processes),
	// skip orqa-devtools AND cargo-tauri (the devtools runs under cargo-tauri).
	const processNames = [
		"orqa-studio",
		"orqa-mcp-server", "orqa-lsp-server", "orqa-search-server", "orqa-validation",
	];
	if (!opts.preserveDevtools) {
		processNames.push("orqa-devtools", "cargo-tauri");
	}
	for (const name of processNames) {
		for (const pid of findPidsByName(name)) {
			logCtrl(`Found ${name} (PID ${pid})`);
			pidsToKill.add(pid);
		}
	}
	// Find by port (Vite, dashboard, daemon).
	// When preserveDevtools, skip port 10421 (devtools Vite) and 5173 (could be devtools fallback).
	const portsToCheck = opts.preserveDevtools
		? [VITE_PORT, getPort("dashboard"), getPort("daemon")]
		: [VITE_PORT, 5173, getPort("dashboard"), getPort("daemon")];
	for (const port of portsToCheck) {
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

// ── Binary Discovery ───────────────────────────────────────────────────────

/**
 * Find a pre-built binary in workspace target dirs.
 * @param root - Project root directory.
 * @param name - Binary name without extension.
 * @returns Full path to the binary, falling back to name-only if not found.
 */
function findBin(root: string, name: string): string {
	const ext = isWindows() ? ".exe" : "";
	const candidates = [
		path.join(root, "target", "debug", `${name}${ext}`),
		path.join(root, "target", "release", `${name}${ext}`),
	];
	for (const c of candidates) {
		if (fs.existsSync(c)) return c;
	}
	return `${name}${ext}`;
}

// ── Controller (foreground long-running process) ────────────────────────────

async function startController(root: string, opts: { watch: boolean; legacyDashboard?: boolean; headless?: boolean } = { watch: true }): Promise<void> {
	const existing = readControlFile(root);
	if (existing && processIsAlive(existing.pid)) {
		logError(
			"ctrl",
			`Controller already running (PID ${existing.pid}). Use 'orqa dev stop' first.`,
		);
		process.exit(1);
	}
	if (existing) removeControlFile(root);

	// Kill any orphaned processes from previous runs.
	// Skip when headless — cmdStartProcesses already called killAll before building.
	if (!opts.headless) {
		await killAll(root);
	}

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

	// Tracks the detached dashboard process so it can be killed on shutdown.
	// Declared here so all writeControlFile calls can reference the PID.
	let dashboardProc: NodeChildProcess | null = null;

	writeControlFile(root, {
		state: "starting",
		app: null,
		search: null,
		dashboard: null,
		devtools: null,
	});

	// ── 0. Start dev UI (unless headless — OrqaDev launches this) ──────
	let devtoolsProc: NodeChildProcess | null = null;

	if (!opts.headless) {
		if (opts.legacyDashboard) {
			const dashboardScript = path.join(root, "tools", "debug", "dev.mjs");
			const logPath = path.join(root, ".state", "dev-controller.log");
			ensureStateDir(root);
			const logFd = fs.openSync(logPath, "a");
			dashboardProc = spawn("node", [dashboardScript], {
				cwd: root,
				detached: true,
				stdio: ["ignore", logFd, logFd],
			});
			fs.closeSync(logFd);
			dashboardProc.unref();
			logSuccess(`Legacy dashboard started (PID ${dashboardProc.pid ?? "?"}): http://localhost:${getPort("dashboard")}`);
		} else {
			const devtoolsBin = findBin(root, "orqa-devtools");
			if (fs.existsSync(devtoolsBin)) {
				logCtrl("Starting OrqaDev...");
				devtoolsProc = spawn(devtoolsBin, [], {
					cwd: root,
					detached: true,
					stdio: "ignore",
					env: rustEnv(root),
				});
				devtoolsProc.unref();
				logSuccess(`OrqaDev started (PID ${devtoolsProc.pid ?? "?"}).`);
			} else {
				logCtrl("orqa-devtools binary not found — skipping OrqaDev launch.");
			}
		}
	} else {
		logCtrl("Headless mode — skipping OrqaDev launch.");
	}

	writeControlFile(root, {
		state: "starting",
		app: null,
		search: null,
		dashboard: dashboardProc?.pid ?? null,
		devtools: devtoolsProc?.pid ?? null,
	});

	// ── 1. Start daemon ─────────────────────────────────────────────────
	logCtrl("Starting daemon...");
	try {
		const { runDaemonCommand } = await import("./daemon.js");
		await runDaemonCommand(["start"]);
	} catch {
		logCtrl("Daemon start failed — services may not have graph access");
	}

	// Wait for the daemon HTTP server to be ready before starting search and app.
	// Polling /health every 250ms; 10 second timeout.
	const daemonPort = getPort("daemon");
	const daemonReady = await waitForDaemon(daemonPort, 10_000);
	if (daemonReady) {
		logSuccess(`Daemon ready on port ${daemonPort}.`);
	} else {
		logCtrl(`Daemon did not respond on port ${daemonPort} within 10s — proceeding anyway.`);
	}

	// ── 2. Start search server (pre-built binary) ────────────────────────
	// MCP and LSP servers are managed exclusively by the daemon — do NOT spawn
	// them here. The daemon started above owns their full lifecycle.
	const searchProc = createManagedProcess("search", COLOURS.orange);

	function startSearch(): void {
		logCtrl("Starting search server...");
		spawnManaged(searchProc, findBin(root, "orqa-search-server"), [appDir], {
			stdinMode: "pipe",
			env: rustEnv(root),
		});
	}

	startSearch();
	logSuccess("Search server started.");

	// ── 4. TypeScript library watch builds ───────────────────────────────
	const tsLibs = ["libs/sdk", "libs/graph-visualiser", "libs/logger", "libs/types"];
	for (const lib of tsLibs) {
		const libDir = path.join(root, lib);
		const tsconfigPath = path.join(libDir, "tsconfig.json");
		if (!fs.existsSync(tsconfigPath)) continue;

		const proc = createManagedProcess(`tsc:${lib.split("/").pop()}`, COLOURS.dim);
		spawnManaged(proc, npx(), ["tsc", "--watch", "--preserveWatchOutput"], { cwd: libDir });
		logCtrl(`TypeScript watch: ${lib}`);
	}

	// ── 4b. Svelte components watch build ───────────────────────────────
	// svelte-package compiles the component library for consumers; --watch
	// re-runs on every source change so downstream packages pick up updates.
	const svelteComponentsDir = path.join(root, "libs", "svelte-components");
	if (fs.existsSync(path.join(svelteComponentsDir, "package.json"))) {
		const svelteWatch = createManagedProcess("tsc:svelte-components", COLOURS.dim);
		spawnManaged(svelteWatch, npx(), ["svelte-package", "--watch"], { cwd: svelteComponentsDir });
		logCtrl("TypeScript watch: libs/svelte-components");
	}

	// ── 5. Start Storybook dev server ───────────────────────────────────
	const storybookDir = path.join(root, "libs", "svelte-components");
	if (fs.existsSync(path.join(storybookDir, ".storybook"))) {
		const storybook = createManagedProcess("storybook", COLOURS.teal);
		const storybookPort = String(getPort("storybook"));
		spawnManaged(storybook, npx(), ["storybook", "dev", "-p", storybookPort, "--no-open"], { cwd: storybookDir });
		logCtrl(`Storybook dev server starting on port ${storybookPort}...`);
	}

	// ── 6. Start Tauri app (cargo tauri dev handles Vite + app) ─────────
	// cargo tauri dev:
	//   - Starts Vite via beforeDevCommand (HMR for frontend)
	//   - Compiles the app in dev mode (uses devUrl, not frontendDist)
	//   - Watches app Rust source and recompiles on change
	//   - Launches the app window
	const app = createManagedProcess("app", COLOURS.magenta);

	async function startRust(): Promise<void> {
		logCtrl("Starting Tauri app (cargo tauri dev)...");

		writeControlFile(root, {
			state: "building",
			app: null,
			search: searchProc.child?.pid ?? null,
			dashboard: dashboardProc?.pid ?? null,
			devtools: devtoolsProc?.pid ?? null,
		});

		spawnManaged(app, "cargo", [
			"tauri", "dev",
		], {
			cwd: appDir,
			env: rustEnv(root),
		});

		// Wait for the app process to appear (cargo tauri dev compiles then launches)
		const appDeadline = Date.now() + 300_000; // 5 min for compilation
		let appReady = false;

		while (Date.now() < appDeadline) {
			await sleep(3_000);

			if (!app.child || app.child.exitCode !== null) {
				logError("ctrl", "Tauri dev exited during startup.");
				break;
			}

			// cargo tauri dev starts Vite first, then compiles, then launches.
			// Detect the app by checking for the orqa-studio process.
			const procs = findPidsByName("orqa-studio");
			if (procs.length > 0) {
				await sleep(2_000); // Let the webview render
				appReady = true;
				break;
			}
		}

		if (appReady) {
			logSuccess("Tauri app loaded.");
		} else {
			logCtrl("Tauri app may still be compiling — check the terminal.");
		}

		writeControlFile(root, {
			state: "running",
			app: app.child?.pid ?? null,
			search: searchProc.child?.pid ?? null,
			dashboard: dashboardProc?.pid ?? null,
			devtools: devtoolsProc?.pid ?? null,
		});

		// When app exits cleanly (code 0 = user closed window), shut down everything.
		// Non-zero = crash → stay alive for restart-tauri.
		app.child?.on("close", (code) => {
			if (code === 0 || code === null) {
				logCtrl("App window closed. Shutting down...");
				killManaged(searchProc);
				if (dashboardProc?.pid) {
					try { process.kill(dashboardProc.pid, "SIGKILL"); } catch { /* already dead */ }
				}
				if (devtoolsProc?.pid) {
					try { process.kill(devtoolsProc.pid, "SIGKILL"); } catch { /* already dead */ }
				}
				removeControlFile(root);
				cleanupSignalFile(root);
				process.exit(0);
			} else {
				writeControlFile(root, {
					state: "app-crashed",
					app: null,
					search: searchProc.child?.pid ?? null,
					dashboard: dashboardProc?.pid ?? null,
					devtools: devtoolsProc?.pid ?? null,
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
		// The Tauri dev process watches app Rust sources. The daemon owns MCP and
		// LSP and watches their source changes internally. The dev controller only
		// needs to watch the search server, which Tauri does not cover.
		setupRustWatchers([
			{
				label: "Search server",
				dir: path.join(root, "engine", "search", "src"),
				manifest: path.join(root, "engine", "search", "Cargo.toml"),
				restart: async () => {
					killManaged(searchProc);
					await sleep(500);
					startSearch();
					writeFullControlFile("running");
				},
			},
			{
				label: "Daemon",
				dir: path.join(root, "daemon", "src"),
				manifest: path.join(root, "daemon", "Cargo.toml"),
				restart: async () => {
					logCtrl("Daemon source changed — restarting daemon...");
					try {
						const { runDaemonCommand } = await import("./daemon.js");
						await runDaemonCommand(["restart"]);
						logSuccess("Daemon restarted.");
					} catch {
						logError("watch", "Daemon restart failed — may need manual restart");
					}
				},
			},
			// MCP and LSP source watching is handled by the daemon
			// Tauri app watching is handled by cargo tauri dev
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
			app: app.child?.pid ?? null,
			search: searchProc.child?.pid ?? null,
			dashboard: dashboardProc?.pid ?? null,
			devtools: devtoolsProc?.pid ?? null,
		});
	}

	async function processSignal(signal: string): Promise<void> {
		if (signal === "restart-app") {
			// The app = cargo tauri dev (Vite + Tauri compilation + app window)
			logCtrl("Restarting app...");
			killManaged(app);
			await sleep(1_000);
			await startRust();
			logSuccess("App restarted.");
		} else if (signal === "restart-search") {
			logCtrl("Restart search signal received — restarting search server...");
			killManaged(searchProc);
			await sleep(500);
			startSearch();
			writeFullControlFile("running");
			logSuccess("Search server restarted.");
		} else if (signal === "restart-mcp" || signal === "restart-lsp") {
			// MCP and LSP are owned by the daemon — restart the daemon to restart them.
			logCtrl(`${signal} received — restarting daemon (daemon owns MCP and LSP)...`);
			try {
				const { runDaemonCommand } = await import("./daemon.js");
				await runDaemonCommand(["restart"]);
				logSuccess("Daemon restarted (MCP and LSP restarted by daemon).");
			} catch {
				logError("ctrl", "Daemon restart failed — may need manual restart");
			}
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
			// Restart daemon (it owns MCP and LSP — restarting it restarts them)
			try {
				const { runDaemonCommand } = await import("./daemon.js");
				await runDaemonCommand(["restart"]);
				logSuccess("Daemon restarted.");
			} catch {
				logError("ctrl", "Daemon restart failed");
			}
			// Restart search server and app
			killManaged(searchProc);
			killManaged(app);
			await sleep(500);
			startSearch();
			// Restart app (cargo tauri dev)
			await sleep(1_000);
			await startRust();
			logSuccess("Full restart complete.");
		} else if (signal === "stop") {
			logCtrl("Stop signal received — shutting down...");
			closeAllWatchers();
			killManaged(app);
			killManaged(searchProc);
			if (dashboardProc?.pid) {
				try { process.kill(dashboardProc.pid, "SIGKILL"); } catch { /* already dead */ }
			}
			if (devtoolsProc?.pid) {
				try { process.kill(devtoolsProc.pid, "SIGKILL"); } catch { /* already dead */ }
			}
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

	// Handle Ctrl+C / terminal close. MCP and LSP are owned by the daemon and
	// will be stopped when the daemon shuts down via the OS process tree.
	function shutdown(): void {
		logCtrl("Shutting down...");
		closeAllWatchers();
		fs.unwatchFile(signalFile);
		killManaged(app);
		killManaged(searchProc);
		if (dashboardProc?.pid) {
			try { process.kill(dashboardProc.pid, "SIGKILL"); } catch { /* already dead */ }
		}
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
 * @param projectRoot - Absolute path to the project root.
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
	const devtoolsDir = path.join(root, "devtools");
	if (!fs.existsSync(path.join(devtoolsDir, "src-tauri", "tauri.conf.json"))) {
		logError("ctrl", "devtools directory not found. Are you in the dev repo root?");
		process.exit(1);
	}

	// Kill any previous OrqaDev instance so cargo can overwrite the binary.
	const staleDevtools = findPidsByName("orqa-devtools");
	if (staleDevtools.length > 0) {
		logCtrl("Stopping previous OrqaDev instance...");
		for (const pid of staleDevtools) {
			killProcessTree(pid);
		}
		await sleep(1_000);
	}

	logCtrl("Launching OrqaDev (cargo tauri dev)...");
	logCtrl("Build output will appear below. Ctrl+C to stop.\n");
	const child = spawn("cargo", ["tauri", "dev"], {
		cwd: devtoolsDir,
		stdio: "inherit",
		env: rustEnv(root),
	});
	child.on("close", (code) => {
		process.exit(code ?? 0);
	});
}

/**
 * Start the process controller in the foreground with stdout/stderr piped.
 * Called by OrqaDev via `orqa dev start-processes`. Runs all prep steps
 * (npm install, cargo build, TS libs, plugin sync) then starts the
 * long-running controller with file watchers.
 */
async function cmdStartProcesses(root: string): Promise<void> {
	// 0. Rebuild the CLI itself (we may be running stale code)
	logCtrl("Rebuilding CLI...");
	try {
		execSync(`${npx()} tsc --project ${path.join(root, "cli/tsconfig.json")}`, {
			cwd: root,
			stdio: "inherit",
		});
	} catch {
		logCtrl("CLI build failed — running with current dist/");
	}

	// 1. Install all workspace dependencies from root
	logCtrl("Installing dependencies...");
	try {
		execSync(`${npm()} install`, { cwd: root, stdio: "inherit" });
	} catch {
		logCtrl("npm install failed — dependencies may be stale");
	}

	// 2. Stop all running OrqaStudio processes so cargo can overwrite binaries.
	// Preserve devtools — this command is called FROM devtools.
	await killAll(root, { preserveDevtools: true });

	// 3. Build all Rust binaries (except the app — cargo tauri dev handles it)
	logCtrl("Building Rust libraries and servers...");
	try {
		execSync("cargo build --workspace --exclude orqa-studio --exclude orqa-devtools --color always", { cwd: root, stdio: "inherit" });
	} catch {
		logCtrl("Rust build failed — some binaries may be stale");
	}

	// 4. Initial TS library build (so dist/ exists for linked packages)
	logCtrl("Building TypeScript libraries...");
	for (const lib of ["libs/sdk", "libs/graph-visualiser", "libs/logger", "libs/types"]) {
		const libDir = path.join(root, lib);
		const distDir = path.join(libDir, "dist");
		if (!fs.existsSync(distDir)) {
			try {
				execSync(`${npm()} run build`, { cwd: libDir, stdio: "inherit" });
			} catch {
				logCtrl(`Initial build failed for ${lib}`);
			}
		}
	}

	// 5. Install plugins from project.json (builds, syncs content, composes schema)
	logCtrl("Installing plugins from project.json...");
	try {
		const { cmdPluginSync } = await import("./install.js");
		cmdPluginSync(root);
	} catch {
		logCtrl("Plugin sync failed — falling back to refresh...");
		try {
			const { runPluginCommand } = await import("./plugin.js");
			await runPluginCommand(["refresh"]);
		} catch {
			// Non-fatal
		}
	}

	// 6. Start the long-running controller (daemon, search, Vite+Tauri, watchers)
	logCtrl("Starting dev environment...");
	await startController(root, { watch: true, headless: true });
}

// ── Subcommand: stop ────────────────────────────────────────────────────────

async function cmdStop(root: string): Promise<void> {
	const ctrl = readControlFile(root);

	if (ctrl && processIsAlive(ctrl.pid)) {
		logCtrl("Signalling controller to stop...");
		ensureStateDir(root);
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
	removeControlFile(root);
	cleanupSignalFile(root);
}

// ── Subcommand: restart (send signal to running controller) ─────────────────

async function cmdSignal(root: string, signal: string, label: string): Promise<void> {
	const ctrl = readControlFile(root);
	if (ctrl && processIsAlive(ctrl.pid)) {
		logCtrl(`Signalling controller to ${label}...`);
		ensureStateDir(root);
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
	if (ctrl.app) console.log(`App PID: ${ctrl.app}`);
	if (ctrl.search) console.log(`Search PID: ${ctrl.search}`);
	console.log("MCP/LSP: managed by daemon (use 'orqa daemon status' for details)");
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

/**
 * Dispatch the dev command: start the dev environment or a subcommand.
 * @param args - CLI arguments after "dev".
 */
export async function runDevCommand(args: string[]): Promise<void> {
	if (args[0] === "--help" || args[0] === "-h") {
		console.log(USAGE);
		return;
	}

	const root = getRoot();

	// Strip global flags before dispatching to subcommands.
	const legacyDashboard = args.includes("--legacy-dashboard");
	const filteredArgs = args.filter((a) => a !== "--legacy-dashboard");
	const sub = filteredArgs[0] ?? "dev";

	switch (sub) {
		case "dev":
			await cmdDev(root);
			break;
		case "start-processes":
			await cmdStartProcesses(root);
			break;
		case "__start-controller":
			await startController(root, { watch: true, legacyDashboard });
			break;
		case "stop":
			await cmdStop(root);
			break;
		case "kill":
			await cmdKill(root);
			break;
		case "restart": {
			const target = filteredArgs[1];
			if (!target) {
				await cmdSignal(root, "restart", "Full restart");
			} else {
				const signalMap: Record<string, [string, string]> = {
					daemon:   ["restart-daemon",  "Restart daemon (restarts MCP and LSP)"],
					app:      ["restart-app",     "Restart app (Vite + Tauri)"],
					search:   ["restart-search",  "Restart search server"],
					// mcp and lsp route through the daemon — the daemon owns their lifecycle
					mcp:      ["restart-mcp",     "Restart MCP (via daemon restart)"],
					lsp:      ["restart-lsp",     "Restart LSP (via daemon restart)"],
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
			cmdIcons(root, filteredArgs.slice(1));
			break;
		case "tool":
			cmdDebugTool(root, filteredArgs.slice(1));
			break;
		default:
			console.error(`Unknown subcommand: ${sub}`);
			console.log(USAGE);
			process.exit(1);
	}
}
