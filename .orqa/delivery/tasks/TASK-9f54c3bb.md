---
id: TASK-9f54c3bb
title: Local embeddings and semantic search
description: "Adds on-device embedding generation using an ONNX model with GPU acceleration, enabling semantic similarity search that ranks results by cosine distance."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - ONNX Runtime loads bge-small-en-v1.5 model
  - DirectML acceleration works (GPU/NPU with CPU fallback)
  - Semantic search returns results ranked by cosine similarity
relationships:
  - target: EPIC-9bdef0ce
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-b470c205
    type: depended-on-by
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
