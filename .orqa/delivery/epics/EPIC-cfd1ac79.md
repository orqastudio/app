---
id: EPIC-cfd1ac79
title: Project Scaffold
description: "The first working version: a Tauri v2 desktop app with Claude conversations via Agent SDK sidecar, streaming, SQLite, and conversation UI."
status: completed
priority: P1
created: 2026-03-02
updated: 2026-03-07
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 5
  dependencies: 5
relationships:
  - target: MS-85b9269b
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-8f5e769b
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-3bafbf7f
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-1368cb7f
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-a1e2f58a
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-efcbcc95
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-b2ebf089
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-8c45c6b4
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-02bf9d1b
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-ab6f8ad9
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-910302bc
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-a5e7df28
    type: delivered-by
    rationale: Epic contains this task
---
## Why P1

Nothing works without the scaffold. Every subsequent feature is built on top of this foundation.

## What Was Done

- Tauri v2 + Svelte 5 project initialised with configured plugins
- Rust backend: Agent SDK sidecar process with streaming via `Channel<T>`
- Rust backend: SQLite database with schema and migrations
- Rust backend: Session and message CRUD operations
- Rust backend: 40+ IPC commands across 8 domains
- Frontend: Four-zone layout (toolbar, sidebar, conversation, status bar)
- Frontend: Conversation UI with streaming token display
- Frontend: Tool call rendering with collapsible cards showing input and output
- Frontend: Session dropdown with history, search, and navigation
- Frontend: Settings view for provider configuration and model selection
- Semantic code search: ONNX embeddings server with DuckDB vector search
- End-to-end integration: send message, stream response, render in UI

## Notes

Retroactively captured. Work preceded the artifact framework. This is the baseline from which all milestone work proceeds.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-8f5e769b](TASK-8f5e769b): Initialize Tauri v2 + Svelte 5 project
- [TASK-3bafbf7f](TASK-3bafbf7f): Implement Rust backend sidecar and streaming
- [TASK-1368cb7f](TASK-1368cb7f): Implement SQLite database and migrations
- [TASK-a1e2f58a](TASK-a1e2f58a): Implement session and message CRUD
- [TASK-efcbcc95](TASK-efcbcc95): Implement remaining IPC commands across all domains
- [TASK-b2ebf089](TASK-b2ebf089): Implement four-zone layout and sidebar
- [TASK-8c45c6b4](TASK-8c45c6b4): Implement conversation UI with streaming
- [TASK-02bf9d1b](TASK-02bf9d1b): Implement tool call rendering
- [TASK-ab6f8ad9](TASK-ab6f8ad9): Implement session management UI
- [TASK-910302bc](TASK-910302bc): Implement settings view and semantic code search
