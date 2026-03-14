---
id: TASK-481
title: "Build ImprovementTrendsWidget with sparklines and trend arrows"
description: "Build a 2x2 sparkline grid widget showing error count trend, warning count trend, rule count trend, and pass rate trend. Each cell displays the current value, a trend arrow, and percentage change. Include a timeframe selector. Place in the 'How You're Improving' column."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-074
depends-on:
  - TASK-479
acceptance:
  - 2x2 sparkline grid shows error count trend, warning count trend, rule count trend, and pass rate trend
  - Each cell displays current value, trend arrow, and percentage change
  - Timeframe selector present and controls the displayed data range
  - Widget placed in the "How You're Improving" column
relationships:
  - target: EPIC-074
    type: delivers
    rationale: Improvement trends widget for the dashboard redesign
  - target: EPIC-074
    type: belongs-to
    rationale: Task belongs to this epic
---
