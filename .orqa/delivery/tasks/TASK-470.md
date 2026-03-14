---
id: TASK-470
title: "Rework ArtifactLink component — status dot inside, ellipsis, hover popover"
description: "Systematic rework of the ArtifactLink component: move status dot inside the chip, add max-width with text-overflow ellipsis for title display, add hover popover showing artifact metadata from the graph."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on: []
assignee: null
skills:
  - SKILL-030
  - SKILL-042
  - SKILL-041
acceptance:
  - Status dot renders inside the artifact link chip
  - Title display mode has max-width with text-overflow ellipsis
  - Hovering over a chip shows a popover with artifact metadata from the graph
  - Relationships no longer appear in the metadata card — only in RelationshipsList
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Artifact link system rework (F6, F7, F9, F10)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
