# OrqaStudio Dev Environment

This is the organisation-mode development environment for OrqaStudio — an AI-assisted clarity engine that turns messy situations into structured understanding and evolving plans.

## Repository Structure

This repo aggregates all OrqaStudio sub-repositories via git submodules:

```
orqastudio-dev/
├── .orqa/              # Product-level governance artifacts
├── app/                # Tauri v2 desktop app (Rust + Svelte 5 + SQLite)
├── libs/
│   ├── types/          # @orqastudio/types — shared TypeScript types + platform/core.json
│   ├── sdk/            # @orqastudio/sdk — Svelte 5 stores, graph SDK, plugin registry
│   ├── integrity-validator/  # @orqastudio/integrity-validator — CLI integrity checker
│   ├── svelte-components/    # @orqastudio/svelte-components — shared UI components
│   └── graph-visualiser/     # @orqastudio/graph-visualiser — Cytoscape graph viz
├── scripts/link-all.sh       # npm link setup (run for fresh checkout)
└── Makefile                  # verify, build, test targets
```

Libraries are wired into the app via `npm link`. Run `bash scripts/link-all.sh` after fresh checkout.

## The Core Idea

**Everything is a node on a graph. Nodes connect through typed relationships. The graph IS the thinking made visible.**

Relationships aren't metadata — they ARE the structured thinking. When a user connects an idea to a pillar, they're making their reasoning visible and traceable.

## Three Pillars

Read these to understand every decision:
- `.orqa/principles/pillars/PILLAR-001.md` — **Clarity Through Structure**: making thinking visible
- `.orqa/principles/pillars/PILLAR-002.md` — **Learning Through Reflection**: the system improves over time
- `.orqa/principles/pillars/PILLAR-003.md` — **Purpose Through Continuity**: intent survives implementation pressure

Product vision: `.orqa/principles/vision/vision.md`

## Key Architecture Decisions

- `AD-051` — Three-layer artifact model (app-fixed, app-required, project-scoped)
- `AD-052` — Canonical relationship vocabulary (relationships are the ONLY connections)
- `AD-053` — Canonical status model (12 universal statuses)
- `AD-054` — Four enforcement layers (app, scanners, integrity, git hooks)
- `AD-055` — Organisation-mode multi-project architecture

These live in `.orqa/process/decisions/`.

## Artifact Layering

### What lives in the app repo (`app/.orqa/`)
Platform-shipped artifacts that define core behaviour:
- `project.json` — artifact types, statuses, delivery hierarchy, project relationships
- `documentation/platform/` — platform docs
- `process/agents/` — 8 core agents (orchestrator, planner, implementer, etc.)
- `process/rules/` — 33 core rules (`layer: core`)
- `process/skills/` — 32 core/setup skills (`layer: core` or `setup`)

### What lives here (`.orqa/`)
Product-level governance spanning the whole ecosystem:
- `principles/` — pillars, vision, personas, grounding
- `discovery/` — ideas, research, wireframes
- `delivery/` — milestones, epics, tasks
- `process/decisions/` — all architecture decisions
- `process/lessons/` — all lessons learned
- `process/rules/` — 13 project rules (`layer: project`)
- `process/skills/` — 20 project skills (`layer: project`)

### What lives in the types lib (`libs/types/src/platform/core.json`)
The single source of truth for canonical types and relationships — loaded at runtime by both Rust and TypeScript. No artifact types or relationship keys are hardcoded in any code path.

## Schema-Driven Enforcement

**Zero hardcoded artifact types or relationships.** Everything is config-driven:

- `core.json` defines platform types, relationships, and **semantic categories** (lineage, hierarchy, governance, grounding, knowledge-flow)
- Checks query semantics: "does this idea have a lineage relationship?" not `if rel === "evolves-into"`
- Inverse maps are derived from relationship config via `buildInverseMap()`
- Type inference comes from `project.json` artifacts config, not path pattern matching
- Three layers merge at runtime: platform defaults → project config → plugin provides

## Relationship Vocabulary

All connections use the `relationships` frontmatter array with `target` and `type` fields.

| Forward | Inverse | Semantic |
|---------|---------|----------|
| `informs` | `informed-by` | knowledge-flow |
| `evolves-into` | `evolves-from` | lineage |
| `drives` | `driven-by` | motivation |
| `governs` | `governed-by` | governance |
| `delivers` | `delivered-by` | hierarchy |
| `enforces` | `enforced-by` | enforcement |
| `grounded` | `grounded-by` | grounding |
| `observes` | `observed-by` | observation |
| `merged-into` | `merged-from` | lineage |
| `synchronised-with` | `synchronised-with` | synchronisation |

**Type constraints** (from core.json):
- `enforces`: only FROM rule, only TO decision
- `grounded`/`grounded-by`: only TO pillar
- `drives`/`driven-by`: only FROM decision
- `observes`/`observed-by`: only FROM agent

Project relationships (e.g. `depends-on`/`depended-on-by`) defined in `project.json`.

## 12 Canonical Statuses

`captured` → `exploring` → `ready` → `prioritised` → `active` → `hold` / `blocked` → `review` → `completed` → `surpassed` / `archived` / `recurring`

## Development Workflow

```bash
# Start the app
cd app && make dev

# Run all checks
make verify

# Individual checks
make verify-integrity      # orqa-integrity on dev env root
make verify-rust           # cargo test (628 tests)
make verify-app            # svelte-check
make verify-types          # tsc on types lib
make verify-sdk            # tsc on SDK

# After changing a lib, rebuild and re-link
cd libs/types && npx tsc && cd ../../app/ui && npm link @orqastudio/types
```

## Current State

- EPIC-082 (schema-driven enforcement) is complete
- Integrity check: **0 errors, 0 warnings** across 1,062 artifacts with 6,813 relationships
- 628 Rust tests, 0 clippy warnings, 0 svelte-check errors
- Active work tracked in `.orqa/delivery/epics/`

## Rules for AI

- **Never hardcode artifact types or relationship keys** — use core.json semantics
- **Relationships are bidirectional** — always add the inverse when creating a forward edge
- **`grounded`/`grounded-by` only targets pillars** — use `informs`/`informed-by` for other knowledge flow
- **`enforces` only from rules to decisions** — use `informs` for other enforcement-like connections
- **Core artifacts stay in app repo** — agents, core skills, core rules ship with the platform
- **Project artifacts stay here** — delivery, discovery, principles, decisions, lessons
- **Run `make verify` before committing** — zero errors is the baseline
