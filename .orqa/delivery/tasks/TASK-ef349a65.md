---
id: "TASK-ef349a65"
type: "task"
title: "Graph health snapshot storage and trend sparklines"
description: "Store graph health snapshots on each integrity scan and display trend sparklines on the dashboard showing how graph health metrics change over time."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "Tauri command stores a health snapshot (node count, edge count, orphans, broken refs, error count, warning count) with timestamp"
  - "Tauri command retrieves recent snapshots for trend display"
  - "SVG sparklines show trends for key metrics on the dashboard"
  - "Snapshots stored in SQLite metrics table"
  - "make typecheck passes (no new errors)"
relationships:
  - target: "EPIC-82dd0bd2"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-6ec25d4b"
    type: "depends-on"
---

## What

Track graph health over time so users can see whether pipeline health is improving or degrading.

## How

1. Add `health_snapshots` SQLite table with graph metrics columns
2. Add `store_health_snapshot` and `get_health_snapshots` Tauri commands
3. After each integrity scan in IntegrityWidget, auto-store a snapshot
4. Create SparklineWidget showing trends for key metrics
5. Add to dashboard below IntegrityWidget

## Verification

- Snapshots persist across app restarts
- Sparklines update after each scan
