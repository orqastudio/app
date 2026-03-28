---
id: "EPIC-8cba3805"
type: "epic"
title: "Governance Bootstrap"
description: "The initial governance layer: filesystem scanner, coverage analysis, recommendations, and governance coverage indicator on the dashboard."
status: archived
priority: "P1"
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-07T00:00:00.000Z
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "MS-063c15b9"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
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

- [TASK-c874fef2](TASK-c874fef2): Implement governance filesystem scanner
- [TASK-2f87a454](TASK-2f87a454): Implement governance coverage analysis
- [TASK-0447b20e](TASK-0447b20e): Implement governance recommendations
- [TASK-bd756cec](TASK-bd756cec): Implement recommendation review UI
- [TASK-9841c4de](TASK-9841c4de): Implement governance coverage dashboard widget
- [TASK-65d79ace](TASK-65d79ace): Wire governance end-to-end integration
