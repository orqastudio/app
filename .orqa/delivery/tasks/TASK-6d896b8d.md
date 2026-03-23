---
id: TASK-6d896b8d
type: task
title: "Fix roadmap column layout and ScrollArea scrolling (F34, F35)"
description: "Fix two roadmap layout bugs: columns must evenly fill the panel width rather than collapsing to content width, and the ScrollArea inside each column must scroll correctly so cards are not clipped or hidden."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - Roadmap columns evenly fill panel width
  - ScrollArea inside columns scrolls correctly
  - Cards not clipped or hidden by the column container
relationships:
  - target: EPIC-b2ca1ea3
    type: delivers
    rationale: UAT findings F34 and F35 — roadmap column layout and scroll fixes
  - target: TASK-697497a9
    type: depended-on-by
  - target: TASK-d7ef4c9d
    type: depended-on-by
---