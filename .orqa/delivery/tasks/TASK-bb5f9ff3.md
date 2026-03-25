---
id: TASK-bb5f9ff3
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
  - "IDEA-102f7014 addressed"
relationships:
  - target: EPIC-c828007a
    type: delivers
  - target: TASK-2c9e0bb4
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-2c9e0bb4"
  - target: TASK-25b352ce
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-25b352ce"
---
## What

Top-down review per IDEA-102f7014:

1. **State machine** — are statuses, transitions, and lifecycles correct for each artifact type? Do they reflect how work actually flows?
2. **Canonical definitions** — what IS each artifact type? Clear unambiguous criteria.
3. **Audit** — check every artifact against definitions, reclassify as needed.

This is research that will produce implementation tasks.
