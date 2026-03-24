---
id: RES-d7a4f1c9
type: research
title: Source of truth duplication audit — session 2026-03-24
description: Audit of duplicate source-of-truth patterns across Cytoscape SDK, Rust backend, and governance artifacts. Identifies 6 findings requiring user decisions.
status: completed
created: 2026-03-24
updated: 2026-03-24
category: code-quality
relationships:
  - target: EPIC-b2f0399e
    type: informs
    rationale: Findings feed the code quality audit epic
  - target: IDEA-b6d8e0f2
    type: references
    rationale: Finding 4 identified this idea as duplicated across delivery/ and discovery/
  - target: IDEA-e3a103e8
    type: references
    rationale: Finding 5 resulted in this idea for graph query performance
---

## Research Questions

### Q1: Where do source-of-truth duplications exist in OrqaStudio?

The audit investigated overlapping implementations between the Cytoscape SDK (`libs/graph-visualiser/`) and the Rust backend, plus governance artifact duplications.

## Findings

### Finding 1: Graph Health — Cytoscape SDK vs Rust Backend

**Status:** Confirmed duplicate. Rust is authoritative.

| Source | Location | Status |
|--------|----------|--------|
| Rust backend | `backend/src-tauri/src/commands/graph_commands.rs` -> `compute_graph_health` | LIVE — consumed by dashboard |
| Cytoscape SDK | `libs/graph-visualiser/src/analysis.ts` -> `computeGraphHealth` | DEAD CODE — no app consumers |

The Rust backend provides a superset of metrics (graph_density, pillar_traceability, bidirectionality_ratio, broken_ref_count) that the Cytoscape version does not compute.

**Decision needed:** Remove dead Cytoscape analysis functions, or keep for potential future use?

### Finding 2: Traceability — ChainTrace (Cytoscape) vs TraceabilityPanel (Rust)

**Status:** Duplicate display — both render in ArtifactViewer side-by-side.

| Source | Component | Data Source |
|--------|-----------|-------------|
| ChainTrace.svelte | BFS chain trace via Cytoscape `traceChain()` | `GraphVisualiser` (Cytoscape) |
| TraceabilityPanel | Traceability via Rust `graph_traceability` MCP tool | Rust backend |

Both show overlapping relationship/dependency information for the same artifact.

**Decision needed:** Consolidate to one source? Which one?

### Finding 3: Cytoscape SDK Analysis — Dead Functions

These functions exist in `libs/graph-visualiser/src/analysis.ts` but have zero app consumers:

- `computeGraphHealth()` — replaced by Rust
- `computeBackboneArtifacts()` — no UI widget consumes this
- `computeKnowledgeGaps()` — no UI widget consumes this
- `computeImpact()` / `impactOf()` — no UI panel consumes this

The `GraphVisualiser` class exposes these as `$derived` reactive properties, but no component reads them.

**Decision needed:** Remove dead analysis code? Or build UI consumers for the unique functions (backbone, gaps, impact)?

### Finding 4: IDEA-b6d8e0f2 Duplicate Files

The same idea existed in two locations:

- `.orqa/delivery/ideas/IDEA-b6d8e0f2.md` (status: promoted, richer description)
- `.orqa/discovery/ideas/IDEA-b6d8e0f2.md` (status: captured, original version)

**Resolution:** `discovery/` is the canonical location for ideas. The `delivery/` copy has been deleted. Note: the `delivery/` copy had evolved to `status: promoted` with a richer description — this content was not present in the `discovery/` copy. The discovery/ copy may need updating to reflect the promoted status.

### Finding 5: Graph Query Performance

Research findings in `tmp/team/graph-perf/research.md`. Key bottleneck: `query_artifacts()` re-reads matching files from disk on every query despite frontmatter being cached in-memory. Logged as [IDEA-e3a103e8](IDEA-e3a103e8) for future work.

### Finding 6: Pre-commit Hook Naming

The existing `plugins/githooks/hooks/pre-commit.sh` has a `.sh` extension. Git hooks discovered via `core.hooksPath` must not have extensions. This means the pre-commit hook is NOT currently firing.

**Decision needed:** Rename to `pre-commit` (no extension) and verify it works.

## Summary

| # | Finding | Severity | Decision Required |
|---|---------|----------|-------------------|
| 1 | Graph health duplicate (Cytoscape vs Rust) | Low — dead code | Remove or keep? |
| 2 | Traceability duplicate display | Medium — confusing UX | Consolidate to one source |
| 3 | Dead Cytoscape analysis functions | Low — dead code | Remove or build consumers? |
| 4 | IDEA-b6d8e0f2 file duplication | Low — resolved | Deleted delivery/ copy |
| 5 | Graph query performance bottleneck | Medium — perf | Tracked as IDEA-e3a103e8 |
| 6 | Pre-commit hook not firing | High — enforcement gap | Rename hook file |
