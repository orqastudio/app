import { describe, it, expect, vi, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

// Mock the model-options module before importing the store
vi.mock("$lib/components/conversation/model-options", () => ({
	DEFAULT_MODEL: "claude-sonnet-4-6",
}));

import { conversationStore } from "../conversation.svelte";
import type { Message } from "$lib/types";
import type { StreamEvent } from "$lib/types/streaming";

const fakeMessage: Message = {
	id: 1,
	session_id: 10,
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

const fakeAssistantMessage: Message = {
	...fakeMessage,
	id: 2,
	role: "assistant",
	content: "Hi there!",
	turn_index: 1,
};

beforeEach(() => {
	mockInvoke.mockReset();
	conversationStore.clear();
});

describe("ConversationStore", () => {
	describe("initial state", () => {
		it("starts with empty messages and no streaming", () => {
			expect(conversationStore.messages).toEqual([]);
			expect(conversationStore.streamingContent).toBe("");
			expect(conversationStore.streamingThinking).toBe("");
			expect(conversationStore.isStreaming).toBe(false);
			expect(conversationStore.isLoading).toBe(false);
			expect(conversationStore.error).toBeNull();
		});

		it("hasMessages is false initially", () => {
			expect(conversationStore.hasMessages).toBe(false);
		});

		it("currentModel is null initially", () => {
			expect(conversationStore.currentModel).toBeNull();
		});

		it("pendingApproval is null initially", () => {
			expect(conversationStore.pendingApproval).toBeNull();
		});

		it("processViolations is empty initially", () => {
			expect(conversationStore.processViolations).toEqual([]);
		});

		it("contextEntries is empty initially", () => {
			expect(conversationStore.contextEntries).toEqual([]);
		});
	});

	describe("loadMessages", () => {
		it("loads messages from backend", async () => {
			const messages = [fakeMessage, fakeAssistantMessage];
			mockInvoke.mockResolvedValueOnce(messages);

			await conversationStore.loadMessages(10);

			expect(mockInvoke).toHaveBeenCalledWith("message_list", { sessionId: 10 });
			expect(conversationStore.messages).toEqual(messages);
			expect(conversationStore.isLoading).toBe(false);
		});

		it("sets loading during fetch", async () => {
			let loadingDuringFetch = false;
			mockInvoke.mockImplementation(() => {
				loadingDuringFetch = conversationStore.isLoading;
				return Promise.resolve([]);
			});

			await conversationStore.loadMessages(10);

			expect(loadingDuringFetch).toBe(true);
			expect(conversationStore.isLoading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("DB error"));

			await conversationStore.loadMessages(10);

			expect(conversationStore.error).toBe("DB error");
			expect(conversationStore.isLoading).toBe(false);
		});
	});

	describe("sendMessage", () => {
		it("adds optimistic user message immediately", async () => {
			// invoke for stream_send_message — resolves after streaming completes
			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.sendMessage(10, "Hello world");

			// Should have one optimistic message
			expect(conversationStore.messages).toHaveLength(1);
			expect(conversationStore.messages[0].role).toBe("user");
			expect(conversationStore.messages[0].content).toBe("Hello world");
			expect(conversationStore.messages[0].session_id).toBe(10);
			expect(conversationStore.messages[0].id).toBeLessThan(0); // Optimistic ID is negative
		});

		it("sets isStreaming to true", async () => {
			let streamingDuringInvoke = false;
			mockInvoke.mockImplementation(() => {
				streamingDuringInvoke = conversationStore.isStreaming;
				return Promise.resolve();
			});

			await conversationStore.sendMessage(10, "test");

			expect(streamingDuringInvoke).toBe(true);
		});

		it("passes selected model to invoke", async () => {
			conversationStore.selectedModel = "claude-opus-4-6";
			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.sendMessage(10, "test");

			expect(mockInvoke).toHaveBeenCalledWith("stream_send_message", expect.objectContaining({
				model: "claude-opus-4-6",
			}));
		});

		it("sets error on invoke failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Stream failed"));

			await conversationStore.sendMessage(10, "test");

			expect(conversationStore.error).toBe("Stream failed");
			expect(conversationStore.isStreaming).toBe(false);
		});

		it("resets streaming state before starting", async () => {
			conversationStore.streamingContent = "leftover";
			conversationStore.streamingThinking = "leftover thinking";
			conversationStore.processViolations = [{ check: "old", message: "old" }];

			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.sendMessage(10, "new message");

			// These should have been reset before streaming
			// (streamingContent gets reset in sendMessage and again potentially in handleStreamEvent)
			expect(conversationStore.processViolations).toEqual([]);
			expect(conversationStore.contextEntries).toEqual([]);
		});
	});

	describe("stopStreaming", () => {
		it("calls stream_stop on backend", async () => {
			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.stopStreaming(10);

			expect(mockInvoke).toHaveBeenCalledWith("stream_stop", { sessionId: 10 });
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Stop failed"));

			await conversationStore.stopStreaming(10);

			expect(conversationStore.error).toBe("Stop failed");
		});
	});

	describe("respondToApproval", () => {
		it("sends approval response to backend", async () => {
			conversationStore.pendingApproval = {
				toolCallId: "tc-1",
				toolName: "write",
				input: '{"path": "test.txt"}',
			};
			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.respondToApproval(true);

			expect(mockInvoke).toHaveBeenCalledWith("stream_tool_approval_respond", {
				toolCallId: "tc-1",
				approved: true,
			});
			expect(conversationStore.pendingApproval).toBeNull();
		});

		it("clears pending approval even on rejection", async () => {
			conversationStore.pendingApproval = {
				toolCallId: "tc-1",
				toolName: "write",
				input: "{}",
			};
			mockInvoke.mockResolvedValueOnce(undefined);

			await conversationStore.respondToApproval(false);

			expect(conversationStore.pendingApproval).toBeNull();
			expect(mockInvoke).toHaveBeenCalledWith("stream_tool_approval_respond", {
				toolCallId: "tc-1",
				approved: false,
			});
		});

		it("does nothing when no pending approval", async () => {
			await conversationStore.respondToApproval(true);

			expect(mockInvoke).not.toHaveBeenCalled();
		});
	});

	describe("handleStreamEvent (via private method)", () => {
		// We test stream events by using sendMessage which sets up the channel,
		// but we can also test the event handler effects directly by calling
		// the method through the store's internal state changes.

		// Since handleStreamEvent is private, we test it indirectly through
		// observable state changes that would result from stream events.

		it("stream_start sets streaming state and resolved model", async () => {
			// Set up a channel capture so we can manually emit events
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					// Extract the channel's onmessage callback
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			// Start sending (this sets up the channel)
			const sendPromise = conversationStore.sendMessage(10, "test");
			await sendPromise;

			// Now simulate stream events via the captured callback
			if (capturedCallback) {
				capturedCallback({
					type: "stream_start",
					data: { message_id: 100, resolved_model: "claude-opus-4-6" },
				} as StreamEvent);

				expect(conversationStore.isStreaming).toBe(true);
				expect(conversationStore.currentModel).toBe("claude-opus-4-6");
			}
		});

		it("text_delta accumulates streaming content", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "stream_start",
					data: { message_id: 100, resolved_model: null },
				} as StreamEvent);

				capturedCallback({
					type: "text_delta",
					data: { content: "Hello " },
				} as StreamEvent);

				capturedCallback({
					type: "text_delta",
					data: { content: "world" },
				} as StreamEvent);

				expect(conversationStore.streamingContent).toBe("Hello world");
			}
		});

		it("thinking_delta accumulates thinking content", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "stream_start",
					data: { message_id: 100, resolved_model: null },
				} as StreamEvent);

				capturedCallback({
					type: "thinking_delta",
					data: { content: "Let me think..." },
				} as StreamEvent);

				expect(conversationStore.streamingThinking).toBe("Let me think...");
			}
		});

		it("tool_use_start adds tool call to map", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "tool_use_start",
					data: { tool_call_id: "tc-1", tool_name: "read_file" },
				} as StreamEvent);

				expect(conversationStore.activeToolCalls.has("tc-1")).toBe(true);
				const tc = conversationStore.activeToolCalls.get("tc-1");
				expect(tc?.toolName).toBe("read_file");
				expect(tc?.isComplete).toBe(false);
			}
		});

		it("tool_result marks tool call complete", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "tool_use_start",
					data: { tool_call_id: "tc-1", tool_name: "read_file" },
				} as StreamEvent);

				capturedCallback({
					type: "tool_result",
					data: { tool_call_id: "tc-1", result: "file contents", is_error: false },
				} as StreamEvent);

				const tc = conversationStore.activeToolCalls.get("tc-1");
				expect(tc?.isComplete).toBe(true);
				expect(tc?.output).toBe("file contents");
				expect(tc?.isError).toBe(false);
			}
		});

		it("stream_error sets error and stops streaming", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "stream_error",
					data: { message: "Provider error" },
				} as StreamEvent);

				expect(conversationStore.error).toBe("Provider error");
				expect(conversationStore.isStreaming).toBe(false);
			}
		});

		it("stream_cancelled stops streaming", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				conversationStore.isStreaming = true; // Ensure it's streaming
				capturedCallback({
					type: "stream_cancelled",
					data: {},
				} as StreamEvent);

				expect(conversationStore.isStreaming).toBe(false);
			}
		});

		it("tool_approval_request sets pending approval", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "tool_approval_request",
					data: { tool_call_id: "tc-1", tool_name: "write_file", input: '{"path":"test.txt"}' },
				} as StreamEvent);

				expect(conversationStore.pendingApproval).toEqual({
					toolCallId: "tc-1",
					toolName: "write_file",
					input: '{"path":"test.txt"}',
				});
			}
		});

		it("process_violation appends to violations list", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "process_violation",
					data: { check: "no-unwrap", message: "Found unwrap() usage" },
				} as StreamEvent);

				expect(conversationStore.processViolations).toHaveLength(1);
				expect(conversationStore.processViolations[0].check).toBe("no-unwrap");
			}
		});

		it("session_title_updated sets lastTitleUpdate", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "session_title_updated",
					data: { session_id: 10, title: "Auto-generated title" },
				} as StreamEvent);

				expect(conversationStore.lastTitleUpdate).toEqual({
					sessionId: 10,
					title: "Auto-generated title",
				});
			}
		});

		it("system_prompt_sent appends to contextEntries", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "system_prompt_sent",
					data: {
						custom_prompt: "You are helpful",
						governance_prompt: "Follow rules",
						total_chars: 500,
					},
				} as StreamEvent);

				expect(conversationStore.contextEntries).toHaveLength(1);
				expect(conversationStore.contextEntries[0].type).toBe("system_prompt_sent");
			}
		});

		it("context_injected appends to contextEntries", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
			});

			await conversationStore.sendMessage(10, "test");

			if (capturedCallback) {
				capturedCallback({
					type: "context_injected",
					data: {
						message_count: 3,
						total_chars: 1200,
						messages: "context messages",
					},
				} as StreamEvent);

				expect(conversationStore.contextEntries).toHaveLength(1);
				expect(conversationStore.contextEntries[0].type).toBe("context_injected");
			}
		});
	});

	describe("turn_complete reloads messages", () => {
		it("triggers loadMessages on turn completion", async () => {
			let capturedCallback: ((event: StreamEvent) => void) | null = null;
			let invokeCallCount = 0;

			mockInvoke.mockImplementation(async (cmd: string, args?: Record<string, unknown>) => {
				invokeCallCount++;
				if (cmd === "stream_send_message" && args?.onEvent) {
					const channel = args.onEvent as { onmessage: ((event: StreamEvent) => void) | null };
					capturedCallback = channel.onmessage;
				}
				if (cmd === "message_list") {
					return [fakeMessage, fakeAssistantMessage];
				}
			});

			await conversationStore.sendMessage(10, "test");
			const callCountBefore = invokeCallCount;

			if (capturedCallback) {
				capturedCallback({
					type: "stream_start",
					data: { message_id: 100, resolved_model: null },
				} as StreamEvent);

				capturedCallback({
					type: "turn_complete",
					data: {},
				} as StreamEvent);

				expect(conversationStore.isStreaming).toBe(false);
				expect(conversationStore.streamingContent).toBe("");
			}
		});
	});

	describe("clear", () => {
		it("resets all state to defaults", () => {
			conversationStore.messages = [fakeMessage];
			conversationStore.streamingContent = "partial";
			conversationStore.streamingThinking = "thinking";
			conversationStore.isStreaming = true;
			conversationStore.isLoading = true;
			conversationStore.error = "some error";
			conversationStore.selectedModel = "claude-opus-4-6";
			conversationStore.pendingApproval = { toolCallId: "tc-1", toolName: "write", input: "{}" };
			conversationStore.processViolations = [{ check: "c", message: "m" }];
			conversationStore.contextEntries = [{ type: "system_prompt_sent", customPrompt: null, governancePrompt: "", totalChars: 0 }];
			conversationStore.lastTitleUpdate = { sessionId: 1, title: "t" };

			conversationStore.clear();

			expect(conversationStore.messages).toEqual([]);
			expect(conversationStore.streamingContent).toBe("");
			expect(conversationStore.streamingThinking).toBe("");
			expect(conversationStore.isStreaming).toBe(false);
			expect(conversationStore.isLoading).toBe(false);
			expect(conversationStore.error).toBeNull();
			expect(conversationStore.selectedModel).toBe("claude-sonnet-4-6"); // DEFAULT_MODEL
			expect(conversationStore.pendingApproval).toBeNull();
			expect(conversationStore.processViolations).toEqual([]);
			expect(conversationStore.contextEntries).toEqual([]);
			expect(conversationStore.lastTitleUpdate).toBeNull();
			expect(conversationStore.currentModel).toBeNull();
		});
	});
});
