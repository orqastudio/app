import type { Message } from "$lib/types";
import type { StreamEvent } from "$lib/types/streaming";
import { invoke, createStreamChannel } from "$lib/ipc/invoke";
import { DEFAULT_MODEL } from "$lib/components/conversation/model-options";

export interface ToolCallState {
	toolCallId: string;
	toolName: string;
	input: string;
	output: string | null;
	isError: boolean;
	isComplete: boolean;
}

/** State for a pending tool approval — drives the approval dialog. */
export interface PendingApproval {
	toolCallId: string;
	toolName: string;
	/** Raw JSON string of tool parameters, for display. */
	input: string;
}

class ConversationStore {
	messages = $state<Message[]>([]);
	streamingContent = $state("");
	streamingThinking = $state("");
	isStreaming = $state(false);
	isLoading = $state(false);
	error = $state<string | null>(null);
	activeToolCalls = $state<Map<string, ToolCallState>>(new Map());
	selectedModel = $state<string>(DEFAULT_MODEL);
	/** Non-null when a write/execute tool is waiting for user approval. */
	pendingApproval = $state<PendingApproval | null>(null);

	private resolvedModel = $state<string | null>(null);
	private streamingMessageId = $state<number | null>(null);

	get currentModel(): string | null {
		return this.resolvedModel;
	}

	get hasMessages(): boolean {
		return this.messages.length > 0;
	}

	async loadMessages(sessionId: number): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.messages = await invoke<Message[]>("message_list", {
				sessionId,
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		} finally {
			this.isLoading = false;
		}
	}

	async sendMessage(sessionId: number, content: string): Promise<void> {
		this.error = null;
		this.streamingContent = "";
		this.streamingThinking = "";
		this.activeToolCalls = new Map();
		this.streamingMessageId = null;
		this.isStreaming = true;

		// Optimistically add the user message to the UI immediately
		const nextTurn = this.messages.length > 0
			? Math.max(...this.messages.map((m) => m.turn_index)) + 1
			: 0;
		const optimisticMessage: Message = {
			id: -Date.now(),
			session_id: sessionId,
			role: "user",
			content_type: "text",
			content,
			tool_call_id: null,
			tool_name: null,
			tool_input: null,
			tool_is_error: false,
			turn_index: nextTurn,
			block_index: 0,
			stream_status: "complete",
			input_tokens: null,
			output_tokens: null,
			created_at: new Date().toISOString(),
		};
		this.messages = [...this.messages, optimisticMessage];

		const channel = createStreamChannel((event: StreamEvent) => {
			this.handleStreamEvent(event);
		});

		try {
			await invoke("stream_send_message", {
				sessionId,
				content,
				model: this.selectedModel,
				onEvent: channel,
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
			this.isStreaming = false;
		}
	}

	async stopStreaming(sessionId: number): Promise<void> {
		try {
			await invoke("stream_stop", { sessionId });
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	clear() {
		this.messages = [];
		this.streamingContent = "";
		this.streamingThinking = "";
		this.isStreaming = false;
		this.isLoading = false;
		this.error = null;
		this.activeToolCalls = new Map();
		this.resolvedModel = null;
		this.streamingMessageId = null;
		this.selectedModel = DEFAULT_MODEL;
		this.pendingApproval = null;
	}

	/** Approve or deny the currently pending tool call, then invoke the backend. */
	async respondToApproval(approved: boolean): Promise<void> {
		const approval = this.pendingApproval;
		if (!approval) return;
		this.pendingApproval = null;
		try {
			await invoke("stream_tool_approval_respond", {
				toolCallId: approval.toolCallId,
				approved,
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	private handleStreamEvent(event: StreamEvent) {
		switch (event.type) {
			case "stream_start":
				this.isStreaming = true;
				this.streamingContent = "";
				this.streamingThinking = "";
				this.streamingMessageId = event.data.message_id;
				if (event.data.resolved_model) {
					this.resolvedModel = event.data.resolved_model;
				}
				break;

			case "text_delta":
				this.streamingContent += event.data.content;
				break;

			case "thinking_delta":
				this.streamingThinking += event.data.content;
				break;

			case "tool_use_start": {
				const newMap = new Map(this.activeToolCalls);
				newMap.set(event.data.tool_call_id, {
					toolCallId: event.data.tool_call_id,
					toolName: event.data.tool_name,
					input: "",
					output: null,
					isError: false,
					isComplete: false,
				});
				this.activeToolCalls = newMap;
				break;
			}

			case "tool_input_delta": {
				const toolCall = this.activeToolCalls.get(event.data.tool_call_id);
				if (toolCall) {
					const updatedMap = new Map(this.activeToolCalls);
					updatedMap.set(event.data.tool_call_id, {
						...toolCall,
						input: toolCall.input + event.data.content,
					});
					this.activeToolCalls = updatedMap;
				}
				break;
			}

			case "tool_result": {
				const existingCall = this.activeToolCalls.get(event.data.tool_call_id);
				if (existingCall) {
					const resultMap = new Map(this.activeToolCalls);
					resultMap.set(event.data.tool_call_id, {
						...existingCall,
						output: event.data.result,
						isError: event.data.is_error,
						isComplete: true,
					});
					this.activeToolCalls = resultMap;
				}
				break;
			}

			case "block_complete":
				// Block completed, no special handling needed
				break;

			case "turn_complete":
				this.isStreaming = false;
				this.streamingContent = "";
				this.streamingThinking = "";
				this.activeToolCalls = new Map();
				// Reload messages from DB to get the finalized state
				if (this.streamingMessageId !== null) {
					// Use the session_id from the first message, or rely on the caller
					const firstMsg = this.messages[0];
					if (firstMsg) {
						this.loadMessages(firstMsg.session_id);
					}
				}
				break;

			case "stream_error":
				this.error = event.data.message;
				this.isStreaming = false;
				break;

			case "stream_cancelled":
				this.isStreaming = false;
				break;

			case "tool_approval_request":
				// Surface the approval request so ConversationView can render the dialog.
				this.pendingApproval = {
					toolCallId: event.data.tool_call_id,
					toolName: event.data.tool_name,
					input: event.data.input,
				};
				break;
		}
	}
}

export const conversationStore = new ConversationStore();
