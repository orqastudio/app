---
id: SKILL-013
title: OrqaStudio Native Search Engine
description: |
  OrqaStudio's embedded code search engine: DuckDB storage, ONNX Runtime embeddings
  with DirectML acceleration, and three search tools (search_regex, search_semantic,
  code_research). Use when working within the OrqaStudio app context.
status: active
created: "2026-03-01"
updated: "2026-03-10"
layer: project
version: 1.0.0
user-invocable: true
---

OrqaStudio has a **native search engine** embedded in the Rust backend. It provides the same
three search tools as ChunkHound but through a completely different implementation. This skill
describes the native engine — use it when working in the **app context**.

> **When to load this skill vs `chunkhound`:**
> - **App context** (OrqaStudio UI) → load `orqa-native-search`
> - **CLI context** (Claude Code terminal) → load `chunkhound`
> Both provide `search_regex`, `search_semantic`, and `code_research` — same tool names,
> independent implementations.

## Architecture

```text
┌─────────────────────────────────────────────────────┐
│                 OrqaStudio App                       │
│                                                     │
│  ┌──────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Chunker  │  │  Embedder    │  │  Store       │  │
│  │          │→ │  (ONNX +     │→ │  (DuckDB)    │  │
│  │ Semantic │  │  DirectML)   │  │  Vectors +   │  │
│  │ boundary │  │  bge-small   │  │  Full-text   │  │
│  │ splitting│  │  384-dim     │  │              │  │
│  └──────────┘  └──────────────┘  └──────────────┘  │
│       ↑              ↑                  ↑           │
│       └──────────────┴──────────────────┘           │
│                      │                              │
│              SearchEngine (mod.rs)                   │
│                      │                              │
│         ┌────────────┼────────────┐                 │
│         │            │            │                 │
│    search_regex  search_semantic  code_research     │
│    (Tauri cmd)   (Tauri cmd)     (tool executor)   │
└─────────────────────────────────────────────────────┘
```

**Key difference from ChunkHound:** Everything runs in-process. No external HTTP server,
no MCP protocol, no localhost:11435. The ONNX model is loaded directly by the Rust
process via the `ort` crate.

## Components

| Module | File | Purpose |
|--------|------|---------|
| Chunker | `src-tauri/src/search/chunker.rs` | Splits code at semantic boundaries (functions, classes, imports) |
| Embedder | `src-tauri/src/search/embedder.rs` | ONNX Runtime + bge-small-en-v1.5 (384-dim vectors), DirectML acceleration |
| Store | `src-tauri/src/search/store.rs` | DuckDB: chunk storage, full-text search, vector cosine similarity |
| SearchEngine | `src-tauri/src/search/mod.rs` | Coordinator: initializes components, exposes search methods |
| Commands | `src-tauri/src/commands/search_commands.rs` | Tauri IPC: `search_regex`, `search_semantic`, `index_codebase`, `get_index_status` |
| Tool executor | `src-tauri/src/domain/tool_executor.rs` | Handles `search_regex`, `search_semantic`, `code_research` as agent tools |

## Three Search Tools

### `search_regex` — Exact Pattern Matching

Searches DuckDB for chunks matching a regular expression. No embeddings needed —
works even if the ONNX model hasn't been loaded.

**Tool name (in app):** `search_regex`
**Tool name (in CLI):** `mcp__chunkhound__search_regex`

### `search_semantic` — Meaning-Based Search

Generates an embedding for the query via ONNX Runtime, then finds the most similar
code chunks via DuckDB `array_cosine_similarity()`.

**Requires:** Embedder initialized (ONNX model loaded)
**Tool name (in app):** `search_semantic`
**Tool name (in CLI):** `mcp__chunkhound__search_semantic`

### `code_research` — Architectural Analysis

Combines semantic search results with LLM analysis to produce a structured report
about how a system works.

**Tool name (in app):** `code_research`
**Tool name (in CLI):** `mcp__chunkhound__code_research`

## Hardware Acceleration

The embedder uses DirectML via ONNX Runtime execution providers:

```rust
let session = ort::session::Session::builder()
    .with_execution_providers([ort::ep::DirectML::default().build()])
    .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
    // ...
```

DirectML automatically selects the best available hardware:
1. **NPU** (if available) — lowest power, dedicated inference
2. **GPU** — fast, shared with display
3. **CPU** — fallback, always works

No code changes needed between acceleration paths — ONNX Runtime handles selection.

## Model Distribution

| Context | How Model Is Provided |
|---------|----------------------|
| Production (installer) | Bundled in the installer (~67MB) |
| Development | Auto-downloaded from Hugging Face on first use |

The model (`bge-small-en-v1.5`) produces 384-dimensional vectors. It's small enough
to bundle but powerful enough for code search.

## Indexing

Indexing happens at startup and can be triggered manually:

| Command | Purpose |
|---------|---------|
| `index_codebase` | Trigger full re-index |
| `get_index_status` | Check indexing state (idle/indexing/complete) |

Progress is shown in the status bar via the startup task tracker.

## Query Patterns

Same patterns as ChunkHound — the tool interfaces are identical:

| Situation | Tool | Example |
|-----------|------|---------|
| Know the exact name | `search_regex` | `create_session` |
| Know the concept | `search_semantic` | `"error handling in Tauri commands"` |
| Need end-to-end understanding | `code_research` | `"how does streaming work"` |

## When Native Search Is Unavailable

If the ONNX model hasn't loaded yet (first startup, download in progress):

- `search_regex` always works (no embeddings needed)
- `search_semantic` and `code_research` will fail gracefully
- Check `get_index_status` for indexing state

## Related

- `chunkhound` skill — CLI-context equivalent (external MCP server)
- AD-024 — Architecture decision documenting the native search engine
- `search-engine-implementation` research doc — Design rationale
