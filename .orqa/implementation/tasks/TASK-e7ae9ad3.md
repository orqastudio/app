---
id: TASK-e7ae9ad3
type: task
title: Refactor KnowledgePipelineWidget as collapsible bottom section
description: "Move the knowledge pipeline widget into the collapsible bottom section of the new dashboard layout. The section collapses by default. When expanded, the widget shows the 6-stage pipeline with connectivity health indicators."
status: archived
priority: P2
scoring:
  impact: 2
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - Knowledge pipeline widget rendered in the collapsible bottom section of the dashboard
  - Section is collapsed by default
  - "When expanded, the 6-stage pipeline is visible with connectivity health indicators"
relationships:
  - target: EPIC-c353971b
    type: delivers
    rationale: Knowledge pipeline widget relocation for the dashboard redesign
  - target: TASK-ddcab065
    type: depends-on
---
