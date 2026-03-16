---
id: TASK-458
title: Migrate epic/milestone and task/epic references to relationship types
description: "The epic field on tasks and milestone field on epics are frontmatter fields, not relationship edges. Migrate these to the relationships array with proper types (e.g. delivers/delivered-by, belongs-to/contains) so all graph edges are scannable as relationship types."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - task-to-epic expressed as a relationship type (not just the epic frontmatter field)
  - epic-to-milestone expressed as a relationship type (not just the milestone frontmatter field)
  - All graph edges use consistent relationship type vocabulary
  - Integrity validator checks these relationships
  - Migration script backfills existing artifacts
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Schema evolution — all references as relationship types
---

## Scope

Currently `epic:` on tasks and `milestone:` on epics are standalone frontmatter fields. The graph builder treats them as forward references but they aren't in the relationships array with a type. This means they can't be scanned, filtered, or displayed the same way as other relationships.

Migrate to relationship types while keeping the frontmatter fields for backwards compatibility during transition. The graph builder should create relationship edges from both sources.
