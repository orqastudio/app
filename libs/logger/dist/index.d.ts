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
export type LogLevel = "debug" | "info" | "warn" | "error" | "perf";
export interface LogEntry {
    level: LogLevel;
    source: string;
    message: string;
    timestamp: number;
    data?: unknown;
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
/**
 * Create a scoped logger for a module.
 *
 * @param source - Module name (e.g. "navigation", "artifact", "graph")
 */
export declare function logger(source: string): Logger;
/** Subscribe to all log entries (for in-app error display, telemetry, etc.) */
export declare function subscribeToLogs(fn: LogSubscriber): () => void;
/** Set the minimum log level for console output. */
export declare function setLogLevel(level: LogLevel): void;
export {};
//# sourceMappingURL=index.d.ts.map