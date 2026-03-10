---
id: TASK-014
title: Local embeddings and semantic search
description: Adds on-device embedding generation using an ONNX model with GPU acceleration, enabling semantic similarity search that ranks results by cosine distance.
status: done
created: "2026-03-04"
updated: "2026-03-09"
epic: EPIC-034
assignee: AGENT-002
skills:
  - SKILL-003
  - SKILL-027
scope:
  - src-tauri/src/search/embedder.rs
  - src-tauri/src/search/mod.rs
  - src-tauri/src/commands/search_commands.rs
acceptance:
  - ONNX Runtime loads bge-small-en-v1.5 model
  - DirectML acceleration works (GPU/NPU with CPU fallback)
  - Semantic search returns results ranked by cosine similarity
---
## What

Add ONNX-based embedding generation and semantic (vector similarity) search
to the native search engine.

## Outcome

Implemented as `search_semantic` Tauri command. Embedder uses `ort` crate with
DirectML execution provider. 384-dimensional vectors stored in DuckDB.
Git commit: `69a9ae3`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
