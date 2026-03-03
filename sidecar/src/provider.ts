/**
 * Anthropic SDK integration for the Forge sidecar.
 *
 * Manages per-session message history and streams Claude API responses
 * back as SidecarResponse events over the NDJSON protocol.
 */

import Anthropic from '@anthropic-ai/sdk';
import type { MessageParam } from '@anthropic-ai/sdk/resources/messages';
import type { SidecarResponse, MessageSummary } from './protocol.js';

// ── Constants ──

const DEFAULT_MODEL = 'claude-sonnet-4-6';
const DEFAULT_MAX_TOKENS = 8192;
const SUMMARY_MAX_TOKENS = 1024;

// ── Session State ──

/** Per-session conversation history stored in memory. */
const sessionHistories = new Map<number, MessageParam[]>();

/** Per-session abort controllers for cancellation. */
const activeStreams = new Map<number, AbortController>();

/** Monotonically increasing message ID counter. */
let nextMessageId = 1;

// ── Client Initialization ──

let client: Anthropic | null = null;

/**
 * Get or create the Anthropic client.
 * Returns null if ANTHROPIC_API_KEY is not set.
 */
function getClient(): Anthropic | null {
    if (client) {
        return client;
    }

    const apiKey = process.env.ANTHROPIC_API_KEY;
    if (!apiKey) {
        return null;
    }

    client = new Anthropic({ apiKey });
    return client;
}

// ── Model Resolution ──

/**
 * Resolve the model string. If "auto" or null/undefined, use the default.
 */
function resolveModel(model: string | null): string {
    if (!model || model === 'auto') {
        return DEFAULT_MODEL;
    }
    return model;
}

// ── Streaming ──

type ResponseSender = (response: SidecarResponse) => void;

/**
 * Track which tool_call_id belongs to each content block index,
 * so we can associate input_json_delta events with the right tool call.
 */
interface BlockTracker {
    toolCallIds: Map<number, string>;
    toolNames: Map<number, string>;
}

/**
 * Stream a message to the Anthropic API and emit SidecarResponse events.
 *
 * Adds the user message to session history, calls the API with full history,
 * streams response events, then adds the assistant response to history.
 */
export async function streamMessage(
    sessionId: number,
    content: string,
    model: string | null,
    systemPrompt: string | null,
    sendResponse: ResponseSender,
): Promise<void> {
    const anthropic = getClient();
    if (!anthropic) {
        sendResponse({
            type: 'stream_error',
            code: 'auth_error',
            message: 'ANTHROPIC_API_KEY not set',
            recoverable: false,
        });
        return;
    }

    const resolvedModel = resolveModel(model);
    const messageId = nextMessageId++;

    // Get or create session history
    if (!sessionHistories.has(sessionId)) {
        sessionHistories.set(sessionId, []);
    }
    const history = sessionHistories.get(sessionId)!;

    // Add user message to history
    history.push({ role: 'user', content });

    // Create abort controller for cancellation
    const abortController = new AbortController();
    activeStreams.set(sessionId, abortController);

    try {
        // Emit stream_start
        sendResponse({
            type: 'stream_start',
            message_id: messageId,
            resolved_model: resolvedModel,
        });

        const stream = anthropic.messages.stream(
            {
                model: resolvedModel,
                max_tokens: DEFAULT_MAX_TOKENS,
                system: systemPrompt || undefined,
                messages: history,
            },
            { signal: abortController.signal },
        );

        let fullText = '';
        const blocks: BlockTracker = {
            toolCallIds: new Map(),
            toolNames: new Map(),
        };

        // Text deltas — emitted in real time
        stream.on('text', (text) => {
            fullText += text;
            sendResponse({ type: 'text_delta', content: text });
        });

        // Thinking deltas — emitted in real time
        stream.on('thinking', (thinkingDelta) => {
            sendResponse({ type: 'thinking_delta', content: thinkingDelta });
        });

        // Raw stream events — captures tool use starts, input deltas,
        // and block completions that the higher-level events do not expose
        stream.on('streamEvent', (event) => {
            if (event.type === 'content_block_start') {
                const block = event.content_block;
                if (block.type === 'tool_use') {
                    blocks.toolCallIds.set(event.index, block.id);
                    blocks.toolNames.set(event.index, block.name);
                    sendResponse({
                        type: 'tool_use_start',
                        tool_call_id: block.id,
                        tool_name: block.name,
                    });
                }
            } else if (event.type === 'content_block_delta') {
                if (event.delta.type === 'input_json_delta') {
                    const toolCallId = blocks.toolCallIds.get(event.index) ?? '';
                    sendResponse({
                        type: 'tool_input_delta',
                        tool_call_id: toolCallId,
                        content: event.delta.partial_json,
                    });
                }
            } else if (event.type === 'content_block_stop') {
                // Determine the content type from what we tracked
                let contentType = 'text';
                if (blocks.toolCallIds.has(event.index)) {
                    contentType = 'tool_use';
                }
                sendResponse({
                    type: 'block_complete',
                    block_index: event.index,
                    content_type: contentType,
                });
            }
        });

        // Wait for the stream to complete and get the final message
        const finalMessage = await stream.finalMessage();

        // Extract token usage from the final message
        const inputTokens = finalMessage.usage.input_tokens;
        const outputTokens = finalMessage.usage.output_tokens;

        // Add assistant response to history
        history.push({
            role: 'assistant',
            content: fullText || finalMessage.content
                .filter((b): b is Anthropic.TextBlock => b.type === 'text')
                .map((b) => b.text)
                .join(''),
        });

        // Emit turn_complete
        sendResponse({
            type: 'turn_complete',
            input_tokens: inputTokens,
            output_tokens: outputTokens,
        });
    } catch (error: unknown) {
        // Check if this was a cancellation
        if (abortController.signal.aborted) {
            sendResponse({ type: 'stream_cancelled' });
            return;
        }

        const errorInfo = classifyError(error);
        sendResponse({
            type: 'stream_error',
            code: errorInfo.code,
            message: errorInfo.message,
            recoverable: errorInfo.recoverable,
        });
    } finally {
        activeStreams.delete(sessionId);
    }
}

// ── Cancellation ──

/**
 * Cancel an active stream for the given session.
 * If no stream is active, sends stream_cancelled anyway (idempotent).
 */
export function cancelStream(
    sessionId: number,
    sendResponse: ResponseSender,
): void {
    const controller = activeStreams.get(sessionId);
    if (controller) {
        controller.abort();
        activeStreams.delete(sessionId);
        // The stream handler will emit stream_cancelled when it detects the abort
    } else {
        sendResponse({ type: 'stream_cancelled' });
    }
}

// ── Summary Generation ──

/**
 * Generate a summary of the given messages using a single non-streaming call.
 */
export async function generateSummary(
    sessionId: number,
    messages: MessageSummary[],
    sendResponse: ResponseSender,
): Promise<void> {
    const anthropic = getClient();
    if (!anthropic) {
        sendResponse({
            type: 'stream_error',
            code: 'auth_error',
            message: 'ANTHROPIC_API_KEY not set',
            recoverable: false,
        });
        return;
    }

    try {
        // Convert MessageSummary to API message format
        const apiMessages: MessageParam[] = messages.map((m) => ({
            role: m.role as 'user' | 'assistant',
            content: m.content,
        }));

        const response = await anthropic.messages.create({
            model: DEFAULT_MODEL,
            max_tokens: SUMMARY_MAX_TOKENS,
            system: 'Summarize the following conversation in 2-3 concise sentences. Focus on the key topics discussed and any decisions or outcomes reached.',
            messages: apiMessages,
        });

        let summary = '';
        for (const block of response.content) {
            if (block.type === 'text') {
                summary += block.text;
            }
        }

        sendResponse({
            type: 'summary_result',
            session_id: sessionId,
            summary,
        });
    } catch (error: unknown) {
        const errorInfo = classifyError(error);
        sendResponse({
            type: 'stream_error',
            code: errorInfo.code,
            message: errorInfo.message,
            recoverable: errorInfo.recoverable,
        });
    }
}

// ── Health Check ──

/**
 * Respond to a health check with the sidecar version.
 */
export function healthCheck(sendResponse: ResponseSender): void {
    sendResponse({
        type: 'health_ok',
        version: '0.1.0',
    });
}

// ── Error Classification ──

interface ErrorInfo {
    code: string;
    message: string;
    recoverable: boolean;
}

/**
 * Classify an error into a code, message, and recoverable flag.
 */
function classifyError(error: unknown): ErrorInfo {
    if (error instanceof Anthropic.APIError) {
        if (error.status === 401) {
            return {
                code: 'auth_error',
                message: 'Invalid API key',
                recoverable: false,
            };
        }
        if (error.status === 429) {
            return {
                code: 'rate_limit',
                message: error.message || 'Rate limit exceeded',
                recoverable: true,
            };
        }
        if (error.status === 529) {
            return {
                code: 'overloaded',
                message: error.message || 'API overloaded',
                recoverable: true,
            };
        }
        if (error.status !== undefined && error.status >= 500) {
            return {
                code: 'server_error',
                message: error.message || 'Server error',
                recoverable: true,
            };
        }
        return {
            code: 'api_error',
            message: error.message || 'API error',
            recoverable: false,
        };
    }

    if (error instanceof Error) {
        if (error.name === 'AbortError' || error.message.includes('aborted')) {
            return {
                code: 'cancelled',
                message: 'Request was cancelled',
                recoverable: false,
            };
        }
        return {
            code: 'unknown_error',
            message: error.message,
            recoverable: false,
        };
    }

    return {
        code: 'unknown_error',
        message: String(error),
        recoverable: false,
    };
}
