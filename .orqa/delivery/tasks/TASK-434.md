---
id: TASK-434
title: Migrate scope fields to relationships array (rules + skills)
description: Replace scope fields on rules and skills with relationships array entries using scoped-to/scoped-by types.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "Rule and skill schemas updated — scope removed, AGENT-NNN references moved to relationships array with type scoped-to/scoped-by"
  - Migration script backfills existing artifacts
  - Integrity validator updated
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Normalizing scope into relationships simplifies the graph model
---

## Scope

Update rule and skill schema.json files to remove scope field and add scoped-to/scoped-by to the relationship type enum. Write a migration script to backfill existing artifacts. Update the integrity validator to check the new format.
