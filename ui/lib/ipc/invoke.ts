import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { Channel } from "@tauri-apps/api/core";
import type { OrqaError } from "$lib/types/errors";
import type { StreamEvent } from "$lib/types/streaming";

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	try {
		return await tauriInvoke<T>(cmd, args);
	} catch (error) {
		if (typeof error === "string") {
			throw JSON.parse(error) as OrqaError;
		}
		throw error;
	}
}

export function createStreamChannel(onEvent: (event: StreamEvent) => void): Channel<StreamEvent> {
	const channel = new Channel<StreamEvent>();
	channel.onmessage = onEvent;
	return channel;
}
