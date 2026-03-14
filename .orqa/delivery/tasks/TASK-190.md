---
id: TASK-190
title: Surface violations in governance UI
description: Display enforcement violations in the app's governance view with history and filtering.
status: todo
created: 2026-03-11
updated: 2026-03-14
epic: EPIC-064
depends-on:
  - TASK-415
assignee: AGENT-002
docs:
  - DOC-021
skills:
  - SKILL-043
  - SKILL-012
  - SKILL-030
  - SKILL-016
  - SKILL-041
acceptance:
  - Governance UI shows violation history
  - Violations are filterable by rule, agent, and time
  - Each violation shows the rule, the blocked action, and the enforcement message
  - Violation count is visible in the governance nav
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Absorbed from EPIC-050 — surface violations in governance UI
  - target: EPIC-064
    type: belongs-to
    rationale: Task belongs to this epic (absorbed from EPIC-050)
---

## What

The governance UI surfaces enforcement violations so users can see what was
blocked, when, and by which rule. This completes the feedback loop from
enforcement to visibility.

## How

1. Create Tauri command to query violation history from SQLite
2. Create Svelte store for violation data
3. Create violations view component in the governance section
4. Add violation count badge to governance nav

## Verification

- Violations appear in the governance UI after enforcement blocks an action
- Filtering by rule/agent/time works
- Violation count in nav updates in real time
