---
id: EPIC-019
title: "MCP Host — External Servers"
status: draft
priority: P2
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 3
  impact: 4
  dependency: 2
  effort: 5
score: 3.8
roadmap-ref: "M9"
docs-required:
  - docs/architecture/mcp-host.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (MCP host plan)
  - docs/architecture/mcp-host.md (update with implementation)
  - docs/architecture/decisions.md (AD for transport selection, trust model)
tags: [mcp, external-tools, integrations]
---

# MCP Host — External Servers

## Tasks

- [ ] MCP host module — JSON-RPC protocol handler, connection state machine
- [ ] stdio transport — spawn external MCP servers, process lifecycle management
- [ ] SSE transport — HTTP client for remote MCP servers
- [ ] Config loader — merge project + user MCP server configs
- [ ] Tool aggregator — merge built-in + external tools, namespace external tools
- [ ] MCP Servers section in Settings — server list, add/remove, test connection, trust levels
