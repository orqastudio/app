---
id: TASK-124
title: "Implement Rust backend sidecar and streaming"
description: "Built the Agent SDK sidecar process management in Rust with NDJSON streaming via Channel<T> for real-time conversation display."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-030
depends-on: []
acceptance:
  - Sidecar process starts and communicates via NDJSON
  - Streaming events reach the frontend via Channel<T>
  - Sidecar lifecycle is managed cleanly
---
## What

Built the sidecar process management layer in Rust, parsing NDJSON output and emitting streaming events to the Svelte frontend via Tauri's `Channel<T>`.

## How

Implemented process spawning with `tauri-plugin-shell`, line-by-line NDJSON parsing from stdout, and a `Channel<T>` emitter that forwards typed stream events to the frontend in real-time.

## Verification

Sidecar starts on demand, streams NDJSON events through the channel to the frontend, and cleans up on process death or timeout.
