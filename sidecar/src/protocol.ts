/**
 * NDJSON protocol types matching the Rust sidecar types.
 *
 * These types mirror src-tauri/src/sidecar/types.rs exactly.
 * The sidecar reads SidecarRequest from stdin and writes
 * SidecarResponse to stdout, one JSON object per line.
 */

// ── Request Types ──

export interface SendMessageRequest {
    type: 'send_message';
    session_id: number;
    content: string;
    model: string | null;
    system_prompt: string | null;
}

export interface CancelStreamRequest {
    type: 'cancel_stream';
    session_id: number;
}

export interface HealthCheckRequest {
    type: 'health_check';
}

export interface GenerateSummaryRequest {
    type: 'generate_summary';
    session_id: number;
    messages: MessageSummary[];
}

export interface MessageSummary {
    role: string;
    content: string;
}

export type SidecarRequest =
    | SendMessageRequest
    | CancelStreamRequest
    | HealthCheckRequest
    | GenerateSummaryRequest;

// ── Response Types ──

export interface StreamStartResponse {
    type: 'stream_start';
    message_id: number;
    resolved_model: string | null;
}

export interface TextDeltaResponse {
    type: 'text_delta';
    content: string;
}

export interface ThinkingDeltaResponse {
    type: 'thinking_delta';
    content: string;
}

export interface ToolUseStartResponse {
    type: 'tool_use_start';
    tool_call_id: string;
    tool_name: string;
}

export interface ToolInputDeltaResponse {
    type: 'tool_input_delta';
    tool_call_id: string;
    content: string;
}

export interface ToolResultResponse {
    type: 'tool_result';
    tool_call_id: string;
    tool_name: string;
    result: string;
    is_error: boolean;
}

export interface BlockCompleteResponse {
    type: 'block_complete';
    block_index: number;
    content_type: string;
}

export interface TurnCompleteResponse {
    type: 'turn_complete';
    input_tokens: number;
    output_tokens: number;
}

export interface StreamErrorResponse {
    type: 'stream_error';
    code: string;
    message: string;
    recoverable: boolean;
}

export interface StreamCancelledResponse {
    type: 'stream_cancelled';
}

export interface HealthOkResponse {
    type: 'health_ok';
    version: string;
}

export interface SummaryResultResponse {
    type: 'summary_result';
    session_id: number;
    summary: string;
}

export type SidecarResponse =
    | StreamStartResponse
    | TextDeltaResponse
    | ThinkingDeltaResponse
    | ToolUseStartResponse
    | ToolInputDeltaResponse
    | ToolResultResponse
    | BlockCompleteResponse
    | TurnCompleteResponse
    | StreamErrorResponse
    | StreamCancelledResponse
    | HealthOkResponse
    | SummaryResultResponse;

// ── Protocol Helpers ──

/**
 * Parse an NDJSON line into a SidecarRequest.
 * Throws if the line is not valid JSON or does not match a known request type.
 */
export function parseRequest(line: string): SidecarRequest {
    const parsed = JSON.parse(line.trim()) as SidecarRequest;

    if (!parsed || typeof parsed !== 'object' || !('type' in parsed)) {
        throw new Error('Invalid request: missing "type" field');
    }

    const validTypes = ['send_message', 'cancel_stream', 'health_check', 'generate_summary'];
    if (!validTypes.includes(parsed.type)) {
        throw new Error(`Unknown request type: ${parsed.type}`);
    }

    return parsed;
}

/**
 * Serialize a SidecarResponse to an NDJSON line (compact JSON + newline).
 */
export function serializeResponse(response: SidecarResponse): string {
    return JSON.stringify(response) + '\n';
}
