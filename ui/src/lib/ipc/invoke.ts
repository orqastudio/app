import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import type { OrqaError } from "$lib/types/errors";
import type { StreamEvent } from "$lib/types/streaming";

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	try {
		return await tauriInvoke<T>(cmd, args);
	} catch (error) {
		if (typeof error === "string") {
			try {
				throw JSON.parse(error) as OrqaError;
			} catch (parseErr) {
				// If the error string isn't valid JSON (e.g. a raw Tauri framework error),
				// wrap it as a plain Error instead of letting the SyntaxError propagate.
				if (parseErr instanceof SyntaxError) {
					throw new Error(error);
				}
				throw parseErr;
			}
		}
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
