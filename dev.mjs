#!/usr/bin/env node
// dev.mjs — OrqaStudio Dev Controller
//
// A persistent process that owns the entire dev lifecycle.
// Replaces `cargo tauri dev` with direct control over Vite and cargo.
//
// Run from the app directory (cwd is used as project root):
//   node <path>/dev.mjs dev            Start dev environment (detached, exits when ready)
//   node <path>/dev.mjs start          Start the controller (long-running, stays in foreground)
//   node <path>/dev.mjs stop           Gracefully stop the controller and all processes
//   node <path>/dev.mjs kill           Force-kill all OrqaStudio processes
//   node <path>/dev.mjs restart-tauri  Restart the Tauri app (Vite stays alive)
//   node <path>/dev.mjs restart-vite   Restart the Vite dev server
//   node <path>/dev.mjs restart        Restart everything (Vite + Tauri)
//   node <path>/dev.mjs status         Show what's running
//
// Why this exists:
//   1. cargo tauri dev orphans Vite on crash (Tauri #10023, #2794, #1626)
//   2. taskkill from MSYS2/Git Bash hangs (path mangling)
//   3. No visibility into build output during development
//   4. Restart kills everything — no way to keep Vite alive
//   See: .orqa/delivery/research/RES-016-tauri-dev-process-management.md

import { execSync, spawn } from "node:child_process";
import { createServer as createNetServer } from "node:net";
import { createServer as createHttpServer } from "node:http";
import { platform } from "node:os";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  writeFileSync,
  readFileSync,
  readdirSync,
  unlinkSync,
  existsSync,
  mkdirSync,
  watchFile,
  unwatchFile,
} from "node:fs";

const IS_WINDOWS = platform() === "win32";
const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(process.cwd());
const CONTROL_FILE = join(PROJECT_ROOT, "tmp", "dev-controller.json");
const DASHBOARD_HTML = join(SCRIPT_DIR, "dev-dashboard.html");
const UI_DIR = join(PROJECT_ROOT, "ui");
const DEV_ROOT = resolve(PROJECT_ROOT, "..");
const LIBS_DIR = join(DEV_ROOT, "libs");
const VITE_PORT = 1420;
const DASHBOARD_PORT = 3001;
const PORT_TIMEOUT_MS = 15_000;
const POLL_INTERVAL_MS = 500;

// ── Logging ─────────────────────────────────────────────────────────────────

const COLOURS = {
  reset: "\x1b[0m",
  dim: "\x1b[2m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  magenta: "\x1b[35m",
  cyan: "\x1b[36m",
};

function prefixLines(prefix, colour, data, isError = false) {
  const text = data.toString().trimEnd();
  if (!text) return;
  for (const line of text.split("\n")) {
    const ts = new Date().toLocaleTimeString("en-GB", { hour12: false });
    console.log(
      `${COLOURS.dim}${ts}${COLOURS.reset} ${colour}[${prefix}]${COLOURS.reset} ${line}`,
    );
    // Broadcast to dashboard SSE clients
    sseLog(prefix, line, isError);
  }
}

function log(prefix, msg) {
  prefixLines(prefix, COLOURS.cyan, msg);
}

function logCtrl(msg) {
  prefixLines("ctrl", COLOURS.yellow, msg);
}

function logError(prefix, msg) {
  prefixLines(prefix, COLOURS.red, msg, true);
}

function logSuccess(msg) {
  prefixLines("ctrl", COLOURS.green, msg);
}

// ── SSE Dashboard Server ────────────────────────────────────────────────────

const sseClients = new Set();

/** Broadcast a named SSE event to all connected clients */
function sseBroadcast(event, data) {
  const payload = `event: ${event}\ndata: ${JSON.stringify(data)}\n\n`;
  for (const res of sseClients) {
    try {
      res.write(payload);
    } catch {
      sseClients.delete(res);
    }
  }
}

/** Broadcast a log line to SSE clients */
function sseLog(source, text, isError = false) {
  sseBroadcast("log", { source, text, error: isError });
}

/** Broadcast process status to SSE clients */
function sseStatus(viteAlive, rustState) {
  sseBroadcast("status", { vite: viteAlive, rust: rustState });
}

/** Signal handler — called by dashboard command buttons */
let signalHandler = null;

function startDashboardServer() {
  const server = createHttpServer((req, res) => {
    // CORS headers for local dev
    res.setHeader("Access-Control-Allow-Origin", "*");

    if (req.method === "GET" && req.url === "/") {
      // Serve dashboard HTML
      try {
        const html = readFileSync(DASHBOARD_HTML, "utf-8");
        res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
        res.end(html);
      } catch (err) {
        res.writeHead(500, { "Content-Type": "text/plain" });
        res.end(`Failed to read dashboard: ${err.message}`);
      }
      return;
    }

    if (req.method === "GET" && req.url === "/open") {
      // Opener page: uses window.open() so the dashboard tab can be closed later.
      // Tabs opened via window.open() allow window.close() — OS-launched tabs don't.
      res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
      res.end(`<!DOCTYPE html><html><body><script>
        window.open('/', '_blank');
        window.close();
        // Fallback if close is blocked: redirect to dashboard
        setTimeout(() => { window.location = '/'; }, 200);
      </script></body></html>`);
      return;
    }

    if (req.method === "GET" && req.url === "/events") {
      // SSE endpoint
      res.writeHead(200, {
        "Content-Type": "text/event-stream",
        "Cache-Control": "no-cache",
        Connection: "keep-alive",
      });
      res.write(":\n\n"); // SSE comment to establish connection
      sseClients.add(res);

      req.on("close", () => {
        sseClients.delete(res);
      });
      return;
    }

    // Frontend logger forwarding endpoint (dev mode only).
    // The SDK logger POSTs structured entries here via sendBeacon/fetch.
    if (req.method === "POST" && req.url === "/log") {
      let body = "";
      req.on("data", (chunk) => { body += chunk; });
      req.on("end", () => {
        try {
          const data = JSON.parse(body);
          // Use the logger's source tag if provided, fall back to "frontend"
          const source = data.source || "frontend";
          const level = data.level || "info";
          const text = data.message || "";
          const isError = level === "error";
          // Broadcast to dashboard with source and level preserved
          sseBroadcast("log", { source, text, error: isError, level });
          prefixLines(source, COLOURS.blue, text, isError);
        } catch {
          // Malformed JSON — ignore silently
        }
        res.writeHead(204);
        res.end();
      });
      return;
    }

    if (req.method === "OPTIONS" && req.url === "/log") {
      // CORS preflight for /log
      res.setHeader("Access-Control-Allow-Methods", "POST, OPTIONS");
      res.setHeader("Access-Control-Allow-Headers", "Content-Type");
      res.writeHead(204);
      res.end();
      return;
    }

    if (req.method === "POST" && req.url?.startsWith("/command/")) {
      const cmd = req.url.replace("/command/", "");
      if (signalHandler && ["start", "restart-tauri", "restart-vite", "restart", "stop"].includes(cmd)) {
        signalHandler(cmd);
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ message: `Signal '${cmd}' sent.` }));
      } else {
        res.writeHead(400, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ message: `Unknown command: ${cmd}` }));
      }
      return;
    }

    res.writeHead(404, { "Content-Type": "text/plain" });
    res.end("Not found");
  });

  server.listen(DASHBOARD_PORT, "127.0.0.1", () => {
    logCtrl(`Dashboard: http://localhost:${DASHBOARD_PORT}`);
  });

  server.on("error", (err) => {
    if (err.code === "EADDRINUSE") {
      logCtrl(
        `Dashboard port ${DASHBOARD_PORT} in use — dashboard disabled (controller still works).`,
      );
    } else {
      logError("ctrl", `Dashboard server error: ${err.message}`);
    }
  });

  return server;
}

// ── Process Utilities ───────────────────────────────────────────────────────

function exec(cmd) {
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

function findPidsOnPort(port) {
  if (IS_WINDOWS) {
    const out = exec("netstat -ano");
    const pids = new Set();
    for (const line of out.split("\n")) {
      if (line.includes(`:${port}`) && line.includes("LISTENING")) {
        const parts = line.trim().split(/\s+/);
        const pid = parseInt(parts[parts.length - 1], 10);
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

function findPidsByName(name) {
  if (IS_WINDOWS) {
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

function killProcessTree(pid) {
  if (pid === process.pid) return;

  if (IS_WINDOWS) {
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

function isPortFree(port) {
  return new Promise((resolve) => {
    const server = createNetServer();
    server.once("error", () => resolve(false));
    server.once("listening", () => server.close(() => resolve(true)));
    server.listen(port, "127.0.0.1");
  });
}

async function waitForPort(port, timeoutMs, wantFree = true) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const free = await isPortFree(port);
    if (wantFree === free) return true;
    await new Promise((r) => setTimeout(r, POLL_INTERVAL_MS));
  }
  return false;
}

// ── Control File (IPC) ──────────────────────────────────────────────────────
// The controller writes its PID and state to tmp/dev-controller.json.
// Other commands (stop, restart-tauri, status) read it and send signals.

function writeControlFile(state) {
  const dir = join(PROJECT_ROOT, "tmp");
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
  writeFileSync(
    CONTROL_FILE,
    JSON.stringify({ pid: process.pid, ...state }, null, 2),
  );
}

function readControlFile() {
  try {
    return JSON.parse(readFileSync(CONTROL_FILE, "utf-8"));
  } catch {
    return null;
  }
}

function removeControlFile() {
  try {
    unlinkSync(CONTROL_FILE);
  } catch {
    /* ignore */
  }
}

// ── Kill All (for stop command from outside) ────────────────────────────────

async function killAll() {
  logCtrl("Stopping OrqaStudio processes...");

  const pidsToKill = new Set();

  for (const name of ["orqa-studio", "cargo-tauri"]) {
    for (const pid of findPidsByName(name)) {
      logCtrl(`Found ${name} (PID ${pid})`);
      pidsToKill.add(pid);
    }
  }
  for (const port of [VITE_PORT, 5173, DASHBOARD_PORT]) {
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

  removeControlFile();
  logSuccess("All processes stopped.");
}

// ── Child Process Management ────────────────────────────────────────────────

class ChildProcess {
  constructor(name, colour) {
    this.name = name;
    this.colour = colour;
    this.process = null;
    this.running = false;
  }

  spawn(cmd, args, opts = {}) {
    this.process = spawn(cmd, args, {
      cwd: PROJECT_ROOT,
      env: { ...process.env },
      stdio: ["ignore", "pipe", "pipe"],
      shell: IS_WINDOWS, // Windows needs shell for .cmd/.bat executables (npx, cargo)
      windowsHide: true, // All output streams through the controller — no child windows needed
      ...opts,
    });

    this.running = true;

    this.process.stdout?.on("data", (data) =>
      prefixLines(this.name, this.colour, data, false),
    );
    // Cargo and the sidecar write normal output (build progress, "Finished",
    // "Running", startup messages) to stderr. Don't mark it as error — real
    // errors are distinguishable by content, not by stream.
    this.process.stderr?.on("data", (data) =>
      prefixLines(this.name, this.colour, data, false),
    );

    this.process.on("close", (code) => {
      this.running = false;
      if (code !== 0 && code !== null) {
        logError(this.name, `Exited with code ${code}`);
      } else {
        log(this.name, "Process exited.");
      }
    });

    return this;
  }

  kill() {
    if (this.process && this.running) {
      log(this.name, "Stopping...");
      killProcessTree(this.process.pid);
      this.running = false;
    }
  }

  get pid() {
    return this.process?.pid;
  }
}

// ── Controller (start command) ──────────────────────────────────────────────

async function start() {
  // Check if controller is already running
  const existing = readControlFile();
  if (existing) {
    try {
      process.kill(existing.pid, 0); // check if alive
      logError(
        "ctrl",
        `Controller already running (PID ${existing.pid}). Use 'make stop' first.`,
      );
      process.exit(1);
    } catch {
      // Stale control file — previous controller died
      removeControlFile();
    }
  }

  // Kill any orphaned processes from previous runs
  await killAll();

  console.log("");
  console.log(
    `${COLOURS.yellow}╔══════════════════════════════════════════════╗${COLOURS.reset}`,
  );
  console.log(
    `${COLOURS.yellow}║      OrqaStudio Dev Controller v1.0          ║${COLOURS.reset}`,
  );
  console.log(
    `${COLOURS.yellow}╚══════════════════════════════════════════════╝${COLOURS.reset}`,
  );
  console.log("");

  writeControlFile({ state: "starting", vite: null, rust: null });

  // ── 0. Start Dashboard Server ────────────────────────────────────────
  let dashboardServer = null;
  dashboardServer = startDashboardServer();

  // Auto-open dashboard in browser
  // We serve a /open endpoint that uses window.open() to create the dashboard tab.
  // Tabs opened by window.open() can be closed by window.close() — tabs opened
  // by OS commands (start, open, xdg-open) cannot. This lets the dashboard
  // auto-close when the controller shuts down.
  setTimeout(() => {
    const openerUrl = `http://localhost:${DASHBOARD_PORT}/open`;
    try {
      if (IS_WINDOWS) {
        execSync(`start "" "${openerUrl}"`, { stdio: "ignore", windowsHide: true });
      } else if (platform() === "darwin") {
        execSync(`open "${openerUrl}"`, { stdio: "ignore" });
      } else {
        execSync(`xdg-open "${openerUrl}"`, { stdio: "ignore" });
      }
    } catch {
      // Browser open failed — user can navigate manually
    }
  }, 500);

  // ── 1. Start Vite ──────────────────────────────────────────────────────

  logCtrl("Starting Vite dev server...");

  const npmCmd = IS_WINDOWS ? "npm.cmd" : "npm";
  const vite = new ChildProcess("vite", COLOURS.cyan);
  vite.spawn(npmCmd, ["run", "dev"], { cwd: UI_DIR });

  // Wait for Vite to be ready (port occupied = listening)
  logCtrl(`Waiting for Vite on port ${VITE_PORT}...`);
  const viteReady = await waitForPort(VITE_PORT, 30_000, false);
  if (!viteReady) {
    logError("ctrl", "Vite failed to start within 30s");
    vite.kill();
    process.exit(1);
  }
  logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
  sseStatus(true, false);

  // ── 1b. Start Library & Plugin Watchers ─────────────────────────────────
  // Watch mode for all first-party packages — changes auto-rebuild to dist/
  // and Vite picks them up via HMR. Production builds are separate (npm run build).
  //
  // Discovery: scans libs/, plugins/, connectors/ for package.json files with
  // a "dev" script (preferred) or a "build" script (falls back to tsc --watch).
  // Packages missing node_modules get `npm install` + npm link for @orqastudio/* deps.

  const libWatchers = [];
  const npxCmd = IS_WINDOWS ? "npx.cmd" : "npx";

  /** Ensure a package has its dependencies installed and @orqastudio/* deps linked. */
  function ensureDeps(dir, pkg) {
    const hasDeps = pkg.dependencies || pkg.devDependencies;
    if (!hasDeps) return;

    const pkgName = pkg.name || dir;

    const hasNodeModules = existsSync(join(dir, "node_modules"));
    if (!hasNodeModules) {
      logCtrl(`Installing deps for ${pkgName}...`);
      try {
        // Install non-@orqastudio deps (--ignore-scripts to avoid build loops)
        execSync(`${npmCmd} install --ignore-scripts`, {
          cwd: dir,
          stdio: "pipe",
          timeout: 60_000,
          windowsHide: true,
        });
        logSuccess(`Deps installed for ${pkgName}`);
      } catch (err) {
        // Classify failure: if all missing packages are @orqastudio/* → info (expected, will link).
        // If any non-@orqastudio package failed → error (real problem).
        const stderr = err.stderr?.toString() || "";
        const hasOrqaFailure = stderr.includes("@orqastudio/");
        const hasNonOrqaFailure = stderr.split("\n").some((line) => {
          // A 404 for a non-@orqastudio package, or a non-404 error
          if (!line.includes("npm error")) return false;
          if (line.includes("404") && !line.includes("@orqastudio/")) {
            // Check if this 404 line references any package (not just a generic 404 message)
            return /404.*(?:Not Found|does not exist)/.test(line) && !hasOrqaFailure;
          }
          // Non-404 errors that aren't about @orqastudio packages
          return line.includes("ETARGET") || line.includes("ERESOLVE") ||
            (line.includes("ERR!") && !line.includes("@orqastudio/"));
        });

        if (hasOrqaFailure && !hasNonOrqaFailure) {
          logCtrl(`${pkgName}: @orqastudio deps not in registry — will link`);
        } else if (hasNonOrqaFailure) {
          // Extract the actual failing package name from stderr
          const failingPkgs = stderr.match(/notarget.*?for\s+(\S+)/g) || [];
          logError("ctrl", `${pkgName}: npm install failed for: ${failingPkgs.map((m) => m.replace(/.*for\s+/, "")).join(", ") || "unknown packages"}`);
        }
      }
    }

    // Link any @orqastudio/* dependencies
    const allDeps = {
      ...pkg.dependencies,
      ...pkg.peerDependencies,
      ...pkg.devDependencies,
    };
    const orqaDeps = Object.keys(allDeps).filter((d) => d.startsWith("@orqastudio/"));
    if (orqaDeps.length > 0) {
      try {
        execSync(`${npmCmd} link ${orqaDeps.join(" ")}`, {
          cwd: dir,
          stdio: "pipe",
          timeout: 30_000,
          windowsHide: true,
        });
        logCtrl(`${pkgName}: linked ${orqaDeps.join(", ")}`);
      } catch (err) {
        const stderr = err.stderr?.toString() || "";
        logError("ctrl", `${pkgName}: failed to link @orqastudio deps:\n  ${stderr.trim()}`);
      }
    }
  }

  function discoverWatchablePackages() {
    const packages = [];
    const searchDirs = [
      join(DEV_ROOT, "libs"),
      join(DEV_ROOT, "plugins"),
      join(DEV_ROOT, "connectors"),
    ];

    for (const searchDir of searchDirs) {
      if (!existsSync(searchDir)) continue;
      let entries;
      try {
        entries = readdirSync(searchDir, { withFileTypes: true });
      } catch { continue; }

      for (const entry of entries) {
        if (!entry.isDirectory()) continue;
        const pkgPath = join(searchDir, entry.name, "package.json");
        if (!existsSync(pkgPath)) continue;

        try {
          const pkg = JSON.parse(readFileSync(pkgPath, "utf-8"));
          const scripts = pkg.scripts || {};
          const dir = join(searchDir, entry.name);
          const shortName = entry.name;

          // Ensure dependencies are installed before starting watcher
          ensureDeps(dir, pkg);

          // Use the dev script ONLY if it's a watch command (contains --watch).
          // Scripts like "vite dev" start a dev server — not what we want.
          if (scripts.dev && scripts.dev.includes("--watch")) {
            packages.push({ name: shortName, dir, cmd: npmCmd, args: ["run", "dev"] });
          } else if (scripts.build) {
            // Infer watch mode from build script
            const build = scripts.build;
            if (build.includes("svelte-package")) {
              packages.push({
                name: shortName, dir, cmd: npxCmd,
                args: ["svelte-package", "-i", "src", "-o", "dist", "--watch", "--preserve-output"],
              });
            } else if (build.includes("tsc") && existsSync(join(dir, "tsconfig.json"))) {
              packages.push({
                name: shortName, dir, cmd: npxCmd,
                args: ["tsc", "--watch", "--preserveWatchOutput"],
              });
            } else if (build.includes("vite build")) {
              packages.push({
                name: shortName, dir, cmd: npxCmd,
                args: ["vite", "build", "--watch"],
              });
            }
          }
        } catch { continue; }
      }
    }

    return packages;
  }

  const watchablePackages = discoverWatchablePackages();

  for (const pkg of watchablePackages) {
    const watcher = new ChildProcess(pkg.name, COLOURS.green);
    watcher.spawn(pkg.cmd, pkg.args, { cwd: pkg.dir });
    libWatchers.push(watcher);
  }

  if (watchablePackages.length > 0) {
    logSuccess(
      `Library watchers started: ${watchablePackages.map((p) => p.name).join(", ")}`,
    );
  }

  // ── 2. Build + Run Rust ────────────────────────────────────────────────

  let rust = null;

  async function startRust() {
    logCtrl("Compiling and starting Rust app...");
    sseStatus(true, "building");

    rust = new ChildProcess("rust", COLOURS.magenta);
    rust.spawn(
      "cargo",
      [
        "run",
        "--manifest-path",
        "backend/src-tauri/Cargo.toml",
        "--no-default-features",
        "--color",
        "always",
      ],
      {
        env: {
          ...process.env,
          // Enable Rust logging so the controller window is useful for debugging
          RUST_LOG: process.env.RUST_LOG || "info",
        },
      },
    );

    writeControlFile({
      state: "running",
      vite: vite.pid,
      rust: rust.pid,
    });
    sseStatus(true, true);

    // When the app exits:
    // - code 0 = user closed the window → shut down everything
    // - non-zero = crash → stay alive for `make restart-tauri`
    rust.process.on("close", (code) => {
      if (code === 0 || code === null) {
        logCtrl("App window closed. Shutting down controller...");
        sseStatus(false, false);
        for (const w of libWatchers) w.kill();
        vite.kill();
        unwatchFile(SIGNAL_FILE);
        if (dashboardServer) dashboardServer.close();
        removeControlFile();
        try {
          unlinkSync(SIGNAL_FILE);
        } catch {
          /* ignore */
        }
        process.exit(0);
      } else {
        writeControlFile({
          state: "app-crashed",
          vite: vite.pid,
          rust: null,
        });
        sseStatus(true, false);
        logCtrl(
          `App crashed (code ${code}). Controller still alive — use 'make restart-tauri' to relaunch.`,
        );
      }
    });
  }

  await startRust();

  // ── 3. Signal Handling ─────────────────────────────────────────────────

  // ── 3a. Shared signal processor ──────────────────────────────────────
  const SIGNAL_FILE = join(PROJECT_ROOT, "tmp", "dev-signal");

  async function processSignal(signal) {
    if (signal === "start") {
      // Re-launch processes when controller is alive but processes are stopped
      if (!vite.running) {
        logCtrl("Starting Vite dev server...");
        vite.spawn(npmCmd, ["run", "dev"], { cwd: UI_DIR });
        const viteReady = await waitForPort(VITE_PORT, 30_000, false);
        if (!viteReady) {
          logError("ctrl", "Vite failed to start within 30s");
          return;
        }
        logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
        sseStatus(true, false);
      }
      if (!rust?.running) {
        await startRust();
      }
      logSuccess("All processes started.");
    } else if (signal === "restart-vite") {
      logCtrl("Restart Vite signal received...");
      sseStatus(false, rust?.running ?? false);
      vite.kill();

      await waitForPort(VITE_PORT, PORT_TIMEOUT_MS, true);

      logCtrl("Restarting Vite...");
      vite.spawn(npmCmd, ["run", "dev"], { cwd: UI_DIR });
      const viteReady = await waitForPort(VITE_PORT, 30_000, false);
      if (!viteReady) {
        logError("ctrl", "Vite failed to restart within 30s");
        return;
      }
      logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
      sseStatus(true, rust?.running ? true : false);
    } else if (signal === "restart-tauri") {
      logCtrl("Restart Tauri signal received — rebuilding app...");
      sseStatus(true, "building");
      rust?.kill();

      // Wait for the app process to fully exit and port to release
      await new Promise((r) => setTimeout(r, 1_000));

      await startRust();
      logSuccess("App restarted. Vite was kept alive.");
    } else if (signal === "restart") {
      logCtrl("Full restart signal received — restarting everything...");
      sseStatus(false, "building");
      rust?.kill();
      vite.kill();

      await waitForPort(VITE_PORT, PORT_TIMEOUT_MS, true);

      // Restart Vite
      logCtrl("Restarting Vite...");
      vite.spawn(npmCmd, ["run", "dev"], { cwd: UI_DIR });
      await waitForPort(VITE_PORT, 30_000, false);
      logSuccess(`Vite ready on http://localhost:${VITE_PORT}`);
      sseStatus(true, "building");

      await startRust();
      logSuccess("Full restart complete.");
    } else if (signal === "stop") {
      logCtrl("Stop signal received — shutting down...");
      sseStatus(false, false);
      rust?.kill();
      for (const w of libWatchers) w.kill();
      vite.kill();
      removeControlFile();
      try {
        unlinkSync(SIGNAL_FILE);
      } catch {
        /* ignore */
      }
      unwatchFile(SIGNAL_FILE);
      if (dashboardServer) dashboardServer.close();
      logSuccess("All processes stopped. Controller exiting.");
      process.exit(0);
    }
  }

  // Wire dashboard command buttons to the signal processor
  signalHandler = (cmd) => processSignal(cmd);

  // Watch for restart signal (written by `make restart-tauri`)
  let lastMtime = 0;

  watchFile(SIGNAL_FILE, { interval: 500 }, async (curr) => {
    if (curr.mtimeMs <= lastMtime) return;
    lastMtime = curr.mtimeMs;

    let signal;
    try {
      signal = readFileSync(SIGNAL_FILE, "utf-8").trim();
    } catch {
      return;
    }

    await processSignal(signal);
  });

  // Handle Ctrl+C / terminal close
  function shutdown() {
    logCtrl("Shutting down...");
    sseStatus(false, false);
    unwatchFile(SIGNAL_FILE);
    rust?.kill();
    for (const w of libWatchers) w.kill();
    vite.kill();
    if (dashboardServer) dashboardServer.close();
    removeControlFile();
    try {
      unlinkSync(SIGNAL_FILE);
    } catch {
      /* ignore */
    }
    process.exit(0);
  }

  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);
  if (IS_WINDOWS) {
    process.on("SIGHUP", shutdown);
  }

  logCtrl("Dev controller running. Press Ctrl+C to stop.");
  logCtrl(`Dashboard: http://localhost:${DASHBOARD_PORT}`);
  logCtrl("Use 'make restart-tauri' to rebuild the app (Vite stays alive).");
  logCtrl("Use 'make stop' to shut everything down.");
}

// ── Stop (graceful — signals the controller) ────────────────────────────────

async function stop() {
  const ctrl = readControlFile();

  if (ctrl) {
    try {
      process.kill(ctrl.pid, 0); // check alive
      logCtrl("Signalling controller to stop...");
      const dir = join(PROJECT_ROOT, "tmp");
      if (!existsSync(dir)) execSync(`mkdir -p "${dir}"`);
      writeFileSync(join(PROJECT_ROOT, "tmp", "dev-signal"), "stop");

      // Wait for controller to exit
      const deadline = Date.now() + 10_000;
      while (Date.now() < deadline) {
        try {
          process.kill(ctrl.pid, 0);
          await new Promise((r) => setTimeout(r, 500));
        } catch {
          logSuccess("Controller stopped.");
          return;
        }
      }
      logCtrl("Controller did not exit gracefully. Use 'make kill' to force.");
      return;
    } catch {
      // Controller is dead but control file exists
      removeControlFile();
    }
  }

  logCtrl("No controller running. Use 'make kill' to force-kill orphaned processes.");
}

// ── Kill (force — kills everything regardless of controller) ────────────────

async function kill() {
  await killAll();
}

// ── Restart (external command) ──────────────────────────────────────────────

async function restartTauri() {
  const ctrl = readControlFile();

  if (ctrl) {
    try {
      process.kill(ctrl.pid, 0); // check alive
      logCtrl("Signalling controller to restart Tauri app...");
      const dir = join(PROJECT_ROOT, "tmp");
      if (!existsSync(dir)) execSync(`mkdir -p "${dir}"`);
      writeFileSync(join(PROJECT_ROOT, "tmp", "dev-signal"), "restart-tauri");
      logSuccess(
        "Restart signal sent. Watch the controller output for progress.",
      );
      return;
    } catch {
      removeControlFile();
    }
  }

  // No controller running — start the full dev environment
  logCtrl("No controller running. Starting dev environment...");
  await dev();
}

// ── Status ──────────────────────────────────────────────────────────────────

function status() {
  const ctrl = readControlFile();
  if (!ctrl) {
    console.log("No dev controller running.");
    return;
  }

  let alive = false;
  try {
    process.kill(ctrl.pid, 0);
    alive = true;
  } catch {
    /* dead */
  }

  console.log(`Controller PID: ${ctrl.pid} (${alive ? "alive" : "dead"})`);
  console.log(`State: ${ctrl.state}`);
  if (ctrl.vite) console.log(`Vite PID: ${ctrl.vite}`);
  if (ctrl.rust) console.log(`Rust PID: ${ctrl.rust}`);
}

// ── Dev (spawn controller, wait for ready, exit) ─────────────────────────────

async function dev() {
  // Check if controller is already running
  const existing = readControlFile();
  if (existing) {
    try {
      process.kill(existing.pid, 0);
      logSuccess(
        `Dev environment already running (PID ${existing.pid}). Dashboard: http://localhost:${DASHBOARD_PORT}`,
      );
      return;
    } catch {
      removeControlFile();
    }
  }

  logCtrl("Starting dev environment...");

  // Spawn the controller as a detached process
  const nodeCmd = process.execPath;
  const child = spawn(nodeCmd, [fileURLToPath(import.meta.url), "start"], {
    cwd: PROJECT_ROOT,
    detached: true,
    stdio: "ignore",
    windowsHide: true, // controller runs hidden — dashboard is the output
    env: { ...process.env },
  });
  child.unref();

  logCtrl(`Controller spawned (PID ${child.pid}). Waiting for ready...`);

  // Poll control file until state is "running" (Vite + Tauri both up)
  const READY_TIMEOUT_MS = 120_000; // 2 minutes for full compile + start
  const deadline = Date.now() + READY_TIMEOUT_MS;

  while (Date.now() < deadline) {
    await new Promise((r) => setTimeout(r, POLL_INTERVAL_MS));

    const ctrl = readControlFile();
    if (!ctrl) continue;

    // Check if the controller process died
    try {
      process.kill(ctrl.pid, 0);
    } catch {
      logError("ctrl", "Controller process died during startup.");
      removeControlFile();
      process.exit(1);
    }

    if (ctrl.state === "running") {
      logSuccess("Dev environment ready.");
      logSuccess(`Dashboard: http://localhost:${DASHBOARD_PORT}`);
      return;
    }
  }

  logError("ctrl", "Timed out waiting for dev environment to become ready.");
  process.exit(1);
}

// ── CLI ─────────────────────────────────────────────────────────────────────

const command = process.argv[2];

switch (command) {
  case "dev":
    await dev();
    break;
  case "start":
    await start();
    break;
  case "stop":
    await stop();
    break;
  case "kill":
    await kill();
    break;
  case "restart-tauri":
    await restartTauri();
    break;
  case "restart-vite": {
    const ctrl2 = readControlFile();
    if (ctrl2) {
      try {
        process.kill(ctrl2.pid, 0);
        writeFileSync(join(PROJECT_ROOT, "tmp", "dev-signal"), "restart-vite");
        logSuccess("Vite restart signal sent.");
        break;
      } catch {
        removeControlFile();
        logError("ctrl", "No controller running. Use 'make dev' first.");
      }
    } else {
      logError("ctrl", "No controller running. Use 'make dev' first.");
    }
    break;
  }
  case "restart": {
    const ctrl = readControlFile();
    if (ctrl) {
      try {
        process.kill(ctrl.pid, 0);
        writeFileSync(join(PROJECT_ROOT, "tmp", "dev-signal"), "restart");
        logSuccess("Full restart signal sent.");
        break;
      } catch {
        removeControlFile();
      }
    }
    // No controller running — start the full dev environment
    logCtrl("No controller running. Starting dev environment...");
    await dev();
    break;
  }
  case "status":
    status();
    break;
  default:
    console.log("OrqaStudio Dev Controller");
    console.log("");
    console.log("Usage: node dev.mjs <command>");
    console.log("");
    console.log("Commands:");
    console.log("  dev            Start dev environment (spawn controller, wait for ready, exit)");
    console.log("  start          Start the dev controller (long-running, stays in foreground)");
    console.log("  stop           Stop the controller gracefully");
    console.log("  kill           Force-kill all OrqaStudio processes");
    console.log("  restart-tauri  Restart Tauri app only (Vite stays alive)");
    console.log("  restart-vite   Restart Vite dev server only");
    console.log("  restart        Restart Vite + Tauri (controller stays alive)");
    console.log("  status         Show process status");
    process.exit(1);
}
