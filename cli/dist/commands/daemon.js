/**
 * Daemon command — manage the orqa-daemon background process.
 *
 * The orqa-daemon is a persistent Rust process that provides file watching,
 * health monitoring, system tray, and MCP/LSP server lifecycle management.
 * This command is the CLI interface for starting, stopping, restarting, and
 * checking the status of that process.
 *
 * Subcommands:
 *   orqa daemon start    — spawn orqa-daemon, verify via health endpoint
 *   orqa daemon stop     — send SIGTERM (Unix) or taskkill (Windows) via PID file
 *   orqa daemon restart  — stop then start
 *   orqa daemon status   — show PID, uptime, and health endpoint response
 */
import { spawn } from "node:child_process";
import { existsSync, readFileSync, mkdirSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { getRoot } from "../lib/root.js";
// ---------------------------------------------------------------------------
// Port resolution
// ---------------------------------------------------------------------------
/** Default port for the orqa-daemon health endpoint (matches health.rs DEFAULT_PORT). */
const DEFAULT_DAEMON_PORT = 9120;
/**
 * Resolve the daemon port from the ORQA_PORT_BASE environment variable.
 *
 * The daemon reads ORQA_PORT_BASE directly as the port number (not as a base
 * for an offset). This matches the Rust daemon's health.rs resolve_port().
 * @returns The daemon port number.
 */
function getDaemonPort() {
    const raw = process.env["ORQA_PORT_BASE"];
    if (raw === undefined || raw === "")
        return DEFAULT_DAEMON_PORT;
    const n = parseInt(raw, 10);
    return Number.isNaN(n) ? DEFAULT_DAEMON_PORT : n;
}
// ---------------------------------------------------------------------------
// Usage
// ---------------------------------------------------------------------------
const USAGE = `
Usage: orqa daemon <subcommand>

Manage the orqa-daemon background process. The daemon provides file watching,
health monitoring, system tray, and MCP/LSP server lifecycle management.

Subcommands:
  start    Start the daemon in the background
  stop     Stop the running daemon (reads PID from .state/daemon.pid)
  restart  Stop then start the daemon
  status   Show daemon status: running/stopped, PID, uptime

Options:
  --help, -h    Show this help message

The daemon port is configured via ORQA_PORT_BASE (default: ${DEFAULT_DAEMON_PORT}).
`.trim();
// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------
/**
 * Dispatch orqa daemon subcommands.
 * @param args - CLI arguments after "daemon".
 */
export async function runDaemonCommand(args) {
    if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
        console.log(USAGE);
        return;
    }
    const subcommand = args[0];
    switch (subcommand) {
        case "start":
            await daemonStart();
            break;
        case "stop":
            await daemonStop();
            break;
        case "restart":
            await daemonRestart();
            break;
        case "status":
            await daemonStatus();
            break;
        default:
            console.error(`Unknown daemon subcommand: ${subcommand}`);
            console.error("Available: start, stop, restart, status");
            process.exit(1);
    }
}
// ---------------------------------------------------------------------------
// start
// ---------------------------------------------------------------------------
/**
 * Start the orqa-daemon binary in the background.
 *
 * Finds the binary in well-known build directories, spawns it detached so it
 * survives the CLI process exit, then waits up to 3 seconds for the health
 * endpoint to respond before reporting success or failure.
 */
async function daemonStart() {
    const projectRoot = getRoot();
    const port = getDaemonPort();
    const pidPath = getPidPath(projectRoot);
    // Check if already running.
    const existing = readPid(pidPath);
    if (existing !== null && processIsAlive(existing)) {
        const health = await fetchHealth(port);
        if (health !== null) {
            console.log(`Daemon already running (PID ${existing}, port ${port}).`);
            return;
        }
        // PID file exists but /health failed — stale, continue to start a fresh instance.
    }
    const binary = findDaemonBinary(projectRoot);
    if (binary === null) {
        throw new Error("orqa-daemon binary not found.\n" +
            "Build with: cargo build -p orqa-daemon\n" +
            "Searched: target/debug/, target/release/");
    }
    ensureStateDir(projectRoot);
    // Spawn detached so the daemon outlives this CLI process.
    // The daemon resolves its own project root from CWD, so set cwd to the project root.
    const child = spawn(binary, [], {
        cwd: projectRoot,
        detached: true,
        stdio: "ignore",
        windowsHide: true,
    });
    child.unref();
    // Wait up to 3 seconds for /health to respond.
    const startedAt = Date.now();
    let health = null;
    while (Date.now() - startedAt < 3000) {
        await sleep(150);
        health = await fetchHealth(port);
        if (health !== null)
            break;
    }
    if (health === null) {
        throw new Error("Daemon did not start within 3 seconds.\n" +
            "Check .state/daemon.log for startup errors.");
    }
    console.log(`Daemon started (PID ${health.pid}, port ${port}, uptime ${health.uptime_seconds}s).`);
}
// ---------------------------------------------------------------------------
// stop
// ---------------------------------------------------------------------------
/**
 * Stop the running daemon by sending SIGTERM via the PID file.
 *
 * On Windows, process.kill() sends the equivalent termination signal. If the
 * PID file is absent or the process is already dead, exits silently after
 * cleaning up the stale PID file.
 */
async function daemonStop() {
    const projectRoot = getRoot();
    const pidPath = getPidPath(projectRoot);
    const pid = readPid(pidPath);
    if (pid === null) {
        // No PID file — nothing to stop, nothing to report.
        return;
    }
    if (!processIsAlive(pid)) {
        // Stale PID file — clean it up silently.
        try {
            unlinkSync(pidPath);
        }
        catch { /* ignore */ }
        return;
    }
    try {
        // On Windows, process.kill() terminates the process (no UNIX signals).
        // On Unix, SIGTERM allows graceful shutdown.
        process.kill(pid, "SIGTERM");
        console.log(`Sent SIGTERM to daemon (PID ${pid}).`);
    }
    catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        console.error(`Failed to terminate daemon (PID ${pid}): ${msg}`);
    }
}
// ---------------------------------------------------------------------------
// restart
// ---------------------------------------------------------------------------
/**
 * Stop the daemon if running, then start a fresh instance.
 *
 * Waits up to 3 seconds for the existing process to exit before starting the
 * new one, to avoid port conflicts.
 */
async function daemonRestart() {
    const projectRoot = getRoot();
    const pidPath = getPidPath(projectRoot);
    const pid = readPid(pidPath);
    // Stop existing daemon if running.
    if (pid !== null && processIsAlive(pid)) {
        try {
            process.kill(pid, "SIGTERM");
            console.log(`Sent SIGTERM to daemon (PID ${pid}). Waiting for exit...`);
            // Wait up to 3 seconds for the process to die.
            const deadline = Date.now() + 3000;
            while (Date.now() < deadline) {
                await sleep(150);
                if (!processIsAlive(pid))
                    break;
            }
        }
        catch {
            // Process may have already exited — continue to start.
        }
    }
    // Start fresh.
    await daemonStart();
}
// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------
/**
 * Report daemon status: running/stopped, PID, port, and uptime from /health.
 *
 * Reads the PID file first. If the PID is alive, calls the health endpoint for
 * uptime and confirms connectivity. Reports all three states: no PID file,
 * stale PID file (process dead), and fully running.
 */
async function daemonStatus() {
    const projectRoot = getRoot();
    const pidPath = getPidPath(projectRoot);
    const pid = readPid(pidPath);
    const port = getDaemonPort();
    if (pid === null) {
        console.log("Daemon: stopped (no PID file)");
        return;
    }
    const alive = processIsAlive(pid);
    if (!alive) {
        console.log(`Daemon: stopped (PID ${pid} is dead, stale PID file at .state/daemon.pid)`);
        return;
    }
    const health = await fetchHealth(port);
    if (health === null) {
        console.log(`Daemon: PID ${pid} alive but /health on port ${port} did not respond.\n` +
            `  Check .state/daemon.log for errors.`);
        return;
    }
    const uptime = formatUptime(health.uptime_seconds);
    console.log(`Daemon: running\n` +
        `  PID    : ${health.pid}\n` +
        `  port   : ${port}\n` +
        `  uptime : ${uptime}`);
}
/**
 * Locate the orqa-daemon binary in well-known build output directories.
 *
 * Checks workspace-level target/ directories (debug before release) and
 * the legacy daemon-specific target/ location. Returns null if not found.
 * @param projectRoot - Absolute path to the project root.
 * @returns Full path to the binary, or null if not found.
 */
function findDaemonBinary(projectRoot) {
    const name = process.platform === "win32" ? "orqa-daemon.exe" : "orqa-daemon";
    const candidates = [
        join(projectRoot, "target", "debug", name),
        join(projectRoot, "target", "release", name),
        join(projectRoot, "daemon", "target", "debug", name),
        join(projectRoot, "daemon", "target", "release", name),
    ];
    for (const c of candidates) {
        if (existsSync(c))
            return c;
    }
    return null;
}
/**
 * Return the path to the daemon PID file (.state/daemon.pid).
 * @param projectRoot - Absolute path to the project root.
 * @returns Absolute path to the PID file.
 */
function getPidPath(projectRoot) {
    return join(projectRoot, ".state", "daemon.pid");
}
/**
 * Create .state/ directory if it does not exist.
 * @param projectRoot - Absolute path to the project root.
 */
function ensureStateDir(projectRoot) {
    const stateDir = join(projectRoot, ".state");
    if (!existsSync(stateDir)) {
        mkdirSync(stateDir, { recursive: true });
    }
}
/**
 * Read the PID from the given PID file path.
 *
 * Returns null if the file does not exist or contains a non-numeric value.
 * @param pidPath - Absolute path to the PID file.
 * @returns The PID number, or null if the file is absent or invalid.
 */
function readPid(pidPath) {
    if (!existsSync(pidPath))
        return null;
    const raw = readFileSync(pidPath, "utf-8").trim();
    const n = parseInt(raw, 10);
    return Number.isNaN(n) ? null : n;
}
/**
 * Return true if a process with the given PID is currently alive.
 *
 * Uses signal 0 (existence check, no actual signal sent). Returns false on
 * any error, treating inaccessible processes as dead for PID file cleanup.
 * @param pid - The process ID to check.
 * @returns True if the process is alive.
 */
function processIsAlive(pid) {
    try {
        // Signal 0: check existence without sending a real signal.
        process.kill(pid, 0);
        return true;
    }
    catch {
        return false;
    }
}
/**
 * Call the daemon health endpoint with a short timeout.
 *
 * Returns the parsed response on success, or null if the daemon is
 * unreachable, the response is not OK, or the body is not valid JSON.
 * @param port - Port number to query.
 * @returns Parsed health response, or null on failure.
 */
async function fetchHealth(port) {
    try {
        const controller = new AbortController();
        const timeout = setTimeout(() => controller.abort(), 500);
        try {
            const response = await fetch(`http://127.0.0.1:${port}/health`, {
                signal: controller.signal,
            });
            if (!response.ok)
                return null;
            return (await response.json());
        }
        finally {
            clearTimeout(timeout);
        }
    }
    catch {
        return null;
    }
}
/**
 * Format uptime_seconds as a human-readable duration string.
 *
 * Examples: "5s", "2m 30s", "1h 5m 3s".
 * @param seconds - Uptime in seconds.
 * @returns Human-readable duration string.
 */
function formatUptime(seconds) {
    if (seconds < 60)
        return `${seconds}s`;
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    if (h > 0)
        return `${h}h ${m}m ${s}s`;
    return `${m}m ${s}s`;
}
/**
 * Pause execution for the given number of milliseconds.
 * @param ms - Duration to sleep in milliseconds.
 * @returns Promise that resolves after the delay.
 */
function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
//# sourceMappingURL=daemon.js.map