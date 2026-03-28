---
id: "TASK-6aa8e1f1"
type: "task"
title: "Create AD for directory reorganization (AD-9687b3cf)"
description: "Architecture decision formalizing the three-level structure (process/delivery/documentation) and the first-class artifact principle."
status: archived
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "AD-9687b3cf exists in .orqa/process/decisions/"
  - "Documents the three-level structure with rationale"
  - "Defines the first-class artifact principle"
  - "Maps current structure to target structure"
rule-overrides: []
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Create an architecture decision documenting the directory reorganization from governance/team/planning to process/delivery/documentation.

## How

1. Create `.orqa/process/decisions/[AD-9687b3cf](AD-9687b3cf).md`
2. Document: current structure, target structure, rationale, migration approach
3. Define the first-class artifact principle formally

## Verification

- [AD-9687b3cf](AD-9687b3cf) exists and passes schema validation
- Decision clearly maps old paths to new paths
