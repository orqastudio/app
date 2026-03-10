---
id: EPIC-034
title: Native Search Engine
description: "Implement native code search engine using DuckDB for storage, ONNX Runtime for embeddings, and DirectML for hardware acceleration. Three search modes: regex, semantic, and code_research."
status: done
priority: P1
created: "2026-03-04"
updated: "2026-03-09"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-016
  - RES-006
docs-required: []
docs-produced:
  - DOC-011
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 4
  effort: 5
  risk: 3
  score: 22
---
## Implementation Design

### Architecture
- **Chunker** (`src-tauri/src/search/chunker.rs`) — Splits code at semantic boundaries (functions, classes, imports)
- **Embedder** (`src-tauri/src/search/embedder.rs`) — ONNX Runtime with bge-small-en-v1.5, DirectML acceleration
- **Store** (`src-tauri/src/search/store.rs`) — DuckDB vector storage and similarity search
- **SearchEngine** (`src-tauri/src/search/mod.rs`) — Coordinator: regex, semantic, code_research

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
