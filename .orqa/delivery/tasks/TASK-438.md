---
id: TASK-438
title: Actions needed icon in artifact list + epics without tasks
description: Show action-needed indicators in artifact list items and flag epics that have no tasks referencing them.
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
  - Artifacts with pending actions show icon indicator in ArtifactListItem
  - Epics with no tasks referencing them show as action needed
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Action indicators in list view surface what needs attention without opening each artifact
  - target: TASK-429
    type: depends-on
---

## Scope

Update ActionsNeeded logic to detect epics without tasks. Update ArtifactListItem.svelte to show an action-needed icon indicator when the artifact has pending actions.
