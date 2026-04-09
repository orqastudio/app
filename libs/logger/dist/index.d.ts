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
export type LogLevel = "debug" | "info" | "warn" | "error" | "perf";
export interface LogEntry {
    readonly level: LogLevel;
    readonly source: string;
    readonly message: string;
    readonly timestamp: number;
    readonly data?: unknown;
    readonly stackFrames?: StackFrame[];
}
/** A single parsed frame from a JS Error.stack string. */
export interface StackFrame {
    readonly file: string;
    readonly line?: number;
    readonly col?: number;
    readonly function?: string;
    readonly raw?: string;
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
/** Logger endpoint configuration set by the app at startup via configureLogger(). */
interface LoggerConfig {
    /** URL of the dev dashboard log ingest endpoint (e.g. http://localhost:10130/log). */
    readonly devLogUrl: string;
    /** URL of the daemon event bus ingest endpoint (e.g. http://localhost:10100/events). */
    readonly daemonEventsUrl: string;
}
/**
 * Configure logger endpoint URLs from ports resolved by the app.
 *
 * Must be called once at app startup before any log entries are emitted.
 * The app derives URLs from infrastructure/ports.json via @orqastudio/constants.
 * Without this call, forwarding to dashboard and daemon silently no-ops.
 * @param config - Endpoint URLs for dashboard and daemon forwarding.
 */
export declare function configureLogger(config: LoggerConfig): void;
/**
 * Create a scoped logger for a module.
 * @param source - Module name (e.g. "navigation", "artifact", "graph")
 * @returns A Logger instance bound to the given source tag.
 */
export declare function logger(source: string): Logger;
/**
 * Subscribe to all log entries (for in-app error display, telemetry, etc.)
 * @param fn - Callback invoked for every emitted log entry.
 * @returns An unsubscribe function that removes the subscriber.
 */
export declare function subscribeToLogs(fn: LogSubscriber): () => void;
/**
 * Set the minimum log level for console output.
 * @param level - Minimum level; entries below this are suppressed from console.
 */
export declare function setLogLevel(level: LogLevel): void;
/**
 * Switch the console log level to "debug".
 *
 * Call this in dev builds or from the OrqaDev dashboard to see verbose output
 * in the browser console. Forwarding to the dashboard and daemon bus is
 * unaffected — those always send regardless of console level.
 */
export declare function initDevConsole(): void;
export {};
//# sourceMappingURL=index.d.ts.map