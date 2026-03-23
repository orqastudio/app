---
id: TASK-2830a2da
type: task
title: Add documents/documented-by relationship types + body-text edge extraction
description: Introduce documents/documented-by relationship types and extract artifact references from markdown body text as graph edges.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - New relationship types documents/documented-by in INVERSE_MAP
  - Graph builder extracts artifact references from markdown body text as edges
  - Documentation pages linked to artifacts they describe
relationships:
  - target: EPIC-58ba6d53
    type: delivers
    rationale: Body-text edge extraction and document relationships enrich the graph automatically
  - target: TASK-464eadf4
    type: depended-on-by
---

## Scope

Add documents/documented-by to the relationship type system and INVERSE_MAP. Update the graph builder to scan markdown body text for artifact reference patterns and create edges automatically. Run backfill across existing documentation pages.