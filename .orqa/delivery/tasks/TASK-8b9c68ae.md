---
id: "TASK-8b9c68ae"
type: task
title: "Create AD for directory reorganization (AD-71d44f5c)"
description: "Architecture decision formalizing the three-level structure (process/delivery/documentation) and the first-class artifact principle."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "AD-71d44f5c exists in .orqa/process/decisions/"
  - "Documents the three-level structure with rationale"
  - "Defines the first-class artifact principle"
  - "Maps current structure to target structure"
rule-overrides: []
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-4b293b82"
    type: "depended-on-by"
  - target: "TASK-1dad83f7"
    type: "depended-on-by"
  - target: "TASK-0cddbf9b"
    type: "depended-on-by"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Create an architecture decision documenting the directory reorganization from governance/team/planning to process/delivery/documentation.

## How

1. Create `.orqa/process/decisions/[AD-71d44f5c](AD-71d44f5c).md`
2. Document: current structure, target structure, rationale, migration approach
3. Define the first-class artifact principle formally

## Verification

- [AD-71d44f5c](AD-71d44f5c) exists and passes schema validation
- Decision clearly maps old paths to new paths