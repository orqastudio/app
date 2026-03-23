---
id: "EPIC-527de6a3"
type: epic
title: "Artifact Graph SDK extraction research"
description: "Research extracting the artifactGraphSDK into a standalone npm package that plugins can import, enabling the plugin architecture's data layer."
status: "captured"
priority: "P2"
created: "2026-03-13"
updated: "2026-03-13"
deadline: null
horizon: "next"
scoring:
  impact: 3
  urgency: 2
  complexity: 3
  dependencies: 2
rule-overrides: []
relationships:
  - target: "RES-00ec6dd1"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-00ec6dd1"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "IDEA-53205849"
    type: "realised-by"
---
## Context

The `artifactGraphSDK` in `ui/src/lib/sdk/artifact-graph.svelte.ts` is the single abstraction layer between the Tauri backend and the frontend. It manages:

- Graph initialization with config (`ArtifactGraphConfig`)
- File watcher lifecycle
- Event subscriptions (artifact-graph-updated, artifact-changed)
- Reactive state (`SvelteMap<string, ArtifactNode>`)
- Integrity scanning, auto-fixes, health snapshots
- Typed relationship traversal

Currently it's a Svelte 5 rune-based singleton tightly coupled to the app. For the plugin architecture ([IDEA-b77e2955](IDEA-b77e2955), [IDEA-53205849](IDEA-53205849)), plugins need to consume this SDK as an importable package — same API, different distribution.

**Bundled idea**: [IDEA-53205849](IDEA-53205849) — Component Library SDK for Plugin Views (the Artifact Graph SDK is item #2 in that idea)

### Research Questions

1. **Package boundary**: What stays in the app vs what goes in the package? The SDK currently uses `invoke()` and `listen()` from `@tauri-apps/api` — how do plugins access these?
2. **Svelte 5 rune reactivity**: Can a Svelte 5 rune-based store (`$state`, `$derived`) be exported from an npm package and consumed by another Svelte app? What are the bundling constraints?
3. **Distribution mechanism**: npm package? Workspace package? Git submodule? What's the simplest path that doesn't over-engineer?
4. **Plugin isolation**: Should plugins get their own SDK instance or share the host app's? Shared = real-time updates, isolated = safety.
5. **Versioning and compatibility**: How does the SDK version relate to the app version? Breaking changes policy?

## Implementation Design

### Phase 1: Research

Investigate the five questions above. Produce findings in [RES-5657d9f6](RES-5657d9f6).

### Phase 2: Extraction (if research validates)

Based on research findings, extract the SDK. Scope TBD by research.

## Tasks

| ID | Title | Phase | Depends On |
|----|-------|-------|------------|
| TASK-TBD-1 | Research SDK extraction feasibility and package boundary | 1 | — |
| TASK-TBD-2 | Reconcile EPIC-527de6a3 | — | all above |

## Out of Scope

- Component library extraction (the other half of [IDEA-53205849](IDEA-53205849) — separate epic)
- View registration API
- Theme token distribution
- Actually building the plugin runtime — this epic is research + extraction only