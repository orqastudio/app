---
id: TASK-82b3e4f5
title: "Fix daemon port mismatch between CLI and MCP server"
type: task
description: "Align the daemon port constant in CLI (libs/cli/src/commands/daemon.ts) and MCP server (libs/mcp-server/src/daemon.rs) so both use the same default port. Remove the --port 9258 workaround."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - CLI daemon.ts and MCP server daemon.rs use the same default port constant
  - orqa daemon start (without --port flag) works correctly with the MCP server
  - The --port 9258 workaround is no longer needed
  - Port value comes from the canonical port allocation table (TASK-71a2d3e4)
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Phase 2 of port allocation epic"
  - target: TASK-71a2d3e4
    type: depends-on
    rationale: "Port value comes from the canonical allocation table"
---

## What

The daemon port is mismatched: CLI starts on 3002, MCP server expects 9258. This task aligns both to the new canonical port from the port allocation table.

## Files to Modify

- `libs/cli/src/commands/daemon.ts` — change DEFAULT_PORT
- `libs/mcp-server/src/daemon.rs` — change DEFAULT_DAEMON_PORT
- Any connector/hook scripts that reference the daemon port

## Verification

1. `orqa daemon start` starts on the canonical port
2. MCP server connects to the daemon without `--port` override
3. `orqa daemon status` reports correctly
