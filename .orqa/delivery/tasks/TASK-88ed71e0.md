---
id: TASK-88ed71e0
title: Artifact link settings — display mode and per-type colour coding
description: Add project settings for artifact link display mode (ID vs title) and per-type colour configuration. Settings stored in project.json.
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Project settings UI has artifact link configuration section
  - Display mode toggle (ID vs title) works and persists
  - Per-type colour configuration available with defaults
  - ArtifactLink component reads settings and applies colours/display mode
relationships:
  - target: EPIC-b2ca1ea3
    type: delivers
    rationale: Artifact link settings (F8, F27, F28)
  - target: TASK-ebc48b97
    type: depends-on
---
