---
id: TASK-84b816cd
type: task
title: Dashboard components use shared library
description: "Replace native elements in dashboard with shared components: ScrollArea for pipeline health table, SelectMenu for category dropdown, proper toggle pattern for filter buttons."
status: archived
priority: P1
scoring:
  impact: 3
  urgency: 3
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Pipeline health table uses ScrollArea component
  - Category dropdown uses SelectMenu component
  - Error/Warning filter toggles use a proper component pattern (mini-buttons or clickable badges)
relationships:
  - target: EPIC-c96c9f12
    type: delivers
    rationale: Dashboard polish (F30, F32, F33)
---
