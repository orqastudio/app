---
id: TASK-226
title: Fix SKILL-045 rule-enforcement duplication
description: rule-enforcement exists as divergent copies in team/skills/ and plugin/skills/. Symlink or assign distinct IDs.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - rule-enforcement exists in exactly one canonical location or has distinct IDs
  - No divergent copies exist
relationships:
  - target: EPIC-054
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-344
    type: depended-on-by
---

## What

rule-enforcement exists as divergent copies in team/skills/ and plugin/skills/. Symlink or assign distinct IDs.

## How

To be determined during implementation.

## Verification

- [ ] rule-enforcement exists in exactly one canonical location or has distinct IDs
- [ ] No divergent copies exist
