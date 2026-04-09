/**
 * Centralized logger for OrqaStudio.
 *
 * Provides structured logging with levels, source tags, and dual forwarding:
 * - Dev dashboard /log endpoint (for live SSE display)
 * - Daemon event bus /events endpoint (for persistence)
 *
 * Endpoint URLs are not hardcoded. The app must call configureLogger() at
 * startup with the URLs derived from infrastructure/ports.json via
 * @orqastudio/constants. This keeps the logger library port-agnostic.
 *
 * Usage:
 *   import { logger, configureLogger } from "@orqastudio/logger";
 *   import { getPort } from "@orqastudio/constants";
 *   configureLogger({
 *     devLogUrl: `http://localhost:${getPort("dashboard")}/log`,
 *     daemonEventsUrl: `http://localhost:${getPort("daemon")}/events`,
 *   });
 *   const log = logger("navigation");
 *   log.info("Opened artifact", path);
 *   log.perf("loadContent", () => fetchContent(path));
 *
 * If either endpoint isn't running, the fire-and-forget request silently fails.
 */
// Endpoint URLs are set by the app at startup. Null before configureLogger() is called —
// forwarding silently no-ops when unconfigured (safe during SSR or unit tests).
let _devLogUrl = null;
let _daemonEventsUrl = null;
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
/**
 * Parse a JS Error.stack string into StackFrame objects, skipping internal frames.
 * Handles both Chrome format (`at fn (file:line:col)`) and Firefox format (`fn@file:line:col`).
 * @param stack - The raw stack trace string from Error().stack.
 * @param skipCount - Number of internal frames to skip from the top.
 * @returns Parsed stack frames with file, line, col, function name, capped at 5 frames.
 */
function parseCallStack(stack, skipCount) {
    if (!stack)
        return [];
    const lines = stack.split("\n").slice(1); // Remove the "Error" header line
    const frames = [];
    let skipped = 0;
    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed)
            continue;
        // Skip the specified number of internal logger frames.
        if (skipped < skipCount) {
            skipped++;
            continue;
        }
        // Chrome/V8 format: "    at FunctionName (file:line:col)" or "    at file:line:col"
        const chromeMatch = trimmed.match(/^at\s+(?:([\w$./<>[\] ]+?)\s+\()?(.+):(\d+):(\d+)\)?$/);
        if (chromeMatch) {
            const [, fnName, file, lineStr, colStr] = chromeMatch;
            frames.push({
                file: file ?? "",
                line: lineStr !== undefined ? Number(lineStr) : undefined,
                col: colStr !== undefined ? Number(colStr) : undefined,
                function: fnName?.trim() || undefined,
                raw: trimmed,
            });
            if (frames.length >= 5)
                break;
            continue;
        }
        // Firefox/Safari format: "functionName@file:line:col" or "@file:line:col"
        const firefoxMatch = trimmed.match(/^([\w$./<>[\]]*)?@(.+):(\d+):(\d+)$/);
        if (firefoxMatch) {
            const [, fnName, file, lineStr, colStr] = firefoxMatch;
            frames.push({
                file: file ?? "",
                line: lineStr !== undefined ? Number(lineStr) : undefined,
                col: colStr !== undefined ? Number(colStr) : undefined,
                function: fnName || undefined,
                raw: trimmed,
            });
            if (frames.length >= 5)
                break;
            continue;
        }
        // Unrecognised frame — include as raw only.
        frames.push({ file: "", raw: trimmed });
        if (frames.length >= 5)
            break;
    }
    return frames;
}
/**
 * Configure logger endpoint URLs from ports resolved by the app.
 *
 * Must be called once at app startup before any log entries are emitted.
 * The app derives URLs from infrastructure/ports.json via @orqastudio/constants.
 * Without this call, forwarding to dashboard and daemon silently no-ops.
 * @param config - Endpoint URLs for dashboard and daemon forwarding.
 */
export function configureLogger(config) {
    _devLogUrl = config.devLogUrl;
    _daemonEventsUrl = config.daemonEventsUrl;
}
function forwardToDashboard(level, source, message) {
    if (!_devLogUrl)
        return;
    try {
        const url = _devLogUrl;
        const body = JSON.stringify({ level, source, message: `[${source}] ${message}` });
        if (typeof navigator !== "undefined" && navigator.sendBeacon) {
            const blob = new Blob([body], { type: "application/json" });
            navigator.sendBeacon(url, blob);
        }
        else if (typeof fetch !== "undefined") {
            void fetch(url, {
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
 * @param level - Log severity level string.
 * @param source - Module name that produced the log entry.
 * @param message - Human-readable log message text.
 * @param stackFrames - Optional parsed call stack frames (warn/error only).
 */
function forwardToDaemonBus(level, source, message, stackFrames) {
    if (!_daemonEventsUrl)
        return;
    try {
        if (typeof fetch === "undefined")
            return;
        const url = _daemonEventsUrl;
        const event = {
            level,
            source: "frontend",
            category: source,
            message: `[${source}] ${message}`,
            timestamp: Date.now(),
        };
        if (stackFrames !== undefined && stackFrames.length > 0) {
            event.stack_frames = stackFrames;
        }
        const body = JSON.stringify([event]);
        void fetch(url, {
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
                const exhaustive = entry.level;
                void exhaustive;
                console.log(prefix, entry.message, entry.data ?? "");
            }
        }
    }
    forwardToDashboard(entry.level, entry.source, entry.message);
    forwardToDaemonBus(entry.level, entry.source, entry.message, entry.stackFrames);
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
 * @param source - Module name (e.g. "navigation", "artifact", "graph")
 * @returns A Logger instance bound to the given source tag.
 */
export function logger(source) {
    return {
        debug(message, ...data) {
            emit({
                level: "debug",
                source,
                message,
                timestamp: Date.now(),
                data: data.length ? data : undefined,
            });
        },
        info(message, ...data) {
            emit({
                level: "info",
                source,
                message,
                timestamp: Date.now(),
                data: data.length ? data : undefined,
            });
        },
        warn(message, ...data) {
            // Capture stack at call site. skipCount=1 skips this warn() frame so
            // frame[0] is the actual caller outside the logger.
            const stackFrames = parseCallStack(new Error().stack, 1);
            emit({
                level: "warn",
                source,
                message,
                timestamp: Date.now(),
                data: data.length ? data : undefined,
                stackFrames: stackFrames.length > 0 ? stackFrames : undefined,
            });
        },
        error(message, ...data) {
            // Capture stack at call site. skipCount=1 skips this error() frame so
            // frame[0] is the actual caller outside the logger.
            const stackFrames = parseCallStack(new Error().stack, 1);
            emit({
                level: "error",
                source,
                message,
                timestamp: Date.now(),
                data: data.length ? data : undefined,
                stackFrames: stackFrames.length > 0 ? stackFrames : undefined,
            });
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
/**
 * Subscribe to all log entries (for in-app error display, telemetry, etc.)
 * @param fn - Callback invoked for every emitted log entry.
 * @returns An unsubscribe function that removes the subscriber.
 */
export function subscribeToLogs(fn) {
    subscribers = [...subscribers, fn];
    return () => {
        subscribers = subscribers.filter((s) => s !== fn);
    };
}
/**
 * Set the minimum log level for console output.
 * @param level - Minimum level; entries below this are suppressed from console.
 */
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