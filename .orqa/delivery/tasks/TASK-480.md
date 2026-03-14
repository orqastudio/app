---
id: TASK-480
title: "Refactor GraphHealthWidget for clarity-focused display"
description: "Redesign the graph health widget to emphasise at-a-glance status clarity: a large status indicator, summary score, breakdown of errors and warnings, and a scan trigger button. Place in the 'Where You Are' column."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-074
depends-on:
  - TASK-479
acceptance:
  - Graph health widget shows large status indicator (green/amber/red)
  - Summary score displayed prominently
  - Breakdown of error count and warning count visible
  - "Run integrity scan" button present and functional
  - Widget placed in the "Where You Are" column
relationships:
  - target: EPIC-074
    type: delivers
    rationale: Graph health widget redesign for clarity
  - target: EPIC-074
    type: belongs-to
    rationale: Task belongs to this epic
---
