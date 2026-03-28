---
id: "EPIC-ff7db83e"
type: "epic"
title: "Graph analysis — Cytoscape algorithms powering governance insights"
description: "Use Cytoscape.js graph analysis algorithms to power dashboard health scoring, dependency chain tracing, impact analysis, knowledge gap detection, and artifact importance ranking. Replaces file-based integrity checks with graph-theoretic analysis."
status: archived
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
  - target: "MS-21d5096a"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
---

## Context

Cytoscape.js is now installed for graph visualization. It includes built-in graph analysis algorithms (components, centrality, PageRank, BFS/DFS, shortest path) that can power governance insights far beyond the current file-based integrity scanning.

## Implementation Design

### Architecture

A headless Cytoscape instance in the artifact graph SDK runs analysis without requiring DOM rendering. Results are exposed as reactive state that dashboard widgets consume.

```text
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

- [ ] [TASK-8922be81](TASK-8922be81): Build headless Cytoscape analysis in artifact graph SDK
- [ ] [TASK-7847c5f7](TASK-7847c5f7): Replace GraphHealthWidget scoring with graph-theoretic metrics
- [ ] [TASK-5aacd611](TASK-5aacd611): Add dependency chain tracing to artifact viewer
- [ ] [TASK-efcbbd1b](TASK-efcbbd1b): Build impact analysis panel for pre-edit preview
- [ ] [TASK-cb526911](TASK-cb526911): Add backbone artifacts widget to dashboard (PageRank)
- [ ] [TASK-a30d8521](TASK-a30d8521): Knowledge gap detection in governance audit
