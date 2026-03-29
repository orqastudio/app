---
id: "RES-44b34cba"
type: discovery-research
title: "Native Search Engine Implementation"
description: "Design and implementation of OrqaStudio's native code search engine using DuckDB for storage, ONNX Runtime for embeddings, and DirectML for hardware acceleration. Three search modes: regex, semantic, and code_research."
status: completed
created: "2026-03-04"
updated: "2026-03-09"
relationships:
  - target: "EPIC-7f3119b1"
    type: "guides"
    rationale: "Research findings informed the design of Native Search Engine"
---

## Investigation

OrqaStudio needed in-app code search that works offline, is fast (<100ms), and supports
both exact pattern matching and meaning-based queries. External search services were
rejected for latency, privacy, and offline-first requirements.

## Key Design Decisions

1. **DuckDB over SQLite for vectors** — DuckDB has native `array_cosine_similarity()` and

   columnar storage optimized for analytical/vector queries. SQLite would require extensions.

2. **ONNX Runtime over PyTorch/TensorFlow** — Lighter weight, supports DirectML for Windows

   NPU/GPU acceleration without CUDA dependency. The `ort` Rust crate provides native bindings.

3. **bge-small-en-v1.5 model** — 33M parameters, 384-dimensional vectors. Balances quality and

   size (~67MB). Small enough to bundle with the installer.

4. **Semantic boundary chunking** — Code is chunked at function/class/import boundaries, not

   fixed-size windows. Preserves meaningful units for search.

5. **Model distribution** — Bundled in installer for production, auto-downloaded from Hugging Face

   for development (first-use download with progress tracking).

## Three Search Modes

| Mode | Implementation | Use Case |
| ------ | --------------- | ---------- |
| `search_regex` | DuckDB full-text scan with regex | Finding specific symbols, imports, patterns |
| `search_semantic` | ONNX embedding → DuckDB cosine similarity | Understanding what code does a thing |
| `code_research` | Combines semantic search with LLM analysis | Answering "how does X work?" |

## Dual-Context Architecture

The app and CLI provide the same three tool names but through independent implementations:

- **App**: Native Rust engine (`backend/src-tauri/src/search/`) with ONNX + DuckDB
- **CLI**: ChunkHound MCP server (external Python tool, separate codebase)

ChunkHound is NOT embedded in the app — it's a CLI companion tool.

## Git Evidence

- `0486837` — Architecture documentation
- `2313f80` — Phase 2: DuckDB indexer + regex search
- `69a9ae3` — Phase 3: ONNX embeddings + semantic search
- `4c191f7` — Auto-download from Hugging Face
- `304a4e6` — Pre-download at startup
- `e4c5f69` — Startup task tracker with status bar UI

## Produced Decision

[PD-7d3d7521](PD-7d3d7521)
