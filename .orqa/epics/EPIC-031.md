---
id: EPIC-031
title: "Phase 2b — Governance Bootstrap"
status: done
priority: P1
milestone: MS-000
created: 2026-03-02
updated: 2026-03-07
deadline: null
plan: null
depends-on: [EPIC-030]
blocks: []
assignee: null
pillar:
  - clarity-through-structure
  - learning-through-reflection
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 1
score: 28.0
roadmap-ref: "Phase 2b"
docs-required:
  - docs/architecture/decisions.md
docs-produced: []
tags: [foundation, governance, scanner, analysis, recommendations]
---

# Phase 2b — Governance Bootstrap

The initial governance layer: a filesystem scanner that collects governance artifacts, analyses them for coverage gaps, produces recommendations, and surfaces a coverage indicator on the dashboard.

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
