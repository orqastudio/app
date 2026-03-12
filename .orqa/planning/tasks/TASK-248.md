---
id: TASK-248
title: "Audit backend-only commands — identify and remove orphaned code"
description: "29 of 75 registered commands have no frontend callers. Some are used by sidecar tool loop, others may be orphaned. Audit and clean up."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "Every registered command is documented as: frontend-called, sidecar-called, or removed"
  - "Orphaned commands are removed from registration and their handler code deleted"
  - "make check passes"
---

## What

29 of 75 registered commands have no frontend callers. Some are used by sidecar tool loop, others may be orphaned. Audit and clean up.

## How

To be determined during implementation.

## Verification

- [ ] Every registered command is documented as: frontend-called, sidecar-called, or removed
- [ ] Orphaned commands are removed from registration and their handler code deleted
- [ ] make check passes
