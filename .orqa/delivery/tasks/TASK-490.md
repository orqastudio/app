---
id: TASK-490
title: "Fix roadmap column layout and ScrollArea scrolling (F34, F35)"
description: "Fix two roadmap layout bugs: columns must evenly fill the panel width rather than collapsing to content width, and the ScrollArea inside each column must scroll correctly so cards are not clipped or hidden."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on: []
acceptance:
  - Roadmap columns evenly fill panel width
  - ScrollArea inside columns scrolls correctly
  - Cards not clipped or hidden by the column container
relationships:
  - target: EPIC-073
    type: delivers
    rationale: UAT findings F34 and F35 — roadmap column layout and scroll fixes
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
