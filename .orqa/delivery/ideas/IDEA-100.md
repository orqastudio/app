---
id: IDEA-100
title: "Software project integration plugin — code-level references into the artifact graph"
description: "A comment structure in code files that allows direct references into the governance system. Entire codebase gets injected into the graph. Could update search to have full search across project structure and remove separate code search."
status: captured
created: "2026-03-15"
updated: "2026-03-15"
pillars:
  - PILLAR-001
  - PILLAR-003
milestone: null
horizon: later
research-needed: []
promoted-to: null
spun-off-from: null
relationships: []
---

## Motivation

Code and governance currently live in separate systems — `.orqa/` contains the artifact graph while `backend/`, `ui/`, and `sidecar/` contain the implementation. There is no machine-readable link between a Rust function and the task it implements, or between a Svelte component and the epic it delivers. A comment-based reference structure (e.g., `// @orqa TASK-045`) would inject the codebase into the artifact graph, enabling traceability from governance artifact down to implementation line. Unified search across both code and governance would become possible, potentially replacing the separate ChunkHound/native search systems with a single graph-aware search layer.

## Sketch

A comment syntax (`@orqa ARTIFACT-ID`) parsed by a scanner plugin. The scanner adds code-file nodes to the artifact graph with edges to the referenced artifacts. Search queries can then traverse from an epic to all code files implementing it, or from a code file to the task it satisfies. Long-term: removes the need for a separate code search index by making the project graph the single search surface.
