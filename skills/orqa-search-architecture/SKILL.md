---
name: orqa-search-architecture
description: "Implementation details of OrqaStudio's embedded search engine: DuckDB storage, ONNX Runtime embeddings, DirectML acceleration, chunker design, and module layout. Use when: Modifying or extending the search engine in backend/src-tauri/src/search/."
user-invocable: true
---

This skill covers the implementation architecture of OrqaStudio's native search engine.
For how to USE the search tools, see `orqa-native-search`.

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
| Chunker | `backend/src-tauri/src/search/chunker.rs` | Splits code at semantic boundaries (functions, classes, imports) |
| Embedder | `backend/src-tauri/src/search/embedder.rs` | ONNX Runtime + bge-small-en-v1.5 (384-dim vectors), DirectML acceleration |
| Store | `backend/src-tauri/src/search/store.rs` | DuckDB: chunk storage, full-text search, vector cosine similarity |
| SearchEngine | `backend/src-tauri/src/search/mod.rs` | Coordinator: initializes components, exposes search methods |
| Commands | `backend/src-tauri/src/commands/search_commands.rs` | Tauri IPC: `search_regex`, `search_semantic`, `index_codebase`, `get_index_status` |
| Tool executor | `backend/src-tauri/src/domain/tool_executor.rs` | Handles `search_regex`, `search_semantic`, `code_research` as agent tools |

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

Indexing happens at startup and can be triggered manually. Progress is shown in the
status bar via the startup task tracker.

## Related

- `orqa-native-search` skill — how to USE the search tools (core, all agents)
- `chunkhound` skill — CLI-context equivalent (external MCP server)
- [AD-99c2a969](AD-99c2a969) — Architecture decision documenting the native search engine
