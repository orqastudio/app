---
id: TASK-483
title: "Refactor KnowledgePipelineWidget as collapsible bottom section"
description: "Move the knowledge pipeline widget into the collapsible bottom section of the new dashboard layout. The section collapses by default. When expanded, the widget shows the 6-stage pipeline with connectivity health indicators."
status: done
priority: P2
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-074
depends-on:
  - TASK-479
acceptance:
  - Knowledge pipeline widget rendered in the collapsible bottom section of the dashboard
  - Section is collapsed by default
  - When expanded, the 6-stage pipeline is visible with connectivity health indicators
relationships:
  - target: EPIC-074
    type: delivers
    rationale: Knowledge pipeline widget relocation for the dashboard redesign
  - target: EPIC-074
    type: belongs-to
    rationale: Task belongs to this epic
---
