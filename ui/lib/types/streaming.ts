export type StreamEvent =
	| { type: "stream_start"; data: { message_id: number; resolved_model: string | null } }
	| { type: "text_delta"; data: { content: string } }
	| { type: "thinking_delta"; data: { content: string } }
	| { type: "tool_use_start"; data: { tool_call_id: string; tool_name: string } }
	| { type: "tool_input_delta"; data: { tool_call_id: string; content: string } }
	| {
			type: "tool_result";
			data: { tool_call_id: string; tool_name: string; result: string; is_error: boolean };
		}
	| { type: "block_complete"; data: { block_index: number; content_type: string } }
	| { type: "turn_complete"; data: { input_tokens: number; output_tokens: number } }
	| {
			type: "stream_error";
			data: { code: string; message: string; recoverable: boolean };
		}
	| { type: "stream_cancelled"; data: null }
	| {
			/** Emitted when a write or execute tool requests user approval before running. */
			type: "tool_approval_request";
			data: { tool_call_id: string; tool_name: string; input: string };
		};
