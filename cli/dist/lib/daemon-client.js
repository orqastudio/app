/**
 * HTTP client for the orqa-validation daemon.
 *
 * Daemon runs at localhost:9120 by default (port is ORQA_PORT_BASE, matched
 * from daemon/src/health.rs resolve_port()). Provides canonical graph
 * scanning, query, and validation endpoints. Falls back to spawning the
 * `orqa-validation` binary when the daemon is unreachable.
 */
import { spawnSync } from "node:child_process";
import { findBinary } from "./validation-engine.js";
// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------
import { getPort } from "./ports.js";
const DAEMON_PORT = getPort("daemon");
const DAEMON_BASE = `http://127.0.0.1:${DAEMON_PORT}`;
const TIMEOUT_MS = 5000;
// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------
/**
 * Call a daemon endpoint. Falls back to the `orqa-validation` binary if
 * the daemon is unreachable.
 * @param method - HTTP method (GET or POST)
 * @param path - Endpoint path (e.g. "/query")
 * @param body - JSON body for POST requests
 * @returns The parsed JSON response from the daemon or binary fallback.
 */
export async function callDaemonGraph(method, path, body) {
    try {
        const options = {
            method,
            headers: { "Content-Type": "application/json" },
            signal: AbortSignal.timeout(TIMEOUT_MS),
        };
        if (body !== undefined) {
            options.body = JSON.stringify(body);
        }
        const res = await fetch(`${DAEMON_BASE}${path}`, options);
        if (!res.ok) {
            const text = await res.text().catch(() => "");
            throw new Error(`daemon ${path} returned ${res.status}: ${text}`);
        }
        return (await res.json());
    }
    catch {
        return callBinaryFallback(method, path, body);
    }
}
/**
 * Check if the daemon is reachable by hitting GET /health.
 * @returns True if the daemon responds to a health check within 1 second.
 */
export async function isDaemonRunning() {
    try {
        const res = await fetch(`${DAEMON_BASE}/health`, {
            method: "GET",
            signal: AbortSignal.timeout(1000),
        });
        return res.ok;
    }
    catch {
        return false;
    }
}
// ---------------------------------------------------------------------------
// Binary fallback
// ---------------------------------------------------------------------------
/**
 * Falls back to spawning `orqa-validation` with a subcommand that matches
 * the daemon endpoint. This is a best-effort fallback — not all endpoints
 * have binary equivalents.
 * @param _method - HTTP method (unused in binary fallback).
 * @param path - Daemon endpoint path to map to a binary subcommand.
 * @param body - Request body to pass as CLI arguments.
 * @returns Parsed JSON output from the binary.
 */
function callBinaryFallback(_method, path, body) {
    const binary = findBinary(process.cwd());
    if (!binary) {
        throw new Error("orqa-validation daemon is not running and binary not found. " +
            "Start the daemon with `orqa-validation daemon` or build the binary.");
    }
    // Map daemon endpoints to CLI subcommands
    const args = buildBinaryArgs(path, body);
    const result = spawnSync(binary, args, {
        encoding: "utf-8",
        timeout: 30000,
        windowsHide: true,
    });
    if (result.error) {
        throw new Error(`orqa-validation fallback failed: ${result.error.message}`);
    }
    if (result.status !== 0) {
        const msg = result.stderr || result.stdout || "unknown error";
        throw new Error(`orqa-validation exited with code ${result.status}: ${msg}`);
    }
    if (!result.stdout) {
        throw new Error("orqa-validation produced no output");
    }
    return JSON.parse(result.stdout);
}
/**
 * Build CLI args from the daemon endpoint path and body.
 * @param path - The daemon endpoint path (e.g. "/query").
 * @param body - The request body to map to CLI arguments.
 * @returns Array of CLI arguments for the binary.
 */
function buildBinaryArgs(path, body) {
    const b = (body ?? {});
    switch (path) {
        case "/query": {
            const args = ["query", "--json"];
            if (b.type)
                args.push("--type", String(b.type));
            if (b.status)
                args.push("--status", String(b.status));
            if (b.id)
                args.push("--id", String(b.id));
            if (b.search)
                args.push("--search", String(b.search));
            return args;
        }
        case "/health":
            return ["health", "--json"];
        case "/validate":
            return ["validate", "--json"];
        default:
            throw new Error(`no binary fallback for endpoint: ${path}`);
    }
}
//# sourceMappingURL=daemon-client.js.map