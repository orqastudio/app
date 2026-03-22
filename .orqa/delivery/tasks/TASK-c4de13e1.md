---
id: TASK-c4de13e1
title: Dynamic artifact table component
description: "Support live artifact tables in markdown that show status, title, and priority from the graph."
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 3
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Markdown renderer supports syntax for live artifact tables
  - "Tables show status, title, priority from graph"
  - "Works for tasks-on-epic, epics-on-milestone"
relationships:
  - target: EPIC-f684378f
    type: delivers
    rationale: Dynamic tables replace manual artifact lists with live graph data
  - target: TASK-e9219bfd
    type: depends-on
---

## Scope

Create new DynamicArtifactTable.svelte component. Extend the markdown renderer to detect a special syntax for artifact tables and render the component. Tables query the graph for artifacts matching the specified relationship and display status, title, and priority.
