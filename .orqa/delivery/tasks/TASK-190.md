---
id: TASK-190
title: Surface violations in governance UI
description: "Display enforcement violations in the app's governance view with history and filtering."
status: completed
created: 2026-03-11
updated: 2026-03-14
assignee: AGENT-002
docs:
  - DOC-021
acceptance:
  - Governance UI shows violation history
  - "Violations are filterable by rule, agent, and time"
  - "Each violation shows the rule, the blocked action, and the enforcement message"
  - Violation count is visible in the governance nav
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Absorbed from EPIC-050 — surface violations in governance UI
  - target: EPIC-050
    type: delivers
    rationale: "Auto-generated inverse of belongs-to relationship from EPIC-050"
  - target: TASK-415
    type: depends-on
  - target: SKILL-043
    type: informed-by
  - target: SKILL-012
    type: informed-by
  - target: SKILL-030
    type: informed-by
  - target: SKILL-016
    type: informed-by
  - target: SKILL-041
    type: informed-by
  - target: TASK-340
    type: depended-on-by
  - target: SKILL-012
    type: informed-by
  - target: SKILL-016
    type: informed-by
  - target: SKILL-030
    type: informed-by
  - target: SKILL-041
    type: informed-by
  - target: SKILL-043
    type: informed-by
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
