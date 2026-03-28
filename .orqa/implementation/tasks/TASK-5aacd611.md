---
id: "TASK-5aacd611"
type: task
title: "Add dependency chain tracing to artifact viewer"
description: "Extend the artifact viewer with a Trace section that shows upward (task→epic→milestone→pillar) and downward (what this artifact affects) dependency chains. Uses BFS over relationship edges from the artifactGraphSDK. Each artifact in the chain is a clickable ArtifactLink."
status: blocked
priority: "P1"
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
created: "2026-03-15"
updated: "2026-03-15"
horizon: "active"
acceptance:
  - "Artifact viewer shows a 'Trace' section with upward chain (task→epic→milestone→pillar)"
  - "Uses BFS from current artifact following relationship edges"
  - "Clickable chain — each artifact in the path is an ArtifactLink"
  - "Shows both upward (why does this exist?) and downward (what does this affect?) chains"
relationships:
  - target: "EPIC-ff7db83e"
    type: "delivers"
  - target: "TASK-8922be81"
    type: "depends-on"
---

## What

Add a "Trace" section to the artifact viewer that shows the full dependency chain in both directions: upward (why does this artifact exist — tracing to its owning epic, milestone, and pillar) and downward (what artifacts depend on or are delivered by this one). Each artifact in the chain is rendered as a clickable `ArtifactLink` for navigation. The traversal uses BFS over the artifact graph edges provided by `artifactGraphSDK`.

## How

1. Add a `traceChain(artifactId, direction)` method to `artifactGraphSDK` that runs BFS:
   - `direction: 'up'` — follows `delivers`, `delivers`, `informs`, `grounded`, `grounded-by`, `enforces` edges upward
   - `direction: 'down'` — follows the inverse edges downward
   - Returns an ordered array of artifact IDs representing the chain path
2. In the artifact viewer component, derive `upwardChain` and `downwardChain` from `traceChain` results using `$derived`.
3. Render a "Trace" section below the main artifact body with two subsections:
   - "Why does this exist?" — upward chain rendered as a breadcrumb-style list of `ArtifactLink` components
   - "What does this affect?" — downward chain rendered as a flat list of `ArtifactLink` components
4. If a chain is empty (e.g., a pillar has no upward chain), hide that subsection entirely.
5. Display components receive chain arrays as props — no `invoke()` inside display components ([RULE-05ae2ce7](RULE-05ae2ce7)).

## Verification

- BFS unit tests: given a known graph, assert correct upward and downward chains for a leaf node and a root node.
- Component test: Trace section renders with correct ArtifactLink entries for a known artifact.
- Component test: empty chain hides its subsection.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
