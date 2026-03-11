---
id: EPIC-031
title: Governance Bootstrap
description: "The initial governance layer: filesystem scanner, coverage analysis, recommendations, and governance coverage indicator on the dashboard."
status: done
priority: P1
created: "2026-03-02"
updated: "2026-03-07"
milestone: MS-000
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on:
  - EPIC-030
blocks: []
docs-required:
  - DOC-001
docs-produced: []
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 1
  score: 34
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

- TASK-133: Implement governance filesystem scanner
- TASK-134: Implement governance coverage analysis
- TASK-135: Implement governance recommendations
- TASK-136: Implement recommendation review UI
- TASK-137: Implement governance coverage dashboard widget
- TASK-138: Wire governance end-to-end integration
