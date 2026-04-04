/**
 * Centralized logger for OrqaStudio.
 *
 * Provides structured logging with levels, source tags, and dual forwarding:
 * - Dev dashboard at localhost:10130/log (for live SSE display)
 * - Daemon event bus at localhost:10100/events (for persistence)
 *
 * Use this instead of bare `console.log` throughout the codebase.
 *
 * Usage:
 *   import { logger } from "@orqastudio/logger";
 *   const log = logger("navigation");
 *   log.info("Opened artifact", path);
 *   log.perf("loadContent", () => fetchContent(path));
 *
 * If either endpoint isn't running, the fire-and-forget request silently fails.
 */

export type LogLevel = "debug" | "info" | "warn" | "error" | "perf";

export interface LogEntry {
	readonly level: LogLevel;
	readonly source: string;
	readonly message: string;
	readonly timestamp: number;
	readonly data?: unknown;
}

export interface Logger {
	debug(message: string, ...data: unknown[]): void;
	info(message: string, ...data: unknown[]): void;
	warn(message: string, ...data: unknown[]): void;
	error(message: string, ...data: unknown[]): void;
	/** Log a performance measurement. Pass a function to auto-time it. */
	perf(label: string, fn?: () => unknown): void;
	/** Async perf measurement. */
	perfAsync<T>(label: string, fn: () => Promise<T>): Promise<T>;
}

type LogSubscriber = (entry: LogEntry) => void;

const DEV_LOG_URL = "http://localhost:10130/log";
// Daemon event bus ingest URL — matches ORQA_PORT_BASE default of 10100.
// This constant mirrors the port used by daemon/src/health.rs.
const DAEMON_EVENTS_URL = "http://localhost:10100/events";

// Immutable reassignment pattern — subscribers is replaced rather than mutated in place.
let subscribers: readonly LogSubscriber[] = [];

let minLevel: LogLevel = "info";

const LEVEL_ORDER: Record<LogLevel, number> = {
	debug: 0,
	info: 1,
	perf: 1,
	warn: 2,
	error: 3,
};

function shouldLog(level: LogLevel): boolean {
	return LEVEL_ORDER[level] >= LEVEL_ORDER[minLevel];
}

function forwardToDashboard(level: string, source: string, message: string): void {
	try {
		const body = JSON.stringify({ level, source, message: `[${source}] ${message}` });
		if (typeof navigator !== "undefined" && navigator.sendBeacon) {
			const blob = new Blob([body], { type: "application/json" });
			navigator.sendBeacon(DEV_LOG_URL, blob);
		} else if (typeof fetch !== "undefined") {
			void fetch(DEV_LOG_URL, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body,
				keepalive: true,
			}).catch(() => {});
		}
	} catch {
		// Never fail
	}
}

/**
 * Forward a log entry to the daemon's POST /events ingest endpoint.
 *
 * The daemon persists events in SQLite so they survive dashboard restarts.
 * The `source` field maps to `EventSource::Frontend` on the Rust side.
 * Fire-and-forget — silently fails when the daemon is not running.
 */
function forwardToDaemonBus(level: string, source: string, message: string): void {
	try {
		if (typeof fetch === "undefined") return;
		const body = JSON.stringify([{
			level,
			source: "frontend",
			category: source,
			message: `[${source}] ${message}`,
			timestamp: Date.now(),
		}]);
		void fetch(DAEMON_EVENTS_URL, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body,
			keepalive: true,
		}).catch(() => {});
	} catch {
		// Never fail
	}
}

function emit(entry: LogEntry): void {
	if (shouldLog(entry.level)) {
		const prefix = `[${entry.source}]`;
		switch (entry.level) {
			case "error":
				console.error(prefix, entry.message, entry.data ?? "");
				break;
			case "warn":
				console.warn(prefix, entry.message, entry.data ?? "");
				break;
			case "debug":
				console.debug(prefix, entry.message, entry.data ?? "");
				break;
			case "perf":
				console.log(`${prefix} ⏱ ${entry.message}`, entry.data ?? "");
				break;
			case "info":
				console.log(prefix, entry.message, entry.data ?? "");
				break;
			default: {
				// Exhaustiveness check — compile error if a new LogLevel is added without a case.
				const _exhaustive: never = entry.level;
				console.log(prefix, entry.message, entry.data ?? "");
			}
		}
	}

	forwardToDashboard(entry.level, entry.source, entry.message);
	forwardToDaemonBus(entry.level, entry.source, entry.message);

	subscribers.forEach((sub) => {
		try {
			sub(entry);
		} catch {
			// Don't let subscriber errors break logging
		}
	});
}

/**
 * Create a scoped logger for a module.
 *
 * @param source - Module name (e.g. "navigation", "artifact", "graph")
 */
export function logger(source: string): Logger {
	return {
		debug(message: string, ...data: unknown[]) {
			emit({ level: "debug", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
		},
		info(message: string, ...data: unknown[]) {
			emit({ level: "info", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
		},
		warn(message: string, ...data: unknown[]) {
			emit({ level: "warn", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
		},
		error(message: string, ...data: unknown[]) {
			emit({ level: "error", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
		},
		perf(label: string, fn?: () => unknown) {
			if (!fn) {
				emit({ level: "perf", source, message: label, timestamp: Date.now() });
				return;
			}
			const start = performance.now();
			fn();
			const ms = (performance.now() - start).toFixed(1);
			emit({ level: "perf", source, message: `${label} (${ms}ms)`, timestamp: Date.now() });
		},
		async perfAsync<T>(label: string, fn: () => Promise<T>): Promise<T> {
			const start = performance.now();
			const result = await fn();
			const ms = (performance.now() - start).toFixed(1);
			emit({ level: "perf", source, message: `${label} (${ms}ms)`, timestamp: Date.now() });
			return result;
		},
	};
}

/** Subscribe to all log entries (for in-app error display, telemetry, etc.) */
export function subscribeToLogs(fn: LogSubscriber): () => void {
	subscribers = [...subscribers, fn];
	return () => {
		subscribers = subscribers.filter((s) => s !== fn);
	};
}

/** Set the minimum log level for console output. */
export function setLogLevel(level: LogLevel): void {
	minLevel = level;
}

/**
 * Switch the console log level to "debug".
 *
 * Call this in dev builds or from the OrqaDev dashboard to see verbose output
 * in the browser console. Forwarding to the dashboard and daemon bus is
 * unaffected — those always send regardless of console level.
 */
export function initDevConsole(): void {
	minLevel = "debug";
}
