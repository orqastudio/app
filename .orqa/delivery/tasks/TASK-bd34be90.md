---
id: "TASK-bd34be90"
type: task
title: "Fix KNOW-e3a559c9 ID collision"
description: "Three skills share KNOW-e3a559c9. Assign unique IDs and update all agent references."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
acceptance:
  - "Every knowledge artifact has a unique KNOW-NNN ID"
  - "All agent frontmatter references resolve to exactly one skill"
relationships:
  - target: "EPIC-6f2d06d4"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-94149697"
    type: "depended-on-by"
---
## What

Three skills share [KNOW-e3a559c9](KNOW-e3a559c9). Assign unique IDs and update all agent references.

## How

To be determined during implementation.

## Verification

- [ ] Every knowledge artifact has a unique KNOW-NNN ID
- [ ] All agent frontmatter references resolve to exactly one skill