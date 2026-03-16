---
id: TASK-286
title: Create AD for directory reorganization (AD-043)
description: Architecture decision formalizing the three-level structure (process/delivery/documentation) and the first-class artifact principle.
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - AD-043 exists in .orqa/process/decisions/
  - Documents the three-level structure with rationale
  - Defines the first-class artifact principle
  - Maps current structure to target structure
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-287
    type: depended-on-by
  - target: TASK-288
    type: depended-on-by
  - target: TASK-289
    type: depended-on-by
  - target: TASK-349
    type: depended-on-by
---

## What

Create an architecture decision documenting the directory reorganization from governance/team/planning to process/delivery/documentation.

## How

1. Create `.orqa/process/decisions/[AD-043](AD-043).md`
2. Document: current structure, target structure, rationale, migration approach
3. Define the first-class artifact principle formally

## Verification

- [AD-043](AD-043) exists and passes schema validation
- Decision clearly maps old paths to new paths
