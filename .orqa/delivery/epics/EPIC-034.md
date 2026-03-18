---
id: EPIC-034
title: Native Search Engine
description: "Implement native code search engine using DuckDB for storage, ONNX Runtime for embeddings, and DirectML for hardware acceleration. Three search modes: regex, semantic, and code_research."
status: completed
priority: P1
created: 2026-03-04
updated: 2026-03-09
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 5
  dependencies: 3
relationships:
  - target: RES-016
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-016
  - target: RES-006
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-006
  - target: MS-001
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-013
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-014
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-015
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-324
    type: delivered-by
    rationale: Epic contains this task
  - target: RES-006
    type: guided-by
  - target: RES-016
    type: guided-by
---
## Implementation Design

### Architecture
- **Chunker** (`backend/src-tauri/src/search/chunker.rs`) — Splits code at semantic boundaries (functions, classes, imports)
- **Embedder** (`backend/src-tauri/src/search/embedder.rs`) — ONNX Runtime with bge-small-en-v1.5, DirectML acceleration
- **Store** (`backend/src-tauri/src/search/store.rs`) — DuckDB vector storage and similarity search
- **SearchEngine** (`backend/src-tauri/src/search/mod.rs`) — Coordinator: regex, semantic, code_research

### IPC Commands
- `search_regex` — Exact pattern matching via DuckDB full-text scan
- `search_semantic` — ONNX embedding → cosine similarity
- `index_codebase` — Background indexing with status bar progress
- `get_index_status` — Check indexing state

### Model Distribution
- Production: bundled in installer (~67MB)
- Development: auto-download from Hugging Face on first use
- Background startup task with progress tracking

## Produced Decision

[AD-024](AD-024)

## Git Evidence

- `0486837` — Architecture docs
- `2313f80` — DuckDB indexer + regex search
- `69a9ae3` — ONNX embeddings + semantic search
- `4c191f7` — Auto-download model
- `304a4e6` — Pre-download at startup
- `e4c5f69` — Startup task tracker

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
