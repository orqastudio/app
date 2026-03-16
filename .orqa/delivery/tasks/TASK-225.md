---
id: TASK-225
title: Fix SKILL-046 ID collision
description: Three skills share SKILL-046. Assign unique IDs and update all agent references.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - Every skill has a unique SKILL-NNN ID
  - All agent frontmatter references resolve to exactly one skill
relationships:
  - target: EPIC-054
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-344
    type: depended-on-by
---

## What

Three skills share [SKILL-046](SKILL-046). Assign unique IDs and update all agent references.

## How

To be determined during implementation.

## Verification

- [ ] Every skill has a unique SKILL-NNN ID
- [ ] All agent frontmatter references resolve to exactly one skill
