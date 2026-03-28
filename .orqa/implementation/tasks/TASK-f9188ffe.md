---
id: "TASK-f9188ffe"
type: "task"
title: "Implement understand-first + docs-before-code gates"
description: "Process gates that fire when the first code write in a session happens without\nprior research or documentation reading.\n"
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "understand-first gate fires on first code write with no prior reads/searches"
  - "docs-before-code gate fires on code write without prior .orqa/documentation/ reads"
  - "Gates fire only once per session"
  - "Gates return systemMessage, not block"
relationships:
  - target: "EPIC-56940fa8"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-f53b9368"
    type: "depends-on"
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
