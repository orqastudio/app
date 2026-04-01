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
  statSync,
  createReadStream,
  watchFile,
  unwatchFile,
} from "node:fs";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(process.cwd());
const CONTROL_FILE = join(PROJECT_ROOT, ".state", "dev-controller.json");
const SIGNAL_FILE = join(PROJECT_ROOT, ".state", "dev-signal");
const DASHBOARD_HTML = join(SCRIPT_DIR, "dev-dashboard.html");
const CONTROLLER_LOG = join(PROJECT_ROOT, ".state", "dev-controller.log");

// Dashboard port is ORQA_PORT_BASE + 30 (default 10130).
const PORT_BASE = parseInt(process.env.ORQA_PORT_BASE || "10100", 10) || 10100;
const DASHBOARD_PORT = PORT_BASE + 30;

// Daemon event bus ingest endpoint. External log sources POST batches here.
const DAEMON_EVENTS_URL = `http://127.0.0.1:${PORT_BASE}/events`;

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

/**
 * Forward a batch of events to the daemon's POST /events ingest endpoint.
 *
 * Each event is a plain object matching IngestEvent in daemon/src/health.rs:
 *   { level, source, category, message, timestamp }
 *
 * Fire-and-forget — failures are silently ignored so the dashboard never
 * crashes when the daemon is not running. Requires Node 18+ for global fetch.
 */
function forwardToDaemon(events) {
  if (typeof fetch === "undefined") return;
  fetch(DAEMON_EVENTS_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(events),
  }).catch(() => { /* daemon not running — ignore */ });
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

// ── Controller Log Tailing ──────────────────────────────────────────────────

// SOURCE_PREFIX matches lines like "[search] something happened" or "[app] ..."
const SOURCE_PREFIX_RE = /^\[([^\]]+)\]\s*(.*)/;

/** Classify a log line level from its text content. */
function classifyLevel(text) {
  const lower = text.toLowerCase();
  if (/\b(error|err!|panic|fatal)\b/.test(lower)) return "error";
  if (/\b(warn|warning|warn!)\b/.test(lower)) return "warn";
  return "info";
}

/** Parse a raw log line into { source, text, level } for SSE broadcast. */
function parseLogLine(line) {
  const trimmed = line.trimEnd();
  if (!trimmed) return null;

  const match = SOURCE_PREFIX_RE.exec(trimmed);
  if (match) {
    const source = match[1].toLowerCase();
    const text = match[2];
    return { source, text, level: classifyLevel(text) };
  }

  // No bracket prefix — emit as "ctrl" source
  return { source: "ctrl", text: trimmed, level: classifyLevel(trimmed) };
}

/**
 * Tail dev-controller.log and broadcast new lines as controller-log SSE events.
 * Re-opens from position 0 when file is truncated (new orqa dev run resets the log).
 */
function startLogTail() {
  // Track the current read position so we can detect truncation
  let readPosition = 0;
  // Incomplete line buffer between stream reads
  let lineBuffer = "";

  function openStreamFrom(position) {
    if (!existsSync(CONTROLLER_LOG)) return;

    const streamOpts = { start: position, encoding: "utf-8" };
    const stream = createReadStream(CONTROLLER_LOG, streamOpts);

    stream.on("data", (chunk) => {
      lineBuffer += chunk;
      const parts = lineBuffer.split("\n");
      // The last part may be an incomplete line — keep it in the buffer
      lineBuffer = parts.pop();

      const daemonBatch = [];
      for (const line of parts) {
        const parsed = parseLogLine(line);
        if (parsed) {
          sseBroadcast("controller-log", parsed);
          // Collect for daemon event bus forwarding.
          daemonBatch.push({
            level: parsed.level,
            source: "dev-controller",
            category: parsed.source,
            message: parsed.text,
            timestamp: Date.now(),
          });
        }
      }
      // Forward the batch to the daemon once per stream chunk to reduce
      // the number of HTTP requests when the log is replayed at startup.
      if (daemonBatch.length > 0) {
        forwardToDaemon(daemonBatch);
      }
    });

    stream.on("end", () => {
      // Update position to end of what we just consumed
      try {
        const stats = statSync(CONTROLLER_LOG);
        readPosition = stats.size;
      } catch { /* file disappeared */ }
    });

    stream.on("error", () => { /* file may not exist yet */ });
  }

  // Watch for new content or file truncation
  watchFile(CONTROLLER_LOG, { interval: 500 }, (curr, prev) => {
    if (curr.size < prev.size) {
      // File was truncated — new orqa dev run started
      readPosition = 0;
      lineBuffer = "";
      log("Log file truncated — resetting tail position");
      openStreamFrom(0);
    } else if (curr.size > readPosition) {
      // New content appended
      openStreamFrom(readPosition);
    }
  });

  // Read whatever is already in the log at startup
  if (existsSync(CONTROLLER_LOG)) {
    openStreamFrom(0);
  }
}

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

  // Frontend log forwarding (SDK logger POSTs here).
  // Each received entry is broadcast to dashboard SSE clients AND forwarded
  // to the daemon event bus so it is persisted in the SQLite store.
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
        // Forward to daemon event bus for persistence.
        forwardToDaemon([{
          level,
          source,
          category: source,
          message: text,
          timestamp: Date.now(),
        }]);
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

  // Begin tailing the controller log and forwarding lines as SSE events
  startLogTail();
});

server.on("error", (err) => {
  if (err.code === "EADDRINUSE") {
    console.error(`Port ${port} is already in use. Try --port <other>`);
  } else {
    console.error(`Server error: ${err.message}`);
  }
  process.exit(1);
});
