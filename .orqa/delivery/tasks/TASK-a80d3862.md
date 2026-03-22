---
id: TASK-a80d3862
title: Loading spinner size increase
description: Increases the artifact viewer loading spinner to the large size variant so the branded logo-pulse animation is clearly visible rather than being lost at the default small size.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - "Loading spinner in main artifact viewer uses \"lg\" size variant"
  - Animated logo pulse is clearly visible and not lost at display size
relationships:
  - target: EPIC-a2fa3068
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-34eaf518
    type: depended-on-by
---
## Findings Addressed

- **F16**: Loading spinner in main viewer too small — animation lost at current size

## Notes

Quick fix — change `<LoadingSpinner />` to `<LoadingSpinner size="lg" />` in ArtifactViewer.svelte (line 122). The "lg" variant (h-12 w-12) already exists and shows the branded logo-pulse animation at a visible size.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
