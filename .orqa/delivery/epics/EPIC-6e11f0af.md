---
id: "EPIC-6e11f0af"
type: "epic"
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
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---
## Context

The `artifactGraphSDK` in `ui/src/lib/sdk/artifact-graph.svelte.ts` is the single abstraction layer between the Tauri backend and the frontend. It manages:

- Graph initialization with config (`ArtifactGraphConfig`)
- File watcher lifecycle
- Event subscriptions (artifact-graph-updated, artifact-changed)
- Reactive state (`SvelteMap<string, ArtifactNode>`)
- Integrity scanning, auto-fixes, health snapshots
- Typed relationship traversal

Currently it's a Svelte 5 rune-based singleton tightly coupled to the app. For the plugin architecture ([IDEA-5113eeae](IDEA-5113eeae), [IDEA-9b26ba54](IDEA-9b26ba54)), plugins need to consume this SDK as an importable package — same API, different distribution.

**Bundled idea**: [IDEA-9b26ba54](IDEA-9b26ba54) — Component Library SDK for Plugin Views (the Artifact Graph SDK is item #2 in that idea)

### Research Questions

1. **Package boundary**: What stays in the app vs what goes in the package? The SDK currently uses `invoke()` and `listen()` from `@tauri-apps/api` — how do plugins access these?
2. **Svelte 5 rune reactivity**: Can a Svelte 5 rune-based store (`$state`, `$derived`) be exported from an npm package and consumed by another Svelte app? What are the bundling constraints?
3. **Distribution mechanism**: npm package? Workspace package? Git submodule? What's the simplest path that doesn't over-engineer?
4. **Plugin isolation**: Should plugins get their own SDK instance or share the host app's? Shared = real-time updates, isolated = safety.
5. **Versioning and compatibility**: How does the SDK version relate to the app version? Breaking changes policy?

## Implementation Design

### Phase 1: Research

Investigate the five questions above. Produce findings in [RES-b0268020](RES-b0268020).

### Phase 2: Extraction (if research validates)

Based on research findings, extract the SDK. Scope TBD by research.

## Tasks

| ID | Title | Phase | Depends On |
|----|-------|-------|------------|
| TASK-TBD-1 | Research SDK extraction feasibility and package boundary | 1 | — |
| TASK-TBD-2 | Reconcile EPIC-6e11f0af | — | all above |

## Out of Scope

- Component library extraction (the other half of [IDEA-9b26ba54](IDEA-9b26ba54) — separate epic)
- View registration API
- Theme token distribution
- Actually building the plugin runtime — this epic is research + extraction only