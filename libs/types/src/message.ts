export interface Message {
	readonly id: number;
	readonly session_id: number;
	readonly role: MessageRole;
	readonly content_type: ContentType;
	readonly content: string | null;
	readonly tool_call_id: string | null;
	readonly tool_name: string | null;
	readonly tool_input: string | null;
	readonly tool_is_error: boolean;
	readonly turn_index: number;
	readonly block_index: number;
	readonly stream_status: StreamStatus;
	readonly input_tokens: number | null;
	readonly output_tokens: number | null;
	readonly created_at: string;
}

export type MessageRole = "user" | "assistant" | "system";
export type ContentType = "text" | "tool_use" | "tool_result" | "thinking" | "image";
export type StreamStatus = "pending" | "complete" | "error";
export type MessageId = number;

export interface SearchResult {
	readonly message_id: number;
	readonly session_id: number;
	readonly session_title: string | null;
	readonly content: string;
	readonly highlighted: string;
	readonly rank: number;
}
