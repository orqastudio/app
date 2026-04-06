import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import type { OrqaError, StreamEvent } from "@orqastudio/types";
import { logger } from "../logger.js";

const log = logger("ipc");

/**
 * Generate a short correlation ID for tracing a single IPC call across the
 * Tauri boundary. Uses the Web Crypto API (available in all Tauri webviews).
 * Returns the first 16 hex characters of a UUID v4 — compact enough for log
 * fields while still having ~64 bits of entropy.
 * @returns A 16-character hex string correlation ID.
 */
function generateCorrelationId(): string {
	return crypto.randomUUID().replace(/-/g, "").slice(0, 16);
}

/**
 * Recursively deep-freeze an object so it cannot be mutated after crossing
 * the IPC boundary. Enforces immutability at the storage↔frontend edge.
 *
 * Returns the same reference (frozen in place) for zero-allocation overhead.
 * Primitives and null pass through unchanged.
 * @param obj - The value to freeze; non-objects pass through unchanged.
 * @returns The same reference, now deeply frozen.
 */
function deepFreeze<T>(obj: T): Readonly<T> {
	if (obj === null || obj === undefined || typeof obj !== "object") {
		return obj;
	}
	Object.freeze(obj);
	const values = Array.isArray(obj) ? obj : Object.values(obj as Record<string, unknown>);
	for (const value of values) {
		if (typeof value === "object" && value !== null && !Object.isFrozen(value)) {
			deepFreeze(value);
		}
	}
	return obj as Readonly<T>;
}

/**
 * Wraps Tauri's invoke with performance timing and structured logging.
 * Logs duration on success and error details on failure before re-throwing.
 * @param cmd - The Tauri command name to invoke.
 * @param args - Optional arguments to pass to the command.
 * @returns The command result, deeply frozen to enforce immutability.
 */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	const start = performance.now();
	// Inject a correlation ID so this frontend action can be linked to daemon
	// log events. The Tauri command receives it as `_correlation_id` and can
	// forward it to any downstream HTTP calls it makes to the daemon.
	const correlationId = generateCorrelationId();
	const argsWithId: Record<string, unknown> = { ...args, _correlation_id: correlationId };
	try {
		const result = await tauriInvoke<T>(cmd, argsWithId);
		const duration_ms = (performance.now() - start).toFixed(1);
		log.perf(`${cmd} (${duration_ms}ms)`);
		// Freeze at the IPC boundary: data from storage is immutable by default.
		// Stores that need mutable copies must explicitly spread/clone.
		return deepFreeze(result);
	} catch (error) {
		const duration_ms = (performance.now() - start).toFixed(1);
		if (typeof error === "string") {
			try {
				const parsed = JSON.parse(error) as OrqaError;
				log.error(`${cmd} failed after ${duration_ms}ms`, parsed.message ?? error);
				throw parsed;
			} catch (parseErr) {
				// If the error string isn't valid JSON (e.g. a raw Tauri framework error),
				// wrap it as a plain Error instead of letting the SyntaxError propagate.
				if (parseErr instanceof SyntaxError) {
					log.error(`${cmd} failed after ${duration_ms}ms`, error);
					throw new Error(error);
				}
				throw parseErr;
			}
		}
		const message = error instanceof Error ? error.message : String(error);
		log.error(`${cmd} failed after ${duration_ms}ms`, message);
		throw error;
	}
}

/**
 * Extract a human-readable message from any error shape (Error, OrqaError, string, unknown).
 * @param err - The thrown value, which may be an Error, OrqaError, string, or unknown.
 * @returns A string message suitable for display to the user.
 */
export function extractErrorMessage(err: unknown): string {
	if (err instanceof Error) return err.message;
	if (typeof err === "string") return err;
	if (typeof err === "object" && err !== null && "message" in err) {
		return String((err as OrqaError).message);
	}
	return String(err);
}

/**
 * Create a Tauri Channel wired to a stream event callback.
 * The channel forwards each incoming backend message to the provided handler.
 * @param onEvent - Callback invoked for each streaming event from the backend.
 * @returns A configured Tauri Channel ready to pass to an invoke call.
 */
export function createStreamChannel(onEvent: (event: StreamEvent) => void): Channel<StreamEvent> {
	const channel = new Channel<StreamEvent>();
	channel.onmessage = onEvent;
	return channel;
}
