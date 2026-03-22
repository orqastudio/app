---
id: TASK-75f8d4bf
title: Refactor GraphHealthWidget for clarity-focused display
description: "Redesign the graph health widget to emphasise at-a-glance status clarity: a large status indicator, summary score, breakdown of errors and warnings, and a scan trigger button. Place in the 'Where You Are' column."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - Graph health widget shows large status indicator (green/amber/red)
  - Summary score displayed prominently
  - Breakdown of error count and warning count visible
  - "Run integrity scan\" button present and functional"
  - "Widget placed in the \"Where You Are\" column"
relationships:
  - target: EPIC-11561c51
    type: delivers
    rationale: Graph health widget redesign for clarity
  - target: TASK-2e078ea2
    type: depends-on
---
