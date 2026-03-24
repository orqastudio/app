---
id: TASK-17f9e7d0
type: task
title: "Research graph query performance bottlenecks"
description: "Investigate the three-layer graph query architecture (MCP Server -> Validation Daemon -> ArtifactGraph) to identify performance bottlenecks. Map data flow for common operations and quantify cost of each bottleneck."
status: completed
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "Architecture documented with data flow for each MCP graph tool"
  - "Bottlenecks identified with estimated cost"
  - "Improvement opportunities ranked by impact"
  - "Findings written to tmp/team/graph-perf/research.md"
relationships:
  - target: EPIC-c311d349
    type: delivers
    rationale: "Research task for the performance epic"
---

## What

Research task to investigate graph query performance. Findings documented in `tmp/team/graph-perf/research.md`.

## Results

Identified 4 bottlenecks:
1. CRITICAL: `query_artifacts()` re-reads every matching file from disk (~40 reads for a rule query)
2. CRITICAL: `graph_traceability` rebuilds entire graph from scratch (~200 file reads)
3. HIGH: Hook evaluation rebuilds graph on every call
4. MEDIUM: Search text filter applied client-side in MCP server