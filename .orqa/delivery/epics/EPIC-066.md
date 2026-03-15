---
id: EPIC-066
title: Package ecosystem extraction for plugin portability
description: Extract types, integrity validator, eslint config, and SDK into standalone @orqastudio/ npm packages hosted on GitHub Packages. Gives plugins parity access to the frontend-backend connection, artifact graph, and code standards.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-003
depends-on: []
research-refs:
  - RES-057
  - RES-058
docs-required: []
docs-produced: []
relationships:
  - target: IDEA-093
    type: informed-by
    rationale: Promoted from IDEA-093 — package ecosystem for plugin portability
  - target: IDEA-092
    type: informed-by
    rationale: CLI scanner parity idea drove the integrity validator extraction
  - target: RES-057
    type: informed-by
    rationale: Integrity check alignment audit identified the need for a single check library
  - target: RES-058
    type: informed-by
    rationale: SDK extraction research confirmed architecture is clean and extraction-ready
  - target: EPIC-064
    type: informs
    rationale: Enforcement bootstrapping benefits from the portable integrity validator
  - target: RES-059
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-059
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-417
    type: contains
    rationale: Epic contains this task
  - target: TASK-418
    type: contains
    rationale: Epic contains this task
  - target: TASK-419
    type: contains
    rationale: Epic contains this task
  - target: TASK-420
    type: contains
    rationale: Epic contains this task
  - target: TASK-421
    type: contains
    rationale: Epic contains this task
  - target: TASK-422
    type: contains
    rationale: Epic contains this task
  - target: TASK-423
    type: contains
    rationale: Epic contains this task
  - target: TASK-424
    type: contains
    rationale: Epic contains this task
  - target: TASK-425
    type: contains
    rationale: Epic contains this task
  - target: TASK-426
    type: contains
    rationale: Epic contains this task
  - target: TASK-427
    type: contains
    rationale: Epic contains this task
  - target: TASK-428
    type: contains
    rationale: Epic contains this task
  - target: TASK-425
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-425"
  - target: TASK-419
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-419"
  - target: TASK-427
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-427"
  - target: TASK-421
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-421"
  - target: TASK-424
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-424"
  - target: TASK-420
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-420"
  - target: TASK-417
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-417"
  - target: TASK-418
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-418"
  - target: TASK-423
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-423"
  - target: TASK-428
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-428"
  - target: TASK-426
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-426"
  - target: TASK-422
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-422"
---
## Context

OrqaStudio plugins need to import shared code — types, validation, the artifact graph SDK, and code standards. Currently everything lives inside the monolith. Three research documents confirmed:

1. [RES-057](RES-057): Integrity checks are duplicated across Rust, CLI, and pre-commit with no single source of truth
2. [RES-058](RES-058): 10 of 11 stores are fully portable; zero circular dependencies; three trivial modifications needed
3. The integrity validator repo already exists but needs types extracted and tests added

## Implementation Design

### Package architecture (Option B from RES-058)

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

- [TASK-417](TASK-417): Create @orqastudio/types package
- [TASK-418](TASK-418): Create @orqastudio/eslint-config package
- [TASK-419](TASK-419): Wire types + eslint-config into integrity validator + add tests
- [TASK-420](TASK-420): Create @orqastudio/sdk package — graph + IPC + frontmatter
- [TASK-421](TASK-421): Extract stores into SDK — session, project, artifact, conversation
- [TASK-422](TASK-422): Extract stores into SDK — enforcement, lessons, setup, settings, errors
- [TASK-423](TASK-423): Wire orqa-studio to import from packages + update pre-commit/make verify
- [TASK-424](TASK-424): Create @orqastudio/test-config — shared vitest + testing utilities
- [TASK-425](TASK-425): Fix CI workflows for independent builds (no file: references)
- [TASK-426](TASK-426): Initial publish of tier-0 packages to GitHub Packages
- [TASK-427](TASK-427): Switch tier-1 packages from file: to published deps + publish
- [TASK-428](TASK-428): Switch orqa-studio from file: to published package versions
