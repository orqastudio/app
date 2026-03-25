---
id: TASK-efcbbd1b
type: task
title: Build impact analysis panel for pre-edit preview
description: "Before editing an artifact, show a non-blocking impact panel that lists artifacts reachable within 1-2 hops via relationship edges, grouped by relationship type. Uses neighborhood traversal from the artifactGraphSDK. Rendered as an expandable info panel, not a modal."
status: blocked
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 3
  dependencies: 2
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - "Before editing an artifact, an impact panel shows 'This change may affect N artifacts'"
  - Uses neighborhood traversal (1-2 hops) from the artifact being edited
  - Lists affected artifacts grouped by relationship type
  - "Non-blocking — shown as an expandable info panel, not a modal"
relationships:
  - target: EPIC-ff7db83e
    type: delivers
  - target: TASK-8922be81
    type: depends-on
---
## What

Add a pre-edit impact panel to the artifact editor. When the user opens an artifact for editing, a collapsible info panel appears showing which other artifacts are reachable within 1-2 relationship hops — giving a quick sense of blast radius before making changes. The panel is non-blocking: it does not prevent editing and can be dismissed or left collapsed.

## How

1. Add a `neighborhood(artifactId, depth)` method to `artifactGraphSDK` that returns all artifacts reachable within `depth` hops (both directions), grouped by the relationship type of the traversed edge.
2. In the artifact editor, derive `impactNeighborhood` from `neighborhood(currentId, 2)` using `$derived`.
3. Render an expandable `<details>` panel (or shadcn Collapsible) above the editor body:
   - Header: "This change may affect N artifacts" (collapsed by default)
   - Expanded: one section per relationship type, each listing affected artifact IDs as `ArtifactLink` components
4. If `impactNeighborhood` is empty or has only 1 entry (the artifact itself), hide the panel entirely.
5. Panel is a pure display component receiving neighborhood data as props — no `invoke()` inside the component.

## Verification

- `neighborhood` unit tests: assert correct 1-hop and 2-hop results for a known graph.
- Component test: panel hidden when neighborhood is empty.
- Component test: panel collapsed by default when neighborhood is non-empty.
- Component test: groups render with correct relationship type labels.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)