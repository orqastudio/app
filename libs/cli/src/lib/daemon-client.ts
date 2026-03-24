/**
 * HTTP client for the orqa-validation daemon.
 *
 * Daemon runs at localhost:10258 and provides the canonical graph scanning,
 * query, and validation endpoints. Falls back to spawning the
 * `orqa-validation` binary when the daemon is unreachable.
 */

import { spawnSync } from "node:child_process";
import { findBinary } from "./validation-engine.js";

// ---------------------------------------------------------------------------
// Types — match Rust daemon response shapes
// ---------------------------------------------------------------------------

/** Mirrors the Rust `ArtifactRef` struct. */
export interface DaemonArtifactRef {
	target_id: string;
	field: string;
	source_id: string;
	relationship_type: string | null;
}

/** Mirrors the Rust `ArtifactNode` struct returned by POST /query. */
export interface DaemonArtifactNode {
	id: string;
	project?: string;
	path: string;
	artifact_type: string;
	title: string;
	description: string | null;
	status: string | null;
	priority: string | null;
	frontmatter: Record<string, unknown>;
	body?: string;
	references_out: DaemonArtifactRef[];
	references_in: DaemonArtifactRef[];
}

/** Response shape from GET /health. */
export interface DaemonHealthResponse {
	status: string;
	artifacts: number;
	rules: number;
}

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
 *
 * @param method  HTTP method (GET or POST)
 * @param path    Endpoint path (e.g. "/query")
 * @param body    JSON body for POST requests
 */
export async function callDaemonGraph<T>(
	method: "GET" | "POST",
	path: string,
	body?: unknown,
): Promise<T> {
	try {
		const options: RequestInit = {
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
		return (await res.json()) as T;
	} catch {
		return callBinaryFallback(method, path, body);
	}
}

/**
 * Check if the daemon is reachable by hitting GET /health.
 */
export async function isDaemonRunning(): Promise<boolean> {
	try {
		const res = await fetch(`${DAEMON_BASE}/health`, {
			method: "GET",
			signal: AbortSignal.timeout(1000),
		});
		return res.ok;
	} catch {
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
 */
function callBinaryFallback<T>(
	_method: string,
	path: string,
	body: unknown,
): T {
	const binary = findBinary(process.cwd());
	if (!binary) {
		throw new Error(
			"orqa-validation daemon is not running and binary not found. " +
			"Start the daemon with `orqa-validation daemon` or build the binary.",
		);
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

	return JSON.parse(result.stdout) as T;
}

/**
 * Build CLI args from the daemon endpoint path and body.
 */
function buildBinaryArgs(path: string, body: unknown): string[] {
	const b = (body ?? {}) as Record<string, unknown>;

	switch (path) {
		case "/query": {
			const args = ["query", "--json"];
			if (b.type) args.push("--type", String(b.type));
			if (b.status) args.push("--status", String(b.status));
			if (b.id) args.push("--id", String(b.id));
			if (b.search) args.push("--search", String(b.search));
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
