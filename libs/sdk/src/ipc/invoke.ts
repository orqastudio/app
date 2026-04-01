import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import type { OrqaError, StreamEvent } from "@orqastudio/types";
import { logger } from "../logger.js";

const log = logger("ipc");

/**
 * Wraps Tauri's invoke with performance timing and structured logging.
 * Logs duration on success and error details on failure before re-throwing.
 */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	const start = performance.now();
	try {
		const result = await tauriInvoke<T>(cmd, args);
		const duration_ms = (performance.now() - start).toFixed(1);
		log.perf(`${cmd} (${duration_ms}ms)`);
		return result;
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

/** Extract a human-readable message from any error shape (Error, OrqaError, string, unknown). */
export function extractErrorMessage(err: unknown): string {
	if (err instanceof Error) return err.message;
	if (typeof err === "string") return err;
	if (typeof err === "object" && err !== null && "message" in err) {
		return String((err as OrqaError).message);
	}
	return String(err);
}

export function createStreamChannel(onEvent: (event: StreamEvent) => void): Channel<StreamEvent> {
	const channel = new Channel<StreamEvent>();
	channel.onmessage = onEvent;
	return channel;
}
