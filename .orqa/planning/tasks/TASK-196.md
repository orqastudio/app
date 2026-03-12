---
id: TASK-196
title: Implement understand-first + docs-before-code gates
description: |
  Process gates that fire when the first code write in a session happens without
  prior research or documentation reading.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-195
---

## What

Two process gates:
- **understand-first**: Fires on first code write with no prior file reads, searches,
  or code research calls. Injects systems thinking prompt.
- **docs-before-code**: Fires on code write without reading any `.orqa/documentation/`
  files. Injects documentation prompt.

## How

1. Add gate logic to enforcement evaluation in both plugin and Rust engine
2. Query WorkflowTracker for read/search history before evaluating write events
3. Return `systemMessage` with thinking prompts (not block — these are nudges)
4. Gate only fires once per session (first code write)

## Verification

- Write to `backend/src-tauri/` with no prior reads → warning fires
- Read docs first, then write → no warning
- Warning only fires once per session
