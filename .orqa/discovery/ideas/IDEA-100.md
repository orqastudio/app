---
id: IDEA-100
title: Software project integration plugin — code-level references into the artifact graph
description: A comment structure in code files that allows direct references into the governance system. Entire codebase gets injected into the graph. Could update search to have full search across project structure and remove separate code search.
status: surpassed
created: 2026-03-15
updated: 2026-03-15
horizon: later
research-needed: []
relationships:
  - type: merged-into
    target: IDEA-113
  - type: realises
    target: EPIC-080
  - target: AD-055
    type: crystallises
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---

> **Surpassed 2026-03-16**: Software project integration concept merged into EPIC-080 via AD-055 as part of plugin infrastructure. Code-governance traceability (comment syntax, code file nodes, unified search) split into IDEA-113 as a distinct feature requiring its own scanner plugin.

## Motivation

Code and governance currently live in separate systems — `.orqa/` contains the artifact graph while `backend/`, `ui/`, and `sidecar/` contain the implementation. There is no machine-readable link between a Rust function and the task it implements, or between a Svelte component and the epic it delivers. A comment-based reference structure (e.g., `// @orqa TASK-045`) would inject the codebase into the artifact graph, enabling traceability from governance artifact down to implementation line. Unified search across both code and governance would become possible, potentially replacing the separate ChunkHound/native search systems with a single graph-aware search layer.

## Sketch

A comment syntax (`@orqa ARTIFACT-ID`) parsed by a scanner plugin. The scanner adds code-file nodes to the artifact graph with edges to the referenced artifacts. Search queries can then traverse from an epic to all code files implementing it, or from a code file to the task it satisfies. Long-term: removes the need for a separate code search index by making the project graph the single search surface.
