---
id: TASK-492
title: "Roadmap all-done state and Done column collapse logic (F36)"
description: "Implement the all-completed state for the roadmap kanban: when all items are in the Done column, show a celebration/completion state with a 'View board' button to reveal the full board. Fix Done column collapse logic so it only auto-collapses when non-done items exist."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on:
  - TASK-490
acceptance:
  - When all roadmap items are Done, an "All completed" state is shown with a "View board" button
  - Done column only auto-collapses when non-done items exist in the board
  - Expanding the Done column shows the full column with all done items
relationships:
  - target: EPIC-073
    type: delivers
    rationale: UAT finding F36 — roadmap all-done state and Done column collapse logic
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
