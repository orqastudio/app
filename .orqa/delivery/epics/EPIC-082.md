---
id: EPIC-082
title: Dev environment migration and schema-driven enforcement
description: >
  Consolidate all OrqaStudio repos into a submodule-based dev environment,
  migrate project-level artifacts there, and make all integrity enforcement
  schema-driven rather than hardcoded.
status: active
priority: P1
scoring:
  impact: 5
  urgency: 5
  complexity: 5
  dependencies: 5
created: 2026-03-16
updated: 2026-03-16
horizon: now
relationships:
  - target: MS-002
    type: fulfils
  - target: AD-055
    type: driven-by
    rationale: Organisation-mode architecture decision drives this work
---

## Objective

Consolidate the OrqaStudio development workflow into a single `orqastudio-dev` repo that uses git submodules for all sub-repositories, with npm link wiring local code instances. Move all project-level governance artifacts into the dev environment. Make ALL integrity enforcement schema-driven — no hardcoded artifact types or relationships anywhere in the codebase.

## Background

OrqaStudio currently has 6 repos developed independently:
- `orqastudio-app` — Tauri desktop app (Rust + Svelte)
- `orqastudio-types` — TypeScript type definitions
- `orqastudio-sdk` — Svelte 5 SDK (stores, graph SDK)
- `orqastudio-integrity-validator` — CLI integrity checker
- `orqastudio-svelte-components` — Shared UI components
- `orqastudio-graph-visualiser` — Cytoscape graph viz

Artifact types and relationships are currently hardcoded as string constants across Rust, TypeScript, and the integrity validator. Core types should be just the default dataset shipped with the platform — identical in kind to what a plugin provides.

## Phase 1: Dev Environment Setup

**Repo:** `git@github.com:orqastudio/orqastudio-dev.git`

### Structure
```
orqastudio-dev/
├── .orqa/                    # Org-level governance artifacts
│   ├── project.json          # organisation: true, lists child projects
│   ├── principles/           # Product-level pillars, vision, personas
│   ├── discovery/            # Product-level ideas, research
│   ├── delivery/             # Product-level milestones, epics, tasks
│   ├── process/              # Product-level decisions, rules, lessons, skills, agents
│   └── documentation/        # Product-level docs
├── app/                      # submodule → orqastudio-app
├── libs/
│   ├── types/                # submodule → orqastudio-types
│   ├── sdk/                  # submodule → orqastudio-sdk
│   ├── integrity-validator/  # submodule → orqastudio-integrity-validator
│   ├── svelte-components/    # submodule → orqastudio-svelte-components
│   └── graph-visualiser/     # submodule → orqastudio-graph-visualiser
├── scripts/
│   └── link-all.sh           # npm link setup script
├── Makefile                  # verify, build, test targets
└── README.md
```

### Tasks
- [x] Clone dev repo, add all submodules
- [ ] Install deps and npm link all libs → app
- [ ] Create `scripts/link-all.sh` for reproducible setup
- [ ] Create dev Makefile with verify/build/test targets
- [ ] Configure `.orqa/project.json` with `organisation: true`

## Phase 2: Artifact Migration

Move project-level `.orqa/` artifacts from `orqastudio-app` to `orqastudio-dev`.

### What moves:
- `principles/` — pillars, vision, personas, grounding
- `discovery/` — ideas, research, wireframes
- `delivery/` — milestones, epics, tasks
- `process/` — decisions, rules, lessons, skills, agents
- `documentation/` — project-level docs

### What stays in app:
- `project.json` — app-specific config (artifact types, statuses, delivery hierarchy)
- `documentation/platform/` — platform documentation that ships with the app
- App-specific `.claude/` config

### Tasks
- [ ] Copy `.orqa/` tree to dev environment
- [ ] Update `project.json` in dev environment with org config
- [ ] Strip project artifacts from app repo (keep platform config only)
- [ ] Verify all artifact relationships still resolve in the merged org graph
- [ ] Run integrity checks from dev environment

## Phase 3: Schema-Driven Enforcement

**The core principle:** Zero hardcoded artifact types or relationships in any code path. Core types/relationships are the default dataset shipped with the platform — loaded from configuration, not compiled into code.

### 3a: Platform Config as Data

Replace `PLATFORM_RELATIONSHIPS` and `PLATFORM_ARTIFACT_TYPES` TypeScript constants with a shipped JSON config file loaded at runtime.

**File:** `orqa-types/platform/core.json` (or similar)
```json
{
  "artifactTypes": [...],
  "relationships": [
    { "key": "informs", "inverse": "informed-by", "label": "Informs", ... },
    ...
  ]
}
```

- `INVERSE_MAP` becomes a derived function: `buildInverseMap(relationships)`
- Both Rust and TS load from the same data source
- The Rust backend reads this from the app's bundled platform config
- The TS validator reads it from disk or package

### 3b: Merged Runtime Registry

At runtime, three layers merge:
1. **Platform defaults** — core.json shipped with the app
2. **Project config** — `project.json` relationships and delivery types
3. **Plugin provides** — each plugin's `provides.relationships` and `provides.schemas`

Both Rust and TS build:
- `TypeRegistry` — all known artifact types with their paths, labels, icons
- `RelationshipRegistry` — all known relationship pairs with inverses and type constraints
- `InverseMap` — derived from RelationshipRegistry

### 3c: Check Logic Driven by Schema

Integrity checks query the registry instead of hardcoding strings:
- "Does this surpassed idea have a relationship whose type has lineage semantics?" (not `if rel === "evolves-into"`)
- "Does this delivery node have the parent relationship declared in the delivery config?" (already works this way in Rust)
- Inverse validation uses the merged InverseMap, not a constant

### Tasks
- [ ] Create `platform/core.json` in orqa-types with canonical types and relationships
- [ ] Replace `INVERSE_MAP` constant with `buildInverseMap()` function
- [ ] Remove `PLATFORM_RELATIONSHIPS` constant, load from JSON
- [ ] Update Rust `INVERSE_MAP` to load from platform config
- [ ] Update integrity validator to load platform + project config
- [ ] Remove all hardcoded relationship type strings from check implementations
- [ ] Remove all hardcoded artifact type strings from type inference functions

## Phase 4: Verification

- [ ] `cargo test` — all tests pass
- [ ] `npx tsc --noEmit` — types and SDK clean
- [ ] `npx svelte-check --threshold warning` — app clean
- [ ] `make verify` from dev environment — all integrity checks pass
- [ ] Open dev environment as org project in app — cross-repo artifacts visible

## Acceptance Criteria

- [ ] All development happens via `orqastudio-dev` with submodules
- [ ] Project-level artifacts live in dev environment, not in app repo
- [ ] No hardcoded artifact type strings in Rust backend or TS validator
- [ ] No hardcoded relationship type strings in Rust backend or TS validator
- [ ] INVERSE_MAP derived from relationship config, not a constant
- [ ] Integrity checks consume schema config, not string literals
- [ ] Old local repo directories can be safely deleted
