#!/usr/bin/env node
// dev.mjs — OrqaStudio Debug Dashboard
//
// Diagnostic tool that provides a web dashboard for inspecting the running
// dev environment. Does NOT manage processes — `orqa dev` does that.
//
// Usage:
//   node dev.mjs                 Start the debug dashboard
//   node dev.mjs --port <port>   Use a custom port (default: 10130)
//
// The dashboard provides:
//   - Real-time process status (reads orqa dev's control file)
//   - Log aggregation from frontend (via POST /log from SDK logger)
//   - SSE stream for live updates
//   - Command buttons that send signals to the orqa dev controller
//
// Requires `orqa dev` to be running. Start it first.

import { createServer as createHttpServer } from "node:http";
import { platform } from "node:os";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  writeFileSync,
  readFileSync,
  existsSync,
} from "node:fs";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(process.cwd());
const DEV_ROOT = resolve(PROJECT_ROOT, "..");
const CONTROL_FILE = join(DEV_ROOT, "tmp", "dev-controller.json");
const SIGNAL_FILE = join(DEV_ROOT, "tmp", "dev-signal");
const DASHBOARD_HTML = join(SCRIPT_DIR, "dev-dashboard.html");

// Dashboard port is ORQA_PORT_BASE + 30 (default 10130). Matches ports.ts
// PORT_OFFSETS.dashboard. The daemon health endpoint uses ORQA_PORT_BASE
// directly (default 10100).
const DASHBOARD_PORT = 10130;

const COLOURS = {
  reset: "\x1b[0m",
  dim: "\x1b[2m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  cyan: "\x1b[36m",
};

function log(msg) {
  const ts = new Date().toLocaleTimeString();
  console.log(`${COLOURS.dim}${ts}${COLOURS.reset} ${COLOURS.cyan}[dashboard]${COLOURS.reset} ${msg}`);
}

// ── SSE Client Management ───────────────────────────────────────────────────

const sseClients = new Set();

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

// ── Status Polling ──────────────────────────────────────────────────────────

function readControlFile() {
  if (!existsSync(CONTROL_FILE)) return null;
  try {
    return JSON.parse(readFileSync(CONTROL_FILE, "utf-8"));
  } catch {
    return null;
  }
}

function pollStatus() {
  const ctrl = readControlFile();
  if (!ctrl) {
    sseBroadcast("status", { state: "not-running", message: "orqa dev not running" });
    return;
  }

  // Check if controller PID is alive
  let alive = false;
  try {
    process.kill(ctrl.pid, 0);
    alive = true;
  } catch { /* dead */ }

  if (!alive) {
    sseBroadcast("status", { state: "dead", message: `Controller PID ${ctrl.pid} is dead (stale control file)` });
    return;
  }

  sseBroadcast("status", {
    state: ctrl.state || "unknown",
    pid: ctrl.pid,
    vite: ctrl.vite,
    rust: ctrl.rust,
    search: ctrl.search,
    mcp: ctrl.mcp,
    lsp: ctrl.lsp,
  });
}

// Poll every 2 seconds
setInterval(pollStatus, 2000);

// ── Command Execution ───────────────────────────────────────────────────────

import { execFile } from "node:child_process";

/** Run an orqa CLI command as a subprocess. Returns a promise with stdout. */
function runOrqaCommand(args) {
  return new Promise((resolve, reject) => {
    execFile("orqa", args, { timeout: 30000 }, (err, stdout, stderr) => {
      if (err) {
        reject(new Error(stderr || err.message));
      } else {
        resolve(stdout.trim());
      }
    });
  });
}

// ── HTTP Server ─────────────────────────────────────────────────────────────

const server = createHttpServer((req, res) => {
  res.setHeader("Access-Control-Allow-Origin", "*");

  // Dashboard HTML
  if (req.method === "GET" && req.url === "/") {
    try {
      const html = readFileSync(DASHBOARD_HTML, "utf-8");
      res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });
      res.end(html);
    } catch (err) {
      res.writeHead(500, { "Content-Type": "text/plain" });
      res.end(`Dashboard HTML not found: ${err.message}`);
    }
    return;
  }

  // SSE endpoint
  if (req.method === "GET" && req.url === "/events") {
    res.writeHead(200, {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    });
    res.write(":\n\n");
    sseClients.add(res);
    req.on("close", () => sseClients.delete(res));

    // Send initial status immediately
    pollStatus();
    return;
  }

  // Frontend log forwarding (SDK logger POSTs here)
  if (req.method === "POST" && req.url === "/log") {
    let body = "";
    req.on("data", (chunk) => { body += chunk; });
    req.on("end", () => {
      try {
        const data = JSON.parse(body);
        const source = data.source || "frontend";
        const level = data.level || "info";
        const text = data.message || "";
        sseBroadcast("log", { source, text, level, error: level === "error" });
      } catch { /* malformed — ignore */ }
      res.writeHead(204);
      res.end();
    });
    return;
  }

  if (req.method === "OPTIONS" && req.url === "/log") {
    res.setHeader("Access-Control-Allow-Methods", "POST, OPTIONS");
    res.setHeader("Access-Control-Allow-Headers", "Content-Type");
    res.writeHead(204);
    res.end();
    return;
  }

  // Command endpoints — trigger orqa CLI commands via subprocess
  if (req.method === "POST" && req.url?.startsWith("/command/")) {
    const cmd = req.url.replace("/command/", "");
    const commandMap = {
      "restart":         ["dev", "restart"],
      "restart-daemon":  ["dev", "restart", "daemon"],
      "restart-frontend":["dev", "restart", "frontend"],
      "restart-app":     ["dev", "restart", "app"],
      "restart-search":  ["dev", "restart", "search"],
      "restart-mcp":     ["dev", "restart", "mcp"],
      "restart-lsp":     ["dev", "restart", "lsp"],
      "stop":            ["dev", "stop"],
      "kill":            ["dev", "kill"],
      "status":          ["dev", "status"],
    };
    const args = commandMap[cmd];
    if (args) {
      sseBroadcast("log", { source: "dashboard", text: `Running: orqa ${args.join(" ")}`, level: "info" });
      runOrqaCommand(args)
        .then((output) => {
          sseBroadcast("log", { source: "dashboard", text: output, level: "info" });
          res.writeHead(200, { "Content-Type": "application/json" });
          res.end(JSON.stringify({ message: `orqa ${args.join(" ")} completed`, output }));
        })
        .catch((err) => {
          sseBroadcast("log", { source: "dashboard", text: err.message, level: "error", error: true });
          res.writeHead(500, { "Content-Type": "application/json" });
          res.end(JSON.stringify({ message: `orqa ${args.join(" ")} failed: ${err.message}` }));
        });
    } else {
      res.writeHead(400, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ message: `Unknown command: ${cmd}` }));
    }
    return;
  }

  // Status API (JSON)
  if (req.method === "GET" && req.url === "/status") {
    const ctrl = readControlFile();
    res.writeHead(200, { "Content-Type": "application/json" });
    res.end(JSON.stringify(ctrl || { state: "not-running" }));
    return;
  }

  res.writeHead(404, { "Content-Type": "text/plain" });
  res.end("Not found");
});

// ── Startup ─────────────────────────────────────────────────────────────────

// Parse --port argument
let port = DASHBOARD_PORT;
const portIdx = process.argv.indexOf("--port");
if (portIdx !== -1 && process.argv[portIdx + 1]) {
  const p = parseInt(process.argv[portIdx + 1], 10);
  if (!Number.isNaN(p)) port = p;
}

// Check if orqa dev is running
const ctrl = readControlFile();
if (!ctrl) {
  console.log(`${COLOURS.yellow}Warning: orqa dev does not appear to be running.${COLOURS.reset}`);
  console.log(`Start it first: orqa dev`);
  console.log(`Dashboard will show status once orqa dev starts.\n`);
}

server.listen(port, "127.0.0.1", () => {
  log(`Debug dashboard: http://localhost:${port}`);
  log("Watching orqa dev status every 2s");
  log("Press Ctrl+C to stop the dashboard");
});

server.on("error", (err) => {
  if (err.code === "EADDRINUSE") {
    console.error(`Port ${port} is already in use. Try --port <other>`);
  } else {
    console.error(`Server error: ${err.message}`);
  }
  process.exit(1);
});
