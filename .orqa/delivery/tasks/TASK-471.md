---
id: TASK-471
title: "Artifact link settings — display mode and per-type colour coding"
description: "Add project settings for artifact link display mode (ID vs title) and per-type colour configuration. Settings stored in project.json."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on:
  - TASK-470
assignee: null
skills:
  - SKILL-030
  - SKILL-016
acceptance:
  - Project settings UI has artifact link configuration section
  - Display mode toggle (ID vs title) works and persists
  - Per-type colour configuration available with defaults
  - ArtifactLink component reads settings and applies colours/display mode
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Artifact link settings (F8, F27, F28)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
