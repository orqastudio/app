---
id: EPIC-076
title: Graph analysis — Cytoscape algorithms powering governance insights
description: "Use Cytoscape.js graph analysis algorithms to power dashboard health scoring, dependency chain tracing, impact analysis, knowledge gap detection, and artifact importance ranking. Replaces file-based integrity checks with graph-theoretic analysis."
status: ready
priority: P1
scoring:
  impact: 5
  urgency: 3
  complexity: 5
  dependencies: 3
created: 2026-03-15
updated: 2026-03-15
deadline: null
horizon: active
relationships:
  - target: RES-065
    type: informed-by
    rationale: Graph analysis research drives the design
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
  - target: RES-065
    type: informs
  - target: TASK-498
    type: delivered-by
  - target: TASK-499
    type: delivered-by
  - target: TASK-500
    type: delivered-by
  - target: TASK-501
    type: delivered-by
  - target: TASK-502
    type: delivered-by
  - target: TASK-503
    type: delivered-by
---

## Context

Cytoscape.js is now installed for graph visualization. It includes built-in graph analysis algorithms (components, centrality, PageRank, BFS/DFS, shortest path) that can power governance insights far beyond the current file-based integrity scanning.

## Implementation Design

### Architecture

A headless Cytoscape instance in the artifact graph SDK runs analysis without requiring DOM rendering. Results are exposed as reactive state that dashboard widgets consume.

```
Artifact Graph SDK
  → buildAnalysisCy() — headless cytoscape instance from graph data
  → graphHealth — { componentCount, orphanPercentage, avgDegree, largestComponentRatio }
  → backboneArtifacts — top N by PageRank
  → traceChain(id, direction) — BFS upward/downward from any artifact
  → impactOf(id) — all artifacts affected by a change to this one
  → knowledgeGaps — per-type unlinked artifact lists
```

### Phases

Phase 1: Graph health scoring + headless analysis in SDK
Phase 2: Dependency chain tracing in artifact viewer
Phase 3: Impact analysis panel
Phase 4: Backbone artifacts widget
Phase 5: Knowledge gap detection

## Tasks

- [ ] [TASK-498](TASK-498): Build headless Cytoscape analysis in artifact graph SDK
- [ ] [TASK-499](TASK-499): Replace GraphHealthWidget scoring with graph-theoretic metrics
- [ ] [TASK-500](TASK-500): Add dependency chain tracing to artifact viewer
- [ ] [TASK-501](TASK-501): Build impact analysis panel for pre-edit preview
- [ ] [TASK-502](TASK-502): Add backbone artifacts widget to dashboard (PageRank)
- [ ] [TASK-503](TASK-503): Knowledge gap detection in governance audit
