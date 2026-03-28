---
id: "TASK-2ab6cb6f"
type: "task"
title: "Consolidate Cytoscape analysis to Rust daemon endpoints"
description: "Move graph health, BFS traversal, PageRank, knowledge gap detection, and impact analysis from the TypeScript Cytoscape SDK (libs/graph-visualiser/src/analysis.ts) to the Rust validation crate. Expose via daemon HTTP endpoints. Strip analysis.ts to a thin fetch layer."
status: archived
priority: "P2"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "libs/graph-visualiser/src/analysis.ts contains no business logic — only daemon HTTP fetch calls"
  - "Rust validation crate has equivalents for PageRank backbone, knowledge gaps, and N-hop impact analysis"
  - "Daemon exposes endpoints for each analysis function"
  - "Graph visualiser UI renders identical results using daemon-backed data"
  - "Dead functions removed: computeGraphHealth() (already replaced by Rust), computeBackboneArtifacts(), computeKnowledgeGaps(), computeImpact()"
  - "Cytoscape is ONLY used for layout/rendering in elements.ts"
relationships:
  - target: "EPIC-0497a1be"
    type: "delivers"
    rationale: "Task delivers work to the deduplication epic"
---

## What

`libs/graph-visualiser/src/analysis.ts` reimplements graph analysis algorithms using Cytoscape's API:

- `computeGraphHealth(cy)` — duplicates Rust `compute_health()` (already dead code, replaced by Rust)
- `computeBackboneArtifacts(cy, graph, topN)` — PageRank via Cytoscape (no Rust equivalent yet)
- `computeKnowledgeGaps(graph)` — knowledge gap detection
- `traceChain(cy, id, direction)` — BFS via Cytoscape (duplicates Rust `trace_to_pillars()`/`trace_descendants()`)
- `computeImpact(cy, graph, id, maxDepth)` — N-hop impact analysis via BFS

The `GraphVisualiser` class exposes these as `$derived` reactive properties, but most have zero app consumers (dead code).

## How

1. Add Rust equivalents in `libs/validation/src/metrics.rs`:
   - `compute_pagerank_backbone(graph, top_n)` — PageRank using iterative power method
   - `compute_knowledge_gaps(graph)` — scan for agents without knowledge, rules without enforcement, etc.
   - `compute_impact(graph, id, max_depth)` — N-hop BFS impact analysis
2. Expose via daemon endpoints: `/backbone`, `/knowledge-gaps`, `/impact`
3. Strip `analysis.ts` to thin fetch calls to daemon endpoints
4. Remove dead Cytoscape analysis functions
5. Verify graph visualiser renders correctly with daemon-backed data

## Files

- `libs/graph-visualiser/src/analysis.ts` — primary target for migration
- `libs/graph-visualiser/src/types.ts` — parallel TypeScript types to reconcile
- `libs/validation/src/metrics.rs` — Rust canonical, add missing functions
- `libs/validation/src/daemon.rs` — add new endpoints
