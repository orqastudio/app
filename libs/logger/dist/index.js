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
const DEV_LOG_URL = "http://localhost:10130/log";
// Daemon event bus ingest URL — matches ORQA_PORT_BASE default of 10100.
// This constant mirrors the port used by daemon/src/health.rs.
const DAEMON_EVENTS_URL = "http://localhost:10100/events";
// Immutable reassignment pattern — subscribers is replaced rather than mutated in place.
let subscribers = [];
let minLevel = "info";
const LEVEL_ORDER = {
    debug: 0,
    info: 1,
    perf: 1,
    warn: 2,
    error: 3,
};
function shouldLog(level) {
    return LEVEL_ORDER[level] >= LEVEL_ORDER[minLevel];
}
function forwardToDashboard(level, source, message) {
    try {
        const body = JSON.stringify({ level, source, message: `[${source}] ${message}` });
        if (typeof navigator !== "undefined" && navigator.sendBeacon) {
            const blob = new Blob([body], { type: "application/json" });
            navigator.sendBeacon(DEV_LOG_URL, blob);
        }
        else if (typeof fetch !== "undefined") {
            void fetch(DEV_LOG_URL, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body,
                keepalive: true,
            }).catch(() => { });
        }
    }
    catch {
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
function forwardToDaemonBus(level, source, message) {
    try {
        if (typeof fetch === "undefined")
            return;
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
        }).catch(() => { });
    }
    catch {
        // Never fail
    }
}
function emit(entry) {
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
                const _exhaustive = entry.level;
                console.log(prefix, entry.message, entry.data ?? "");
            }
        }
    }
    forwardToDashboard(entry.level, entry.source, entry.message);
    forwardToDaemonBus(entry.level, entry.source, entry.message);
    subscribers.forEach((sub) => {
        try {
            sub(entry);
        }
        catch {
            // Don't let subscriber errors break logging
        }
    });
}
/**
 * Create a scoped logger for a module.
 *
 * @param source - Module name (e.g. "navigation", "artifact", "graph")
 */
export function logger(source) {
    return {
        debug(message, ...data) {
            emit({ level: "debug", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
        },
        info(message, ...data) {
            emit({ level: "info", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
        },
        warn(message, ...data) {
            emit({ level: "warn", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
        },
        error(message, ...data) {
            emit({ level: "error", source, message, timestamp: Date.now(), data: data.length ? data : undefined });
        },
        perf(label, fn) {
            if (!fn) {
                emit({ level: "perf", source, message: label, timestamp: Date.now() });
                return;
            }
            const start = performance.now();
            fn();
            const ms = (performance.now() - start).toFixed(1);
            emit({ level: "perf", source, message: `${label} (${ms}ms)`, timestamp: Date.now() });
        },
        async perfAsync(label, fn) {
            const start = performance.now();
            const result = await fn();
            const ms = (performance.now() - start).toFixed(1);
            emit({ level: "perf", source, message: `${label} (${ms}ms)`, timestamp: Date.now() });
            return result;
        },
    };
}
/** Subscribe to all log entries (for in-app error display, telemetry, etc.) */
export function subscribeToLogs(fn) {
    subscribers = [...subscribers, fn];
    return () => {
        subscribers = subscribers.filter((s) => s !== fn);
    };
}
/** Set the minimum log level for console output. */
export function setLogLevel(level) {
    minLevel = level;
}
/**
 * Switch the console log level to "debug".
 *
 * Call this in dev builds or from the OrqaDev dashboard to see verbose output
 * in the browser console. Forwarding to the dashboard and daemon bus is
 * unaffected — those always send regardless of console level.
 */
export function initDevConsole() {
    minLevel = "debug";
}
//# sourceMappingURL=index.js.map