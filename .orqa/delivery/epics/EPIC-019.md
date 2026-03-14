---
id: EPIC-019
title: MCP Host — External Servers
description: Implement MCP host with JSON-RPC protocol handler, stdio and SSE transports, tool aggregation, and settings UI for external servers.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-002
horizon: next
pillars:
  - PILLAR-001
depends-on: []
blocks: []
docs-required: []
docs-produced:
  - AD-010
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

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
