---
id: TASK-37bafa1c
type: task
title: Add synchronised-with constraint to core.json for skills
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-d1d42012
    type: delivers
  - target: TASK-14e74691
    type: depended-on-by
  - target: TASK-7c725cf8
    type: depended-on-by
---

# TASK-37bafa1c: Enforce Skill Documentation Constraint

## Acceptance Criteria

1. `core.json` updated: `synchronised-with` relationship gets `constraints.required: true, constraints.minCount: 1` when source type is `skill`
2. Integrity scanner reports skills without `synchronised-with` as errors
3. Existing skills flagged correctly (expected: ~70 violations until docs created)
4. Plugin-provided skills included in the check