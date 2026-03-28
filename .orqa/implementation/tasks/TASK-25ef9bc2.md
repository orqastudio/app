---
id: "TASK-25ef9bc2"
type: "task"
title: "Fix KNOW-f5ee4e0d ID collision"
description: "Three skills share KNOW-f5ee4e0d. Assign unique IDs and update all agent references."
status: archived
created: "2026-03-12"
updated: "2026-03-12"
acceptance:
  - "Every knowledge artifact has a unique KNOW-NNN ID"
  - "All agent frontmatter references resolve to exactly one skill"
relationships:
  - target: "EPIC-770f9ce9"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Three skills share [KNOW-f5ee4e0d](KNOW-f5ee4e0d). Assign unique IDs and update all agent references.

## How

To be determined during implementation.

## Verification

- [ ] Every knowledge artifact has a unique KNOW-NNN ID
- [ ] All agent frontmatter references resolve to exactly one skill
