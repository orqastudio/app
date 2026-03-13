---
id: TASK-389
title: "Add typed traversal methods to artifactGraphSDK"
description: "Add traverse(), pipelineChain(), missingInverses(), and relationship-filtered query methods to the artifact graph SDK so components can follow typed edges without parsing frontmatter."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-060
depends-on:
  - TASK-388
acceptance:
  - "sdk.traverse(id, 'enforced-by') returns ArtifactNode[] following edges of that relationship type"
  - "sdk.pipelineChain(id) returns the full upstream/downstream chain following pipeline relationship types"
  - "sdk.missingInverses() returns ArtifactRef[] where A→B exists but B→A doesn't for relationship edges"
  - "sdk.relationshipsFrom(id) returns typed relationship edges (relationship_type !== null) as enriched objects with resolved target nodes"
  - "All methods are synchronous (operate on cached graph data)"
  - "make check passes"
---

## What

The graph SDK has `referencesFrom()` and `referencesTo()` but these return all edges without type filtering. Components need to traverse specific relationship types (e.g., "show me all rules that enforce this AD").

## How

Add to `ArtifactGraphSDK`:

1. `traverse(id: string, relationshipType: string): ArtifactNode[]` — filter `references_out` by `relationship_type`, resolve targets
2. `traverseIncoming(id: string, relationshipType: string): ArtifactNode[]` — same for `references_in`
3. `pipelineChain(id: string): { upstream: ArtifactNode[], downstream: ArtifactNode[] }` — walk pipeline edges recursively (grounded→grounded-by for upstream, enforced-by→enforces for downstream)
4. `missingInverses(): { ref: ArtifactRef, expectedInverse: string }[]` — for each relationship edge, check if the inverse exists on the target
5. `relationshipsFrom(id: string): { target: ArtifactNode, type: string, rationale?: string }[]` — enriched relationship view with resolved nodes

## Verification

- Unit tests for traverse, pipelineChain, missingInverses
- `make check` passes
- Manual verification: `sdk.traverse("AD-029", "enforced-by")` returns the rules that enforce AD-029

## Lessons

(none yet)
