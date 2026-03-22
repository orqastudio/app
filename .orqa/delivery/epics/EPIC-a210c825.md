---
id: "EPIC-a210c825"
title: "Package ecosystem extraction for plugin portability"
description: "Extract types, integrity validator, eslint config, and SDK into standalone @orqastudio/ npm packages hosted on GitHub Packages. Gives plugins parity access to the frontend-backend connection, artifact graph, and code standards."
status: "completed"
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 5
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
horizon: "active"
relationships:
  - target: "RES-d3f58f2d"
    type: "guided-by"
    rationale: "Integrity check alignment audit identified the need for a single check library"
  - target: "RES-f9356600"
    type: "guided-by"
    rationale: "SDK extraction research confirmed architecture is clean and extraction-ready"
  - target: "RES-0fd8fe85"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-0fd8fe85"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-e6732238"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ff56038f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-248a0485"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-3b56186e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2aca491a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-51959c4d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b9cb39f8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a6dcd46b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f3091875"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-c3abf5c1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-82add1d1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-9593e951"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-c4f10c20"
    type: "realised-by"
  - target: "IDEA-a620e1f5"
    type: "realised-by"
---
## Context

OrqaStudio plugins need to import shared code — types, validation, the artifact graph SDK, and code standards. Currently everything lives inside the monolith. Three research documents confirmed:

1. [RES-d3f58f2d](RES-d3f58f2d): Integrity checks are duplicated across Rust, CLI, and pre-commit with no single source of truth
2. [RES-f9356600](RES-f9356600): 10 of 11 stores are fully portable; zero circular dependencies; three trivial modifications needed
3. The integrity validator repo already exists but needs types extracted and tests added

## Implementation Design

### Package architecture (Option B from RES-f9356600)

```
@orqastudio/types                    ← stable, tiny, used by everything
  ↑
@orqastudio/integrity-validator      ← graph checks (repo exists)
@orqastudio/eslint-config            ← shared code standards
@orqastudio/test-config              ← shared vitest config + test utilities
@orqastudio/sdk                      ← graph, stores, IPC, frontmatter
```

### Phase 1: Foundation (types + eslint-config)

Create `@orqastudio/types` with all 13 type files from `ui/src/lib/types/` plus shared constants. Create `@orqastudio/eslint-config` extracted from the main repo's ESLint and TypeScript config. Wire both into the existing integrity validator repo.

### Phase 2: Integrity validator completion

Add unit tests to the integrity validator. Import types from `@orqastudio/types`. Wire `@orqastudio/eslint-config`. Ensure it matches the Rust scanner's full check suite.

### Phase 3: SDK extraction

Create `@orqastudio/sdk` with the ArtifactGraphSDK, all portable stores, IPC wrapper, and frontmatter parser. Three modifications: DEFAULT_MODEL as config param, theme application as injectable callback, browser error handlers as opt-in.

### Phase 4: Integration

Wire orqa-studio main app to import from the packages instead of local files. Update pre-commit hook to use `@orqastudio/integrity-validator`. Update `make verify` targets.

### Out of Scope

- UI component library extraction (separate epic per user direction)
- `@orqastudio/create-plugin` scaffolding CLI (after SDK and components exist)
- NavigationStore extraction (too coupled to app layout)

## Tasks

- [TASK-e6732238](TASK-e6732238): Create @orqastudio/types package
- [TASK-ff56038f](TASK-ff56038f): Create @orqastudio/eslint-config package
- [TASK-248a0485](TASK-248a0485): Wire types + eslint-config into integrity validator + add tests
- [TASK-3b56186e](TASK-3b56186e): Create @orqastudio/sdk package — graph + IPC + frontmatter
- [TASK-2aca491a](TASK-2aca491a): Extract stores into SDK — session, project, artifact, conversation
- [TASK-51959c4d](TASK-51959c4d): Extract stores into SDK — enforcement, lessons, setup, settings, errors
- [TASK-b9cb39f8](TASK-b9cb39f8): Wire orqa-studio to import from packages + update pre-commit/make verify
- [TASK-a6dcd46b](TASK-a6dcd46b): Create @orqastudio/test-config — shared vitest + testing utilities
- [TASK-f3091875](TASK-f3091875): Fix CI workflows for independent builds (no file: references)
- [TASK-c3abf5c1](TASK-c3abf5c1): Initial publish of tier-0 packages to GitHub Packages
- [TASK-82add1d1](TASK-82add1d1): Switch tier-1 packages from file: to published deps + publish
- [TASK-9593e951](TASK-9593e951): Switch orqa-studio from file: to published package versions
