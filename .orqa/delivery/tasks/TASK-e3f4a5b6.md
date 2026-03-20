---
id: TASK-e3f4a5b6
type: task
title: "Debug MCP/LSP server registration"
description: "Investigate and fix the MCP/LSP server registration issue. Determine why the server is not registering correctly and implement the fix."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - Root cause of MCP/LSP registration failure identified and documented
  - Fix implemented with no unwrap()/expect() in Rust code
  - MCP server registers successfully on app startup
  - LSP server registers successfully on app startup
  - make test passes after fix
relationships:
  - target: EPIC-663d52ac
    type: delivers
---

## What

Investigate the MCP/LSP server registration bug. This is an independent debug track — it does not depend on the knowledge rename work and can proceed in parallel.

The registration issue was noted in the pre-switch audit (commit a3c6ea5). Determine:
1. Which registration step is failing (MCP server, LSP server, or both)
2. Whether it's a timing issue, configuration issue, or code bug
3. The minimal fix to make both servers register correctly

## How

Use diagnostic methodology:
1. Read the current registration code in `backend/src-tauri/src/` for MCP and LSP
2. Check `AD-059` (Central LSP/MCP registration decision) for the intended design
3. Identify the gap between the design and the current implementation
4. Implement the fix following RULE-010 (end-to-end completeness) — all layers updated together

Search `tmp/session-state.md` and recent git log for any prior debugging notes on this issue.

Error handling: all Rust functions must return `Result<T, E>` — no `unwrap()` or `expect()`.

## Verification

1. MCP server connects successfully on app startup (visible in logs)
2. LSP server registers successfully (no registration error in logs)
3. `make test` passes
4. `make lint-backend` passes (zero warnings)
5. The fix is consistent with AD-059 design intent
