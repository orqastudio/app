/**
 * Dev environment — thin CLI dispatch layer.
 *
 * `orqa dev` is the primary entry point for the development environment.
 * Process management (build, watch, restart, service lifecycle) is handled by
 * ProcessManager in lib/process-manager.ts. This file is a dispatch layer only.
 *
 * orqa dev                        Launch OrqaDev (cargo tauri dev for devtools)
 * orqa dev start-processes        Build + start all dev processes (called by OrqaDev)
 * orqa dev stop                   Stop gracefully via signal file
 * orqa dev kill                   Force-kill all processes
 * orqa dev restart [target]       Send restart signal to running controller
 * orqa dev status                 Show process status
 * orqa dev graph                  Print the dependency graph build tiers
 * orqa dev icons [--deploy]       Generate brand icons from SVG sources
 * orqa dev tool [args...]         Run the debug-tool submodule
 */
import { spawn, execSync } from "node:child_process";
import { createServer as createNetServer } from "node:net";
import * as path from "node:path";
import * as fs from "node:fs";
import { platform } from "node:os";
import { getRoot } from "../lib/root.js";
import { getPort } from "../lib/ports.js";
import { ProcessManager, exec, findPidsByName, findPidsByNames, killProcessTree, isWindows, npm, npx, rustEnv, sleep, } from "../lib/process-manager.js";
const VITE_PORT = getPort("vite");
const PORT_TIMEOUT_MS = 15_000;
const POLL_INTERVAL_MS = 500;
const COLOURS = {
    reset: "\x1b[0m",
    dim: "\x1b[2m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
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
  restart app         Restart Tauri app (rebuild + relaunch)
  restart search      Restart search server
  status              Show process status
  graph               Print dependency graph build tiers
  icons [--deploy]    Generate brand icons from SVG sources
  tool [args...]      Run the debug-tool submodule

Flags:
  --legacy-dashboard  Use the legacy dev.mjs web dashboard instead of OrqaDev (deprecated)
`.trim();
// ── Logging ───────────────────────────────────────────────────────────────────
function prefixLines(prefix, colour, data) {
    const text = data.toString().trimEnd();
    if (!text)
        return;
    for (const line of text.split("\n")) {
        const ts = new Date().toLocaleTimeString("en-GB", { hour12: false });
        console.log(`${COLOURS.dim}${ts}${COLOURS.reset} ${colour}[${prefix}]${COLOURS.reset} ${line}`);
    }
}
function logCtrl(msg) {
    prefixLines("ctrl", COLOURS.yellow, msg);
}
function logError(prefix, msg) {
    prefixLines(prefix, COLOURS.red, msg);
}
function logSuccess(msg) {
    prefixLines("ctrl", COLOURS.green, msg);
}
// ── Port / process utilities (local to dev.ts) ────────────────────────────────
/**
 * Find PIDs listening on a specific port.
 * @param port - The TCP port number to scan.
 * @returns Array of PIDs with an active LISTENING socket on the given port.
 */
function findPidsOnPort(port) {
    if (isWindows()) {
        const out = exec("netstat -ano");
        const pids = new Set();
        for (const line of out.split("\n")) {
            if (line.includes(`:${port}`) && line.includes("LISTENING")) {
                const parts = line.trim().split(/\s+/);
                const pid = parseInt(parts[parts.length - 1] ?? "", 10);
                if (pid > 0)
                    pids.add(pid);
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
    return exec(`ss -tlnp sport = :${port} 2>/dev/null | awk 'NR>1{match($0,/pid=([0-9]+)/,a); print a[1]}'`)
        .split("\n")
        .map((s) => parseInt(s, 10))
        .filter((n) => n > 0);
}
function isPortFree(port) {
    return new Promise((resolve) => {
        const server = createNetServer();
        server.once("error", () => resolve(false));
        server.once("listening", () => server.close(() => resolve(true)));
        server.listen(port, "127.0.0.1");
    });
}
async function waitForPort(port, timeoutMs, wantFree) {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
        const free = await isPortFree(port);
        if (wantFree === free)
            return true;
        await sleep(POLL_INTERVAL_MS);
    }
    return false;
}
function processIsAlive(pid) {
    try {
        process.kill(pid, 0);
        return true;
    }
    catch {
        return false;
    }
}
function getControlFilePath(root) {
    return path.join(root, ".state", "dev-controller.json");
}
function getSignalFilePath(root) {
    return path.join(root, ".state", "dev-signal");
}
function ensureStateDir(root) {
    const stateDir = path.join(root, ".state");
    if (!fs.existsSync(stateDir)) {
        fs.mkdirSync(stateDir, { recursive: true });
    }
}
function readControlFile(root) {
    try {
        return JSON.parse(fs.readFileSync(getControlFilePath(root), "utf-8"));
    }
    catch {
        return null;
    }
}
function removeControlFile(root) {
    try {
        fs.unlinkSync(getControlFilePath(root));
    }
    catch {
        /* ignore */
    }
}
function cleanupSignalFile(root) {
    try {
        fs.unlinkSync(getSignalFilePath(root));
    }
    catch {
        /* ignore */
    }
}
// ── Kill all (orphan cleanup) ─────────────────────────────────────────────────
/**
 * Find and kill all OrqaStudio processes by name and port.
 * Used before a fresh start and by `orqa dev kill`.
 * @param root - Absolute path to the dev repo root.
 * @param opts - Optional kill modifiers.
 * @param opts.preserveDevtools - When true, skip killing the Tauri devtools process.
 */
async function killAll(root, opts = {}) {
    logCtrl("Stopping OrqaStudio processes...");
    try {
        const { runDaemonCommand } = await import("./daemon.js");
        await runDaemonCommand(["stop"]);
        logCtrl("Daemon stopped.");
    }
    catch {
        /* not running */
    }
    const pidsToKill = new Set();
    // Batch discovery — single PowerShell call on Windows instead of one per name.
    const processNames = [
        "orqa-studio",
        "orqa-mcp-server",
        "orqa-lsp-server",
        "orqa-validation",
    ];
    if (!opts.preserveDevtools) {
        processNames.push("orqa-devtools", "cargo-tauri");
    }
    const pidsByName = findPidsByNames(processNames);
    for (const [name, pids] of pidsByName) {
        for (const pid of pids) {
            logCtrl(`Found ${name} (PID ${pid})`);
            pidsToKill.add(pid);
        }
    }
    // Base ports that killAll always reclaims (daemon + its subservices + app vite).
    const basePorts = [
        getPort("daemon"),
        getPort("lsp"),
        getPort("mcp"),
        VITE_PORT,
        getPort("dashboard"),
        getPort("sync"),
    ];
    // Devtools-owned ports — only reclaim when we're NOT preserving devtools.
    const devtoolsPorts = [
        getPort("devtools"),
        getPort("storybook"),
        5173, // legacy default Vite port
    ];
    const portsToCheck = opts.preserveDevtools ? basePorts : [...basePorts, ...devtoolsPorts];
    for (const port of portsToCheck) {
        for (const pid of findPidsOnPort(port)) {
            logCtrl(`Found process on port ${port} (PID ${pid})`);
            pidsToKill.add(pid);
        }
    }
    if (pidsToKill.size === 0) {
        logCtrl("No OrqaStudio processes found.");
    }
    else {
        for (const pid of pidsToKill) {
            logCtrl(`Killing PID ${pid}...`);
            killProcessTree(pid);
        }
    }
    logCtrl("Waiting for ports to release...");
    // Wait on every port we just reclaimed — a zombie on any of them will
    // block the next `orqa dev` start. Previously only VITE_PORT was checked,
    // which let the devtools Vite (port 10140) survive.
    for (const port of portsToCheck) {
        const freed = await waitForPort(port, PORT_TIMEOUT_MS, true);
        if (freed)
            continue;
        for (const pid of findPidsOnPort(port)) {
            logCtrl(`Force killing PID ${pid} on port ${port}...`);
            killProcessTree(pid);
        }
        const retried = await waitForPort(port, 5_000, true);
        if (!retried) {
            logError("ctrl", `FAILED: Port ${port} still in use`);
            process.exit(1);
        }
    }
    // Remove the stale daemon PID file unconditionally. Windows reuses PIDs,
    // so leaving a dead daemon's PID in .state/daemon.pid causes the next
    // daemon start to see the reused PID as "a live daemon" and exit with
    // "a daemon instance is already running". Deleting the file here closes
    // that race regardless of how the daemon was terminated.
    const daemonPidFile = path.join(root, ".state", "daemon.pid");
    if (fs.existsSync(daemonPidFile)) {
        try {
            fs.unlinkSync(daemonPidFile);
            logCtrl("Removed stale daemon PID file.");
        }
        catch (e) {
            logCtrl(`Could not remove ${daemonPidFile}: ${e.message}`);
        }
    }
    removeControlFile(root);
    logSuccess("All processes stopped.");
}
// ── Subcommand: dev (launch OrqaDev) ─────────────────────────────────────────
/**
 * Launch OrqaDev (the devtools Tauri app) via `cargo tauri dev`.
 * This is the default subcommand — the user runs `orqa dev` from their shell.
 * @param root - Absolute path to the dev repo root.
 */
async function cmdDev(root) {
    const devtoolsDir = path.join(root, "devtools");
    if (!fs.existsSync(path.join(devtoolsDir, "src-tauri", "tauri.conf.json"))) {
        logError("ctrl", "devtools directory not found. Are you in the dev repo root?");
        process.exit(1);
    }
    const staleDevtools = findPidsByName("orqa-devtools");
    if (staleDevtools.length > 0) {
        logCtrl("Stopping previous OrqaDev instance...");
        for (const pid of staleDevtools)
            killProcessTree(pid);
        await sleep(1_000);
    }
    // Stop all running OrqaStudio processes so cargo can overwrite binaries.
    // Wait for Windows to release file handles on the killed processes.
    await killAll(root, { preserveDevtools: true });
    await sleep(2_000);
    // ── Step 1: Build all packages via the process manager ──────────────
    logCtrl("Building all packages...");
    const pm = await ProcessManager.create(root);
    const result = await pm.buildAll();
    if (result.failed.length > 0) {
        for (const f of result.failed)
            logError("pm", `Build failed: ${f.nodeId}: ${f.error}`);
        logCtrl("Some builds failed. Starting what we can...");
    }
    // Emit the dependency graph so devtools can render the process graph view.
    pm.emitGraphTopology();
    // ── Step 2: Build app backend explicitly ────────────────────────────
    // The rust-workspace build excludes orqa-studio. Build it here so the
    // binary reflects the latest code before cargo tauri dev starts in
    // watch mode.
    logCtrl("Building app backend...");
    try {
        execSync("cargo build -p orqa-studio --color always", {
            cwd: path.join(root, "app"),
            stdio: "inherit",
            env: rustEnv(root),
        });
    }
    catch {
        logError("ctrl", "App build failed — launching with existing binary");
    }
    // ── Step 3: Start services (daemon, search) and the main app ────────
    logCtrl("Starting services and app...");
    await pm.startServices();
    await pm.startApp();
    pm.watchAll();
    // ── Step 3: Build and launch devtools ────────────────────────────────
    // The rust-workspace build excludes orqa-devtools, so we build it
    // explicitly before launching cargo tauri dev. This ensures the binary
    // picks up any Rust changes committed since the last build.
    logCtrl("Building devtools backend...");
    try {
        execSync("cargo build -p orqa-devtools --color always", {
            cwd: devtoolsDir,
            stdio: "inherit",
            env: rustEnv(root),
        });
    }
    catch {
        logError("ctrl", "Devtools build failed — launching with existing binary");
    }
    logCtrl("Launching OrqaDev...");
    const devtoolsChild = spawn("cargo", ["tauri", "dev"], {
        cwd: devtoolsDir,
        stdio: "inherit",
        env: { ...rustEnv(root), ORQA_DEV_MODE: "1" },
    });
    // ── Shutdown: Ctrl+C stops everything ──────────────────────────────
    let shuttingDown = false;
    async function shutdown() {
        if (shuttingDown)
            return;
        shuttingDown = true;
        logCtrl("Shutting down...");
        devtoolsChild.kill();
        await pm.shutdown();
        process.exit(0);
    }
    process.on("SIGINT", () => void shutdown());
    process.on("SIGTERM", () => void shutdown());
    if (isWindows())
        process.on("SIGHUP", () => void shutdown());
    // If the devtools window closes, shut everything down.
    devtoolsChild.on("close", () => {
        if (!shuttingDown)
            void shutdown();
    });
    logCtrl("Dev environment running. Close the devtools window or press Ctrl+C to stop.");
}
// ── Subcommand: start-processes (called by OrqaDev) ───────────────────────────
/**
 * Build all packages and start the dev environment in the foreground.
 * Called by OrqaDev via `orqa dev start-processes`. All process orchestration
 * is delegated to ProcessManager — this function is a thin setup wrapper.
 * @param root - Absolute path to the dev repo root.
 */
async function cmdStartProcesses(root) {
    logCtrl("Rebuilding CLI...");
    try {
        execSync(`${npx()} tsc --project ${path.join(root, "cli/tsconfig.json")}`, {
            cwd: root,
            stdio: "inherit",
        });
    }
    catch {
        logCtrl("CLI build failed — running with current dist/");
    }
    logCtrl("Installing dependencies...");
    try {
        execSync(`${npm()} install`, { cwd: root, stdio: "inherit" });
    }
    catch {
        logCtrl("npm install failed — dependencies may be stale");
    }
    // Stop all running OrqaStudio processes so cargo can overwrite binaries.
    // Preserve devtools — this command is called FROM devtools.
    await killAll(root, { preserveDevtools: true });
    // Build, start, and watch via ProcessManager.
    const pm = await ProcessManager.create(root);
    const result = await pm.buildAll();
    if (result.failed.length > 0) {
        for (const f of result.failed)
            logError("pm", `Build failed: ${f.nodeId}: ${f.error}`);
        logCtrl("Some builds failed. Starting what we can...");
    }
    pm.emitGraphTopology();
    await pm.startServices();
    await pm.startApp();
    pm.watchAll();
    process.on("SIGINT", () => {
        void pm.shutdown().then(() => process.exit(0));
    });
    process.on("SIGTERM", () => {
        void pm.shutdown().then(() => process.exit(0));
    });
    if (isWindows())
        process.on("SIGHUP", () => {
            void pm.shutdown().then(() => process.exit(0));
        });
    logCtrl("Dev environment running. Press Ctrl+C to stop.");
}
// ── Subcommand: stop ──────────────────────────────────────────────────────────
async function cmdStop(root) {
    const ctrl = readControlFile(root);
    if (ctrl && processIsAlive(ctrl.pid)) {
        logCtrl("Signalling controller to stop...");
        ensureStateDir(root);
        fs.writeFileSync(getSignalFilePath(root), "stop");
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
    if (ctrl)
        removeControlFile(root);
    logCtrl("No controller running. Use 'orqa dev kill' to force-kill orphaned processes.");
}
// ── Subcommand: kill ──────────────────────────────────────────────────────────
async function cmdKill(root) {
    await killAll(root);
    removeControlFile(root);
    cleanupSignalFile(root);
}
// ── Subcommand: restart (send signal to running controller) ───────────────────
async function cmdSignal(root, signal, label) {
    const ctrl = readControlFile(root);
    if (ctrl && processIsAlive(ctrl.pid)) {
        logCtrl(`Signalling controller to ${label}...`);
        ensureStateDir(root);
        fs.writeFileSync(getSignalFilePath(root), signal);
        logSuccess(`${label} signal sent. Watch the controller output for progress.`);
        return;
    }
    if (ctrl)
        removeControlFile(root);
    if (signal === "restart-tauri" || signal === "restart") {
        logCtrl("No controller running. Starting dev environment...");
        await cmdDev(root);
        return;
    }
    logError("ctrl", "No controller running. Use 'orqa dev' first.");
}
// ── Subcommand: status ────────────────────────────────────────────────────────
function cmdStatus(root) {
    const ctrl = readControlFile(root);
    if (!ctrl) {
        console.log("No dev controller running.");
        return;
    }
    const alive = processIsAlive(ctrl.pid);
    console.log(`Controller PID: ${ctrl.pid} (${alive ? "alive" : "dead"})`);
    console.log(`State: ${ctrl.state}`);
    if (ctrl.app)
        console.log(`App PID: ${ctrl.app}`);
    if (ctrl.search)
        console.log(`Search PID: ${ctrl.search}`);
    console.log("MCP/LSP: managed by daemon (use 'orqa daemon status' for details)");
}
// ── Subcommand: graph ─────────────────────────────────────────────────────────
/**
 * Print the dependency graph build tiers computed by ProcessManager.
 * Useful for diagnosing dependency ordering and verifying graph reader output.
 * @param root - Absolute path to the dev repo root.
 */
async function cmdGraph(root) {
    const pm = await ProcessManager.create(root);
    for (let i = 0; i < pm.graph.buildTiers.length; i++) {
        const tier = pm.graph.buildTiers[i];
        for (const id of tier) {
            const node = pm.graph.nodes.get(id);
            const deps = node.dependsOn.length > 0 ? ` → [${node.dependsOn.join(", ")}]` : "";
            console.log(`  Tier ${i}: ${node.name} (${node.kind})${deps}`);
        }
    }
}
// ── Subcommand: icons ─────────────────────────────────────────────────────────
function cmdIcons(root, args) {
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
    }
    catch {
        process.exit(1);
    }
}
// ── Subcommand: tool ──────────────────────────────────────────────────────────
function cmdDebugTool(root, args) {
    const debugToolPaths = [
        path.join(root, "debug-tool", "debug-tool.sh"),
        path.join(root, "node_modules", ".bin", "orqa-debug"),
    ];
    let debugToolPath = null;
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
    try {
        execSync(`"${debugToolPath}" ${args.join(" ")}`, { encoding: "utf-8", stdio: "inherit" });
    }
    catch {
        process.exit(1);
    }
}
// ── Main entry point ──────────────────────────────────────────────────────────
/**
 * Dispatch the dev command to the appropriate subcommand handler.
 * @param args - Positional arguments passed after `orqa dev`, used to select the subcommand.
 */
export async function runDevCommand(args) {
    if (args[0] === "--help" || args[0] === "-h") {
        console.log(USAGE);
        return;
    }
    const root = getRoot();
    const legacyDashboard = args.includes("--legacy-dashboard");
    void legacyDashboard; // retained for flag-strip only; OrqaDev no longer uses it
    const filteredArgs = args.filter((a) => a !== "--legacy-dashboard");
    const sub = filteredArgs[0] ?? "dev";
    switch (sub) {
        case "dev":
            await cmdDev(root);
            break;
        case "start-processes":
            await cmdStartProcesses(root);
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
            }
            else {
                const signalMap = {
                    daemon: ["restart-daemon", "Restart daemon (restarts MCP and LSP)"],
                    app: ["restart-app", "Restart app (Vite + Tauri)"],
                    search: ["restart-search", "Restart search server"],
                    mcp: ["restart-mcp", "Restart MCP (via daemon restart)"],
                    lsp: ["restart-lsp", "Restart LSP (via daemon restart)"],
                };
                const entry = signalMap[target];
                if (entry) {
                    await cmdSignal(root, entry[0], entry[1]);
                }
                else {
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
        case "graph":
            await cmdGraph(root);
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
//# sourceMappingURL=dev.js.map