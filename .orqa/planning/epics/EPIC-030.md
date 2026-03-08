---
id: EPIC-030
title: "Phase 1 — Scaffold"
status: done
priority: P1
milestone: MS-000
created: 2026-03-02
updated: 2026-03-07
deadline: null
plan: null
depends-on: [EPIC-025, EPIC-026, EPIC-027, EPIC-028, EPIC-029]
blocks: [EPIC-031]
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
score: 22.0
roadmap-ref: "Phase 1"
docs-required:
  - docs/architecture/decisions.md
  - docs/architecture/
  - docs/ui/
docs-produced: []
description: >
  The first working version: a Tauri v2 desktop app with Claude
  conversations via Agent SDK sidecar, streaming, SQLite, and conversation UI.
tags: [foundation, scaffold, tauri, svelte, rust, sqlite, streaming]
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
