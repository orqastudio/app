---
id: TASK-2eeb847c
title: Add synchronised-with constraint to core.json for skills
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-8f8b1dba
    type: depended-on-by
  - target: TASK-bd4e7250
    type: depended-on-by
---

# TASK-2eeb847c: Enforce Skill Documentation Constraint

## Acceptance Criteria

1. `core.json` updated: `synchronised-with` relationship gets `constraints.required: true, constraints.minCount: 1` when source type is `skill`
2. Integrity scanner reports skills without `synchronised-with` as errors
3. Existing skills flagged correctly (expected: ~70 violations until docs created)
4. Plugin-provided skills included in the check
