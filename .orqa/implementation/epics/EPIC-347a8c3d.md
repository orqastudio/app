---
id: "EPIC-347a8c3d"
type: epic
title: "Rust artifact platform engine"
description: "Extend libs/validation from validator-only to the canonical artifact engine. Single parser/serialiser for markdown+YAML → validated JSON. Content access, rule evaluation, hook lifecycle, daemon mode. Eliminates all hand-rolled frontmatter parsing."
status: captured
priority: "P0"
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "Clarity Through Structure — single engine for all artifact operations"
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# Rust Artifact Platform Engine

## Problem

The Rust binary (`libs/validation`) currently only validates graph integrity and schema compliance. Every other consumer (CLI, connector, LSP, MCP, app tools) has its own code for parsing frontmatter, loading rules, reading knowledge content, etc. There are 9+ copies of parseFrontmatter across the codebase.

## Target State

The Rust binary is the ONLY thing that reads markdown+YAML. Everything else gets structured JSON.

### Capabilities

1. **Artifact parsing**: any .md file → `{ id, type, status, frontmatter: {...}, content: "body markdown" }` validated against plugin schemas
2. **Artifact querying**: by type, status, ID, relationships — returns JSON arrays
3. **Hook lifecycle**: `--event PreAction --context '{...}'` → evaluate rules → return HookResult JSON
4. **Rule content**: load enforcement entries, filter by event/mechanism, return structured data
5. **Behavioral messages**: extract all mechanism:behavioral messages from active rules
6. **Agent loading**: find agent by type, return preamble + metadata as JSON
7. **Knowledge loading**: find knowledge by key, return content + metadata as JSON
8. **Pattern evaluation**: bash/file regex matching against tool context
9. **Daemon mode**: long-lived process with HTTP API, keep schemas/rules/graph in memory
10. **Daemon lifecycle**: `orqa daemon start|stop|status`

### Daemon

- Standalone process — framework works without the app
- HTTP API for low-latency calls from hooks, LSP, MCP, CLI
- Started by `orqa daemon start` (CLI manages lifecycle)
- `orqa dev` starts daemon + watchers + Vite + Tauri
- App ensures daemon is running during startup
- Connector SessionStart calls `orqa daemon start`

## Tasks

1. Add artifact parsing endpoint (file → structured JSON with content field)
2. Add artifact query endpoint (by type, status, relationships)
3. Add hook lifecycle (accept HookContext, return HookResult)
4. Add rule content loading (return enforcement entries as JSON)
5. Add behavioral message extraction
6. Add agent preamble loading
7. Add knowledge content loading
8. Add bash/file pattern evaluation with tool context
9. Add daemon mode (HTTP server, in-memory state)
10. Add daemon lifecycle to CLI (start/stop/status)
11. Update `orqa dev` to start daemon
12. Tests for all capabilities
