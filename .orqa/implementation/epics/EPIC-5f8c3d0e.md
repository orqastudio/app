---
id: EPIC-5f8c3d0e
type: epic
title: Graph Traversal & Health Metrics
description: Implement the typed graph query API over SurrealDB supporting all 32 relationship types, forward-only writes with computed inverses, ancestry and sibling traversals, cycle detection, and live graph health metrics surfaced in the frontend.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: next
scoring:
  impact: 5
  urgency: 3
  complexity: 5
  dependencies: 4
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Stream 1 — graph traversal enables the navigable knowledge graph experience
  - target: EPIC-3d9fc182
    type: depends-on
    rationale: Requires SurrealDB daemon integration
---

## Context

The artifact graph needs typed traversals (not just flat lists) to deliver the "navigable knowledge graph" experience. This epic builds the query layer on top of SurrealDB that the frontend can call to render relationship trees, health dashboards, and pipeline views.

## Acceptance Criteria

- [ ] All 32 relationship types are queryable via the daemon HTTP API
- [ ] Forward-only writes enforced — inverse edges computed by query, not stored
- [ ] Ancestry traversal: given any artifact, retrieve all ancestors up to root
- [ ] Sibling traversal: given any artifact, retrieve all siblings in the same parent scope
- [ ] Cycle detection: query returns an error (not an infinite loop) if a cycle exists
- [ ] Graph health metrics: orphan count, broken links, archived-but-referenced artifacts
- [ ] Health score surfaced in the Pipeline Health Dashboard (EPIC-82dd0bd2 replacement work)
