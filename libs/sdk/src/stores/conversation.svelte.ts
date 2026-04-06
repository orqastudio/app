import { SvelteMap, SvelteDate } from "svelte/reactivity";
import type { Message, StreamEvent } from "@orqastudio/types";
import { assertNever } from "@orqastudio/types";
import { invoke, createStreamChannel, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";

const log = logger("conversation");

export interface ToolCallState {
	readonly toolCallId: string;
	readonly toolName: string;
	readonly input: string;
	readonly output: string | null;
	readonly isError: boolean;
	readonly isComplete: boolean;
}

export type ContextEntry =
	| {
			readonly type: "system_prompt_sent";
			readonly customPrompt: string | null;
			readonly governancePrompt: string;
			readonly totalChars: number;
	  }
	| {
			readonly type: "context_injected";
			readonly messageCount: number;
			readonly totalChars: number;
			readonly messages: string;
	  };

/** State for a pending tool approval — drives the approval dialog. */
export interface PendingApproval {
	readonly toolCallId: string;
	readonly toolName: string;
	/** Raw JSON string of tool parameters, for display. */
	readonly input: string;
}

/**
 * Conversation status as a discriminated union.
 * - idle: no active operation
 * - loading: loading messages from the backend
 * - streaming: model is actively generating a response
 * - error: last operation failed (error message in ConversationStore.error)
 */
export type ConversationStatus = "idle" | "loading" | "streaming" | "error";

/** Default model used when no explicit model is configured. */
const DEFAULT_MODEL_FALLBACK = "auto";

/** Reactive state for the active conversation: messages, streaming, tool calls, and approvals. */
export class ConversationStore {
	/** All messages in the active session. */
	messages = $state<Message[]>([]);
	/** Accumulated text content while the model is streaming. */
	streamingContent = $state("");
	/** Accumulated thinking content while the model is streaming. */
	streamingThinking = $state("");
	/** Current conversation status — use this instead of inspecting isStreaming/isLoading separately. */
	status = $state<ConversationStatus>("idle");
	/**
	 * True while the model is actively streaming a response. Derived from status.
	 * @returns Whether the conversation is currently in the streaming state.
	 */
	get isStreaming(): boolean {
		return this.status === "streaming";
	}
	/**
	 * True while messages are being loaded from the backend. Derived from status.
	 * @returns Whether the conversation is currently in the loading state.
	 */
	get isLoading(): boolean {
		return this.status === "loading";
	}
	/** Last error message, or null if none. */
	error = $state<string | null>(null);
	/** Active tool call states, keyed by tool_call_id. */
	activeToolCalls = $state<SvelteMap<string, ToolCallState>>(new SvelteMap());
	/** Currently selected model identifier. */
	selectedModel = $state<string>(DEFAULT_MODEL_FALLBACK);
	/** Non-null when a write/execute tool is waiting for user approval. */
	pendingApproval = $state<PendingApproval | null>(null);
	/** Process compliance violations from the most recent turn. */
	processViolations = $state<Array<{ readonly check: string; readonly message: string }>>([]);
	/** Context entries sent to the model at the start of the most recent turn. */
	contextEntries = $state<ContextEntry[]>([]);
	/**
	 * Last auto-generated title update received from the backend. Components observe this
	 * to propagate the change to the session store without creating a cross-store dependency.
	 */
	lastTitleUpdate = $state<{ readonly sessionId: number; readonly title: string } | null>(null);

	private resolvedModel = $state<string | null>(null);
	private streamingMessageId = $state<number | null>(null);
	private defaultModel: string = DEFAULT_MODEL_FALLBACK;
	private streamStartTime: number | null = null;

	/**
	 * The resolved model name as reported by the backend, or null before the first stream.
	 * @returns Model identifier string, or null.
	 */
	get currentModel(): string | null {
		return this.resolvedModel;
	}

	/**
	 * True if there is at least one message in the active session.
	 * @returns True when messages array is non-empty.
	 */
	get hasMessages(): boolean {
		return this.messages.length > 0;
	}

	/**
	 * Set the default model used when the store is cleared.
	 * Call this during app initialization to configure the default model
	 * without hardcoding it in the SDK.
	 * @param model - Model identifier to use as the default.
	 */
	setDefaultModel(model: string): void {
		this.defaultModel = model;
		this.selectedModel = model;
	}

	/**
	 * Load all messages for the given session from the backend.
	 * @param sessionId - ID of the session whose messages to load.
	 */
	async loadMessages(sessionId: number): Promise<void> {
		this.status = "loading";
		this.error = null;
		try {
			this.messages = await invoke<Message[]>("message_list", {
				sessionId,
			});
			this.status = "idle";
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.status = "error";
		}
	}

	/**
	 * Send a user message and stream the assistant response.
	 * @param sessionId - ID of the session to send into.
	 * @param content - Text content of the user message.
	 */
	async sendMessage(sessionId: number, content: string): Promise<void> {
		this.error = null;
		this.streamingContent = "";
		this.streamingThinking = "";
		this.activeToolCalls = new SvelteMap();
		this.streamingMessageId = null;
		this.processViolations = [];
		this.contextEntries = [];
		this.status = "streaming";

		// Optimistically add the user message to the UI immediately
		const nextTurn =
			this.messages.length > 0 ? Math.max(...this.messages.map((m) => m.turn_index)) + 1 : 0;
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
			created_at: new SvelteDate().toISOString(),
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
			this.error = extractErrorMessage(err);
			this.status = "error";
		}
	}

	/**
	 * Cancel an active streaming response for the given session.
	 * @param sessionId - ID of the session to stop.
	 */
	async stopStreaming(sessionId: number): Promise<void> {
		try {
			await invoke("stream_stop", { sessionId });
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/** Reset all conversation state, preparing for a new session. */
	clear() {
		this.messages = [];
		this.streamingContent = "";
		this.streamingThinking = "";
		this.status = "idle";
		this.error = null;
		this.activeToolCalls = new SvelteMap();
		this.resolvedModel = null;
		this.streamingMessageId = null;
		this.selectedModel = this.defaultModel;
		this.pendingApproval = null;
		this.processViolations = [];
		this.contextEntries = [];
		this.lastTitleUpdate = null;
	}

	/**
	 * Send a one-shot message to the sidecar and collect the full response text.
	 * Does not persist to a session or update any store state. Use this when you
	 * need a quick inference result (e.g., AI suggestions in a dialog) without
	 * creating a visible conversation turn.
	 * @param content - Text content of the message to send.
	 * @returns The complete response text, or throws if the sidecar is unavailable.
	 */
	async oneShotMessage(content: string): Promise<string> {
		let collected = "";
		const channel = createStreamChannel((event: StreamEvent) => {
			// Only collect text deltas — no store state mutations for one-shot calls.
			if (event.type === "text_delta") {
				collected += event.data.content;
			}
		});

		await invoke("stream_send_message", {
			sessionId: -1,
			content,
			model: this.selectedModel,
			onEvent: channel,
		});
		return collected;
	}

	/**
	 * Approve or deny the currently pending tool call, then invoke the backend.
	 * @param approved - True to allow the tool call, false to deny it.
	 */
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
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Dispatch a single stream event to the appropriate store state update.
	 * Exhaustively handles all StreamEvent variants — adding a new variant without
	 * a case here will produce a compile error via assertNever.
	 * @param event - The stream event received from the sidecar.
	 */
	private handleStreamEvent(event: StreamEvent) {
		switch (event.type) {
			case "stream_start":
				this.status = "streaming";
				this.streamingContent = "";
				this.streamingThinking = "";
				this.streamingMessageId = event.data.message_id;
				this.streamStartTime = performance.now();
				// Note: contextEntries are NOT reset here — SystemPromptSent and
				// ContextInjected arrive BEFORE stream_start (emitted by Rust before
				// the sidecar is called). Resetting here would wipe them out.
				// They are reset in sendMessage() instead.
				if (event.data.resolved_model) {
					this.resolvedModel = event.data.resolved_model;
				}
				log.info(
					`stream_start: model=${event.data.resolved_model ?? "unknown"} message_id=${event.data.message_id}`,
				);
				break;

			case "text_delta":
				this.streamingContent += event.data.content;
				break;

			case "thinking_delta":
				this.streamingThinking += event.data.content;
				break;

			case "tool_use_start": {
				const newMap = new SvelteMap(this.activeToolCalls);
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
					const updatedMap = new SvelteMap(this.activeToolCalls);
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
					const resultMap = new SvelteMap(this.activeToolCalls);
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

			case "turn_complete": {
				const elapsed_ms =
					this.streamStartTime !== null
						? (performance.now() - this.streamStartTime).toFixed(1)
						: null;
				this.streamStartTime = null;
				log.info(`turn_complete: elapsed_ms=${elapsed_ms ?? "unknown"}`);
				this.status = "idle";
				this.streamingContent = "";
				this.streamingThinking = "";
				this.activeToolCalls = new SvelteMap();
				// Reload messages from DB to get the finalized state
				if (this.streamingMessageId !== null) {
					// Use the session_id from the first message, or rely on the caller
					const firstMsg = this.messages[0];
					if (firstMsg) {
						this.loadMessages(firstMsg.session_id);
					}
				}
				break;
			}

			case "stream_error":
				log.error(`stream_error: ${event.data.message}`);
				this.error = event.data.message;
				this.status = "error";
				this.streamStartTime = null;
				break;

			case "stream_cancelled":
				this.status = "idle";
				break;

			case "tool_approval_request":
				// Surface the approval request so ConversationView can render the dialog.
				this.pendingApproval = {
					toolCallId: event.data.tool_call_id,
					toolName: event.data.tool_name,
					input: event.data.input,
				};
				break;

			case "process_violation":
				this.processViolations = [
					...this.processViolations,
					{ check: event.data.check, message: event.data.message },
				];
				break;

			case "session_title_updated":
				this.lastTitleUpdate = {
					sessionId: event.data.session_id,
					title: event.data.title,
				};
				break;

			case "system_prompt_sent":
				this.contextEntries = [
					...this.contextEntries,
					{
						type: "system_prompt_sent",
						customPrompt: event.data.custom_prompt,
						governancePrompt: event.data.governance_prompt,
						totalChars: event.data.total_chars,
					},
				];
				break;

			case "context_injected":
				this.contextEntries = [
					...this.contextEntries,
					{
						type: "context_injected",
						messageCount: event.data.message_count,
						totalChars: event.data.total_chars,
						messages: event.data.messages,
					},
				];
				break;

			default:
				assertNever(event);
		}
	}
}
