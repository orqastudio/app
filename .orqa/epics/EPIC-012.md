---
id: EPIC-012
title: "Process Visibility Dashboard"
status: draft
priority: P1
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 5
  impact: 4
  dependency: 2
  effort: 4
score: 6.8
roadmap-ref: "M2"
docs-required:
  - docs/wireframes/dashboard.md
  - docs/architecture/enforcement.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (dashboard plan)
  - docs/wireframes/dashboard.md (update with scanner and metrics wireframes)
  - docs/architecture/rust-modules.md (update with scanner runner module)
tags: [dashboard, scanners, metrics]
---

# Process Visibility Dashboard

Governance coverage exists. Missing: richer process visibility.

## Why P1

Pillar 1 (Clarity Through Structure) — governance must be visible, not buried in terminal output.

## Tasks

- [ ] Scanner runner — execute code quality checks (clippy, eslint, tests) and collect results
- [ ] Scanner dashboard — pass/fail history, violation details, trend charts (LayerChart)
- [ ] Metrics dashboard — KPI cards for key project health indicators
- [ ] Agent activity panel — which agent is active, what tools it's using, current task
