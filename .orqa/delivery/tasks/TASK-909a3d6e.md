---
id: TASK-909a3d6e
type: task
title: "Create @orqastudio/sdk package — graph + IPC + frontmatter"
description: "Create the SDK package with ArtifactGraphSDK, IPC invoke wrapper, and frontmatter parser. This is the core layer — stores are added in subsequent tasks."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 5
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - New repo orqastudio/orqastudio-sdk exists with CI + publish workflow
  - ArtifactGraphSDK extracted from artifact-graph.svelte.ts
  - IPC invoke wrapper extracted from ui/src/lib/ipc/invoke.ts
  - Frontmatter parser extracted from ui/src/lib/utils/frontmatter.ts
  - "Types imported from @orqastudio/types"
  - Svelte 5 as peer dependency (runes used in SDK)
  - "@tauri-apps/api as peer dependency"
  - "Unit tests for graph resolution, traversal, and subscription API"
  - Published to GitHub Packages
relationships:
  - target: EPIC-90cb7349
    type: delivers
    rationale: Core SDK — graph, IPC, and utilities
  - target: TASK-752a79e7
    type: depends-on
  - target: TASK-b1039dca
    type: depended-on-by
  - target: TASK-ddb2414f
    type: depended-on-by
---

## Scope

### From ui/src/lib/sdk/
- `artifact-graph.svelte.ts` → `src/graph/artifact-graph.svelte.ts`

### From ui/src/lib/ipc/
- `invoke.ts` → `src/ipc/invoke.ts` (invoke wrapper, extractErrorMessage, createStreamChannel)

### From ui/src/lib/utils/
- `frontmatter.ts` → `src/utils/frontmatter.ts`

### Peer dependencies
- `svelte` ≥ 5.0 (SvelteMap, runes)
- `@tauri-apps/api` (invoke, Channel, event listen)