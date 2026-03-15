---
id: TASK-502
title: "Add backbone artifacts widget to dashboard (PageRank)"
description: "Add a dashboard widget that shows the top 10 artifacts by PageRank score, computed from the artifact graph via the artifactGraphSDK. Each entry shows artifact ID, title, type, and score. Entries are clickable and navigate to the artifact. Widget is labeled 'Backbone Artifacts — most structurally important'."
status: todo
priority: P2
created: "2026-03-15"
updated: "2026-03-15"
epic: EPIC-076
milestone: null
horizon: active
depends-on:
  - TASK-498
acceptance:
  - "Dashboard widget shows top 10 artifacts by PageRank score"
  - "Each entry shows artifact ID, title, type, PageRank score"
  - "Clickable — navigates to artifact"
  - "Labeled as 'Backbone Artifacts — most structurally important'"
relationships:
  - type: delivers
    target: EPIC-076
  - type: belongs-to
    target: EPIC-076
---

## What

Add a "Backbone Artifacts" widget to the main dashboard that surfaces the top 10 artifacts by PageRank score. PageRank identifies artifacts that many other artifacts point to — i.e. the structural keystone nodes in the governance graph. This gives users a quick view of which artifacts are most load-bearing and most important to keep accurate.

## How

1. Add a `pageRank()` method to `artifactGraphSDK` that runs Cytoscape's built-in PageRank algorithm on the headless `cy` instance and returns a map of `artifactId → score`.
2. Derive `backboneArtifacts` as the top 10 entries from the PageRank map, sorted descending, enriched with title and type from the graph node data.
3. Build a `BackboneArtifactsWidget` display component that:
   - Renders a titled card: "Backbone Artifacts — most structurally important"
   - Lists top 10 entries as rows: artifact ID (as `ArtifactLink`), title, type badge, PageRank score (formatted to 4 decimal places)
   - Each row is clickable and navigates to the artifact
4. Register the widget on the dashboard page alongside existing widgets.
5. Use `<EmptyState>` if the graph has no edges (PageRank is undefined on an edgeless graph).
6. Widget is a pure display component — receives `backboneArtifacts` as a prop.

## Verification

- `pageRank()` unit test: known graph with a clear hub node scores that node highest.
- Component test: top 10 entries render in descending score order.
- Component test: `<EmptyState>` renders when artifact list is empty.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
