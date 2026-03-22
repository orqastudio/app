---
id: TASK-1f8f15b9
title: "Reorder artifact viewer layout — actions needed, pipeline, metadata, acceptance, body"
description: Change the ArtifactViewer component ordering so the most actionable information appears first.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "Viewer order is: actions needed → pipeline stepper → title + metadata → acceptance criteria (tasks) → body content → relationships panel"
  - ArtifactViewer.svelte renders sections in the specified order for all artifact types
  - "Sections that have no data (e.g., no actions needed) are hidden, not rendered empty"
relationships:
  - target: EPIC-58ba6d53
    type: delivers
    rationale: Reordered viewer layout puts actionable information first
  - target: TASK-d5e090bf
    type: depends-on
---

## Scope

Modify ArtifactViewer.svelte component ordering to present actions needed first, then pipeline stepper, then title and metadata, acceptance criteria for tasks, body content, and relationships panel last.
