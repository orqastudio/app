---
id: TASK-a4d5e6b7
title: "Extract search engine from MCP server into standalone process"
type: task
description: "Move the ONNX embedding and DuckDB search functionality from the MCP server into a standalone search server process. MCP server becomes a thin protocol adapter that delegates search to the standalone engine."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - Search engine runs as a standalone process with its own port
  - MCP server delegates search operations to the standalone engine via HTTP/IPC
  - search_regex, search_semantic, search_research tools continue to work unchanged from the user's perspective
  - Search engine can be started/stopped independently of MCP server
  - Search engine has a health check endpoint
  - No ONNX or DuckDB dependencies remain in the MCP server after extraction
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Phase 4 of port allocation epic"
  - target: TASK-93c4f5a6
    type: depends-on
    rationale: "Needs CLI process lifecycle commands to manage the new service"
---

## What

The MCP server currently embeds the ONNX runtime and DuckDB for semantic search. This task extracts that functionality into a standalone process so:

1. Search survives MCP server restarts
2. Search can be independently scaled/monitored
3. MCP server is lighter and focused on protocol translation
4. The search engine can serve both CLI and app contexts

## Architecture After Extraction

```
Search Engine (standalone, port 10260)
  ├── ONNX Runtime — embedding generation
  ├── DuckDB — vector storage and similarity search
  └── HTTP API: /embed, /search, /index, /health

MCP Server (thin adapter, port 10259)
  ├── search_regex    → delegates to Search Engine
  ├── search_semantic → delegates to Search Engine
  ├── search_research → delegates to Search Engine
  └── graph_*         → delegates to Daemon
```

## Verification

1. Search engine starts independently and responds to health check
2. MCP server search tools return correct results via the standalone engine
3. Indexing works through the standalone engine
4. No ONNX/DuckDB imports remain in MCP server code
