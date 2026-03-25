---
id: TASK-fcd3d5c6
type: task
title: Documentation relationship audit — documents/documented-by edges
description: Audit all documentation pages and populate documents/documented-by relationship edges across all docs.
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - All documentation pages have relationships to the artifacts they describe
  - documents/documented-by edges populated across all docs
relationships:
  - target: EPIC-952ff2c9
    type: delivers
    rationale: Document relationship edges connect documentation to the artifacts they describe
  - target: TASK-da473493
    type: depends-on
---

## Scope

Audit all documentation pages in .orqa/documentation/ and backfill documents/documented-by relationship edges. Create a backfill script if needed. Depends on TASK-da473493 for the relationship type definitions.