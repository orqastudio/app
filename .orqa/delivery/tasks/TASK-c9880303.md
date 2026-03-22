---
id: TASK-c9880303
title: MCP server — Rust backend artifact graph API
status: active
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-367f0026
    type: depends-on
  - target: TASK-6675ad7c
    type: depended-on-by
---

# TASK-c9880303: MCP Server

## Acceptance Criteria

1. MCP server module added to Tauri app (`src/servers/mcp.rs`)
2. Exposes tools: graph.query, graph.resolve, graph.relationships, graph.stats, graph.validate, graph.read, graph.create
3. Exposes resources: orqa://schema/core.json, orqa://schema/project.json
4. Serves over stdio when invoked with `--mcp` flag
5. `.mcp.json` added to connector for Claude Code registration
6. Reuses existing Rust command implementations
