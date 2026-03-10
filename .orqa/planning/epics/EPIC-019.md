---
id: EPIC-019
title: MCP Host — External Servers
description: Implement MCP host with JSON-RPC protocol handler, stdio and SSE transports, tool aggregation, and settings UI for external servers.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-002
pillars:
  - PILLAR-001
depends-on: []
blocks: []
docs-required:
  - docs/architecture/mcp-host.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (MCP host plan)
  - docs/architecture/mcp-host.md (update with implementation)
  - docs/architecture/decisions.md (AD for transport selection
  - trust model)
scoring:
  pillar: 3
  impact: 4
  dependency: 2
  effort: 5
  score: 4.6
---
## Tasks

- [ ] MCP host module — JSON-RPC protocol handler, connection state machine
- [ ] stdio transport — spawn external MCP servers, process lifecycle management
- [ ] SSE transport — HTTP client for remote MCP servers
- [ ] Config loader — merge project + user MCP server configs
- [ ] Tool aggregator — merge built-in + external tools, namespace external tools
- [ ] MCP Servers section in Settings — server list, add/remove, test connection, trust levels
