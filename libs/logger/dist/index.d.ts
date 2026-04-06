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