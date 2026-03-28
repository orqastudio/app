---
id: "TASK-d4dade11"
type: "task"
title: "Surface violations in governance UI"
description: "Display enforcement violations in the app's governance view with history and filtering."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
docs:
  - "DOC-9814ec3c"
acceptance:
  - "Governance UI shows violation history"
  - "Violations are filterable by rule, agent, and time"
  - "Each violation shows the rule, the blocked action, and the enforcement message"
  - "Violation count is visible in the governance nav"
relationships:
  - target: "EPIC-12fba656"
    type: "delivers"
    rationale: "Absorbed from EPIC-9a1eba3f — surface violations in governance UI"
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Auto-generated inverse of belongs-to relationship from EPIC-9a1eba3f"
  - target: "TASK-8b51938b"
    type: "depends-on"
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
