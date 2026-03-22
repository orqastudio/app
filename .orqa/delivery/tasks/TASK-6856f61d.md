---
id: TASK-6856f61d
title: Fix KNOW-bcfeb64e rule-enforcement duplication
description: rule-enforcement exists as divergent copies in team/skills/ and plugin/skills/. Symlink or assign distinct IDs.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - rule-enforcement exists in exactly one canonical location or has distinct IDs
  - No divergent copies exist
relationships:
  - target: EPIC-6f2d06d4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-94149697
    type: depended-on-by
---

## What

rule-enforcement exists as divergent copies in team/skills/ and plugin/skills/. Symlink or assign distinct IDs.

## How

To be determined during implementation.

## Verification

- [ ] rule-enforcement exists in exactly one canonical location or has distinct IDs
- [ ] No divergent copies exist
