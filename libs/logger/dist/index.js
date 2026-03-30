/**
 * Centralized logger for OrqaStudio.
 *
 * Provides structured logging with levels, source tags, and dev controller
 * forwarding. Use this instead of bare `console.log` throughout the codebase.
 *
 * Usage:
 *   import { logger } from "@orqastudio/logger";
 *   const log = logger("navigation");
 *   log.info("Opened artifact", path);
 *   log.perf("loadContent", () => fetchContent(path));
 *
 * All output is forwarded to the dev dashboard via HTTP POST to localhost:10130/log.
 * If the dashboard isn't running, the fire-and-forget request silently fails.
 */
const DEV_LOG_URL = "http://localhost:10130/log";
const subscribers = [];
let minLevel = "debug";
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
            default:
                console.log(prefix, entry.message, entry.data ?? "");
        }
    }
    forwardToDashboard(entry.level, entry.source, entry.message);
    for (const sub of subscribers) {
        try {
            sub(entry);
        }
        catch {
            // Don't let subscriber errors break logging
        }
    }
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
    subscribers.push(fn);
    return () => {
        const idx = subscribers.indexOf(fn);
        if (idx >= 0)
            subscribers.splice(idx, 1);
    };
}
/** Set the minimum log level for console output. */
export function setLogLevel(level) {
    minLevel = level;
}
//# sourceMappingURL=index.js.map