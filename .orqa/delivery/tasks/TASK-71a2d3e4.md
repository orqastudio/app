---
id: TASK-71a2d3e4
title: "Remap all service ports to 10000+ range"
type: task
description: "Audit all port constants and config files across the codebase and update them to use the 10000+ range. Create a single canonical port allocation table."
status: completed
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - All port constants in CLI, MCP server, search engine, Vite config, and Tauri config use ports above 10000
  - A canonical port allocation table exists in one location (e.g., project config or documentation)
  - No hardcoded port values below 10000 remain for OrqaStudio services
  - search_regex confirms no stale port references remain
  - make check passes after all port changes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Phase 1 of port allocation epic"
  - target: TASK-82b3e4f5
    type: depended-on-by
    rationale: "Auto-generated inverse of depends-on relationship from TASK-82b3e4f5"
  - target: TASK-93c4f5a6
    type: depended-on-by
    rationale: "Auto-generated inverse of depends-on relationship from TASK-93c4f5a6"
  - target: TASK-f3a4b5c6
    type: depended-on-by
  - target: TASK-b7c8d9e0
    type: depended-on-by
  - target: TASK-a5b6c7d8
    type: depended-on-by
---

## What

Audit every config file and source file for port assignments. Update all OrqaStudio service ports to the 10000+ range per the port allocation table in the epic.

## Files to Check (Complete Audit)

**Daemon port (3002):**
- `libs/cli/src/commands/daemon.ts:14` — `DEFAULT_PORT = 3002`
- `libs/cli/src/commands/enforce.ts:115` — `const port = 3002`
- `connectors/claude-code/src/hooks/shared.ts:11` — `DAEMON_BASE = "http://localhost:3002"`
- `libs/validation/src/bin/server.rs:562` — fallback default `3002`
- `app/backend/src-tauri/src/commands/daemon_commands.rs:32` — hardcoded `127.0.0.1:3002`

**Daemon port (9258):**
- `libs/mcp-server/src/daemon.rs:22` — `DEFAULT_DAEMON_PORT: u16 = 9258`
- `libs/mcp-server/src/bin/server.rs:50` — defaults to 9258
- `libs/lsp-server/src/bin/server.rs:28` — `DEFAULT_DAEMON_PORT: u16 = 9258`

**App LSP hardcoded port:**
- `app/backend/src-tauri/src/servers/lsp.rs:15` — hardcoded `3002` (see [TASK-f3a4b5c6](TASK-f3a4b5c6))

**Dashboard port (3001):**
- `tools/debug/dev.mjs:54` — `DASHBOARD_PORT`
- `libs/logger/src/index.ts:40` — forwards to `localhost:3001`
- `app/ui/src/lib/utils/dev-console.ts:13` — forwards to `localhost:3001`
- `connectors/claude-code/src/hooks/telemetry.ts:8` — forwards to `localhost:3001`
- `infrastructure/sync-bridge/src/config.ts:20` — defaults to `3001` (see [TASK-a5b6c7d8](TASK-a5b6c7d8))

**Vite dev server (1420):**
- `tools/debug/dev.mjs:53` — `VITE_PORT`
- `app/backend/src-tauri/tauri.conf.json:8` — `devUrl`

**Infrastructure (Forgejo):**
- `infrastructure/orqastudio-git/docker-compose.yml:37-38` — ports 3030 and 222

## Verification

1. `search_regex` for port numbers 1420, 3001, 3002, 3030, 9258 returns zero results after changes
2. All services start on their new ports
3. `make check` passes
