---
id: AD-fcd55d44
title: Streaming Pipeline
description: "Streaming pipeline: Agent SDK (SSE) -> sidecar (NDJSON) -> Rust (Channel<T>) -> Svelte store. Clarifies AD-a334623b."
status: completed
created: 2026-03-02
updated: 2026-03-02
relationships:
  - target: RES-7f87b149
    type: informed-by
    rationale: RES-7f87b149 documented the Agent SDK's SSE streaming events (content_block_delta, text_delta, thinking_delta), defining the sidecar input format
  - target: RES-00c5dbc3
    type: informed-by
    rationale: RES-00c5dbc3 confirmed Tauri's Channel<T> provides ordered streaming with negligible latency, validating it as the IPC streaming mechanism
  - target: EPIC-46e5f406
    type: drives
  - target: DOC-eda128cd
    type: documented-by
  - target: DOC-52bbfba5
    type: documented-by
  - target: DOC-3c65a1e3
    type: documented-by
---
## Decision

The streaming pipeline flows: Agent SDK (SSE from Claude API) → TypeScript sidecar (translates to `ProviderEvent` NDJSON) → stdout → Rust (`CommandEvent::Stdout`, parse JSON, deserialize) → `Channel<T>` (Tauri IPC) → Svelte store (`$state`, fine-grained reactivity) → component re-render.

**Clarifies [AD-a334623b](AD-a334623b):** [AD-a334623b](AD-a334623b) states "streaming data uses Tauri events (emit/listen)". This is refined: streaming uses `Channel<T>` (faster than events, ordered delivery, type-safe on Rust side) for high-throughput data like Claude token streams. Events (`emit`/`listen`) remain appropriate for low-frequency notifications (file changes, app lifecycle).

## Rationale

`Channel<T>` is Tauri's recommended streaming mechanism — faster than events, with ordered delivery and index-based sequencing. The sidecar hop adds ~0.1-0.5ms per event, which is negligible compared to Claude's 30-100ms per token.

## Consequences

The Rust backend must define a `ProviderEvent` enum that covers all streaming event types (text deltas, thinking deltas, tool use events, completion, errors). The Svelte store must accumulate tokens efficiently using `$state` for fine-grained DOM updates.
