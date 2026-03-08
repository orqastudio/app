---
id: orqa-streaming
title: "Orqa Streaming Pipeline"
name: orqa-streaming
description: |
  OrqaStudio streaming pipeline: Agent SDK → sidecar (Bun) → NDJSON → Rust Channel<T> → Svelte.
  Covers ProviderEvent types, StreamEvent types, sidecar protocol, error handling, and tool approval.
  Use when: Modifying the streaming pipeline, adding new event types, debugging streaming issues,
  or working with the sidecar.
version: 1.0.0
tags: [orqa, streaming, sidecar, ndjson, channel, agent-sdk, claude]
user-invocable: true
---


OrqaStudio streams Claude conversations through a multi-layer pipeline. Understanding this pipeline is critical for anyone touching streaming, tool execution, or the sidecar.

## Pipeline Overview

```
Claude API (Anthropic)
    ↑↓
Agent SDK (in sidecar process)
    ↓ events
Sidecar (Bun-compiled binary: sidecar/src/)
    ↓ NDJSON over stdout
Rust (src-tauri/src/sidecar/ + src-tauri/src/commands/stream_commands.rs)
    ↓ Channel<T> (Tauri streaming)
Svelte (ui/lib/stores/conversation.svelte.ts)
    ↓ reactive state
UI Components (ConversationView, StreamingIndicator, etc.)
```

## Layer 1: Sidecar (TypeScript/Bun)

The sidecar is a Bun-compiled binary in `sidecar/`. It communicates with Rust via NDJSON over stdin/stdout.

### Protocol Types

```typescript
// sidecar/src/protocol.ts
export interface SendMessageRequest {
    type: 'send_message';
    session_id: number;
    content: string;
    model: string | null;
    system_prompt: string | null;
    sdk_session_id: string | null;
    enable_thinking: boolean;
}

export interface ToolResultRequest {
    type: 'tool_result';
    tool_call_id: string;
    output: string;
    is_error: boolean;
}

export interface ToolApprovalRequest {
    type: 'tool_approval';
    tool_call_id: string;
    approved: boolean;
    reason: string | null;
}
```

### Provider (Agent SDK Integration)

```typescript
// sidecar/src/provider.ts — simplified pattern
const response = await query({
    model: request.model,
    system: request.system_prompt ? [{ text: request.system_prompt }] : undefined,
    ...(request.sdk_session_id ? { resume: request.sdk_session_id } : {}),
    tools: [...fileTools, ...searchTools],
});

for await (const event of response) {
    if (event.type === 'text') {
        emit({ type: 'text_delta', content: event.text });
    } else if (event.type === 'tool_use') {
        emit({ type: 'tool_execute', ... });
        // Wait for Rust to send back tool_result via stdin
    }
}
```

### NDJSON Communication

```typescript
// sidecar/src/index.ts — stdin/stdout protocol
// Read requests from stdin (one JSON object per line)
for await (const line of readLines(process.stdin)) {
    const request = JSON.parse(line);
    await handleRequest(request);
}

// Write responses to stdout (one JSON object per line)
function emit(response: SidecarResponse) {
    process.stdout.write(JSON.stringify(response) + '\n');
}
```

## Layer 2: Rust Sidecar Bridge

Rust spawns the sidecar as a child process and parses NDJSON from its stdout.

### Sidecar Types (Mirror TypeScript)

```rust
// src-tauri/src/sidecar/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarRequest {
    SendMessage {
        session_id: i64,
        content: String,
        model: Option<String>,
        system_prompt: Option<String>,
        sdk_session_id: Option<String>,
        enable_thinking: bool,
    },
    ToolResult {
        tool_call_id: String,
        output: String,
        is_error: bool,
    },
    ToolApproval {
        tool_call_id: String,
        approved: bool,
        reason: Option<String>,
    },
    CancelStream { session_id: i64 },
    HealthCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SidecarResponse {
    TextDelta { content: String },
    ThinkingDelta { content: String },
    ToolExecute { tool_call_id: String, tool_name: String, input: String },
    ToolApprovalRequest { tool_call_id: String, tool_name: String, input: String },
    TurnComplete { input_tokens: i64, output_tokens: i64, sdk_session_id: Option<String> },
    StreamError { code: String, message: String, recoverable: bool },
    // ...
}
```

## Layer 3: Channel<T> to Frontend

The Rust stream command receives `Channel<StreamEvent>` from Tauri and forwards events.

### StreamEvent Enum (Rust)

```rust
// src-tauri/src/domain/provider_event.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum StreamEvent {
    StreamStart { message_id: i64, resolved_model: Option<String> },
    TextDelta { content: String },
    ThinkingDelta { content: String },
    ToolUseStart { tool_call_id: String, tool_name: String },
    ToolInputDelta { tool_call_id: String, content: String },
    ToolResult { tool_call_id: String, tool_name: String, result: String, is_error: bool },
    TurnComplete { input_tokens: i64, output_tokens: i64 },
    StreamError { code: String, message: String, recoverable: bool },
    StreamCancelled,
    ToolApprovalRequest { tool_call_id: String, tool_name: String, input: String },
    ProcessViolation { check: String, message: String },
    SessionTitleUpdated { session_id: i64, title: String },
    SystemPromptSent { custom_prompt: Option<String>, governance_prompt: String, total_chars: i64 },
    ContextInjected { message_count: i32, total_chars: i64, messages: String },
}
```

### StreamEvent Union (TypeScript)

```typescript
// ui/lib/types/streaming.ts
export type StreamEvent =
    | { type: "stream_start"; data: { message_id: number; resolved_model: string | null } }
    | { type: "text_delta"; data: { content: string } }
    | { type: "thinking_delta"; data: { content: string } }
    | { type: "tool_use_start"; data: { tool_call_id: string; tool_name: string } }
    | { type: "tool_result"; data: { tool_call_id: string; tool_name: string; result: string; is_error: boolean } }
    | { type: "turn_complete"; data: { input_tokens: number; output_tokens: number } }
    | { type: "stream_error"; data: { code: string; message: string; recoverable: boolean } }
    | { type: "stream_cancelled"; data: null }
    | { type: "tool_approval_request"; data: { tool_call_id: string; tool_name: string; input: string } }
    | { type: "process_violation"; data: { check: string; message: string } }
    | { type: "session_title_updated"; data: { session_id: number; title: string } }
    | { type: "system_prompt_sent"; data: { custom_prompt: string | null; governance_prompt: string; total_chars: number } }
    | { type: "context_injected"; data: { message_count: number; total_chars: number; messages: string } }
```

## Layer 4: Svelte Store

The conversation store processes streaming events via a switch statement:

```typescript
// ui/lib/stores/conversation.svelte.ts
private handleStreamEvent(event: StreamEvent) {
    switch (event.type) {
        case "stream_start":
            this.streamingMessageId = event.data.message_id;
            this.resolvedModel = event.data.resolved_model;
            break;
        case "text_delta":
            this.streamingContent += event.data.content;
            break;
        case "tool_use_start":
            this.activeToolCalls.set(event.data.tool_call_id, {
                toolCallId: event.data.tool_call_id,
                toolName: event.data.tool_name,
                input: "", output: null, isError: false, isComplete: false,
            });
            break;
        case "tool_approval_request":
            this.pendingApproval = { ...event.data };
            break;
        case "turn_complete":
            this.isStreaming = false;
            this.streamingContent = "";
            break;
    }
}
```

## Tool Execution Flow

Tools have a special round-trip through the pipeline:

```
1. Claude requests tool use → sidecar emits tool_execute
2. Rust receives tool_execute → checks if read-only
3a. Read-only tool → Rust executes immediately → sends tool_result to sidecar
3b. Write tool → Rust emits ToolApprovalRequest to frontend
4. User approves/denies → frontend calls stream_tool_approval_respond
5. Rust sends tool_approval to sidecar
6. If approved, Rust executes tool → sends tool_result to sidecar
7. Sidecar feeds result back to Agent SDK → Claude continues
```

Read-only tools (auto-approved): `read_file`, `glob`, `grep`, `search_regex`, `search_semantic`, `load_skill`, `code_research`

## Anti-Patterns

### Adding a StreamEvent variant without updating both sides
If you add a new variant to the Rust `StreamEvent` enum, you MUST also add the matching entry to the TypeScript `StreamEvent` union AND handle it in `handleStreamEvent()`.

### Parsing streaming data in the frontend
All response parsing happens in Rust (`src-tauri/src/sidecar/`). The frontend receives pre-parsed `StreamEvent` objects. Never parse raw API responses in Svelte.

### Blocking the stream loop
The Rust stream command runs a synchronous loop reading from sidecar stdout. Tool execution blocks this loop until the tool completes. Long-running tools should be async.

## Key Files

| File | Purpose |
|------|---------|
| `sidecar/src/provider.ts` | Agent SDK integration, event streaming |
| `sidecar/src/protocol.ts` | NDJSON protocol types |
| `sidecar/src/index.ts` | stdin/stdout communication loop |
| `src-tauri/src/sidecar/types.rs` | Rust-side sidecar protocol types |
| `src-tauri/src/sidecar/protocol.rs` | NDJSON parsing and sidecar process management |
| `src-tauri/src/commands/stream_commands.rs` | Stream command handler, tool execution, Channel<T> |
| `src-tauri/src/domain/provider_event.rs` | StreamEvent enum definition |
| `ui/lib/types/streaming.ts` | TypeScript StreamEvent union |
| `ui/lib/stores/conversation.svelte.ts` | Frontend event handling and state management |
