---
id: TASK-9fb85edf
title: Add native search tools to MCP server (search_regex, search_semantic)
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-cdb03816
    type: delivers
  - target: TASK-6827dc2b
    type: depended-on-by
---

# TASK-9fb85edf: Native Search as MCP Tools

## Acceptance Criteria

1. MCP server exposes `search.regex({ pattern, paths?, limit? })` tool
2. MCP server exposes `search.semantic({ query, limit? })` tool
3. Both reuse existing Rust `search/` module (store.rs, embedder.rs)
4. Embeddings server initialises on first semantic query (lazy init)
5. ONNX model downloaded if not present (BGE-small-en)
6. Results include file path, line number, content, and relevance score
