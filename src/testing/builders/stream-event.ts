/**
 * Test builder for streaming events.
 *
 * Produces StreamEvent objects matching the orqa-studio streaming shape.
 * The @orqastudio/types dependency will be added later — for now,
 * the interface is defined locally.
 */

/** Discriminated union of stream event types. */
export type StreamEventType =
	| "content_delta"
	| "thinking_delta"
	| "message_start"
	| "message_complete"
	| "tool_use_start"
	| "tool_use_delta"
	| "tool_result"
	| "error";

export interface StreamEvent {
	type: StreamEventType;
	data: Record<string, unknown>;
}

/**
 * Create a test StreamEvent.
 *
 * ```ts
 * const event = createStreamEvent("content_delta", { text: "Hello" });
 * const error = createStreamEvent("error", { message: "Rate limited" });
 * ```
 */
export function createStreamEvent(
	type: StreamEventType,
	data: Record<string, unknown> = {},
): StreamEvent {
	return { type, data };
}
