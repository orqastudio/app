---
id: TASK-297
title: Documentation inventory
description: "Audit documentation/: what docs NEED to exist vs what IS there. Identify stale content. Every remaining doc must be human-consumption content. Flag placeholder directories."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - Inventory of all documentation files with current/needed status
  - Stale content identified and removed or updated
  - All remaining docs are human-consumption content
  - Placeholder directories flagged or removed
  - "Gap analysis: what docs are missing"
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-285
    type: depends-on
  - target: TASK-349
    type: depended-on-by
---

## What

Full inventory of documentation/ to understand current state and identify gaps.

## How

1. List all files in documentation/ recursively
2. For each: is it current? stale? placeholder? human-consumption or process-definition?
3. Remove stale content
4. Flag gaps — what documentation SHOULD exist but doesn't
5. Ensure remaining docs reference process artifacts rather than defining them

## Verification

- Documentation inventory report
- No stale or placeholder content remains
- Gap list for future documentation work
