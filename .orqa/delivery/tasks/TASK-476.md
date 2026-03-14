---
id: TASK-476
title: "Dashboard components use shared library"
description: "Replace native elements in dashboard with shared components: ScrollArea for pipeline health table, SelectMenu for category dropdown, proper toggle pattern for filter buttons."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on: []
assignee: null
skills:
  - SKILL-030
  - SKILL-041
acceptance:
  - Pipeline health table uses ScrollArea component
  - Category dropdown uses SelectMenu component
  - Error/Warning filter toggles use a proper component pattern (mini-buttons or clickable badges)
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Dashboard polish (F30, F32, F33)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
