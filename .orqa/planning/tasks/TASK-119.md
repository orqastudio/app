---
id: TASK-119
title: "Design streaming pipeline"
description: "Designed the Agent SDK to Svelte event flow including the sidecar NDJSON protocol, Rust Channel<T>, and frontend event handling."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-029
depends-on: []
scope:
  - Design sidecar-to-Rust NDJSON line protocol
  - Define Rust Channel<T> event types for streaming
  - Design frontend event handling and state transitions
  - Document the full pipeline from SDK to Svelte
acceptance:
  - Pipeline is documented end-to-end with event types at each boundary
  - NDJSON protocol is specified with all message types
  - Channel events cover text, tool use, errors, and completion
---
## What

Designed the full streaming pipeline from the Agent SDK through the sidecar NDJSON protocol, Rust Channel<T> events, and into Svelte store state transitions.

## How

Specified each NDJSON message type the sidecar emits (stream_start, text_delta, tool_use_start, tool_use_end, stream_end, error), defined the corresponding Rust Channel event enum variants, and documented how the conversation store handles each event type.

## Verification

Streaming pipeline is documented end-to-end, NDJSON protocol enumerates all message types, and Channel event variants cover text, tool use, errors, and completion.
