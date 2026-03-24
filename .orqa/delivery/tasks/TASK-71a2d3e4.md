---
id: TASK-71a2d3e4
title: "Remap all service ports to 10000+ range"
type: task
description: "Audit all port constants and config files across the codebase and update them to use the 10000+ range. Create a single canonical port allocation table."
status: ready
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
---

## What

Audit every config file and source file for port assignments. Update all OrqaStudio service ports to the 10000+ range per the port allocation table in the epic.

## Files to Check

- `libs/cli/src/commands/daemon.ts` — DEFAULT_PORT
- `libs/mcp-server/src/daemon.rs` — DEFAULT_DAEMON_PORT
- `ui/vite.config.ts` — Vite dev server port
- `backend/src-tauri/tauri.conf.json` — devUrl port
- Any docker-compose, env files, or other config referencing ports
- Documentation referencing specific port numbers

## Verification

1. `search_regex` for port numbers 1420, 3002, 9258 returns zero results after changes
2. All services start on their new ports
3. `make check` passes
