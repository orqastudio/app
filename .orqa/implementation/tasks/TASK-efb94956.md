---
id: TASK-efb94956
type: task
title: "Relationships panel — equal columns, overflow toggle, status dots"
description: "Redesign the relationships panel with equal-width columns, expandable overflow, and status dots from graph node data."
status: archived
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Equal width label/value columns
  - "One row per relationship type with '...' to expand overflow, 'hide' to collapse"
  - Each chip shows status dot from graph node data
relationships:
  - target: EPIC-469add1c
    type: delivers
    rationale: Improved relationships panel provides better artifact context at a glance
  - target: TASK-3c586ee4
    type: depends-on
---

## Scope

Redesign ReferencesPanel.svelte and RelationshipsList.svelte. Implement equal-width columns for label and value. Add overflow toggle per relationship type row. Show status dots on relationship chips using enriched graph node data from TASK-3c586ee4.
