---
id: TASK-491
title: "Fix roadmap drag-drop, add sort dropdown, column headers (F37, F38, F39)"
description: "Fix drag-and-drop between roadmap kanban columns so it updates the underlying artifact field. Add a sort/group dropdown to the kanban. Fix column headers to remove the item count, show done progress as 'X/X done', capitalise words, and use badge styling."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on:
  - TASK-490
acceptance:
  - Drag-drop between columns works and updates the artifact's status/field
  - Sort/group dropdown present on the kanban view
  - Column headers show "X/X done" progress format instead of "X items"
  - Column header words are capitalised
  - Column headers use badge styling
relationships:
  - target: EPIC-073
    type: delivers
    rationale: UAT findings F37, F38, F39 — roadmap drag-drop, sort dropdown, and column header fixes
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
