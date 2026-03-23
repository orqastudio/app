---
id: EPIC-599fb1fd
type: epic
title: "LSP and MCP as daemon interfaces"
description: "Refactor LSP and MCP servers to be thin protocol translators over the daemon. LSP translates LSP JSON-RPC to daemon calls. MCP translates MCP tool calls to daemon calls. Remove local reimplementations."
status: captured
priority: P2
relationships:
  - target: EPIC-81c336c1
    type: depends-on
    rationale: "Needs daemon HTTP API for artifact queries, validation, content"
---
# LSP and MCP as Daemon Interfaces

- LSP server: translate LSP didOpen/didChange/didSave → daemon validation calls → LSP diagnostics
- MCP server: translate MCP tool calls (graph_query, search_semantic, etc.) → daemon API
- Remove LSP's own hardcoded status list, local frontmatter parsing, local graph building
- Remove MCP's own graph building if it has any
- Both servers connect to daemon at startup