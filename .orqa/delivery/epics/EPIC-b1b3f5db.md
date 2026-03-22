---
id: EPIC-b1b3f5db
title: Governance Bootstrap
description: "The initial governance layer: filesystem scanner, coverage analysis, recommendations, and governance coverage indicator on the dashboard."
status: completed
priority: P1
created: 2026-03-02
updated: 2026-03-07
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: MS-85b9269b
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-71c613fb
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-975eb726
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-1c443823
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-41cb6dda
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-132f8783
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-2000d343
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-51007152
    type: delivered-by
    rationale: Epic contains this task
---
## Why P1

Orqa Studio's Pillar 2 (Process Governance) requires the app to be able to inspect and reason about its own governance. Without this, governance is invisible — documents that exist but can't be surfaced in the app.

## What Was Done

- Governance scanner — filesystem walk collecting `.claude/` agents, rules, skills, and hooks
- Governance analysis — evaluates collected artifacts and identifies coverage gaps
- Recommendations — structured suggestions based on coverage analysis
- Recommendation review and approval UI — user can review and act on suggestions
- Governance coverage indicator — dashboard widget showing coverage health at a glance

## Notes

Retroactively captured. Work preceded the artifact framework. This capability underpins the governance browsing and enforcement features built in later milestones.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-71c613fb](TASK-71c613fb): Implement governance filesystem scanner
- [TASK-975eb726](TASK-975eb726): Implement governance coverage analysis
- [TASK-1c443823](TASK-1c443823): Implement governance recommendations
- [TASK-41cb6dda](TASK-41cb6dda): Implement recommendation review UI
- [TASK-132f8783](TASK-132f8783): Implement governance coverage dashboard widget
- [TASK-2000d343](TASK-2000d343): Wire governance end-to-end integration
