---
id: EPIC-031
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
  - target: MS-000
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-133
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-134
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-135
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-136
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-137
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-138
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-321
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

- [TASK-133](TASK-133): Implement governance filesystem scanner
- [TASK-134](TASK-134): Implement governance coverage analysis
- [TASK-135](TASK-135): Implement governance recommendations
- [TASK-136](TASK-136): Implement recommendation review UI
- [TASK-137](TASK-137): Implement governance coverage dashboard widget
- [TASK-138](TASK-138): Wire governance end-to-end integration
