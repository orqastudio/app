---
id: TASK-72bb47b5
title: "Fix daemon port mismatch between CLI and MCP server"
type: task
description: "Align the daemon port constant in CLI (libs/cli/src/commands/daemon.ts) and MCP server (libs/mcp-server/src/daemon.rs) so both use the same default port. Remove the --port 9258 workaround."
status: archived
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - CLI daemon.ts and MCP server daemon.rs use the same default port constant
  - orqa daemon start (without --port flag) works correctly with the MCP server
  - The --port 9258 workaround is no longer needed
  - Port value comes from the canonical port allocation table (TASK-35444c5b)
  - make check passes
relationships:
  - target: EPIC-9e3d320b
    type: delivers
    rationale: "Phase 2 of port allocation epic"
  - target: TASK-35444c5b
    type: depends-on
    rationale: "Port value comes from the canonical allocation table"
---

## What

The daemon port is mismatched: CLI starts on 3002, MCP server expects 9258. This task aligns both to the new canonical port from the port allocation table.

## Files to Modify

**CLI ecosystem (currently 3002):**

- `libs/cli/src/commands/daemon.ts:14` — `DEFAULT_PORT`
- `libs/cli/src/commands/enforce.ts:115` — hardcoded port
- `connectors/claude-code/src/hooks/shared.ts:11` — `DAEMON_BASE`
- `libs/validation/src/bin/server.rs:562` — fallback default
- `app/backend/src-tauri/src/commands/daemon_commands.rs:32` — hardcoded address

**Rust library ecosystem (currently 9258):**

- `libs/mcp-server/src/daemon.rs:22` — `DEFAULT_DAEMON_PORT`
- `libs/mcp-server/src/bin/server.rs:50` — defaults to 9258
- `libs/lsp-server/src/bin/server.rs:28` — `DEFAULT_DAEMON_PORT`

**App LSP (currently 3002, separate task):**

- `app/backend/src-tauri/src/servers/lsp.rs:15` — see [TASK-ec4a3c53](TASK-ec4a3c53)

## Verification

1. `orqa daemon start` starts on the canonical port
2. MCP server connects to the daemon without `--port` override
3. `orqa daemon status` reports correctly
