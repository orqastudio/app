---
id: IDEA-033
title: Live Roadmap View
description: Replace the static roadmap markdown document with a database-driven roadmap view that auto-updates as epic/milestone statuses change. First artifact type to be driven by SQLite rather than file scanning.
status: captured
created: 2026-03-07
updated: 2026-03-13
horizon: later
research-needed:
  - Data model for roadmap state in SQLite (derived from milestone + epic frontmatter)
  - Sync strategy from .orqa/ file changes to SQLite roadmap table
  - "UI design for roadmap view (timeline, kanban, grouped list)"
  - "What happens to the markdown roadmap doc — deprecate or auto-generate?"
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---
## Problem

The roadmap document (`.orqa/documentation/about/roadmap.md`) is a manually maintained markdown file that lists milestones, epics, and their statuses. It drifts constantly — every time an epic status changes, the roadmap must be manually updated. This violates the single-source-of-truth principle: the real status lives in epic/milestone frontmatter, but the roadmap is a stale copy.

## Concept

Build a roadmap view in the UI that is **derived directly from artifact data**:

1. **SQLite-backed** — scan milestone and epic frontmatter into a roadmap table
2. **Auto-updating** — when an epic moves to `done`, the roadmap view reflects it immediately
3. **First-class UI** — not a markdown render, but a purpose-built view (timeline, progress bars, grouping by milestone)
4. **Replaces the document** — the markdown roadmap becomes either auto-generated from the database or deprecated entirely

## Why This Matters

This is the first step toward database-driven artifact views. The pattern established here (file frontmatter → SQLite → reactive UI) will apply to other aggregate views: dashboards, dependency graphs, progress tracking.

## Implementation Approach: Core vs Plugin

This could be built as either a core feature or a plugin. Both have trade-offs.

**Core feature (built into the app):**
- Pro: Roadmap is fundamental to project management — every project benefits
- Pro: Tighter integration with artifact scanning and status change events
- Pro: Can establish the file→SQLite→UI pattern that other features will follow
- Con: Adds complexity to the core app for something not every user may need

**Plugin (ecosystem contribution):**
- Pro: Keeps the core lean — only structured thinking primitives in the app
- Pro: Different users could have different roadmap visualisations
- Pro: Tests the plugin API early with a real use case
- Con: Plugin API doesn't exist yet — building the roadmap would force plugin architecture decisions prematurely
- Con: Roadmap needs deep access to milestone/epic state — may need privileged plugin APIs

**Recommendation to investigate:** If the traceability graph [IDEA-032](IDEA-032) is built as core infrastructure, the roadmap could be a view on top of that graph — making it a natural plugin candidate. But if the roadmap is needed before the graph, it may be better as a core feature that later migrates to the plugin layer.

## Scope Note

The static roadmap document should be maintained until this is implemented. Once the live view exists, the document can be deprecated or converted to an auto-generated snapshot.
