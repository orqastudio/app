/**
 * Test builder for conversation Message objects.
 *
 * Produces valid messages matching the orqa-studio Message shape.
 * The @orqastudio/types dependency will be added later — for now,
 * the interface is defined locally.
 */

/** Minimal Message shape for test building. */
export interface Message {
	id: number;
	session_id: number;
	role: "user" | "assistant" | "system";
	content_type: string;
	content: string;
	tool_call_id: string | null;
	tool_name: string | null;
	tool_input: string | null;
	tool_is_error: boolean;
	turn_index: number;
	block_index: number;
	stream_status: string;
	input_tokens: number | null;
	output_tokens: number | null;
	created_at: string;
}

const defaults: Message = {
	id: 1,
	session_id: 1,
	role: "user",
	content_type: "text",
	content: "Hello",
	tool_call_id: null,
	tool_name: null,
	tool_input: null,
	tool_is_error: false,
	turn_index: 0,
	block_index: 0,
	stream_status: "complete",
	input_tokens: null,
	output_tokens: null,
	created_at: "2026-01-01T00:00:00Z",
};

/**
 * Create a test Message with sensible defaults.
 *
 * ```ts
 * const msg = createMessage({ role: "assistant", content: "Hi!" });
 * ```
 * @param overrides
 */
export function createMessage(overrides: Partial<Message> = {}): Message {
	return { ...defaults, ...overrides };
}
