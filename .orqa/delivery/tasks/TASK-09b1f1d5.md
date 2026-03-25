---
id: TASK-09b1f1d5
type: task
title: "Fix roadmap drag-drop, add sort dropdown, column headers (F37, F38, F39)"
description: "Fix drag-and-drop between roadmap kanban columns so it updates the underlying artifact field. Add a sort/group dropdown to the kanban. Fix column headers to remove the item count, show done progress as 'X/X done', capitalise words, and use badge styling."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "Drag-drop between columns works and updates the artifact's status/field"
  - Sort/group dropdown present on the kanban view
  - "Column headers show \"X/X done\" progress format instead of \"X items"
  - Column header words are capitalised
  - Column headers use badge styling
relationships:
  - target: EPIC-c96c9f12
    type: delivers
    rationale: UAT findings F37, F38, F39 — roadmap drag-drop, sort dropdown, and column header fixes
  - target: TASK-2665cbf6
    type: depends-on
---