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
import { spawn, execSync } from "node:child_process";
import { createServer as createNetServer } from "node:net";
import * as path from "node:path";
import * as fs from "node:fs";
import { platform } from "node:os";
import { getRoot } from "../lib/root.js";
import { getPort } from "../lib/ports.js";
function isWindows() { return platform() === "win32"; }
function npm() { return isWindows() ? "npm.cmd" : "npm"; }
function npx() { return isWindows() ? "npx.cmd" : "npx"; }
function rustEnv() {
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
// ── Process Utilities ───────────────────────────────────────────────────────
function exec(cmd) {
    try {
        return execSync(cmd, {
            encoding: "utf-8",
            timeout: 10_000,
            windowsHide: true,
        }).trim();
    }
    catch {
        return "";
    }
}
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
function findPidsByName(name) {
    if (isWindows()) {
        return exec(`powershell.exe -NoProfile -Command "Get-Process -Name '${name}' -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Id"`)
            .split("\n")
            .map((s) => parseInt(s.trim(), 10))
            .filter((n) => n > 0);
    }
    return exec(`pgrep -f "${name}"`)
        .split("\n")
        .map((s) => parseInt(s, 10))
        .filter((n) => n > 0);
}
function killProcessTree(pid) {
    if (pid === process.pid)
        return;
    if (isWindows()) {
        const childPids = exec(`powershell.exe -NoProfile -Command "function Get-Tree($id){Get-CimInstance Win32_Process|Where-Object{$_.ParentProcessId -eq $id}|ForEach-Object{Get-Tree $_.ProcessId;$_.ProcessId}};Get-Tree ${pid}"`)
            .split("\n")
            .map((s) => parseInt(s.trim(), 10))
            .filter((n) => n > 0);
        for (const childPid of childPids) {
            try {
                process.kill(childPid, "SIGKILL");
            }
            catch {
                /* already dead */
            }
        }
    }
    try {
        process.kill(pid, "SIGKILL");
    }
    catch {
        /* already dead */
    }
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
function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
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
function writeControlFile(root, state) {
    ensureStateDir(root);
    fs.writeFileSync(getControlFilePath(root), JSON.stringify({ pid: process.pid, ...state }, null, 2));
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
function processIsAlive(pid) {
    try {
        process.kill(pid, 0);
        return true;
    }
    catch {
        return false;
    }
}
function createManagedProcess(name, colour) {
    return { name, colour, child: null, running: false };
}
function spawnManaged(mp, cmd, args, opts = {}) {
    const stdinMode = opts.stdinMode ?? "ignore";
    mp.child = spawn(cmd, args, {
        cwd: opts.cwd,
        env: opts.env ?? { ...process.env },
        stdio: [stdinMode, "pipe", "pipe"],
        shell: isWindows(),
        windowsHide: true,
    });
    mp.running = true;
    mp.child.stdout?.on("data", (data) => prefixLines(mp.name, mp.colour, data.toString()));
    mp.child.stderr?.on("data", (data) => prefixLines(mp.name, mp.colour, data.toString()));
    mp.child.on("close", (code) => {
        mp.running = false;
        if (code !== 0 && code !== null) {
            logError(mp.name, `Exited with code ${code}`);
        }
    });
    return mp;
}
function killManaged(mp) {
    if (mp.child && mp.running && mp.child.pid) {
        prefixLines(mp.name, mp.colour, "Stopping...");
        killProcessTree(mp.child.pid);
        mp.running = false;
    }
}
// ── Kill All (orphan cleanup) ───────────────────────────────────────────────
async function killAll(root) {
    logCtrl("Stopping OrqaStudio processes...");
    // Stop daemon gracefully first (it manages its own PID file)
    try {
        const { runDaemonCommand } = await import("./daemon.js");
        await runDaemonCommand(["stop"]);
        logCtrl("Daemon stopped.");
    }
    catch { /* not running */ }
    const pidsToKill = new Set();
    // Find all OrqaStudio processes by name
    for (const name of [
        "orqa-studio", "cargo-tauri",
        "orqa-mcp-server", "orqa-lsp-server", "orqa-search-server", "orqa-validation",
    ]) {
        for (const pid of findPidsByName(name)) {
            logCtrl(`Found ${name} (PID ${pid})`);
            pidsToKill.add(pid);
        }
    }
    // Find by port (Vite, dashboard, daemon)
    for (const port of [VITE_PORT, 5173, getPort("dashboard"), getPort("daemon")]) {
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
async function startController(root, opts = { watch: true }) {
    const existing = readControlFile(root);
    if (existing && processIsAlive(existing.pid)) {
        logError("ctrl", `Controller already running (PID ${existing.pid}). Use 'orqa dev stop' first.`);
        process.exit(1);
    }
    if (existing)
        removeControlFile(root);
    // Kill any orphaned processes from previous runs
    await killAll(root);
    console.log("");
    console.log(`${COLOURS.yellow}╔══════════════════════════════════════════════╗${COLOURS.reset}`);
    console.log(`${COLOURS.yellow}║      OrqaStudio Dev Environment               ║${COLOURS.reset}`);
    console.log(`${COLOURS.yellow}╚══════════════════════════════════════════════╝${COLOURS.reset}`);
    console.log("");
    const appDir = path.join(root, "app");
    const libsDir = path.join(root, "libs");
    writeControlFile(root, {
        state: "starting",
        app: null,
        search: null,
        mcp: null,
        lsp: null,
    });
    // ── 1. Start daemon ─────────────────────────────────────────────────
    logCtrl("Starting daemon...");
    try {
        const { runDaemonCommand } = await import("./daemon.js");
        await runDaemonCommand(["start"]);
    }
    catch {
        logCtrl("Daemon start failed — services may not have graph access");
    }
    // ── 2. Start search, MCP, LSP servers (pre-built binaries) ──────────
    const searchProc = createManagedProcess("search", COLOURS.orange);
    const mcpProc = createManagedProcess("mcp", COLOURS.teal);
    const lspProc = createManagedProcess("lsp", COLOURS.pink);
    /**
     * Find a pre-built binary in workspace target dirs.
     * @param name - Binary name without extension.
     * @returns Full path to the binary, falling back to name-only if not found.
     */
    function findBin(name) {
        const ext = isWindows() ? ".exe" : "";
        const candidates = [
            path.join(root, "target", "debug", `${name}${ext}`),
            path.join(root, "target", "release", `${name}${ext}`),
        ];
        for (const c of candidates) {
            if (fs.existsSync(c))
                return c;
        }
        return `${name}${ext}`;
    }
    function startSearch() {
        logCtrl("Starting search server...");
        spawnManaged(searchProc, findBin("orqa-search-server"), [appDir], {
            stdinMode: "pipe",
            env: rustEnv(),
        });
    }
    function startMcp() {
        logCtrl("Starting MCP server...");
        spawnManaged(mcpProc, findBin("orqa-mcp-server"), [appDir], {
            stdinMode: "pipe",
            env: rustEnv(),
        });
    }
    function startLsp() {
        logCtrl("Starting LSP server...");
        spawnManaged(lspProc, findBin("orqa-lsp-server"), [appDir], {
            stdinMode: "pipe",
            env: rustEnv(),
        });
    }
    startSearch();
    await sleep(2_000);
    startMcp();
    startLsp();
    logSuccess("Search, MCP, and LSP servers started.");
    // ── 4. TypeScript library watch builds ───────────────────────────────
    const tsLibs = ["libs/sdk", "libs/graph-visualiser", "libs/logger"];
    for (const lib of tsLibs) {
        const libDir = path.join(root, lib);
        const tsconfigPath = path.join(libDir, "tsconfig.json");
        if (!fs.existsSync(tsconfigPath))
            continue;
        const proc = createManagedProcess(`tsc:${lib.split("/").pop()}`, COLOURS.dim);
        spawnManaged(proc, npx(), ["tsc", "--watch", "--preserveWatchOutput"], { cwd: libDir });
        logCtrl(`TypeScript watch: ${lib}`);
    }
    // ── 5. Start Tauri app (cargo tauri dev handles Vite + app) ─────────
    // cargo tauri dev:
    //   - Starts Vite via beforeDevCommand (HMR for frontend)
    //   - Compiles the app in dev mode (uses devUrl, not frontendDist)
    //   - Watches app Rust source and recompiles on change
    //   - Launches the app window
    const app = createManagedProcess("app", COLOURS.magenta);
    async function startRust() {
        logCtrl("Starting Tauri app (cargo tauri dev)...");
        writeControlFile(root, {
            state: "building",
            app: null,
            search: searchProc.child?.pid ?? null,
            mcp: mcpProc.child?.pid ?? null,
            lsp: lspProc.child?.pid ?? null,
        });
        spawnManaged(app, "cargo", [
            "tauri", "dev",
        ], {
            cwd: appDir,
            env: rustEnv(),
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
        }
        else {
            logCtrl("Tauri app may still be compiling — check the terminal.");
        }
        writeControlFile(root, {
            state: "running",
            app: app.child?.pid ?? null,
            search: searchProc.child?.pid ?? null,
            mcp: mcpProc.child?.pid ?? null,
            lsp: lspProc.child?.pid ?? null,
        });
        // When app exits cleanly (code 0 = user closed window), shut down everything.
        // Non-zero = crash → stay alive for restart-tauri.
        app.child?.on("close", (code) => {
            if (code === 0 || code === null) {
                logCtrl("App window closed. Shutting down...");
                killManaged(searchProc);
                killManaged(mcpProc);
                killManaged(lspProc);
                removeControlFile(root);
                cleanupSignalFile(root);
                process.exit(0);
            }
            else {
                writeControlFile(root, {
                    state: "app-crashed",
                    app: null,
                    search: searchProc.child?.pid ?? null,
                    mcp: mcpProc.child?.pid ?? null,
                    lsp: lspProc.child?.pid ?? null,
                });
                logCtrl(`App crashed (code ${code}). Use 'orqa dev restart-tauri' to relaunch.`);
            }
        });
    }
    await startRust();
    // ── 4. Rust Source File Watchers ────────────────────────────────────
    if (opts.watch) {
        logCtrl("Setting up Rust source file watchers...");
        // daemon (engine/validation) and search (engine/search) are watched by cargo tauri dev.
        // We only watch MCP and LSP which Tauri doesn't cover.
        setupRustWatchers([
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
            // Tauri app watching is handled by cargo tauri dev — not our watcher
        ]);
        logSuccess("Rust file watchers active.");
        logCtrl("Setting up plugin source watchers...");
        setupPluginWatchers(root);
        logSuccess("Plugin file watchers active.");
    }
    else {
        logCtrl("File watching disabled (--no-watch).");
    }
    // ── 5. Signal File Handling ──────────────────────────────────────────
    const signalFile = getSignalFilePath(root);
    let lastMtime = 0;
    function writeFullControlFile(state) {
        writeControlFile(root, {
            state,
            app: app.child?.pid ?? null,
            search: searchProc.child?.pid ?? null,
            mcp: mcpProc.child?.pid ?? null,
            lsp: lspProc.child?.pid ?? null,
        });
    }
    async function processSignal(signal) {
        if (signal === "restart-app") {
            // The app = cargo tauri dev (Vite + Tauri compilation + app window)
            logCtrl("Restarting app...");
            killManaged(app);
            await sleep(1_000);
            await startRust();
            logSuccess("App restarted.");
        }
        else if (signal === "restart-search") {
            logCtrl("Restart search signal received — restarting search (and MCP)...");
            killManaged(mcpProc);
            killManaged(searchProc);
            await sleep(500);
            startSearch();
            await sleep(2_000);
            startMcp();
            writeFullControlFile("running");
            logSuccess("Search and MCP restarted.");
        }
        else if (signal === "restart-mcp") {
            logCtrl("Restart MCP signal received...");
            killManaged(mcpProc);
            await sleep(500);
            startMcp();
            writeFullControlFile("running");
            logSuccess("MCP server restarted.");
        }
        else if (signal === "restart-lsp") {
            logCtrl("Restart LSP signal received...");
            killManaged(lspProc);
            await sleep(500);
            startLsp();
            writeFullControlFile("running");
            logSuccess("LSP server restarted.");
        }
        else if (signal === "restart-daemon") {
            logCtrl("Restart daemon signal received...");
            try {
                const { runDaemonCommand } = await import("./daemon.js");
                await runDaemonCommand(["restart"]);
                logSuccess("Daemon restarted.");
            }
            catch {
                logError("ctrl", "Daemon restart failed — may need manual restart");
            }
        }
        else if (signal === "restart") {
            logCtrl("Full restart signal received — restarting everything...");
            // Restart daemon first
            try {
                const { runDaemonCommand } = await import("./daemon.js");
                await runDaemonCommand(["restart"]);
                logSuccess("Daemon restarted.");
            }
            catch {
                logError("ctrl", "Daemon restart failed");
            }
            killManaged(app);
            killManaged(mcpProc);
            killManaged(lspProc);
            killManaged(searchProc);
            // Restart daemon
            try {
                const { runDaemonCommand } = await import("./daemon.js");
                await runDaemonCommand(["restart"]);
            }
            catch { /* ignore */ }
            // Restart services
            killManaged(searchProc);
            killManaged(mcpProc);
            killManaged(lspProc);
            await sleep(500);
            startSearch();
            await sleep(2_000);
            startMcp();
            startLsp();
            // Restart app (cargo tauri dev)
            killManaged(app);
            await sleep(1_000);
            await startRust();
            logSuccess("Full restart complete.");
        }
        else if (signal === "stop") {
            logCtrl("Stop signal received — shutting down...");
            closeAllWatchers();
            killManaged(app);
            killManaged(searchProc);
            killManaged(mcpProc);
            killManaged(lspProc);
            removeControlFile(root);
            cleanupSignalFile(root);
            fs.unwatchFile(signalFile);
            logSuccess("All processes stopped.");
            process.exit(0);
        }
    }
    // Watch for signal file changes (written by restart subcommands)
    fs.watchFile(signalFile, { interval: 500 }, async (curr) => {
        if (curr.mtimeMs <= lastMtime)
            return;
        lastMtime = curr.mtimeMs;
        try {
            const signal = fs.readFileSync(signalFile, "utf-8").trim();
            await processSignal(signal);
        }
        catch {
            // Signal file may not exist yet
        }
    });
    // Handle Ctrl+C / terminal close
    function shutdown() {
        logCtrl("Shutting down...");
        closeAllWatchers();
        fs.unwatchFile(signalFile);
        killManaged(app);
        killManaged(searchProc);
        killManaged(mcpProc);
        killManaged(lspProc);
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
function cleanupSignalFile(root) {
    try {
        fs.unlinkSync(getSignalFilePath(root));
    }
    catch {
        /* ignore */
    }
}
/** Active fs.watch handles — closed on shutdown */
const activeWatchers = [];
function setupRustWatchers(targets) {
    for (const target of targets) {
        if (!fs.existsSync(target.dir)) {
            logCtrl(`Watch: skipping ${target.label} — ${target.dir} does not exist`);
            continue;
        }
        let debounceTimer = null;
        let rebuilding = false;
        const watcher = fs.watch(target.dir, { recursive: true }, (_event, filename) => {
            // Only care about Rust source files
            if (!filename || !filename.endsWith(".rs"))
                return;
            // Debounce: reset timer on each change
            if (debounceTimer)
                clearTimeout(debounceTimer);
            debounceTimer = setTimeout(() => {
                void handleChange(target, filename ?? "unknown");
            }, WATCH_DEBOUNCE_MS);
        });
        async function handleChange(t, file) {
            if (rebuilding)
                return;
            rebuilding = true;
            const relPath = path.relative(t.dir, path.join(t.dir, file));
            logCtrl(`Detected change in ${t.label}: ${relPath} → rebuilding...`);
            try {
                await runCargoBuild(t.manifest);
                logSuccess(`${t.label} rebuilt successfully. Restarting...`);
                await t.restart();
                logSuccess(`${t.label} restarted.`);
            }
            catch (err) {
                const msg = err instanceof Error ? err.message : String(err);
                logError("watch", `Build failed for ${t.label}: ${msg}`);
                logCtrl("Keeping old binary running. Fix the error and save again.");
            }
            finally {
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
function runCargoBuild(manifestPath) {
    return new Promise((resolve, reject) => {
        const child = spawn("cargo", ["build", "--manifest-path", manifestPath, "--color", "always"], {
            stdio: ["ignore", "pipe", "pipe"],
            shell: isWindows(),
            windowsHide: true,
        });
        child.stdout?.on("data", (data) => prefixLines("build", COLOURS.yellow, data.toString()));
        child.stderr?.on("data", (data) => prefixLines("build", COLOURS.yellow, data.toString()));
        child.on("close", (code) => {
            if (code === 0)
                resolve();
            else
                reject(new Error(`cargo build exited with code ${code}`));
        });
        child.on("error", reject);
    });
}
function closeAllWatchers() {
    for (const w of activeWatchers) {
        try {
            w.close();
        }
        catch { /* ignore */ }
    }
    activeWatchers.length = 0;
}
// ── Plugin File Watchers ────────────────────────────────────────────────────
/**
 * Watch plugin src/ directories for changes. When a plugin source file changes,
 * rebuild the plugin (`npm run build`) and refresh its content (`orqa plugin refresh`).
 * @param projectRoot - Absolute path to the project root.
 */
function setupPluginWatchers(projectRoot) {
    const pluginDirs = ["plugins", "connectors"];
    const npmCmd = npm();
    for (const baseDir of pluginDirs) {
        const absBase = path.join(projectRoot, baseDir);
        if (!fs.existsSync(absBase))
            continue;
        for (const entry of fs.readdirSync(absBase, { withFileTypes: true })) {
            if (!entry.isDirectory())
                continue;
            const pluginDir = path.join(absBase, entry.name);
            const pkgPath = path.join(pluginDir, "package.json");
            if (!fs.existsSync(pkgPath))
                continue;
            let pkg;
            try {
                pkg = JSON.parse(fs.readFileSync(pkgPath, "utf-8"));
            }
            catch {
                continue;
            }
            if (!pkg.scripts?.["build"])
                continue;
            const srcDir = path.join(pluginDir, "src");
            if (!fs.existsSync(srcDir))
                continue;
            const label = pkg.name ?? entry.name;
            let debounceTimer = null;
            let building = false;
            const watcher = fs.watch(srcDir, { recursive: true }, (_event, filename) => {
                if (!filename)
                    return;
                // Watch TS, Svelte, CSS, and JSON source files
                if (!/\.(ts|svelte|css|json|js)$/.test(filename))
                    return;
                if (debounceTimer)
                    clearTimeout(debounceTimer);
                debounceTimer = setTimeout(() => {
                    void rebuildPlugin(label, pluginDir, npmCmd, filename ?? "unknown");
                }, WATCH_DEBOUNCE_MS);
            });
            async function rebuildPlugin(pluginLabel, dir, npm, file) {
                if (building)
                    return;
                building = true;
                logCtrl(`Plugin change: ${pluginLabel} (${file}) → rebuilding...`);
                try {
                    await runNpmBuild(npm, dir);
                    logSuccess(`${pluginLabel} rebuilt.`);
                    // Refresh content so .orqa/ gets the latest
                    try {
                        const { runPluginCommand } = await import("./plugin.js");
                        await runPluginCommand(["refresh", "--plugin", pluginLabel]);
                    }
                    catch {
                        logCtrl(`Content refresh skipped for ${pluginLabel}`);
                    }
                }
                catch (err) {
                    const msg = err instanceof Error ? err.message : String(err);
                    logError("watch", `Plugin build failed for ${pluginLabel}: ${msg}`);
                }
                finally {
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
function runNpmBuild(npmCmd, cwd) {
    return new Promise((resolve, reject) => {
        const child = spawn(npmCmd, ["run", "build"], {
            cwd,
            stdio: ["ignore", "pipe", "pipe"],
            shell: isWindows(),
            windowsHide: true,
        });
        child.stdout?.on("data", (data) => prefixLines("plugin", COLOURS.pink, data.toString()));
        child.stderr?.on("data", (data) => prefixLines("plugin", COLOURS.pink, data.toString()));
        child.on("close", (code) => {
            if (code === 0)
                resolve();
            else
                reject(new Error(`npm run build exited with code ${code}`));
        });
        child.on("error", reject);
    });
}
// ── Subcommand: dev (spawn controller in background) ────────────────────────
async function cmdDev(root) {
    // Always start fresh — kill existing processes first
    const existing = readControlFile(root);
    if (existing && processIsAlive(existing.pid)) {
        logCtrl("Stopping existing dev environment...");
    }
    // 0. Rebuild the CLI itself (we may be running stale code)
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
    // 1. Install all workspace dependencies from root
    logCtrl("Installing dependencies...");
    try {
        execSync(`${npm()} install`, { cwd: root, stdio: "inherit" });
    }
    catch {
        logCtrl("npm install failed — dependencies may be stale");
    }
    // 2. Stop all running OrqaStudio processes so cargo can overwrite binaries
    await killAll(root);
    // 3. Build all Rust binaries
    logCtrl("Building Rust libraries and servers...");
    try {
        // Build everything except the app (which needs special feature flags)
        execSync("cargo build --workspace --exclude orqa-studio --color always", { cwd: root, stdio: "inherit" });
    }
    catch {
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
                execSync(`${npm()} run build`, { cwd: libDir, stdio: "inherit" });
            }
            catch {
                logCtrl(`Initial build failed for ${lib}`);
            }
        }
    }
    // 4. Install plugins from project.json (builds, syncs content, composes schema)
    logCtrl("Installing plugins from project.json...");
    try {
        const { cmdPluginSync } = await import("./install.js");
        cmdPluginSync(root);
    }
    catch {
        // Non-fatal — content may be stale but dev can still start
        logCtrl("Plugin sync failed — falling back to refresh...");
        try {
            const { runPluginCommand } = await import("./plugin.js");
            await runPluginCommand(["refresh"]);
        }
        catch {
            // Non-fatal
        }
    }
    logCtrl("Starting dev environment...");
    // Spawn the controller as a detached process (this same script with __start-controller)
    const nodeCmd = process.execPath;
    const cliEntry = path.join(root, "cli/dist/cli.js");
    // Write controller output to a log file so we can debug startup failures
    const logFile = path.join(root, ".state", "dev-controller.log");
    const logFd = fs.openSync(logFile, "w");
    const child = spawn(nodeCmd, [cliEntry, "dev", "__start-controller"], {
        cwd: root,
        detached: true,
        stdio: ["ignore", logFd, logFd],
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
        if (!ctrl)
            continue;
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
            }
            else if (ctrl.state === "starting") {
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
async function cmdStop(root) {
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
    if (ctrl)
        removeControlFile(root);
    logCtrl("No controller running. Use 'orqa dev kill' to force-kill orphaned processes.");
}
// ── Subcommand: kill ────────────────────────────────────────────────────────
async function cmdKill(root) {
    await killAll(root);
    removeControlFile(root);
    cleanupSignalFile(root);
}
// ── Subcommand: restart (send signal to running controller) ─────────────────
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
    // For restart-tauri and restart (full), start the dev env if not running
    if (signal === "restart-tauri" || signal === "restart") {
        logCtrl("No controller running. Starting dev environment...");
        await cmdDev(root);
        return;
    }
    logError("ctrl", "No controller running. Use 'orqa dev' first.");
}
// ── Subcommand: status ──────────────────────────────────────────────────────
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
    if (ctrl.mcp)
        console.log(`MCP PID: ${ctrl.mcp}`);
    if (ctrl.lsp)
        console.log(`LSP PID: ${ctrl.lsp}`);
}
// ── Subcommand: icons ───────────────────────────────────────────────────────
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
// ── Subcommand: tool ────────────────────────────────────────────────────────
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
    const cmd = `"${debugToolPath}" ${args.join(" ")}`;
    try {
        execSync(cmd, { encoding: "utf-8", stdio: "inherit" });
    }
    catch {
        process.exit(1);
    }
}
// ── Main entry point ────────────────────────────────────────────────────────
/**
 * Dispatch the dev command: start the dev environment or a subcommand.
 * @param args - CLI arguments after "dev".
 */
export async function runDevCommand(args) {
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
            }
            else {
                const signalMap = {
                    daemon: ["restart-daemon", "Restart daemon"],
                    app: ["restart-app", "Restart app (Vite + Tauri)"],
                    search: ["restart-search", "Restart search (+ MCP)"],
                    mcp: ["restart-mcp", "Restart MCP"],
                    lsp: ["restart-lsp", "Restart LSP"],
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
//# sourceMappingURL=dev.js.map