---
id: TASK-f3a4b5c6
title: "Fix app LSP hardcoded daemon port"
type: task
description: "The app backend at app/backend/src-tauri/src/servers/lsp.rs:15 hardcodes 3002 as the daemon port passed to orqa_lsp_server::run_stdio(). This conflicts with the LSP server's own default of 9258 and must be updated to the canonical port 10258."
status: completed
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - app/backend/src-tauri/src/servers/lsp.rs uses the canonical daemon port (10258) instead of hardcoded 3002
  - The port value comes from the canonical port allocation table, not a magic number
  - App-embedded LSP connects to the same daemon port as the standalone LSP binary
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Fixes app LSP hardcoded port discovered during port audit"
  - target: TASK-71a2d3e4
    type: depends-on
    rationale: "Canonical port allocation must be established first"
---

## What

`app/backend/src-tauri/src/servers/lsp.rs:15` passes `3002` as the daemon port to `orqa_lsp_server::run_stdio()`. This means the app-embedded LSP connects to the daemon on 3002, while the standalone LSP binary defaults to 9258. After port remapping, both must use 10258.

## Root Cause

The app LSP integration was written against the CLI default (3002) rather than using a shared constant. The LSP server library itself defaults to 9258 (the MCP/LSP ecosystem default). This is a separate hardcoded value that needs its own fix beyond the general port remapping task.

## Files to Modify

- `app/backend/src-tauri/src/servers/lsp.rs` — replace hardcoded `3002` with canonical port constant or config lookup

## Verification

1. `search_regex` for `3002` in `app/backend/src-tauri/src/servers/lsp.rs` returns zero results
2. App-embedded LSP connects to daemon on the same port as standalone LSP
3. `make check` passes
