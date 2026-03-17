---
id: TASK-498
title: Build headless Cytoscape analysis in artifact graph SDK
description: "Extend the artifactGraphSDK with a headless Cytoscape instance for graph-theoretic analysis. Exposes graph health metrics (component count, orphans, average degree, largest component ratio) as reactive properties recomputed whenever graph data changes."
status: ready
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 5
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - artifactGraphSDK has a buildAnalysisCy() method that creates a headless cytoscape instance from graph data
  - "graphHealth reactive property exposes: componentCount, orphanCount, orphanPercentage, avgDegree, largestComponentRatio"
  - Analysis re-runs when graph data changes
  - "No DOM rendering — headless: true"
  - cy.destroy() called on cleanup
relationships:
  - target: EPIC-076
    type: delivers
  - target: TASK-499
    type: depended-on-by
  - target: TASK-500
    type: depended-on-by
  - target: TASK-501
    type: depended-on-by
  - target: TASK-502
    type: depended-on-by
  - target: TASK-503
    type: depended-on-by
---

## What

Add a `buildAnalysisCy()` method to the artifact graph SDK that constructs a headless Cytoscape instance from the current graph data. Derive a `graphHealth` reactive property exposing component count, orphan count, orphan percentage, average degree, and largest component ratio. This is the foundational analysis layer that all other EPIC-076 tasks depend on.

## How

1. Add `cytoscape` as a dependency to the SDK package (headless usage — no DOM renderer needed).
2. Implement `buildAnalysisCy(nodes, edges)` that builds a Cytoscape graph with `headless: true`.
3. Implement `graphHealth` as a derived reactive property (`$derived`) that calls `buildAnalysisCy` and computes:
   - `componentCount` — number of weakly connected components
   - `orphanCount` — nodes with in-degree 0 and out-degree 0
   - `orphanPercentage` — orphanCount / totalNodes * 100
   - `avgDegree` — mean of (in-degree + out-degree) across all nodes
   - `largestComponentRatio` — largest component size / totalNodes
4. Register a `$effect` that calls `cy.destroy()` on cleanup when the SDK instance is torn down.
5. Ensure `buildAnalysisCy` is not called on every render — gate it on graph data identity change.

## Verification

- Unit tests: given a known graph (e.g., 10 nodes, 2 isolated), assert all five metrics are correct.
- Unit tests: orphan detection uses structural 0-degree, not file-scan heuristics.
- Unit tests: `cy.destroy()` is called exactly once on cleanup (spy or mock).
- `make check` passes with zero warnings.

## Lessons

(To be filled during/after implementation)
