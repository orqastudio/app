---
id: EPIC-030
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
  - target: MS-000
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-123
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-124
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-125
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-126
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-127
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-128
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-129
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-130
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-131
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-132
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-320
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

- [TASK-123](TASK-123): Initialize Tauri v2 + Svelte 5 project
- [TASK-124](TASK-124): Implement Rust backend sidecar and streaming
- [TASK-125](TASK-125): Implement SQLite database and migrations
- [TASK-126](TASK-126): Implement session and message CRUD
- [TASK-127](TASK-127): Implement remaining IPC commands across all domains
- [TASK-128](TASK-128): Implement four-zone layout and sidebar
- [TASK-129](TASK-129): Implement conversation UI with streaming
- [TASK-130](TASK-130): Implement tool call rendering
- [TASK-131](TASK-131): Implement session management UI
- [TASK-132](TASK-132): Implement settings view and semantic code search
