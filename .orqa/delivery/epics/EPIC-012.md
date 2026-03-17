---
id: EPIC-012
title: Process Visibility Dashboard
description: "Build scanner runner, scanner dashboard, metrics dashboard, and agent activity panel for richer process visibility."
status: captured
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 4
  urgency: 3
  complexity: 4
  dependencies: 3
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
---
## Why P1

Pillar 1 (Clarity Through Structure) — governance must be visible, not buried in terminal output.

## Tasks

- [ ] Scanner runner — execute code quality checks (clippy, eslint, tests) and collect results
- [ ] Scanner dashboard — pass/fail history, violation details, trend charts (LayerChart)
- [ ] Metrics dashboard — KPI cards for key project health indicators
- [ ] Agent activity panel — which agent is active, what tools it's using, current task

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
