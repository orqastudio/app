---
id: TASK-74f5fcdf
type: task
title: "Artifact system review — state machine, definitions, audit"
status: archived
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "State machine reviewed — statuses, transitions, lifecycle validated against actual workflow"
  - "Canonical definitions written — clear criteria for what is a rule vs knowledge vs doc vs decision"
  - "Audit performed against definitions — miscategorised artifacts reclassified"
  - "IDEA-dbebfa2b addressed"
relationships:
  - target: EPIC-4304bdcc
    type: delivers
  - target: TASK-272b3d07
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-272b3d07"
  - target: TASK-cc86ee65
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-cc86ee65"
---
## What

Top-down review per IDEA-dbebfa2b:

1. **State machine** — are statuses, transitions, and lifecycles correct for each artifact type? Do they reflect how work actually flows?
2. **Canonical definitions** — what IS each artifact type? Clear unambiguous criteria.
3. **Audit** — check every artifact against definitions, reclassify as needed.

This is research that will produce implementation tasks.
