/**
 * Centralized logger for OrqaStudio.
 *
 * Provides structured logging with levels, source tags, and dev controller
 * forwarding. Use this instead of bare `console.log` throughout the codebase.
 *
 * Usage:
 *   import { logger } from "@orqastudio/sdk";
 *   const log = logger("navigation");
 *   log.info("Opened artifact", path);
 *   log.perf("loadContent", () => fetchContent(path));
 *
 * In dev mode, all output is forwarded to the dev dashboard via HTTP POST.
 * In production, only warn/error are retained for in-app error display.
 */

export type LogLevel = "debug" | "info" | "warn" | "error" | "perf";

export interface LogEntry {
	level: LogLevel;
	source: string;
	message: string;
	timestamp: number;
	data?: unknown;
}

type LogSubscriber = (entry: LogEntry) => void;

const DEV_LOG_URL = "http://localhost:3001/log";

/** Global log subscribers. */
const subscribers: LogSubscriber[] = [];

/** Whether we're in dev mode. Resolved once at module load. */
let isDev = false;
try {
	// Vite injects import.meta.env at build time — access dynamically to avoid
	// TypeScript errors in non-Vite contexts (NodeNext module resolution).
	const meta = import.meta as unknown as Record<string, unknown>;
	const env = meta["env"] as Record<string, unknown> | undefined;
	isDev = env?.["DEV"] === true;
} catch {
	// Not in a Vite context (e.g. Node.js)
}

/** Minimum level for console output. */
let minLevel: LogLevel = isDev ? "debug" : "warn";

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

function formatArgs(args: unknown[]): string {
	return args
		.map((a) => {
			if (typeof a === "string") return a;
			try {
				return JSON.stringify(a);
			} catch {
				return String(a);
			}
		})
		.join(" ");
}

function forwardToDashboard(level: string, source: string, message: string) {
	if (!isDev) return;
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

function emit(entry: LogEntry) {
	// Console output
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
			default:
				console.log(prefix, entry.message, entry.data ?? "");
		}
	}

	// Dashboard forwarding
	forwardToDashboard(entry.level, entry.source, entry.message);

	// Subscribers
	for (const sub of subscribers) {
		try {
			sub(entry);
		} catch {
			// Don't let subscriber errors break logging
		}
	}
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
	subscribers.push(fn);
	return () => {
		const idx = subscribers.indexOf(fn);
		if (idx >= 0) subscribers.splice(idx, 1);
	};
}

/** Set the minimum log level for console output. */
export function setLogLevel(level: LogLevel) {
	minLevel = level;
}
