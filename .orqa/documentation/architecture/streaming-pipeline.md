---
id: DOC-014
title: Streaming Pipeline
description: End-to-end streaming architecture from Agent SDK through sidecar NDJSON to Rust Channel<T> to Svelte.
created: "2026-03-02"
updated: "2026-03-09"
---

**Date:** 2026-03-02 | **Status:** Phase 0e specification | **References:** [AI Provider Research](RES-002) ([AD-007](AD-007), [AD-009](AD-009))

End-to-end design for streaming AI responses through the provider sidecar, Rust backend, and into the Svelte UI. The sidecar implements a provider interface — currently using the Claude Agent SDK, with the architecture designed for additional providers. Covers the NDJSON protocol, parsing, event routing, token buffering, persistence, error handling, backpressure, reconnection, and cancellation.

---

## 1. Pipeline Overview

```
AI Provider API (SSE)
       |
       v
  Provider SDK (TypeScript, e.g. Claude Agent SDK)
       |
       v
  Sidecar (Bun-compiled binary)
       |  translates SSE into ProviderEvent NDJSON
       v
  stdout (newline-delimited JSON)
       |
       v
  Rust parser (line-by-line deserialization)
       |
       v
  Channel<T> (Tauri IPC, ordered delivery)
       |
       v
  Svelte $state (fine-grained reactivity)
       |
       v
  DOM (batched ~16ms frame updates)
```

Detailed flow with latency annotations:

```
+------------------+     SSE      +------------------+    NDJSON     +------------------+
|  AI Provider API | -----------> |  Bun Sidecar     | -----------> |  Rust Backend     |
|  (30-100ms/tok)  |   network    |  (Provider SDK)  |   stdout     |  (std::process::  |
|                  |              |  ~0.1-0.5ms hop  |   pipe       |   Command spawn)  |
+------------------+              +------------------+              +------------------+
                                                                           |
                                                                    Channel<T>
                                                                    (ordered, indexed)
                                                                           |
                                                                           v
                                                                    +------------------+
                                                                    |  Svelte 5 Store  |
                                                                    |  ($state runes)  |
                                                                    +------------------+
                                                                           |
                                                                    requestAnimationFrame
                                                                    (~16ms batches)
                                                                           |
                                                                           v
                                                                    +------------------+
                                                                    |       DOM        |
                                                                    +------------------+
```

The sidecar hop adds ~0.1-0.5ms per event, which is negligible compared to AI providers' typical 30-100ms per token generation time. `Channel<T>` is Tauri's recommended streaming mechanism — faster than `emit`/`listen` events, with ordered delivery and index-based sequencing ([AD-009](AD-009)).

---

## 2. Sidecar Protocol

The sidecar emits one JSON object per line to stdout. Each line is a self-contained `ProviderEvent`. The Rust backend reads stdout line by line. No framing, no length prefix -- newline is the delimiter.

### Message Types

#### text_delta

A chunk of text from Claude's response. Arrives at token generation speed (30-100ms intervals).

```json
{"type":"text_delta","delta":"Here is the","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","turn_index":1,"block_index":0}
```

#### thinking

A chunk from Claude's extended thinking output. Arrives before or interleaved with text_delta events.

```json
{"type":"thinking","delta":"Let me analyze the code structure...","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","turn_index":1,"block_index":0}
```

#### tool_use_start

Claude has decided to call a tool. Contains the tool name and begins the input accumulation.

```json
{"type":"tool_use_start","tool_call_id":"toolu_01ABC123","tool_name":"Read","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","turn_index":1,"block_index":1}
```

#### tool_use_delta

Incremental input JSON for a tool call. The sidecar streams this as the Agent SDK receives it.

```json
{"type":"tool_use_delta","tool_call_id":"toolu_01ABC123","delta":"{\"file_path\":\"src/main.rs\"}","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","turn_index":1,"block_index":1}
```

#### tool_result

The result of a tool execution. Sent after OrqaStudio™'s MCP server processes the tool call and returns the result through the Agent SDK.

```json
{"type":"tool_result","tool_call_id":"toolu_01ABC123","content":"fn main() {\n    println!(\"Hello\");\n}","is_error":false,"message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","turn_index":2,"block_index":0}
```

#### error

An error from the sidecar, Claude API, or Agent SDK. The `code` field enables programmatic handling.

```json
{"type":"error","code":"rate_limit","message":"Rate limited. Retry after 30 seconds.","retryable":true,"retry_after_ms":30000}
```

Error codes: `rate_limit`, `auth_error`, `network_error`, `invalid_request`, `server_error`, `sidecar_error`.

#### model_resolved

Sent once at the start of streaming when the session model is `"auto"`. Reports which model the provider actually selected for this response. The Rust backend forwards this to the frontend so the status bar can display "Auto → Sonnet 4.6".

```json
{"type":"model_resolved","resolved_model":"claude-sonnet-4-6","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL"}
```

When the sidecar receives `"auto"` as the model, it delegates model selection to the provider SDK (e.g., the Agent SDK handles rate-limit-aware routing for Max subscriptions). The sidecar emits a `model_resolved` event as soon as the provider reports which model was chosen — typically before or alongside the first `text_delta`.

#### done

The response is complete. Contains final token usage for the turn. When auto model selection is active, `resolved_model` contains the model that was actually used for this turn.

```json
{"type":"done","message_id":"msg_01XFDUDYJgAACzvnptvVoYEL","input_tokens":1523,"output_tokens":487,"stop_reason":"end_turn","resolved_model":"claude-sonnet-4-6"}
```

### Protocol Rules

- One JSON object per line. No multi-line JSON.
- Lines are UTF-8 encoded and terminated with `\n`.
- Unknown `type` values must be ignored (forward compatibility).
- The sidecar must not write anything other than valid NDJSON to stdout. Diagnostic output goes to stderr.
- `message_id`, `turn_index`, and `block_index` are present on all content events for correlation with the `messages` table.
- When the session model is `"auto"`, the sidecar emits a `model_resolved` event early in the stream (before or alongside the first `text_delta`). Rust forwards this to the frontend via the `StreamStart` event's `resolved_model` field so the status bar can display the actual model.

---

## 3. Rust NDJSON Parser

The Rust backend spawns the sidecar via `tauri-plugin-shell` and reads its stdout line by line. Each line is deserialized into a `ProviderEvent` and routed to the streaming pipeline.

```rust
use serde::Deserialize;
use tauri_plugin_shell::process::CommandEvent;
use tokio::sync::mpsc;

/// Raw event from the sidecar's NDJSON stdout.
/// This is the deserialization target -- one variant per sidecar message type.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderEvent {
    TextDelta {
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },
    Thinking {
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },
    ToolUseStart {
        tool_call_id: String,
        tool_name: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },
    ToolUseDelta {
        tool_call_id: String,
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },
    ToolResult {
        tool_call_id: String,
        content: String,
        is_error: bool,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },
    Error {
        code: String,
        message: String,
        retryable: bool,
        retry_after_ms: Option<u64>,
    },
    ModelResolved {
        resolved_model: String,
        message_id: String,
    },
    Done {
        message_id: String,
        input_tokens: u32,
        output_tokens: u32,
        stop_reason: String,
        resolved_model: Option<String>,
    },
}
```

### Parser Loop

The parser runs as a Tokio task, reading `CommandEvent::Stdout` lines from the sidecar's process handle. Each line is deserialized and forwarded to the stream coordinator.

```rust
use tauri_plugin_shell::ShellExt;

/// Spawns the sidecar and returns a receiver for parsed events.
pub async fn spawn_sidecar(
    app: &tauri::AppHandle,
    session_id: i64,
    prompt: &str,
) -> Result<mpsc::Receiver<ProviderEvent>, StreamError> {
    let (tx, rx) = mpsc::channel::<ProviderEvent>(256);

    let shell = app.shell();
    let (mut child_rx, _child) = shell
        .sidecar("orqa-studio-sidecar")
        .expect("sidecar binary not found")
        .args([
            "--session-id", &session_id.to_string(),
            "--prompt", prompt,
        ])
        .spawn()
        .map_err(|e| StreamError::SidecarSpawn(e.to_string()))?;

    // Parser task: reads stdout line by line, deserializes, forwards
    tokio::spawn(async move {
        let mut line_buffer = String::new();

        while let Some(event) = child_rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    let chunk = String::from_utf8_lossy(&bytes);
                    line_buffer.push_str(&chunk);

                    // Process all complete lines in the buffer
                    while let Some(newline_pos) = line_buffer.find('\n') {
                        let line = line_buffer[..newline_pos].trim().to_string();
                        line_buffer = line_buffer[newline_pos + 1..].to_string();

                        if line.is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<ProviderEvent>(&line) {
                            Ok(provider_event) => {
                                if tx.send(provider_event).await.is_err() {
                                    // Receiver dropped -- stream was cancelled
                                    return;
                                }
                            }
                            Err(e) => {
                                tracing::warn!(
                                    line = %line,
                                    error = %e,
                                    "Failed to parse sidecar NDJSON line"
                                );
                                // Skip malformed lines; do not crash the stream
                            }
                        }
                    }
                }
                CommandEvent::Stderr(bytes) => {
                    let msg = String::from_utf8_lossy(&bytes);
                    tracing::debug!(sidecar_stderr = %msg);
                }
                CommandEvent::Terminated(status) => {
                    tracing::info!(
                        exit_code = ?status.code,
                        "Sidecar process terminated"
                    );
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(rx)
}
```

### Key Design Decisions

- **Line buffering:** stdout may deliver partial lines. The parser accumulates bytes in `line_buffer` and only processes complete `\n`-terminated lines.
- **Unknown types:** `serde_json::from_str` will fail on unknown `type` values. The `Err` branch logs and skips, preserving forward compatibility.
- **Channel capacity 256:** Provides sufficient buffer for burst token delivery without unbounded memory growth. See Section 9 (Backpressure).
- **Stderr passthrough:** Sidecar diagnostic output is logged at debug level, never parsed as events.

---

## 4. Channel\<T\> Event Types

After parsing, `ProviderEvent` is mapped to `StreamEvent` -- the Rust enum sent to the frontend via Tauri's `Channel<T>`. This is the contract between Rust and Svelte.

```rust
use serde::Serialize;
use tauri::ipc::Channel;

/// Events sent to the frontend via Channel<T>.
/// This is the IPC contract -- changes here require frontend updates.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum StreamEvent {
    /// A chunk of response text.
    TextDelta {
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },

    /// A chunk of extended thinking text.
    ThinkingDelta {
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },

    /// Claude has initiated a tool call.
    ToolCallStart {
        tool_call_id: String,
        tool_name: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },

    /// Incremental input JSON for an in-progress tool call.
    ToolCallDelta {
        tool_call_id: String,
        delta: String,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },

    /// A tool call has finished executing with a result.
    ToolCallComplete {
        tool_call_id: String,
        content: String,
        is_error: bool,
        message_id: String,
        turn_index: u32,
        block_index: u32,
    },

    /// The provider resolved which model to use (sent when session model is "auto").
    /// Emitted early in the stream so the frontend can update the status bar.
    ModelResolved {
        resolved_model: String,
        message_id: String,
    },

    /// The full response is complete.
    MessageComplete {
        message_id: String,
        input_tokens: u32,
        output_tokens: u32,
        stop_reason: String,
        resolved_model: Option<String>,
    },

    /// An error occurred during streaming.
    Error {
        code: String,
        message: String,
        retryable: bool,
        retry_after_ms: Option<u64>,
    },
}
```

### Mapping from ProviderEvent to StreamEvent

```rust
impl From<ProviderEvent> for StreamEvent {
    fn from(event: ProviderEvent) -> Self {
        match event {
            ProviderEvent::TextDelta { delta, message_id, turn_index, block_index } => {
                StreamEvent::TextDelta { delta, message_id, turn_index, block_index }
            }
            ProviderEvent::Thinking { delta, message_id, turn_index, block_index } => {
                StreamEvent::ThinkingDelta { delta, message_id, turn_index, block_index }
            }
            ProviderEvent::ToolUseStart { tool_call_id, tool_name, message_id, turn_index, block_index } => {
                StreamEvent::ToolCallStart { tool_call_id, tool_name, message_id, turn_index, block_index }
            }
            ProviderEvent::ToolUseDelta { tool_call_id, delta, message_id, turn_index, block_index } => {
                StreamEvent::ToolCallDelta { tool_call_id, delta, message_id, turn_index, block_index }
            }
            ProviderEvent::ToolResult { tool_call_id, content, is_error, message_id, turn_index, block_index } => {
                StreamEvent::ToolCallComplete { tool_call_id, content, is_error, message_id, turn_index, block_index }
            }
            ProviderEvent::Error { code, message, retryable, retry_after_ms } => {
                StreamEvent::Error { code, message, retryable, retry_after_ms }
            }
            ProviderEvent::ModelResolved { resolved_model, message_id } => {
                StreamEvent::ModelResolved { resolved_model, message_id }
            }
            ProviderEvent::Done { message_id, input_tokens, output_tokens, stop_reason, resolved_model } => {
                StreamEvent::MessageComplete { message_id, input_tokens, output_tokens, stop_reason, resolved_model }
            }
        }
    }
}
```

### Tauri Command

The Tauri command that initiates streaming and returns events via `Channel<T>`:

```rust
#[tauri::command]
pub async fn send_message(
    app: tauri::AppHandle,
    session_id: i64,
    content: String,
    on_event: Channel<StreamEvent>,
) -> Result<(), String> {
    let mut rx = spawn_sidecar(&app, session_id, &content)
        .await
        .map_err(|e| e.to_string())?;

    // Stream coordinator: receives parsed events, sends to frontend,
    // and manages SQLite buffering (Section 7)
    tokio::spawn(async move {
        let mut db_buffer = StreamBuffer::new(session_id);

        while let Some(provider_event) = rx.recv().await {
            let stream_event: StreamEvent = provider_event.into();

            // Buffer for SQLite persistence
            db_buffer.accumulate(&stream_event);

            // Send to frontend via Channel<T>
            if let Err(e) = on_event.send(stream_event) {
                tracing::error!(error = %e, "Failed to send stream event to frontend");
                break;
            }
        }

        // Final flush to SQLite
        db_buffer.flush_final().await;
    });

    Ok(())
}
```

---

## 5. Svelte Store Integration

The frontend listens to `Channel<T>` events and updates `$state` runes for fine-grained reactivity. The store is the single source of truth for the active streaming response.

### Stream State Type

```typescript
// ui/lib/types/stream.ts

export type StreamStatus = 'idle' | 'waiting' | 'streaming' | 'complete' | 'error';

export interface ToolCall {
  toolCallId: string;
  toolName: string;
  input: string;        // accumulated JSON input
  result?: string;
  isError: boolean;
  status: 'running' | 'completed' | 'error';
}

export interface StreamingMessage {
  messageId: string;
  turnIndex: number;
  blocks: ContentBlock[];
  inputTokens?: number;
  outputTokens?: number;
}

export interface ContentBlock {
  blockIndex: number;
  type: 'text' | 'thinking' | 'tool_use' | 'tool_result';
  content: string;
  toolCall?: ToolCall;
}
```

### Stream Store

```svelte
<script lang="ts" module>
  // ui/lib/stores/stream.svelte.ts
  import { invoke, Channel } from '@tauri-apps/api/core';
  import type {
    StreamStatus,
    StreamingMessage,
    ContentBlock,
    ToolCall,
  } from '$lib/types/stream';

  export function createStreamStore() {
    let status = $state<StreamStatus>('idle');
    let currentMessage = $state<StreamingMessage | null>(null);
    let error = $state<{ code: string; message: string; retryable: boolean } | null>(null);

    // Token buffer for batched DOM updates (Section 6)
    let pendingDeltas: Array<{ blockIndex: number; delta: string }> = [];
    let flushScheduled = false;

    function flushDeltas() {
      if (!currentMessage || pendingDeltas.length === 0) return;

      // Apply all buffered deltas in a single $state mutation
      for (const { blockIndex, delta } of pendingDeltas) {
        const block = currentMessage.blocks.find(b => b.blockIndex === blockIndex);
        if (block) {
          block.content += delta;
        }
      }
      pendingDeltas = [];
      flushScheduled = false;
    }

    function scheduleFlush() {
      if (!flushScheduled) {
        flushScheduled = true;
        requestAnimationFrame(flushDeltas);
      }
    }

    function handleEvent(event: StreamEvent) {
      switch (event.type) {
        case 'textDelta': {
          if (status === 'waiting') status = 'streaming';

          ensureBlock(event.turnIndex, event.blockIndex, 'text');
          pendingDeltas.push({ blockIndex: event.blockIndex, delta: event.delta });
          scheduleFlush();
          break;
        }

        case 'thinkingDelta': {
          if (status === 'waiting') status = 'streaming';

          ensureBlock(event.turnIndex, event.blockIndex, 'thinking');
          pendingDeltas.push({ blockIndex: event.blockIndex, delta: event.delta });
          scheduleFlush();
          break;
        }

        case 'toolCallStart': {
          ensureBlock(event.turnIndex, event.blockIndex, 'tool_use');
          const block = currentMessage!.blocks.find(
            b => b.blockIndex === event.blockIndex
          )!;
          block.toolCall = {
            toolCallId: event.toolCallId,
            toolName: event.toolName,
            input: '',
            isError: false,
            status: 'running',
          };
          break;
        }

        case 'toolCallDelta': {
          const block = currentMessage?.blocks.find(
            b => b.toolCall?.toolCallId === event.toolCallId
          );
          if (block?.toolCall) {
            block.toolCall.input += event.delta;
          }
          break;
        }

        case 'toolCallComplete': {
          const block = currentMessage?.blocks.find(
            b => b.toolCall?.toolCallId === event.toolCallId
          );
          if (block?.toolCall) {
            block.toolCall.result = event.content;
            block.toolCall.isError = event.isError;
            block.toolCall.status = event.isError ? 'error' : 'completed';
          }
          break;
        }

        case 'messageComplete': {
          // Flush any remaining buffered tokens
          flushDeltas();

          if (currentMessage) {
            currentMessage.inputTokens = event.inputTokens;
            currentMessage.outputTokens = event.outputTokens;
          }
          status = 'complete';
          break;
        }

        case 'error': {
          flushDeltas();
          error = {
            code: event.code,
            message: event.message,
            retryable: event.retryable,
          };
          status = 'error';
          break;
        }
      }
    }

    function ensureBlock(turnIndex: number, blockIndex: number, type: ContentBlock['type']) {
      if (!currentMessage) {
        currentMessage = {
          messageId: '',
          turnIndex,
          blocks: [],
        };
      }
      if (!currentMessage.blocks.find(b => b.blockIndex === blockIndex)) {
        currentMessage.blocks.push({
          blockIndex,
          type,
          content: '',
        });
        // Keep blocks sorted by index
        currentMessage.blocks.sort((a, b) => a.blockIndex - b.blockIndex);
      }
    }

    async function sendMessage(sessionId: number, content: string) {
      // Reset state for new message
      status = 'waiting';
      currentMessage = null;
      error = null;
      pendingDeltas = [];

      const channel = new Channel<StreamEvent>();
      channel.onmessage = handleEvent;

      try {
        await invoke('send_message', {
          sessionId,
          content,
          onEvent: channel,
        });
      } catch (e) {
        status = 'error';
        error = {
          code: 'invoke_error',
          message: String(e),
          retryable: true,
        };
      }
    }

    return {
      get status() { return status; },
      get currentMessage() { return currentMessage; },
      get error() { return error; },
      sendMessage,
    };
  }
</script>
```

### Component Usage

```svelte
<!-- ui/lib/components/conversation/StreamingMessage.svelte -->
<script lang="ts">
  import type { StreamingMessage } from '$lib/types/stream';
  import TextBlock from './TextBlock.svelte';
  import ThinkingBlock from './ThinkingBlock.svelte';
  import ToolCallCard from './ToolCallCard.svelte';

  let { message, status }: {
    message: StreamingMessage;
    status: 'streaming' | 'complete';
  } = $props();
</script>

{#each message.blocks as block (block.blockIndex)}
  {#if block.type === 'text'}
    <TextBlock content={block.content} streaming={status === 'streaming'} />
  {:else if block.type === 'thinking'}
    <ThinkingBlock content={block.content} />
  {:else if block.type === 'tool_use' && block.toolCall}
    <ToolCallCard toolCall={block.toolCall} />
  {/if}
{/each}
```

Svelte 5's fine-grained reactivity means that when `block.content` changes, only that specific text node updates -- not the entire message tree. This is critical for avoiding layout thrashing during streaming ([AD-004](AD-004)).

---

## 6. Token Buffering

Tokens arrive from Claude at 30-100ms intervals. Updating the DOM on every single token wastes frame budget and causes jank. Instead, tokens are buffered and flushed once per animation frame (~16ms on a 60Hz display).

### Strategy

```
Token arrives (StreamEvent::TextDelta)
    |
    v
Push to pendingDeltas array (no DOM update)
    |
    v
Schedule requestAnimationFrame (if not already scheduled)
    |
    v
On next frame: apply all buffered deltas to $state in one batch
    |
    v
Svelte reactivity updates a single text node
```

### Why requestAnimationFrame

- `requestAnimationFrame` fires once per display frame, naturally aligning with the browser's render cycle.
- Batching 1-3 tokens per frame (at 60fps and ~50ms/token) means each frame update appends a small string -- cheap.
- During fast bursts (cached responses, short tokens), the buffer may hold 5-10 tokens per frame. The concatenation is still negligible.
- No `setInterval` or `setTimeout` -- those can fire mid-frame and cause double layouts.

### Cost Analysis

| Scenario | Tokens/frame | String concat cost | Layout cost |
|----------|-------------|-------------------|-------------|
| Normal streaming (50ms/token) | ~1 | Negligible | Single text node update |
| Fast burst (10ms/token) | ~2 | Negligible | Single text node update |
| Cached/instant (1ms/token) | ~16 | ~320 bytes concat | Single text node update |

Even in the worst case, the cost is a short string concatenation and one text node mutation per frame.

### Markdown Rendering

During active streaming, accumulated text is displayed as raw characters -- not rendered as markdown. At natural pause points (~500ms gap between tokens or end of a content block), accumulated text is re-rendered as formatted markdown. This avoids the cost of re-parsing markdown on every frame while ensuring the final output is properly formatted.

The `TextBlock` component handles this:

```svelte
<!-- ui/lib/components/conversation/TextBlock.svelte -->
<script lang="ts">
  import Markdown from '$lib/components/shared/Markdown.svelte';

  let { content, streaming }: {
    content: string;
    streaming: boolean;
  } = $props();

  // During streaming: show raw text for performance
  // After completion: render full markdown
  let rendered = $derived(!streaming);
</script>

{#if rendered}
  <Markdown source={content} />
{:else}
  <span class="whitespace-pre-wrap">{content}</span>
{/if}
```

---

## 7. SQLite Write Strategy

Streaming tokens are buffered in Rust memory and flushed to SQLite periodically. This avoids writing to disk on every token (which would be ~10-30 writes/second) while preserving data on crash.

### Buffer Design

```rust
use std::time::Instant;

/// Accumulates streaming tokens in memory and flushes to SQLite periodically.
pub struct StreamBuffer {
    session_id: i64,
    /// Accumulated text per (turn_index, block_index)
    text_buffers: Vec<BlockBuffer>,
    /// Timestamp of last flush
    last_flush: Instant,
    /// Flush interval
    flush_interval: std::time::Duration,
    /// Whether any data has changed since last flush
    dirty: bool,
}

struct BlockBuffer {
    turn_index: u32,
    block_index: u32,
    content_type: String,
    content: String,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    tool_input: Option<String>,
    db_row_id: Option<i64>,  // None until first INSERT
}

impl StreamBuffer {
    pub fn new(session_id: i64) -> Self {
        Self {
            session_id,
            text_buffers: Vec::new(),
            last_flush: Instant::now(),
            flush_interval: std::time::Duration::from_millis(500),
            dirty: false,
        }
    }

    /// Accumulate a stream event into the buffer.
    pub fn accumulate(&mut self, event: &StreamEvent) {
        match event {
            StreamEvent::TextDelta { delta, turn_index, block_index, .. } => {
                let buf = self.ensure_block(*turn_index, *block_index, "text");
                buf.content.push_str(delta);
                self.dirty = true;
            }
            StreamEvent::ThinkingDelta { delta, turn_index, block_index, .. } => {
                let buf = self.ensure_block(*turn_index, *block_index, "thinking");
                buf.content.push_str(delta);
                self.dirty = true;
            }
            StreamEvent::ToolCallStart { tool_call_id, tool_name, turn_index, block_index, .. } => {
                let buf = self.ensure_block(*turn_index, *block_index, "tool_use");
                buf.tool_call_id = Some(tool_call_id.clone());
                buf.tool_name = Some(tool_name.clone());
                self.dirty = true;
            }
            StreamEvent::ToolCallDelta { tool_call_id, delta, .. } => {
                if let Some(buf) = self.find_tool_block(tool_call_id) {
                    let input = buf.tool_input.get_or_insert_with(String::new);
                    input.push_str(delta);
                    self.dirty = true;
                }
            }
            _ => {}
        }

        // Check if it is time to flush
        if self.dirty && self.last_flush.elapsed() >= self.flush_interval {
            // flush() is called by the stream coordinator
        }
    }

    /// Flush all buffered data to SQLite.
    pub async fn flush(&mut self, db: &Database) -> Result<(), StreamError> {
        if !self.dirty {
            return Ok(());
        }

        for buf in &mut self.text_buffers {
            match buf.db_row_id {
                Some(row_id) => {
                    // UPDATE existing row with accumulated content
                    db.execute(
                        "UPDATE messages SET content = ?, tool_input = ? WHERE id = ?",
                        (&buf.content, &buf.tool_input, row_id),
                    ).await?;
                }
                None => {
                    // INSERT new row, store the row ID for subsequent updates
                    let row_id = db.execute(
                        "INSERT INTO messages (session_id, role, content_type, content, \
                         tool_call_id, tool_name, tool_input, turn_index, block_index, \
                         stream_status) \
                         VALUES (?, 'assistant', ?, ?, ?, ?, ?, ?, ?, 'pending')",
                        (
                            self.session_id,
                            &buf.content_type,
                            &buf.content,
                            &buf.tool_call_id,
                            &buf.tool_name,
                            &buf.tool_input,
                            buf.turn_index,
                            buf.block_index,
                        ),
                    ).await?;
                    buf.db_row_id = Some(row_id);
                }
            }
        }

        self.dirty = false;
        self.last_flush = Instant::now();
        Ok(())
    }

    /// Final flush: write all remaining data and mark rows as complete.
    pub async fn flush_final(&mut self, db: &Database) -> Result<(), StreamError> {
        self.flush(db).await?;

        // Mark all pending rows as complete
        for buf in &self.text_buffers {
            if let Some(row_id) = buf.db_row_id {
                db.execute(
                    "UPDATE messages SET stream_status = 'complete' WHERE id = ?",
                    (row_id,),
                ).await?;
            }
        }

        Ok(())
    }

    // ... helper methods ensure_block, find_tool_block ...
}
```

### stream_status Lifecycle

```
Message INSERT with stream_status = 'pending'
    |
    v
Periodic UPDATEs (content grows as tokens arrive)
    |
    v
Final UPDATE: stream_status = 'complete'   (normal completion)
         or:  stream_status = 'error'      (sidecar crash, API error)
```

On app startup, recover interrupted streams:

```sql
UPDATE messages SET stream_status = 'error'
WHERE stream_status = 'pending';
```

This ensures no message is left in a permanently pending state after a crash.

### Write Frequency

| Phase | Write frequency | What gets written |
|-------|----------------|-------------------|
| First token | Immediate INSERT | Creates the row with `stream_status = 'pending'` |
| Streaming | UPDATE every ~500ms | Accumulated content |
| Completion | Final UPDATE | Full content + `stream_status = 'complete'` |
| Tool result | Immediate INSERT | New row for tool_result block |

WAL mode ([AD-005](AD-005), see [SQLite Schema](DOC-013)) ensures the UI can read session data concurrently while streaming writes occur.

---

## 8. Error Handling

Every error in the pipeline has a defined detection mechanism, UI response, and recovery path.

### Error Classification

| Error | Detection | UI Response | Recovery |
|-------|-----------|------------|----------|
| **Sidecar crash** | `CommandEvent::Terminated` with non-zero exit code | Toast: "Connection lost. Reconnecting..." + error block in conversation | Auto-restart sidecar (Section 10). Partial response preserved. |
| **Malformed NDJSON** | `serde_json::from_str` fails | None (logged, line skipped) | Parser continues with next line. Malformed lines are never fatal. |
| **Network error** | `ProviderEvent::Error { code: "network_error" }` | Error block: "Network error. Check your connection." + retry button | User clicks retry. Status bar shows yellow dot. |
| **Rate limit** | `ProviderEvent::Error { code: "rate_limit" }` | Error block: "Rate limited. Retry in {N}s." + countdown timer + retry button | Auto-retry after `retry_after_ms`. User can retry manually. |
| **Auth error** | `ProviderEvent::Error { code: "auth_error" }` | Error block: "Authentication failed." + link to settings | User re-authenticates via settings (`claude login`). |
| **Server error** | `ProviderEvent::Error { code: "server_error" }` | Error block: "Claude is temporarily unavailable." + retry button | Exponential backoff retry: 1s, 2s, 4s, max 3 attempts. |
| **Invalid request** | `ProviderEvent::Error { code: "invalid_request" }` | Error block: "Request error: {message}" | Non-retryable. User must modify the request. |
| **Sidecar spawn failure** | `spawn()` returns error | Alert in conversation: "Failed to start sidecar." + link to settings | Check CLI installation, permissions. |
| **Channel send failure** | `on_event.send()` returns error | Stream silently stops (frontend disconnected) | Frontend re-navigated; no recovery needed. |

### Error Propagation Flow

```
Sidecar error → ProviderEvent::Error → StreamEvent::Error → Channel<T> → Svelte store
                                              |
                                              v
                                     StreamBuffer marks rows
                                     stream_status = 'error'
```

### Partial Response Preservation

When an error occurs mid-stream, the partial response is never discarded. The `StreamBuffer` flushes whatever tokens have accumulated, and the frontend displays them above the error block. The user sees what Claude said before the error and can retry from that point.

---

## 9. Backpressure

The pipeline uses bounded channels to prevent unbounded memory growth when tokens arrive faster than the UI can render.

### Backpressure Points

```
Sidecar stdout (OS pipe buffer: 64KB typical)
       |
       v
mpsc::channel(256)  <-- bounded: Rust parser to stream coordinator
       |
       v
Channel<T>          <-- Tauri IPC: Rust to frontend (unbounded, but frame-batched on receive)
       |
       v
requestAnimationFrame buffer  <-- pendingDeltas array (unbounded but tiny per frame)
       |
       v
DOM
```

### What Happens Under Pressure

1. **Sidecar produces faster than Rust can parse:** The OS pipe buffer (64KB) absorbs bursts. If the buffer fills, the sidecar's `stdout.write()` blocks, which naturally slows the Agent SDK, which naturally slows API consumption. This is OS-level backpressure and requires no application code.

2. **Rust parser produces faster than Channel\<T\> can deliver:** The `mpsc::channel(256)` buffer absorbs bursts. At ~50 bytes per event, 256 slots hold ~12.5KB -- well within memory budget. If the channel fills, `tx.send().await` suspends the parser task until the stream coordinator consumes. This is Tokio-level backpressure.

3. **Channel\<T\> delivers faster than the UI can render:** The `requestAnimationFrame` batching absorbs this entirely. Even if 100 events arrive between frames, they are buffered in `pendingDeltas` and applied in a single DOM mutation on the next frame. The `pendingDeltas` array grows linearly with burst size but is cleared every ~16ms.

### Worst-Case Memory Analysis

Claude generates at most ~100 tokens/second (Sonnet). Each token event is ~100 bytes JSON. At 100 events/second:

- OS pipe buffer: 64KB (holds ~640 events = 6.4 seconds)
- Rust channel: 256 events = ~25KB (holds ~2.5 seconds)
- pendingDeltas: ~2 events per frame at 60fps (negligible)

Total backpressure buffer: ~89KB. Memory pressure from streaming is not a concern.

---

## 10. Reconnection

The sidecar may crash due to bugs, OOM, or OS signals. The Rust backend detects the crash and manages restart.

### Detection

```rust
CommandEvent::Terminated(status) => {
    if status.code != Some(0) {
        // Abnormal termination -- initiate restart
        tracing::error!(
            exit_code = ?status.code,
            "Sidecar crashed, initiating restart"
        );
        handle_sidecar_crash(&app, session_id).await;
    }
}
```

### Restart vs. Resume

| Scenario | Behavior |
|----------|----------|
| **Crash during streaming** | Restart sidecar. Mark current stream as error. Preserve partial response. UI shows error block with retry button. User clicks retry to re-send the last message. |
| **Crash while idle** | Restart sidecar silently. Toast: "Connection restored." No user action needed. |
| **Repeated crashes (3+ in 60s)** | Stop retrying. Status bar: red dot. Error alert: "Sidecar keeps crashing. Check CLI installation." Link to settings. |

### Why Not Resume

Resuming a partial stream requires the sidecar to maintain conversation state and support mid-stream pickup. The Agent SDK does not expose this capability. Additionally, Claude's API does not support resuming a partial response -- a new request must be sent.

The cost of re-sending the last user message is minimal (Claude re-generates from the last turn). The benefit of simplicity (no partial state synchronization) outweighs the cost of re-generation.

### Restart Implementation

```rust
/// Manages sidecar lifecycle including crash recovery.
pub struct SidecarManager {
    app: tauri::AppHandle,
    crash_timestamps: Vec<Instant>,
    max_crashes_per_minute: usize,
}

impl SidecarManager {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self {
            app,
            crash_timestamps: Vec::new(),
            max_crashes_per_minute: 3,
        }
    }

    /// Called when the sidecar process terminates unexpectedly.
    pub async fn handle_crash(&mut self, session_id: i64) -> Result<RestartAction, StreamError> {
        let now = Instant::now();

        // Prune old crash timestamps (older than 60s)
        self.crash_timestamps.retain(|t| now.duration_since(*t).as_secs() < 60);
        self.crash_timestamps.push(now);

        if self.crash_timestamps.len() > self.max_crashes_per_minute {
            tracing::error!("Sidecar crashed {} times in 60s, giving up", self.crash_timestamps.len());
            return Ok(RestartAction::GiveUp);
        }

        // Flush pending writes for the interrupted stream
        // (StreamBuffer::flush_final with error status is called by the stream coordinator)

        tracing::info!("Restarting sidecar (crash {} of {})",
            self.crash_timestamps.len(), self.max_crashes_per_minute);

        Ok(RestartAction::Restart)
    }
}

pub enum RestartAction {
    Restart,
    GiveUp,
}
```

---

## 11. Stop / Cancel

The user can cancel a streaming response at any time by clicking the stop button or pressing `Escape` while the conversation panel is focused.

### Cancel Flow

```
User clicks Stop
       |
       v
Frontend: invoke('cancel_stream', { sessionId })
       |
       v
Rust: send SIGTERM to sidecar process (or kill on Windows)
       |
       v
Sidecar receives signal, terminates Agent SDK session
       |
       v
Sidecar process exits (CommandEvent::Terminated)
       |
       v
Rust: StreamBuffer flushes partial content to SQLite
      stream_status = 'complete' (partial response is valid)
       |
       v
Rust: sends StreamEvent::MessageComplete { stop_reason: "user_cancelled" }
       |
       v
Frontend: status transitions to 'complete'
          Partial response displayed as-is (not discarded)
          Input re-enabled, send button re-enabled
```

### Tauri Command

```rust
use tauri_plugin_shell::process::CommandChild;
use std::sync::Mutex;

/// Global handle to the active sidecar process for cancellation.
struct ActiveSidecar(Mutex<Option<CommandChild>>);

#[tauri::command]
pub async fn cancel_stream(
    state: tauri::State<'_, ActiveSidecar>,
) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(child) = guard.take() {
        child.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

### Frontend Stop Button

```svelte
<!-- Integrated into the conversation input area -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let { status, sessionId }: {
    status: 'idle' | 'waiting' | 'streaming' | 'complete' | 'error';
    sessionId: number;
  } = $props();

  async function handleStop() {
    await invoke('cancel_stream', { sessionId });
  }
</script>

{#if status === 'waiting' || status === 'streaming'}
  <button
    onclick={handleStop}
    class="stop-button"
    aria-label="Stop generating"
  >
    Stop
  </button>
{/if}
```

### Partial Response Handling

When the user cancels, the partial response is preserved and treated as a complete message. This matches user expectation -- they asked the AI something, got a partial answer, and stopped it. The partial answer is still useful and searchable.

The `stop_reason: "user_cancelled"` field distinguishes user-cancelled responses from natural completions (`end_turn`) or tool use stops (`tool_use`) in the database and UI.

---

## Design Constraints Summary

| Constraint | Solution | Reference |
|-----------|----------|-----------|
| Thick backend owns domain logic | Rust parses, buffers, and persists; Svelte only renders | [AD-001](AD-001) |
| IPC via Tauri commands only | `invoke('send_message')` + `Channel<T>` for streaming | [AD-002](AD-002), [AD-009](AD-009) |
| No panics in production | All parser errors logged and skipped, never fatal | [AD-003](AD-003) |
| Svelte 5 runes only | `$state`, `$derived`, `$props` for all reactivity | [AD-004](AD-004) |
| SQLite for persistence | Buffered writes every ~500ms, WAL mode for concurrency | [AD-005](AD-005) |
| Provider-agnostic protocol | `ProviderEvent` enum is neutral; sidecar is swappable | [AD-017](AD-017) |

---

## 12. Current Implementation Reference

> **Note:** Sections 1–11 above reflect the original design specification. This section documents the actual implemented state of the streaming pipeline as of [EPIC-001](EPIC-001) (AI Transparency Wiring).

### Implemented StreamEvent Enum

The Rust `StreamEvent` enum is defined in `src-tauri/src/domain/provider_event.rs`. It is serialized with `#[serde(tag = "type", content = "data", rename_all = "snake_case")]`, producing `{ "type": "...", "data": { ... } }` on the wire.

| Variant | Serialized `type` | Source |
|---------|-------------------|--------|
| `SystemPromptSent` | `system_prompt_sent` | Emitted by Rust in `stream_send_message()` |
| `StreamStart` | `stream_start` | Relayed from sidecar |
| `TextDelta` | `text_delta` | Relayed from sidecar |
| `ThinkingDelta` | `thinking_delta` | Relayed from sidecar |
| `ToolUseStart` | `tool_use_start` | Relayed from sidecar |
| `ToolInputDelta` | `tool_input_delta` | Relayed from sidecar |
| `ToolResult` | `tool_result` | Relayed from sidecar |
| `BlockComplete` | `block_complete` | Relayed from sidecar |
| `TurnComplete` | `turn_complete` | Relayed from sidecar |
| `StreamError` | `stream_error` | Relayed from sidecar |
| `StreamCancelled` | `stream_cancelled` | Relayed from sidecar |
| `ToolApprovalRequest` | `tool_approval_request` | Relayed from sidecar; stream loop blocks until response |
| `ProcessViolation` | `process_violation` | Emitted by Rust after `TurnComplete` |
| `SessionTitleUpdated` | `session_title_updated` | Emitted by Rust after assistant message is persisted |
| `ContextInjected` | `context_injected` | Emitted by Rust in `stream_send_message()` when prior messages exist |

The TypeScript mirror of this union is `ui/lib/types/streaming.ts`. Both must stay in sync — adding a variant requires updating both files in the same commit.

### Event Sequence per Turn

For a single user turn, events arrive in this order:

```
1. SystemPromptSent        ← Rust, before sidecar send (only when prompt is present)
2. StreamStart             ← sidecar, when API call begins
3. ThinkingDelta*          ← sidecar, zero or more (extended thinking only)
4. TextDelta*              ← sidecar, zero or more
5. ToolUseStart / ToolInputDelta* / ToolResult*   ← sidecar, per tool call
6. ToolApprovalRequest?    ← sidecar, if write/execute tool needs approval (stream blocks)
7. BlockComplete*          ← sidecar, when each content block closes
8. TurnComplete            ← sidecar, response complete; carries token counts
9. ProcessViolation*       ← Rust, after TurnComplete, only on violations
10. SessionTitleUpdated?   ← Rust, after assistant message persisted, only if auto-named
```

`StreamError` or `StreamCancelled` may replace any event after `StreamStart`.

`ContextInjected` appears before `StreamStart` on any turn where prior messages exist in the session (i.e., not the first message). It shows the user what conversation history the AI has access to.

### SystemPromptSent — Emission Point and Fields

`SystemPromptSent` is emitted in `stream_send_message()` (`src-tauri/src/commands/stream_commands.rs`, lines 830–836) after `resolve_system_prompt()` returns and before `SidecarRequest::SendMessage` is constructed. It is only emitted when a system prompt is present (i.e., an active project with governance artifacts is configured).

```
resolve_system_prompt(&state)   →   Option<String>
        │
        │  if Some(prompt)
        ▼
on_event.send(StreamEvent::SystemPromptSent {
    custom_prompt: None,           // always None until EPIC-002
    governance_prompt: prompt,     // full governance prompt built from project artifacts
    total_chars: prompt.len(),     // character count
})
        │
        ▼
SidecarRequest::SendMessage { system_prompt: prompt, ... }
        │
        ▼
state.sidecar.send(&request)
```

Fields:

| Field | Type | Description |
|-------|------|-------------|
| `custom_prompt` | `string \| null` | User-supplied custom prompt prefix. Always `null` until [EPIC-002](EPIC-002) is implemented. |
| `governance_prompt` | `string` | Full governance prompt built from project artifacts (CLAUDE.md, AGENTS.md, etc.). |
| `total_chars` | `number` | Character count of the combined prompt. |

### Emission Points Summary

| Event | Location | When |
|-------|----------|------|
| `system_prompt_sent` | `stream_send_message()`, after `resolve_system_prompt()` | Before sidecar `send()` |
| `stream_start` through `turn_complete` | `run_stream_loop()`, per `SidecarResponse` line | As sidecar stdout is read |
| `tool_approval_request` | `run_stream_loop()`, on `SidecarResponse::ToolApprovalRequest` | Stream blocks until `stream_tool_approval_respond` called |
| `process_violation` | `emit_process_violations()`, after `TurnComplete` processed | After assistant message persisted |
| `context_injected` | `stream_send_message()`, after `SystemPromptSent` | When session has prior messages (count > 0) |
| `session_title_updated` | `stream_send_message()`, after assistant message persisted | Only when session is auto-named |

### Future Work

- **[EPIC-002](EPIC-002) (Custom System Prompt):** The `custom_prompt` field on `system_prompt_sent` will carry the user-supplied custom prompt prefix. Currently always `null`.
- **[EPIC-003](EPIC-003) (Context Injection on Failed Resume):** When a provider session resume fails, Rust will load prior messages from SQLite and inject them into the sidecar conversation. The `ContextInjected` event emission already exists ([EPIC-001](EPIC-001)) — [EPIC-003](EPIC-003) adds the actual injection mechanism.

### Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | N/A |
| Process Governance | The `system_prompt_sent` event surfaces the exact governance prompt sent to the AI on every turn, making the enforcement layer visible and auditable in the conversation UI. |
