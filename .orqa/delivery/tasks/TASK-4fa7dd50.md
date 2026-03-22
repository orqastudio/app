---
id: TASK-4fa7dd50
title: Implement plan-before-build + structure-before-code gates
description: |
  Process gates that fire when code is written without epic/task context or
  without a plan being referenced.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - plan-before-build gate fires on code write without prior epic/task reads
  - structure-before-code gate fires on code write without .orqa/delivery/ reads
  - Gates fire only once per session
relationships:
  - target: EPIC-4440cdd4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-ff48daa1
    type: depends-on
  - target: TASK-34007190
    type: depended-on-by
  - target: TASK-d6727d2f
    type: depended-on-by
---

## What

Two process gates:
- **plan-before-build**: Fires on code write without any epic/task artifacts
  being read in the session. Injects planning prompt.
- **structure-before-code**: Fires on code write without `.orqa/delivery/` files
  being read. Injects structure prompt.

## How

1. Add gate logic checking WorkflowTracker for `.orqa/delivery/` reads
2. Return `systemMessage` with planning prompts
3. Gate fires once per session

## Verification

- Write code without reading any planning artifacts → warning fires
- Read epic/task first, then write → no warning
