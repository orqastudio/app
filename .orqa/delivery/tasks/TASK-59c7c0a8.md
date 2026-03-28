---
id: TASK-59c7c0a8
type: task
title: Project settings UI for managing delivery types and state machine
description: "Add a settings section where users can configure delivery artifact types (add/edit/remove types, define parent-child hierarchy) and the status state machine (statuses, transitions, auto_rules, icons). Changes persist to project.json."
status: archived
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 4
  dependencies: 3
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - "Settings UI has a \"Delivery Pipeline\" section showing configured types"
  - "Users can add/edit/remove delivery types with key, label, path, parent relationship"
  - Users can reorder types to change hierarchy
  - "Settings UI has a \"Status Machine\" section showing all statuses with icons, transitions, auto_rules"
  - Users can add/edit/remove statuses
  - Users can configure transitions per status (which statuses it can move to)
  - Users can configure auto_rules per status
  - All changes persist to project.json
  - App refreshes to reflect new configuration without restart
relationships:
  - target: EPIC-bbac7876
    type: delivers
    rationale: Settings UI for managing the configurable delivery pipeline
  - target: TASK-6035b69e
    type: depends-on
---
