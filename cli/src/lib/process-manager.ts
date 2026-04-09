/**
 * Process manager for the OrqaStudio dev environment.
 *
 * Provides a dependency-aware graph of all build/service/watch processes.
 * The graph is read from package.json and Cargo.toml at startup — no hardcoded
 * lists. Topological sort (Kahn's algorithm) determines build order and which
 * nodes can build concurrently within a tier.
 *
 * Covers: types, graph construction, topological sort, build execution, and the
 * ProcessManager class. Service lifecycle (Task 3) and file-watching (Task 4)
 * fill in the remaining placeholder methods.
 */

import * as path from "node:path";
import * as fs from "node:fs";
import { spawn, execSync, type ChildProcess } from "node:child_process";
import { platform } from "node:os";
import { getPort } from "./ports.js";
import { assertNever } from "@orqastudio/types";

// ── Types ────────────────────────────────────────────────────────────────────

/**
 * Discriminated union of process node types. Determines build command, watch
 * strategy, and lifecycle behaviour.
 */
export type NodeKind =
	| "ts-library" // tsc build, watches src/, emits to dist/
	| "svelte-library" // svelte-package, watches src/, emits to dist/
	| "rust-workspace" // cargo build --workspace; implicit root for Rust nodes
	| "tauri-app" // cargo tauri dev (manages its own watch/rebuild cycle)
	| "service" // long-running process (daemon, search server)
	| "plugin"; // npm run build + orqa plugin refresh

/**
 * Status of a single process node at a point in time.
 */
export type NodeStatus =
	| "pending" // not yet built
	| "building" // build in progress
	| "built" // build succeeded, not yet running (for services/apps)
	| "build-failed" // build failed
	| "starting" // service/app is launching
	| "running" // service/app is alive and healthy
	| "watching" // built and watching for changes (libraries)
	| "rebuilding" // file change detected, rebuilding
	| "stopping" // graceful shutdown in progress
	| "stopped" // service was running but is now stopped
	| "crashed"; // service exited unexpectedly

/**
 * A single node in the process dependency graph. Represents one buildable or
 * runnable unit — a library, service, plugin, or the Tauri app.
 * Identity and structure fields are readonly; lifecycle fields (status, pid,
 * lastBuiltAt, lastError) are mutable — the ProcessManager updates them in place.
 */
export interface ProcessNode {
	/** Unique identifier (e.g. "constants", "sdk", "daemon", "app"). */
	readonly id: string;
	/** Human-readable display name (e.g. "@orqastudio/sdk"). */
	readonly name: string;
	/** What kind of process this is. */
	readonly kind: NodeKind;
	/** Current lifecycle status. */
	status: NodeStatus;
	/** Absolute path to the package/crate root directory. */
	readonly rootDir: string;
	/** Absolute path to the directory to watch for changes. */
	readonly watchDir: string;
	/** Glob patterns for files that trigger a rebuild (e.g. "**\/*.ts"). */
	readonly watchPatterns: string[];
	/** Node IDs that this node depends on (must build before this). */
	dependsOn: string[];
	/** Node IDs that depend on this node (must rebuild after this). Computed from dependsOn. */
	dependents: string[];
	/** PID of the running process (services/apps only). */
	pid: number | null;
	/** Timestamp of last successful build (ms since epoch). */
	lastBuiltAt: number | null;
	/** Last build error message, if any. */
	lastError: string | null;
}

/**
 * Structured event emitted by the process manager for each status transition.
 * Consumed by the OrqaDev UI and the CLI progress renderer.
 */
export interface NodeEvent {
	/** Unix timestamp in milliseconds. */
	readonly timestamp: number;
	/** Which node changed. */
	readonly nodeId: string;
	/** Human-readable node name. */
	readonly nodeName: string;
	/** The new status after this event. */
	readonly status: NodeStatus;
	/** What triggered this transition. */
	readonly trigger: "startup" | "file-change" | "dependency-rebuild" | "manual" | "crash";
	/** Duration of the operation in ms (for build/rebuild events). */
	readonly durationMs: number | null;
	/** Error message (for build-failed and crashed events). */
	readonly error: string | null;
	/** Changed file path that triggered a rebuild (for file-change triggers). */
	readonly changedFile: string | null;
}

/**
 * Immutable snapshot of the full dependency graph, computed once at startup.
 * Includes both the flat build order and the parallel tier decomposition.
 */
export interface ProcessGraph {
	/** All nodes keyed by ID. */
	nodes: Map<string, ProcessNode>;
	/** Topologically sorted flat list of node IDs. */
	readonly buildOrder: string[];
	/** Parallel build tiers — nodes in the same tier can build concurrently. */
	readonly buildTiers: string[][];
}

/**
 * Result of a buildAll() call. Separates successes from failures so callers
 * can decide whether to proceed to service/app startup.
 */
export interface BuildResult {
	/** Node IDs that built successfully. */
	readonly succeeded: string[];
	/** Node IDs that failed, paired with their error messages. */
	readonly failed: ReadonlyArray<{ readonly nodeId: string; readonly error: string }>;
}

// ── Package.json shape (minimal) ──────────────────────────────────────────────

/** Minimal shape of package.json files we read. */
interface PackageJson {
	name?: string;
	scripts?: Record<string, string>;
	dependencies?: Record<string, string>;
	peerDependencies?: Record<string, string>;
}

// ── Graph readers ─────────────────────────────────────────────────────────────

/**
 * Read a package.json file, returning null if it doesn't exist or cannot be
 * parsed. Safe to call on arbitrary paths.
 * @param pkgPath - Absolute path to the package.json file.
 * @returns The parsed package.json object, or null on failure.
 */
function readPackageJson(pkgPath: string): PackageJson | null {
	try {
		const raw = fs.readFileSync(pkgPath, "utf-8");
		return JSON.parse(raw) as PackageJson;
	} catch {
		return null;
	}
}

/**
 * Strip the `@orqastudio/` scope from a package name to produce a node ID.
 * e.g. `@orqastudio/sdk` → `sdk`.
 * @param name - The full scoped package name to strip.
 * @returns The node ID with the scope prefix removed.
 */
function packageNameToId(name: string): string {
	return name.replace(/^@orqastudio\//, "");
}

/**
 * Return the newest file mtime (in ms since epoch) anywhere under `dir`,
 * walking recursively. Returns 0 when `dir` is empty or unreadable.
 *
 * Used for incremental build skipping: if the newest dist/ file is newer than
 * the newest src/ file, the library is up-to-date and can skip a rebuild.
 * Skips node_modules and dotfiles for speed — those directories aren't source
 * inputs or build outputs.
 * @param dir - Absolute path to the directory to scan.
 * @returns The newest mtime in milliseconds since epoch, or 0 if unreadable.
 */
function newestMtime(dir: string): number {
	let newest = 0;
	const stack: string[] = [dir];
	while (stack.length > 0) {
		const current = stack.pop();
		if (current === undefined) break;
		let entries: fs.Dirent[];
		try {
			entries = fs.readdirSync(current, { withFileTypes: true });
		} catch {
			continue;
		}
		for (const entry of entries) {
			if (entry.name.startsWith(".") || entry.name === "node_modules") continue;
			const full = path.join(current, entry.name);
			if (entry.isDirectory()) {
				stack.push(full);
			} else if (entry.isFile()) {
				try {
					const mtime = fs.statSync(full).mtimeMs;
					if (mtime > newest) newest = mtime;
				} catch {
					// Ignore stat failures — best-effort scan.
				}
			}
		}
	}
	return newest;
}

/**
 * Return the watchPatterns for a given NodeKind.
 * @param kind - The node kind to look up patterns for.
 * @returns Array of glob patterns matching source files for that kind.
 */
function watchPatternsForKind(kind: NodeKind): string[] {
	switch (kind) {
		case "ts-library":
		case "svelte-library":
		case "plugin":
			return ["**/*.ts", "**/*.svelte", "**/*.js"];
		case "rust-workspace":
		case "service":
			return ["**/*.rs"];
		case "tauri-app":
			return ["**/*.ts", "**/*.svelte", "**/*.rs"];
		default:
			return assertNever(kind);
	}
}

/**
 * Determine NodeKind for an npm package from its directory location and build
 * script. Returns null if the package should not be included in the graph
 * (no build script, or explicitly excluded).
 * @param relDir - Relative directory path from the repo root (e.g. `libs/sdk`).
 * @param pkg - The parsed package.json for the package.
 * @returns The NodeKind for this package, or null to exclude it from the graph.
 */
function npmKind(relDir: string, pkg: PackageJson): NodeKind | null {
	const buildScript = pkg.scripts?.["build"] ?? "";

	// Plugins and connectors are always "plugin" regardless of build script.
	if (relDir.startsWith("plugins/") || relDir.startsWith("connectors/")) {
		if (!buildScript) return null; // no build script → skip
		return "plugin";
	}

	// App directory is the Tauri app.
	if (relDir === "app") {
		return "tauri-app";
	}

	// Library directories: classify by build script.
	if (relDir.startsWith("libs/")) {
		if (!buildScript) return null; // no build script → skip
		if (buildScript.includes("svelte-package")) return "svelte-library";
		if (buildScript.includes("tsc")) return "ts-library";
		return null; // unrecognised build → skip
	}

	return null;
}

/**
 * Scan npm packages and return a partial node map (without dependents, which
 * require the full merged graph).
 *
 * Scans: libs/*, plugins/*, connectors/*, app/
 * Excludes: devtools/ and cli/ (host process and launcher).
 *
 * For each package.json:
 *   1. Reads `dependencies` and `peerDependencies` (NOT devDependencies).
 *   2. Filters to `@orqastudio/*` entries to build the dependency edge list.
 *   3. Strips the `@orqastudio/` scope to map to node IDs.
 *   4. Determines NodeKind from directory and build script.
 * @param root - Absolute path to the repo root.
 * @returns Partial node map keyed by node ID (dependents not yet populated).
 */
function readNpmGraph(root: string): Map<string, ProcessNode> {
	const nodes = new Map<string, ProcessNode>();

	// Directories to scan, relative to root.
	const scanDirs: Array<{ glob: string; recurse: boolean }> = [
		{ glob: "libs", recurse: true },
		{ glob: "plugins", recurse: true },
		{ glob: "connectors", recurse: true },
		{ glob: "app", recurse: false },
	];

	for (const { glob: dir, recurse } of scanDirs) {
		const dirPath = path.join(root, dir);
		if (!fs.existsSync(dirPath)) continue;

		if (!recurse) {
			// Single package (app/).
			const pkgPath = path.join(dirPath, "package.json");
			const pkg = readPackageJson(pkgPath);
			if (!pkg) continue;

			const kind = npmKind(dir, pkg);
			if (!kind) continue;

			const id = packageNameToId(pkg.name ?? dir);
			const watchDir = dirPath; // app watches its own root (Tauri manages internally)
			const node: ProcessNode = {
				id,
				name: pkg.name ?? id,
				kind,
				status: "pending",
				rootDir: dirPath,
				watchDir,
				watchPatterns: watchPatternsForKind(kind),
				dependsOn: computeNpmDeps(pkg),
				dependents: [],
				pid: null,
				lastBuiltAt: null,
				lastError: null,
			};
			nodes.set(id, node);
		} else {
			// Scan immediate subdirectories.
			let entries: fs.Dirent[];
			try {
				entries = fs.readdirSync(dirPath, { withFileTypes: true });
			} catch {
				continue;
			}

			for (const entry of entries) {
				if (!entry.isDirectory()) continue;
				const subDir = path.join(dirPath, entry.name);
				const pkgPath = path.join(subDir, "package.json");
				const pkg = readPackageJson(pkgPath);
				if (!pkg) continue;

				const relDir = `${dir}/${entry.name}`;
				const kind = npmKind(relDir, pkg);
				if (!kind) continue;

				const id = packageNameToId(pkg.name ?? entry.name);
				const watchDir = path.join(subDir, "src");
				const node: ProcessNode = {
					id,
					name: pkg.name ?? id,
					kind,
					status: "pending",
					rootDir: subDir,
					watchDir,
					watchPatterns: watchPatternsForKind(kind),
					dependsOn: computeNpmDeps(pkg),
					dependents: [],
					pid: null,
					lastBuiltAt: null,
					lastError: null,
				};
				nodes.set(id, node);
			}
		}
	}

	return nodes;
}

/**
 * Extract `@orqastudio/*` dependency IDs from a package.json.
 * Only reads `dependencies` and `peerDependencies` — devDependencies are
 * build-time tooling, not runtime graph edges.
 * @param pkg - The parsed package.json object to extract dependencies from.
 * @returns Array of node IDs derived from `@orqastudio/*` dependency names.
 */
function computeNpmDeps(pkg: PackageJson): string[] {
	const allDeps = {
		...(pkg.dependencies ?? {}),
		...(pkg.peerDependencies ?? {}),
	};
	return Object.keys(allDeps)
		.filter((k) => k.startsWith("@orqastudio/"))
		.map(packageNameToId);
}

/**
 * Build the Rust side of the process graph.
 *
 * Creates two nodes:
 *   - rust-workspace: the entire Cargo workspace built as a unit.
 *   - daemon:         long-running service that depends on rust-workspace.
 *
 * Engine crates are NOT individual nodes — they are built as part of the
 * workspace unit. Individual nodes only exist for top-level binaries.
 * @param root - Absolute path to the repo root.
 * @returns Node map for Rust workspace and daemon service nodes.
 */
function readCargoGraph(root: string): Map<string, ProcessNode> {
	const nodes = new Map<string, ProcessNode>();

	// rust-workspace — builds the entire Cargo workspace.
	const rustWorkspace: ProcessNode = {
		id: "rust-workspace",
		name: "rust-workspace",
		kind: "rust-workspace",
		status: "pending",
		rootDir: root,
		// Watchers are registered for engine/*/src and daemon/src separately.
		watchDir: root,
		watchPatterns: watchPatternsForKind("rust-workspace"),
		dependsOn: [],
		dependents: [],
		pid: null,
		lastBuiltAt: null,
		lastError: null,
	};
	nodes.set("rust-workspace", rustWorkspace);

	// daemon — the persistent background service.
	const daemonDir = path.join(root, "daemon");
	if (fs.existsSync(daemonDir)) {
		const daemon: ProcessNode = {
			id: "daemon",
			name: "daemon",
			kind: "service",
			status: "pending",
			rootDir: daemonDir,
			watchDir: path.join(daemonDir, "src"),
			watchPatterns: watchPatternsForKind("service"),
			dependsOn: ["rust-workspace"],
			dependents: [],
			pid: null,
			lastBuiltAt: null,
			lastError: null,
		};
		nodes.set("daemon", daemon);
	}

	return nodes;
}

/**
 * Compute the reverse edges (dependents) for every node in the merged graph.
 * Mutates the nodes in place — must be called after all nodes are merged.
 * @param nodes - The merged node map to populate dependents on.
 */
function computeDependents(nodes: Map<string, ProcessNode>): void {
	for (const [id, node] of nodes) {
		for (const depId of node.dependsOn) {
			const dep = nodes.get(depId);
			if (dep && !dep.dependents.includes(id)) {
				dep.dependents.push(id);
			}
		}
	}
}

/**
 * Compute parallel build tiers using modified Kahn's algorithm.
 *
 * Returns an array of tiers. Each tier contains node IDs that have no
 * unsatisfied dependencies and can build concurrently.
 *
 * Throws an error if a dependency cycle is detected.
 *
 * Expected tiers for the current codebase:
 *   Tier 0: [constants, types, logger, rust-workspace]
 *   Tier 1: [sdk, graph-visualiser, daemon, search]
 *   Tier 2: [svelte-components]
 *   Tier 3: [app, plugins...]
 * @param nodes - The merged node map with dependents already populated.
 * @returns Ordered array of tiers, each containing concurrently-buildable node IDs.
 */
function computeBuildTiers(nodes: Map<string, ProcessNode>): string[][] {
	const inDegree = new Map<string, number>();
	// adjacency list: dep → list of nodes that depend on it
	const adjList = new Map<string, string[]>();

	for (const [id, node] of nodes) {
		inDegree.set(id, node.dependsOn.length);
		for (const dep of node.dependsOn) {
			const list = adjList.get(dep) ?? [];
			list.push(id);
			adjList.set(dep, list);
		}
	}

	const tiers: string[][] = [];
	let ready = [...nodes.keys()].filter((id) => inDegree.get(id) === 0);

	while (ready.length > 0) {
		tiers.push([...ready]);
		const nextReady: string[] = [];

		for (const id of ready) {
			for (const dependent of adjList.get(id) ?? []) {
				const newDeg = (inDegree.get(dependent) ?? 1) - 1;
				inDegree.set(dependent, newDeg);
				if (newDeg === 0) nextReady.push(dependent);
			}
		}

		ready = nextReady;
	}

	// Cycle detection: any node still with inDegree > 0 is part of a cycle.
	const processed = tiers.flat().length;
	if (processed < nodes.size) {
		const remaining = [...nodes.keys()].filter((id) => !tiers.flat().includes(id));
		throw new Error(
			`Circular dependency detected among: ${remaining.join(", ")}. ` +
				"Fix the dependency cycle before running the dev environment.",
		);
	}

	return tiers;
}

/**
 * Build the complete process graph from the project root.
 *
 * Reads npm and Cargo graphs, merges them, computes dependents (reverse
 * edges), runs topological sort, and returns an immutable ProcessGraph.
 * @param root - Absolute path to the repo root.
 * @returns The fully resolved ProcessGraph with build tiers and reverse edges.
 */
export async function buildProcessGraph(root: string): Promise<ProcessGraph> {
	const npmNodes = readNpmGraph(root);
	const cargoNodes = readCargoGraph(root);

	// Merge both ecosystems into a single map.
	const nodes = new Map<string, ProcessNode>([...npmNodes, ...cargoNodes]);

	// Remove dependsOn references to nodes not present in the graph (e.g. cli,
	// devtools, or packages resolved from npm registry rather than workspace).
	for (const node of nodes.values()) {
		node.dependsOn = node.dependsOn.filter((id) => nodes.has(id));
	}

	// Compute reverse edges.
	computeDependents(nodes);

	// Topological sort → tiers + flat build order.
	const buildTiers = computeBuildTiers(nodes);
	const buildOrder = buildTiers.flat();

	return { nodes, buildOrder, buildTiers };
}

// ── Platform helpers ──────────────────────────────────────────────────────────

/**
 * Returns true when running on Windows.
 * @returns True if the current platform is win32.
 */
export function isWindows(): boolean {
	return platform() === "win32";
}

/**
 * Locate the Windows SDK bin directory containing RC.EXE for the host architecture.
 * Scans the standard Windows Kits install path for the newest SDK version.
 * @returns Absolute path to the SDK bin directory, or null if not found.
 */
function findWindowsSdkRcDir(): string | null {
	const kitsRoot = "C:\\Program Files (x86)\\Windows Kits\\10\\bin";
	if (!fs.existsSync(kitsRoot)) return null;

	// Find the newest SDK version directory that contains rc.exe for x64.
	const versions = fs
		.readdirSync(kitsRoot)
		.filter((d) => d.startsWith("10."))
		.sort()
		.reverse();

	for (const ver of versions) {
		const dir = path.join(kitsRoot, ver, "x64");
		if (fs.existsSync(path.join(dir, "rc.exe"))) return dir;
	}
	return null;
}

/**
 * Returns the platform-correct npm executable name.
 * @returns `npm.cmd` on Windows, `npm` on Unix.
 */
export function npm(): string {
	return isWindows() ? "npm.cmd" : "npm";
}

/**
 * Returns the platform-correct npx executable name.
 * @returns `npx.cmd` on Windows, `npx` on Unix.
 */
export function npx(): string {
	return isWindows() ? "npx.cmd" : "npx";
}

// ── Service helpers ───────────────────────────────────────────────────────────

/**
 * Execute a shell command synchronously, returning trimmed stdout.
 * Returns empty string on failure — callers treat empty as "not found".
 * Exported so dev.ts can use it inside killAll without duplication.
 * @param cmd - The shell command string to execute.
 * @returns Trimmed stdout of the command, or empty string on failure.
 */
export function exec(cmd: string): string {
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

/**
 * Build the Rust process environment: inherit current env, add RUST_LOG and
 * ORQA_PROJECT_ROOT so binaries pick up the correct project root.
 * @param root - Absolute path to the project root, written to ORQA_PROJECT_ROOT.
 * @returns A process environment record suitable for passing to spawn options.
 */
export function rustEnv(root: string): NodeJS.ProcessEnv {
	const env: NodeJS.ProcessEnv = {
		...process.env,
		RUST_LOG: process.env["RUST_LOG"] ?? "debug",
		ORQA_PROJECT_ROOT: root,
	};

	// On Windows, tauri-winres needs RC.EXE from the Windows SDK.
	// Detect the SDK bin directory and prepend it to PATH if not already present.
	if (isWindows()) {
		const sdkBin = findWindowsSdkRcDir();
		if (sdkBin && !(env.PATH ?? "").includes(sdkBin)) {
			env.PATH = `${sdkBin};${env.PATH ?? ""}`;
		}
	}

	return env;
}

/**
 * Find PIDs of all processes matching the given name.
 * Uses PowerShell on Windows, pgrep on Unix.
 * Exported for use by dev.ts killAll helper.
 * @param name - The process name to search for (without path or extension).
 * @returns Array of matching PIDs, or empty array if none found.
 */
export function findPidsByName(name: string): number[] {
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

/**
 * Find PIDs of all processes matching ANY of the given names in a single
 * PowerShell/pgrep call. Much faster than calling findPidsByName per name.
 * Returns a Map of name → PIDs.
 * @param names - Array of process names to search for simultaneously.
 * @returns Map from each process name to its matching PIDs (empty array if none).
 */
export function findPidsByNames(names: string[]): Map<string, number[]> {
	const result = new Map<string, number[]>();
	for (const name of names) result.set(name, []);

	if (names.length === 0) return result;

	if (isWindows()) {
		const namesArg = names.map((n) => `'${n}'`).join(",");
		const out = exec(
			`powershell.exe -NoProfile -Command "Get-Process -Name ${namesArg} -ErrorAction SilentlyContinue | ForEach-Object { $_.ProcessName + ':' + $_.Id }"`,
		);
		for (const line of out.split("\n")) {
			const sep = line.lastIndexOf(":");
			if (sep < 0) continue;
			const name = line.substring(0, sep).trim();
			const pid = parseInt(line.substring(sep + 1).trim(), 10);
			if (pid > 0) {
				const match = names.find((n) => n.toLowerCase() === name.toLowerCase());
				if (match) result.get(match)?.push(pid);
			}
		}
	} else {
		for (const name of names) {
			result.set(name, findPidsByName(name));
		}
	}

	return result;
}

/**
 * Kill a process and all of its children.
 * Uses `taskkill /T /F` on Windows (native tree kill, fast — no WMI).
 * Falls back to SIGKILL on Unix.
 * Exported for use by dev.ts killAll helper.
 * @param pid - The process ID to kill along with its entire child tree.
 */
export function killProcessTree(pid: number): void {
	if (pid === process.pid) return;

	if (isWindows()) {
		try {
			execSync(`taskkill /T /F /PID ${pid}`, {
				encoding: "utf-8",
				timeout: 5_000,
				windowsHide: true,
				stdio: "ignore",
			});
		} catch {
			/* already dead or access denied */
		}
		return;
	}

	try {
		process.kill(pid, "SIGKILL");
	} catch {
		/* already dead */
	}
}

/**
 * Resolve after ms milliseconds.
 * @param ms - Number of milliseconds to wait before resolving.
 * @returns A promise that resolves after the given delay.
 */
export function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Poll the daemon health endpoint every 250ms until it responds 200 or
 * the timeout elapses. Returns true if the daemon became healthy in time.
 * @param port - The port the daemon health endpoint is listening on.
 * @param timeoutMs - Maximum milliseconds to wait before giving up.
 * @returns True if the daemon responded healthy within the timeout, false otherwise.
 */
async function waitForDaemon(port: number, timeoutMs: number): Promise<boolean> {
	const deadline = Date.now() + timeoutMs;
	const url = `http://127.0.0.1:${port}/health`;

	while (Date.now() < deadline) {
		try {
			const res = await fetch(url);
			if (res.ok) return true;
		} catch {
			// Daemon not yet listening — keep polling.
		}
		await sleep(250);
	}

	return false;
}

// ── ProcessManager class ───────────────────────────────────────────────────────

/**
 * Coordinates the lifecycle of all dev-environment processes.
 *
 * Constructed via ProcessManager.create(root) which reads the dependency graph
 * from disk. Service lifecycle (Task 3) and file-watching (Task 4) fill in the
 * remaining placeholder methods.
 */
export class ProcessManager {
	/** The resolved dependency graph, read-only after construction. */
	readonly graph: ProcessGraph;
	/** Absolute path to the project root. */
	private root: string;
	/** Registered event listeners; subscriptions managed via onEvent(). */
	private eventListeners: Array<(event: NodeEvent) => void> = [];
	/** Long-running child processes keyed by node ID (services, app). */
	private managedProcesses: Map<string, ChildProcess> = new Map();
	/** Set to true when shutdown() is called to suppress crash-restart loops. */
	private shutdownRequested = false;
	/** Active fs.FSWatcher instances — closed in shutdown(). */
	private watchers: fs.FSWatcher[] = [];
	/** Per-node debounce timers keyed by node ID. */
	private debounceTimers: Map<string, ReturnType<typeof setTimeout>> = new Map();
	/** Node IDs currently executing a rebuild cascade. */
	private rebuilding: Set<string> = new Set();
	/** Queued file-change path per node, held while the node is rebuilding. */
	private rebuildQueue: Map<string, string> = new Map();
	/** Active build child processes — killed during shutdown to prevent orphans. */
	private buildProcesses: Set<ChildProcess> = new Set();

	private constructor(root: string, graph: ProcessGraph) {
		this.root = root;
		this.graph = graph;
	}

	/**
	 * Read the dependency graph from disk and return a configured ProcessManager.
	 * This is the only way to construct an instance.
	 * @param root - Absolute path to the project root.
	 * @returns A fully configured ProcessManager instance.
	 */
	static async create(root: string): Promise<ProcessManager> {
		const graph = await buildProcessGraph(root);
		return new ProcessManager(root, graph);
	}

	// ── Build execution ───────────────────────────────────────────────────────

	/**
	 * Build all nodes in dependency order, running nodes in the same tier
	 * concurrently. Skips nodes whose dependencies failed (poisoned set).
	 * Updates each node's status and emits NodeEvents throughout.
	 * @returns Build result listing succeeded and failed node IDs.
	 */
	async buildAll(): Promise<BuildResult> {
		const succeeded: string[] = [];
		const failed: Array<{ nodeId: string; error: string }> = [];
		// poisoned: nodes that must be skipped because a dependency failed.
		const poisoned = new Set<string>();

		for (const tier of this.graph.buildTiers) {
			// Partition the tier into nodes to build and nodes to skip.
			const toBuild = tier.filter((id) => !poisoned.has(id));
			const toSkip = tier.filter((id) => poisoned.has(id));

			// Mark skipped nodes as failed immediately so downstream tiers see them.
			for (const id of toSkip) {
				const node = this.graph.nodes.get(id);
				if (node) {
					node.status = "build-failed";
					node.lastError = "Skipped: dependency failed";
					failed.push({ nodeId: id, error: "Skipped: dependency failed" });
					this.propagatePoison(id, poisoned);
				}
			}

			// Build all ready nodes in this tier concurrently.
			const results = await Promise.allSettled(
				toBuild.map((id) => this.buildNode(this.graph.nodes.get(id)!)),
			);

			for (let i = 0; i < results.length; i++) {
				const result = results[i]!;
				const nodeId = toBuild[i]!;
				if (result.status === "fulfilled") {
					succeeded.push(nodeId);
				} else {
					const error: string =
						result.reason instanceof Error ? result.reason.message : String(result.reason);
					failed.push({ nodeId, error });
					// Mark all downstream nodes as poisoned so they are skipped.
					this.propagatePoison(nodeId, poisoned);
				}
			}
		}

		return { succeeded, failed };
	}

	/**
	 * Add nodeId and all of its transitive dependents to the poisoned set.
	 * Used after a build failure to prevent downstream nodes from attempting
	 * to build against a broken dependency.
	 * @param nodeId - The node ID of the failed build to start poisoning from.
	 * @param poisoned - The mutable set of poisoned node IDs to add to.
	 */
	private propagatePoison(nodeId: string, poisoned: Set<string>): void {
		const node = this.graph.nodes.get(nodeId);
		if (!node) return;
		for (const depId of node.dependents) {
			if (!poisoned.has(depId)) {
				poisoned.add(depId);
				this.propagatePoison(depId, poisoned);
			}
		}
	}

	/**
	 * Build a single node. Emits building → built on success, building →
	 * build-failed on failure. Updates node.status, node.lastBuiltAt, and
	 * node.lastError in place.
	 *
	 * Services and the tauri-app are skipped here — they are started separately
	 * (services via startServices, app via startApp). The rust-workspace covers
	 * the service binaries as part of its workspace build.
	 * @param node - The node to build.
	 */
	private async buildNode(node: ProcessNode): Promise<void> {
		// Services and tauri-app are not pre-built in isolation.
		if (node.kind === "service" || node.kind === "tauri-app") {
			node.status = "built";
			node.lastBuiltAt = Date.now();
			return;
		}

		// Skip rebuild when dist/ is already newer than src/. This avoids redundant
		// rebuilds at startup when nothing has changed since the last build.
		if (this.isLibraryUpToDate(node)) {
			node.status = "built";
			node.lastBuiltAt = Date.now();
			this.log(node.id, `${node.name} is up to date — skipping rebuild`);
			this.emitEvent({
				timestamp: Date.now(),
				nodeId: node.id,
				nodeName: node.name,
				status: "built",
				trigger: "startup",
				durationMs: 0,
				error: null,
				changedFile: null,
			});
			return;
		}

		const startedAt = Date.now();
		node.status = "building";
		node.lastError = null;

		this.emitEvent({
			timestamp: startedAt,
			nodeId: node.id,
			nodeName: node.name,
			status: "building",
			trigger: "startup",
			durationMs: null,
			error: null,
			changedFile: null,
		});

		this.log(node.id, `building ${node.name}`);

		try {
			await this.spawnAsync(...this.buildCommand(node), {
				cwd: node.rootDir,
				nodeId: node.id,
				nodeName: node.name,
			});

			const durationMs = Date.now() - startedAt;
			node.status = "built";
			node.lastBuiltAt = Date.now();

			const event: NodeEvent = {
				timestamp: Date.now(),
				nodeId: node.id,
				nodeName: node.name,
				status: "built",
				trigger: "startup",
				durationMs,
				error: null,
				changedFile: null,
			};
			this.emitEvent(event);
			this.logJson(event);
			this.log(node.id, `built ${node.name} in ${durationMs}ms`);
		} catch (err) {
			const durationMs = Date.now() - startedAt;
			const error = err instanceof Error ? err.message : String(err);
			node.status = "build-failed";
			node.lastError = error;

			const event: NodeEvent = {
				timestamp: Date.now(),
				nodeId: node.id,
				nodeName: node.name,
				status: "build-failed",
				trigger: "startup",
				durationMs,
				error,
				changedFile: null,
			};
			this.emitEvent(event);
			this.logJson(event);
			this.log(node.id, `build failed for ${node.name}: ${error}`);
			throw err;
		}
	}

	/**
	 * Return true when the node's dist/ output is newer than every file in src/.
	 *
	 * Applies only to ts-library and svelte-library nodes — those are the kinds
	 * that delete their output dir during rebuild and cause Vite resolution races
	 * when consumers are already reading from dist/. Other kinds (rust-workspace,
	 * plugin, tauri-app) always rebuild because cargo/plugins have their own
	 * incremental logic.
	 *
	 * Conservative: returns false on any I/O error, missing dist/, or empty src/.
	 * A false return just means "rebuild to be safe" — never produces a wrong
	 * skip.
	 * @param node - The node to check for up-to-date output.
	 * @returns True if the dist/ output is newer than all src/ inputs, false otherwise.
	 */
	private isLibraryUpToDate(node: ProcessNode): boolean {
		if (node.kind !== "ts-library" && node.kind !== "svelte-library") {
			return false;
		}
		const srcDir = path.join(node.rootDir, "src");
		const distDir = path.join(node.rootDir, "dist");
		if (!fs.existsSync(srcDir) || !fs.existsSync(distDir)) return false;
		try {
			const srcMtime = newestMtime(srcDir);
			const distMtime = newestMtime(distDir);
			return distMtime >= srcMtime && distMtime > 0;
		} catch {
			return false;
		}
	}

	/**
	 * Return the [executable, args] pair for the build command of a node.
	 * Does not handle "service" or "tauri-app" — those are skipped by buildNode.
	 * @param node - The node to derive the build command for.
	 * @returns A tuple of [executable, args] suitable for passing to spawn.
	 */
	private buildCommand(node: ProcessNode): [string, string[]] {
		switch (node.kind) {
			case "ts-library":
				return [npx(), ["tsc"]];
			case "svelte-library":
				// --preserve-output prevents rimraf of dist/ before writing. Without it,
				// Vite sees files disappear then reappear, triggering an HMR avalanche.
				// Incremental write means Vite sees clean file updates → single HMR cycle.
				return [npx(), ["svelte-package", "-i", "src", "-o", "dist", "--preserve-output"]];
			case "rust-workspace":
				return [
					"cargo",
					[
						"build",
						"--workspace",
						"--exclude",
						"orqa-studio",
						"--exclude",
						"orqa-devtools",
						"--color",
						"always",
					],
				];
			case "plugin":
				return [npm(), ["run", "build"]];
			case "tauri-app":
			case "service":
				// These are handled in buildNode before reaching here — should never arrive.
				throw new Error(`No build command for kind: ${node.kind}`);
			default:
				return assertNever(node.kind);
		}
	}

	/**
	 * Spawn a child process, pipe stdout/stderr through the event/log system,
	 * resolve on exit code 0, and reject with an error message on non-zero exit.
	 * @param cmd - The executable to spawn.
	 * @param args - Arguments to pass to the executable.
	 * @param opts - Spawn options controlling working directory and node context.
	 * @param opts.cwd - Working directory for the child process.
	 * @param opts.nodeId - Node ID used to prefix log output.
	 * @param opts.nodeName - Human-readable node name used in error messages.
	 * @returns A promise that resolves on exit code 0 or rejects on failure.
	 */
	private spawnAsync(
		cmd: string,
		args: string[],
		opts: { cwd: string; nodeId: string; nodeName: string },
	): Promise<void> {
		return new Promise((resolve, reject) => {
			const child = spawn(cmd, args, {
				cwd: opts.cwd,
				stdio: ["ignore", "pipe", "pipe"],
				windowsHide: true,
				shell: isWindows(),
			});
			this.buildProcesses.add(child);

			let stderr = "";

			child.stdout?.on("data", (chunk: Buffer) => {
				const text = chunk.toString();
				for (const line of text.trimEnd().split("\n")) {
					if (line.trim()) this.log(opts.nodeId, line);
				}
			});

			child.stderr?.on("data", (chunk: Buffer) => {
				const text = chunk.toString();
				stderr += text;
				for (const line of text.trimEnd().split("\n")) {
					if (line.trim()) this.log(opts.nodeId, line);
				}
			});

			child.on("error", (err) => {
				reject(new Error(`Failed to spawn ${cmd}: ${err.message}`));
			});

			child.on("close", (code) => {
				this.buildProcesses.delete(child);
				if (code === 0) {
					resolve();
				} else {
					// Use last line of stderr as the error summary; full output was
					// already streamed line-by-line above.
					const summary = stderr.trimEnd().split("\n").pop()?.trim() || `exited with code ${code}`;
					reject(new Error(summary));
				}
			});
		});
	}

	// ── Service lifecycle ─────────────────────────────────────────────────────

	/**
	 * Start the daemon and auxiliary dev services.
	 *
	 * Daemon is started via runDaemonCommand("start") which manages its own PID
	 * file. Emits starting → running events for each service node.
	 */
	async startServices(): Promise<void> {
		await this.startDaemon();
		this.startStorybook();
	}

	/**
	 * Start the Storybook dev server for svelte-components.
	 * Non-blocking — runs in the background. Managed process is tracked
	 * for cleanup in shutdown().
	 */
	private startStorybook(): void {
		const storybookDir = path.join(this.root, "libs", "svelte-components");
		if (!fs.existsSync(path.join(storybookDir, ".storybook"))) return;

		const storybookPort = String(getPort("storybook"));
		this.log("storybook", `Starting Storybook on port ${storybookPort}...`);

		const child = spawn(npx(), ["storybook", "dev", "-p", storybookPort, "--no-open"], {
			cwd: storybookDir,
			stdio: ["ignore", "pipe", "pipe"],
			windowsHide: true,
			shell: isWindows(),
		});

		child.stdout?.on("data", (chunk: Buffer) => {
			for (const line of chunk.toString().trimEnd().split("\n")) {
				if (line.trim()) this.log("storybook", line);
			}
		});
		child.stderr?.on("data", (chunk: Buffer) => {
			for (const line of chunk.toString().trimEnd().split("\n")) {
				if (line.trim()) this.log("storybook", line);
			}
		});

		this.managedProcesses.set("storybook", child);
	}

	/**
	 * Start the daemon via its command module and wait for the health endpoint.
	 * Emits starting → running (or build-failed on timeout).
	 */
	private async startDaemon(): Promise<void> {
		const node = this.graph.nodes.get("daemon");
		if (!node) return;

		node.status = "starting";
		this.emitServiceEvent(node, "starting", "startup");
		this.log("daemon", "Starting daemon...");

		try {
			const { runDaemonCommand } = await import("../commands/daemon.js");
			await runDaemonCommand(["start"]);
		} catch {
			this.log("daemon", "Daemon start command failed — proceeding to health check.");
		}

		const port = getPort("daemon");
		const ready = await waitForDaemon(port, 10_000);

		if (ready) {
			node.status = "running";
			this.emitServiceEvent(node, "running", "startup");
			this.log("daemon", `Daemon ready on port ${port}.`);
		} else {
			node.status = "build-failed";
			node.lastError = `Did not respond on port ${port} within 10s`;
			this.emitServiceEvent(node, "build-failed", "startup", node.lastError);
			this.log("daemon", node.lastError);
		}
	}

	/**
	 * Spawn a service process with crash-restart and exponential backoff.
	 * Tracks consecutive crash count; stops retrying after maxCrashes.
	 * Stores the ChildProcess in managedProcesses keyed by node ID.
	 * @param node - The process graph node representing the service to spawn.
	 * @param cmd - Absolute path or command name of the binary to execute.
	 * @param args - Command-line arguments forwarded to the child process.
	 * @param opts - Optional spawn overrides.
	 * @param opts.env - Environment variables merged into the child process environment.
	 */
	private async spawnService(
		node: ProcessNode,
		cmd: string,
		args: string[],
		opts: { env?: NodeJS.ProcessEnv } = {},
	): Promise<void> {
		// Backoff delays in ms — capped at 30s.
		const backoffDelays = [2_000, 4_000, 8_000, 16_000, 30_000];
		const maxCrashes = 5;
		let crashes = 0;

		const launch = (): void => {
			node.status = "starting";
			this.emitServiceEvent(node, "starting", crashes === 0 ? "startup" : "crash");
			this.log(node.id, `Starting ${node.name}...`);

			const child = spawn(cmd, args, {
				cwd: node.rootDir,
				env: opts.env ?? process.env,
				stdio: ["pipe", "pipe", "pipe"],
				windowsHide: true,
				shell: isWindows(),
			});

			this.managedProcesses.set(node.id, child);
			node.pid = child.pid ?? null;

			child.stdout?.on("data", (chunk: Buffer) => {
				for (const line of chunk.toString().trimEnd().split("\n")) {
					if (line.trim()) this.log(node.id, line);
				}
			});

			child.stderr?.on("data", (chunk: Buffer) => {
				for (const line of chunk.toString().trimEnd().split("\n")) {
					if (line.trim()) this.log(node.id, line);
				}
			});

			child.on("error", (err) => {
				this.log(node.id, `Failed to spawn: ${err.message}`);
			});

			// Mark running shortly after spawn — the binary starts immediately.
			setTimeout(() => {
				if (node.status === "starting" && !this.shutdownRequested) {
					node.status = "running";
					this.emitServiceEvent(node, "running", crashes === 0 ? "startup" : "crash");
					this.log(node.id, `${node.name} running (PID ${child.pid ?? "?"}).`);
				}
			}, 500);

			child.on("close", (code) => {
				node.pid = null;
				this.managedProcesses.delete(node.id);

				if (this.shutdownRequested) return;

				if (code === 0 || code === null) {
					// Clean exit — treat as stopped, do not restart.
					node.status = "stopped";
					this.emitServiceEvent(node, "stopped", "manual");
					return;
				}

				// Unexpected exit — crash.
				crashes++;
				node.status = "crashed";
				node.lastError = `Exited with code ${code}`;
				this.emitServiceEvent(node, "crashed", "crash", node.lastError);
				this.log(node.id, `${node.name} crashed (code ${code}). Crash #${crashes}.`);

				if (crashes >= maxCrashes) {
					this.log(node.id, `${node.name} crashed ${maxCrashes} times. Giving up.`);
					return;
				}

				const delay = backoffDelays[Math.min(crashes - 1, backoffDelays.length - 1)] ?? 30_000;
				this.log(node.id, `Restarting in ${delay}ms...`);
				setTimeout(launch, delay);
			});
		};

		launch();
	}

	/**
	 * Start the Tauri app via `cargo tauri dev`. Polls for the orqa-studio
	 * process to appear (up to 5 min compilation timeout).
	 *
	 * Clean exit (code 0) triggers shutdown. Non-zero exit emits "crashed" and
	 * leaves the manager running so the user can restart-tauri.
	 */
	async startApp(): Promise<void> {
		const node =
			this.graph.nodes.get("orqa-studio") ??
			// fall back to first tauri-app node in the graph
			[...this.graph.nodes.values()].find((n) => n.kind === "tauri-app");
		if (!node) return;

		const appDir = path.join(this.root, "app");
		node.status = "starting";
		this.emitServiceEvent(node, "starting", "startup");
		this.log("app", "Starting Tauri app (cargo tauri dev)...");

		const child = spawn("cargo", ["tauri", "dev"], {
			cwd: appDir,
			env: rustEnv(this.root),
			stdio: ["ignore", "pipe", "pipe"],
			windowsHide: true,
			shell: isWindows(),
		});

		this.managedProcesses.set(node.id, child);
		node.pid = child.pid ?? null;

		child.stdout?.on("data", (chunk: Buffer) => {
			for (const line of chunk.toString().trimEnd().split("\n")) {
				if (line.trim()) this.log("app", line);
			}
		});

		child.stderr?.on("data", (chunk: Buffer) => {
			for (const line of chunk.toString().trimEnd().split("\n")) {
				if (line.trim()) this.log("app", line);
			}
		});

		// Poll for orqa-studio process appearance (up to 5 min for compilation).
		const appDeadline = Date.now() + 300_000;
		let appReady = false;

		while (Date.now() < appDeadline) {
			await sleep(3_000);

			if (child.exitCode !== null) {
				this.log("app", "cargo tauri dev exited during startup.");
				break;
			}

			const procs = findPidsByName("orqa-studio");
			if (procs.length > 0) {
				await sleep(2_000); // Let the webview render.
				appReady = true;
				break;
			}
		}

		if (appReady) {
			node.status = "running";
			this.emitServiceEvent(node, "running", "startup");
			this.log("app", "Tauri app loaded.");
		} else if (child.exitCode === null) {
			this.log("app", "Tauri app may still be compiling — check the terminal.");
		}

		// Wire exit handler after the startup window.
		child.on("close", (code) => {
			node.pid = null;
			this.managedProcesses.delete(node.id);

			if (this.shutdownRequested) return;

			if (code === 0 || code === null) {
				// User closed the app window — shut everything down.
				this.log("app", "App window closed. Shutting down...");
				void this.shutdown();
			} else {
				// Crash — emit event and leave manager alive for restart-tauri.
				node.status = "crashed";
				node.lastError = `Exited with code ${code}`;
				this.emitServiceEvent(node, "crashed", "crash", node.lastError);
				this.log("app", `App crashed (code ${code}). Use restart-tauri to relaunch.`);
			}
		});
	}

	/**
	 * Gracefully shut down all services, the app, and clean up state files.
	 * File-watcher teardown is wired by Task 4 — the hook exists here already.
	 *
	 * Sequence:
	 *   1. Set shutdownRequested to suppress crash-restart loops.
	 *   2. Close all file watchers.
	 *   3. Kill all managed processes (app, storybook, etc.).
	 *   4. Stop the daemon via runDaemonCommand("stop").
	 *   5. Remove control and signal files.
	 */
	async shutdown(): Promise<void> {
		if (this.shutdownRequested) return;
		this.shutdownRequested = true;
		this.log("ctrl", "Shutting down...");

		// Kill in-flight build processes (cargo, node, tsc) to prevent orphans.
		for (const child of this.buildProcesses) {
			if (child.pid) {
				this.log("ctrl", `Killing build process (PID ${child.pid})...`);
				killProcessTree(child.pid);
			}
		}
		this.buildProcesses.clear();

		// Close all file watchers and clear debounce timers.
		for (const timer of this.debounceTimers.values()) clearTimeout(timer);
		this.debounceTimers.clear();
		for (const watcher of this.watchers) {
			try {
				watcher.close();
			} catch {
				/* already closed */
			}
		}
		this.watchers = [];

		// Kill all managed processes (app, storybook, etc.)
		for (const [id, child] of this.managedProcesses) {
			if (child.pid) {
				this.log("ctrl", `Killing ${id} (PID ${child.pid})...`);
				killProcessTree(child.pid);
			}
		}
		this.managedProcesses.clear();

		// Stop daemon gracefully (daemon is not in managedProcesses — it's started via runDaemonCommand).
		try {
			const { runDaemonCommand } = await import("../commands/daemon.js");
			await runDaemonCommand(["stop"]);
			this.log("ctrl", "Daemon stopped.");
		} catch {
			this.log("ctrl", "Daemon was not running.");
		}

		// Remove state files.
		const controlFile = path.join(this.root, ".state", "dev-controller.json");
		const signalFile = path.join(this.root, ".state", "dev-signal");
		for (const f of [controlFile, signalFile]) {
			try {
				fs.unlinkSync(f);
			} catch {
				/* already gone */
			}
		}

		this.log("ctrl", "Shutdown complete.");
	}

	/**
	 * Emit a NodeEvent for a service node status transition.
	 * Service events use a subset of triggers; error is optional.
	 * @param node - The process graph node whose status changed.
	 * @param status - The new status to broadcast.
	 * @param trigger - The cause of the status transition.
	 * @param error - Optional error message string when the transition is failure-driven.
	 */
	private emitServiceEvent(
		node: ProcessNode,
		status: NodeStatus,
		trigger: NodeEvent["trigger"],
		error: string | null = null,
	): void {
		const event: NodeEvent = {
			timestamp: Date.now(),
			nodeId: node.id,
			nodeName: node.name,
			status,
			trigger,
			durationMs: null,
			error,
			changedFile: null,
		};
		this.emitEvent(event);
		this.logJson(event);
	}

	// ── Watch coordinator ─────────────────────────────────────────────────────

	/**
	 * Activate file watchers for all nodes that support watch-mode rebuilds.
	 *
	 * Skipped:
	 *   - tauri-app: cargo tauri dev manages its own watch loop.
	 *   - service: restarted as part of the rust-workspace cascade.
	 *
	 * Special case for rust-workspace: registers individual watchers for each
	 * engine crate's src/ directory and daemon/src/ rather than the workspace
	 * root, so changes in deeply nested crate sources are detected reliably.
	 */
	watchAll(): void {
		for (const node of this.graph.nodes.values()) {
			if (node.kind === "tauri-app" || node.kind === "service") continue;

			if (node.kind === "rust-workspace") {
				this.watchRustWorkspace(node);
			} else {
				this.watchNode(node);
			}
		}
	}

	/**
	 * Register an fs.FSWatcher for a single node's watchDir.
	 * Filters events by watchPatterns (file extension match).
	 * Debounces per-node at 500ms.
	 * @param node - The process graph node whose watchDir to monitor.
	 */
	private watchNode(node: ProcessNode): void {
		if (!fs.existsSync(node.watchDir)) return;

		const watcher = fs.watch(node.watchDir, { recursive: true }, (_event, filename) => {
			if (!filename) return;
			if (!this.matchesPatterns(filename, node.watchPatterns)) return;
			const changedFile = path.join(node.watchDir, filename);
			this.scheduleRebuild(node.id, changedFile);
		});

		watcher.on("error", (err) => {
			this.log(node.id, `Watcher error: ${err.message}`);
		});

		this.watchers.push(watcher);
		this.log(node.id, `Watching ${node.watchDir}`);
	}

	/**
	 * Register individual source watchers for the rust-workspace node.
	 * Watches each engine crate's src/ and daemon/src/ individually so
	 * fs.watch recursive mode reliably captures nested .rs file changes.
	 * @param node - The rust-workspace process graph node to watch.
	 */
	private watchRustWorkspace(node: ProcessNode): void {
		const watchTargets: string[] = [];

		// engine/*/src/ — all engine crate source directories.
		const engineDir = path.join(this.root, "engine");
		if (fs.existsSync(engineDir)) {
			let entries: fs.Dirent[];
			try {
				entries = fs.readdirSync(engineDir, { withFileTypes: true });
			} catch {
				entries = [];
			}
			for (const entry of entries) {
				if (!entry.isDirectory()) continue;
				const srcDir = path.join(engineDir, entry.name, "src");
				if (fs.existsSync(srcDir)) watchTargets.push(srcDir);
			}
		}

		// daemon/src/ — daemon source (separate from workspace for clarity).
		const daemonSrc = path.join(this.root, "daemon", "src");
		if (fs.existsSync(daemonSrc)) watchTargets.push(daemonSrc);

		for (const dir of watchTargets) {
			const watcher = fs.watch(dir, { recursive: true }, (_event, filename) => {
				if (!filename) return;
				if (!this.matchesPatterns(filename, node.watchPatterns)) return;
				const changedFile = path.join(dir, filename);
				this.scheduleRebuild(node.id, changedFile);
			});

			watcher.on("error", (err) => {
				this.log(node.id, `Watcher error (${dir}): ${err.message}`);
			});

			this.watchers.push(watcher);
			this.log(node.id, `Watching ${dir}`);
		}
	}

	/**
	 * Returns true if filename matches any of the given glob patterns.
	 * Patterns are simple extension wildcards (e.g. "**\/*.ts") — we match
	 * by file extension only, which is sufficient for all current patterns.
	 * @param filename - The changed file name as reported by fs.watch.
	 * @param patterns - Glob-style patterns to test against the file extension.
	 * @returns True when at least one pattern matches the file's extension.
	 */
	private matchesPatterns(filename: string, patterns: string[]): boolean {
		const ext = path.extname(filename);
		if (!ext) return false;
		return patterns.some((p) => p.endsWith(`*${ext}`));
	}

	/**
	 * Schedule a rebuild for nodeId after the 500ms debounce window.
	 * Resets the timer on each call so rapid successive changes coalesce.
	 * @param nodeId - Identifier of the graph node to rebuild.
	 * @param changedFile - Path of the file that triggered the rebuild, used for logging.
	 */
	private scheduleRebuild(nodeId: string, changedFile: string): void {
		const existing = this.debounceTimers.get(nodeId);
		if (existing) clearTimeout(existing);

		const timer = setTimeout(() => {
			this.debounceTimers.delete(nodeId);
			void this.triggerCascade(nodeId, changedFile);
		}, 500);

		this.debounceTimers.set(nodeId, timer);
	}

	/**
	 * Rebuild the changed node then cascade to all transitive dependents in
	 * topological order. Service dependents are restarted after their
	 * prerequisite workspace build completes. Plugin dependents get an extra
	 * `orqa plugin refresh` call.
	 *
	 * If the node is already rebuilding, the incoming change is queued and
	 * processed after the current cascade finishes.
	 * @param nodeId - Identifier of the graph node that changed.
	 * @param changedFile - Path of the file that triggered the cascade, used for logging.
	 */
	private async triggerCascade(nodeId: string, changedFile: string): Promise<void> {
		if (this.rebuilding.has(nodeId)) {
			this.rebuildQueue.set(nodeId, changedFile);
			return;
		}

		const node = this.graph.nodes.get(nodeId);
		if (!node) return;

		this.rebuilding.add(nodeId);
		this.log(nodeId, `File changed: ${changedFile}`);

		// Update status for the initiating node.
		node.status = "rebuilding";
		this.emitServiceEvent(node, "rebuilding", "file-change");

		try {
			// Rebuild the changed node first.
			await this.buildNode(node);
		} catch {
			// Build failed — don't cascade to dependents.
			this.rebuilding.delete(nodeId);
			this.drainQueue(nodeId);
			return;
		}

		// After a successful plugin build, refresh the plugin registry.
		if (node.kind === "plugin") {
			await this.refreshPlugin(node);
		}

		// Cascade to transitive dependents in topological order.
		const downstream = this.collectDownstream(nodeId);
		for (const depId of downstream) {
			if (this.shutdownRequested) break;
			const depNode = this.graph.nodes.get(depId);
			if (!depNode) continue;

			if (depNode.kind === "tauri-app" || depNode.kind === "svelte-library") {
				// Tauri apps and svelte-library nodes pick up dependency changes
				// via Vite HMR — Vite watches their node_modules/dist files directly.
				// Rebuilding them triggers redundant HMR avalanches.
				this.log(depId, `${depNode.name} picks up changes via Vite HMR — skipping`);
				continue;
			} else if (depNode.kind === "service") {
				await this.restartService(depId);
			} else {
				depNode.status = "rebuilding";
				this.emitServiceEvent(depNode, "rebuilding", "dependency-rebuild");
				try {
					await this.buildNode(depNode);
					if (depNode.kind === "plugin") await this.refreshPlugin(depNode);
				} catch {
					// Dependent build failed — continue with remaining dependents
					// that don't transitively depend on this one (best-effort).
				}
			}
		}

		this.rebuilding.delete(nodeId);
		this.drainQueue(nodeId);
	}

	/**
	 * If a change was queued while nodeId was rebuilding, trigger another
	 * cascade immediately for that queued change.
	 * @param nodeId - Identifier of the graph node whose pending queue to drain.
	 */
	private drainQueue(nodeId: string): void {
		const queued = this.rebuildQueue.get(nodeId);
		if (queued) {
			this.rebuildQueue.delete(nodeId);
			void this.triggerCascade(nodeId, queued);
		}
	}

	/**
	 * BFS from nodeId's direct dependents, returning all transitive dependents
	 * in topological order (breadth-first = shallowest first).
	 * The starting node itself is not included.
	 * @param nodeId - Identifier of the root graph node to expand from.
	 * @returns Ordered array of downstream node IDs, shallowest dependencies first.
	 */
	private collectDownstream(nodeId: string): string[] {
		const visited = new Set<string>();
		const result: string[] = [];
		const queue: string[] = [...(this.graph.nodes.get(nodeId)?.dependents ?? [])];

		while (queue.length > 0) {
			const current = queue.shift()!;
			if (visited.has(current)) continue;
			visited.add(current);
			result.push(current);
			const node = this.graph.nodes.get(current);
			if (node) queue.push(...node.dependents);
		}

		return result;
	}

	/**
	 * Restart a service node after its workspace dependency was rebuilt.
	 * Daemon uses its CLI command module for graceful restart.
	 * Emits stopping → starting → running events.
	 * @param nodeId - Identifier of the service graph node to restart.
	 */
	private async restartService(nodeId: string): Promise<void> {
		const node = this.graph.nodes.get(nodeId);
		if (!node) return;

		node.status = "stopping";
		this.emitServiceEvent(node, "stopping", "dependency-rebuild");
		this.log(nodeId, `Restarting ${node.name}...`);

		if (nodeId === "daemon") {
			try {
				const { runDaemonCommand } = await import("../commands/daemon.js");
				await runDaemonCommand(["restart"]);
				node.status = "running";
				this.emitServiceEvent(node, "running", "dependency-rebuild");
				this.log(nodeId, "Daemon restarted.");
			} catch (err) {
				const error = err instanceof Error ? err.message : String(err);
				node.status = "crashed";
				node.lastError = error;
				this.emitServiceEvent(node, "crashed", "dependency-rebuild", error);
				this.log(nodeId, `Daemon restart failed: ${error}`);
			}
		}
	}

	/**
	 * Run `orqa plugin refresh --plugin <name>` after a successful plugin build
	 * so the installed plugin copy reflects the new build output.
	 * @param node - The plugin process graph node whose installed copy to refresh.
	 */
	private async refreshPlugin(node: ProcessNode): Promise<void> {
		const pluginName = node.name.replace(/^@orqastudio\//, "");
		this.log(node.id, `Refreshing plugin ${pluginName}...`);
		try {
			const { runPluginCommand } = await import("../commands/plugin.js");
			await runPluginCommand(["refresh", "--plugin", pluginName]);
			this.log(node.id, `Plugin ${pluginName} refreshed.`);
		} catch (err) {
			const error = err instanceof Error ? err.message : String(err);
			this.log(node.id, `Plugin refresh failed: ${error}`);
		}
	}

	// ── Event system ──────────────────────────────────────────────────────────

	/**
	 * Register a listener for node lifecycle events. Returns an unsubscribe
	 * function — call it to stop receiving events.
	 * @param listener - Callback invoked with each NodeEvent as it is emitted.
	 * @returns An unsubscribe function that removes the listener when called.
	 */
	onEvent(listener: (event: NodeEvent) => void): () => void {
		this.eventListeners.push(listener);
		return () => {
			this.eventListeners = this.eventListeners.filter((l) => l !== listener);
		};
	}

	/**
	 * Emit a NodeEvent to all registered listeners. Called internally by build,
	 * service, and watch methods as status transitions occur.
	 * @param event - The NodeEvent to broadcast to all registered listeners.
	 */
	protected emitEvent(event: NodeEvent): void {
		for (const listener of this.eventListeners) listener(event);
		this.writeProcessStatus();
	}

	/**
	 * Write current status of all nodes to `.state/process-status.json`.
	 * The devtools reads this file to sync node statuses when PM events
	 * don't flow through the dev_controller (new cmdDev flow).
	 */
	private writeProcessStatus(): void {
		const statuses: Record<string, string> = {};
		for (const [id, node] of this.graph.nodes) {
			statuses[id] = node.status;
		}
		try {
			const stateDir = path.join(this.root, ".state");
			if (!fs.existsSync(stateDir)) fs.mkdirSync(stateDir, { recursive: true });
			fs.writeFileSync(path.join(stateDir, "process-status.json"), JSON.stringify(statuses));
		} catch {
			// Best effort.
		}
	}

	// ── Logging ───────────────────────────────────────────────────────────────

	/**
	 * Write a human-readable prefixed log line to stdout.
	 * Format: HH:MM:SS [nodeId] message
	 * @param prefix - Short label printed in square brackets, typically a node ID.
	 * @param msg - The message text; multi-line strings are split and each line prefixed.
	 */
	private log(prefix: string, msg: string): void {
		const ts = new Date().toLocaleTimeString("en-GB", { hour12: false });
		for (const line of msg.trimEnd().split("\n")) {
			if (line.trim()) process.stdout.write(`${ts} [${prefix}] ${line}\n`);
		}
	}

	/**
	 * Write a JSON-serialised NodeEvent to stdout as a single line.
	 * The OrqaDev UI reads these lines to update process status panels.
	 * @param event - The NodeEvent to serialise and write to stdout.
	 */
	private logJson(event: NodeEvent): void {
		process.stdout.write(JSON.stringify(event) + "\n");
	}

	/**
	 * Emit the full dependency graph topology as a JSON event.
	 * Called after buildAll() so the devtools can render the process graph view
	 * with dependency edges. This is a one-shot event, not a NodeEvent.
	 */
	emitGraphTopology(): void {
		const nodes = [...this.graph.nodes.values()].map((n) => ({
			id: n.id,
			name: n.name,
			kind: n.kind,
			status: n.status,
			dependsOn: n.dependsOn,
			dependents: n.dependents,
		}));
		const payload = { type: "graph-topology", nodes };
		process.stdout.write(JSON.stringify(payload) + "\n");

		// Write to .state/ so devtools can read it on startup even when
		// the topology event was emitted before the devtools window opened.
		const stateDir = path.join(this.root, ".state");
		try {
			if (!fs.existsSync(stateDir)) fs.mkdirSync(stateDir, { recursive: true });
			fs.writeFileSync(
				path.join(stateDir, "graph-topology.json"),
				JSON.stringify(payload, null, 2),
			);
		} catch {
			// Best effort — devtools will fall back to flat card grid.
		}
	}

	// ── Status query ──────────────────────────────────────────────────────────

	/**
	 * Return a snapshot of all node statuses at the current moment.
	 * @returns A map of node ID to its current NodeStatus.
	 */
	getStatus(): Map<string, NodeStatus> {
		const result = new Map<string, NodeStatus>();
		for (const [id, node] of this.graph.nodes) result.set(id, node.status);
		return result;
	}
}
