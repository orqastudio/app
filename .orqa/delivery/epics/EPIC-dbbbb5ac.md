---
id: "EPIC-dbbbb5ac"
type: epic
title: "Graph analysis — Cytoscape algorithms powering governance insights"
description: "Use Cytoscape.js graph analysis algorithms to power dashboard health scoring, dependency chain tracing, impact analysis, knowledge gap detection, and artifact importance ranking. Replaces file-based integrity checks with graph-theoretic analysis."
status: "surpassed"
priority: "P1"
scoring:
  impact: 5
  urgency: 3
  complexity: 5
  dependencies: 3
created: "2026-03-15"
updated: "2026-03-24"
deadline: null
horizon: "active"
relationships:
  - target: "RES-f0bd8d42"
    type: "guided-by"
    rationale: "Graph analysis research drives the design"
  - target: "MS-eea45fa8"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-f8f9b6e7"
    type: "delivered-by"
  - target: "TASK-595ffcaf"
    type: "delivered-by"
  - target: "TASK-afe17917"
    type: "delivered-by"
  - target: "TASK-21decc93"
    type: "delivered-by"
  - target: "TASK-75917a42"
    type: "delivered-by"
  - target: "TASK-0190fabe"
    type: "delivered-by"
  - target: "TASK-a7c3d1e9"
    type: "delivered-by"
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

- [ ] [TASK-f8f9b6e7](TASK-f8f9b6e7): Build headless Cytoscape analysis in artifact graph SDK
- [ ] [TASK-595ffcaf](TASK-595ffcaf): Replace GraphHealthWidget scoring with graph-theoretic metrics
- [ ] [TASK-afe17917](TASK-afe17917): Add dependency chain tracing to artifact viewer
- [ ] [TASK-21decc93](TASK-21decc93): Build impact analysis panel for pre-edit preview
- [ ] [TASK-75917a42](TASK-75917a42): Add backbone artifacts widget to dashboard (PageRank)
- [ ] [TASK-0190fabe](TASK-0190fabe): Knowledge gap detection in governance audit