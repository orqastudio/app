---
id: TASK-440
title: "Pipeline health: collapse all-clear, remove Refresh, auto-rescan after refresh"
description: "Streamline the pipeline health display — collapse all-clear state, remove redundant Refresh button, auto-run integrity scan after graph refresh."
status: done
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-068
depends-on: []
assignee: null
skills: []
acceptance:
  - "All clear shows as subtle inline indicator not a full card"
  - "Refresh button removed from dashboard (Re-index in statusbar suffices)"
  - "Integrity scan auto-runs after any graph refresh"
relationships:
  - target: EPIC-068
    type: delivers
    rationale: "Reduced visual noise on dashboard when everything is healthy"
---

## Scope

Update IntegrityWidget.svelte to show all-clear as a compact inline indicator. Remove the Refresh button from ProjectDashboard.svelte. Wire integrity scan to auto-trigger after any graph refresh event.
