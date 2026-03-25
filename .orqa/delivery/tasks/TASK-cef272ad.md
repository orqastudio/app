---
id: "TASK-cef272ad"
type: "task"
title: "Audit backend-only commands — identify and remove orphaned code"
description: "29 of 75 registered commands have no frontend callers. Some are used by sidecar tool loop, others may be orphaned. Audit and clean up."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "Every registered command is documented as: frontend-called, sidecar-called, or removed"
  - "Orphaned commands are removed from registration and their handler code deleted"
  - "make check passes"
relationships:
  - target: "EPIC-a1555708"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

29 of 75 registered commands have no frontend callers. Some are used by sidecar tool loop, others may be orphaned. Audit and clean up.

## How

To be determined during implementation.

## Verification

- [ ] Every registered command is documented as: frontend-called, sidecar-called, or removed
- [ ] Orphaned commands are removed from registration and their handler code deleted
- [ ] make check passes