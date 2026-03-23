---
id: TASK-e0165bca
type: task
title: Configurable relationship chip display per type in project settings
description: Allow project settings to configure whether relationship chips show title or id as primary content per artifact type.
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Project settings has config for relationship chip display — choose title or id as primary content per artifact type
  - Default is title
relationships:
  - target: EPIC-58ba6d53
    type: delivers
    rationale: Configurable chip display lets users choose the most useful identifier per type
  - target: TASK-e9219bfd
    type: depends-on
---

## Scope

Add relationship chip display configuration to the project.json schema. Read the configuration in RelationshipsList and apply to chip rendering. Default to showing title as primary content.