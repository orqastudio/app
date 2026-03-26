---
id: "IDEA-7f5c18e9"
type: idea
title: "Full-Codebase Artifact Graph"
description: "Expand the artifact graph beyond .orqa/ to include source code files, creating a unified knowledge graph that maps relationships between governance artifacts and the code they govern."
status: captured
created: "2026-03-10"
updated: "2026-03-13"
horizon: "someday"
research-needed:
  - "What source code entities should be nodes? Files, functions, modules, types?"
  - "How to keep a large graph performant (thousands of source files vs hundreds of artifacts)?"
  - "Incremental graph updates vs full rebuild — the full snapshot watcher event won't scale"
  - "How do task scope fields, code comments, and inline references create edges to source code?"
  - "Integration with native search engine (semantic search over graph nodes)"
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-c4afd86b"
    type: "benefits"
---
## Motivation

[EPIC-d45b4dfd](EPIC-d45b4dfd) establishes an artifact graph covering `.orqa/` — governance artifacts and their relationships. But the real power comes from connecting governance to code: which tasks modified which files, which rules govern which modules, which decisions affected which components.

A full-codebase graph would enable:
- Impact analysis: "If I change this module, which epics and decisions are affected?"
- Traceability: "Show me every artifact related to the streaming pipeline"
- Plugin queries: "Find all files that haven't been touched by any task"
- Governance coverage: "Which source directories have no governing rules?"

## Sketch

The current `.orqa/`-only graph uses full snapshot refresh via file watcher events. At codebase scale (thousands of files), this needs to become incremental — only rebuild nodes for changed files. The watcher event payload would shift from full graph snapshot to a list of changed node IDs with diffs.

Source code nodes would be lighter than artifact nodes (no frontmatter, just path + type + metadata extracted from AST or heuristics). Edges from artifacts to code would come from task `scope` fields, rule `scope` fields, and potentially inline code comments referencing artifact IDs.