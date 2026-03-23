---
id: "RES-5484edbd"
type: research
title: "Artifact Layer Taxonomy and Core vs Project Classification"
description: "Investigation into how governance artifacts should be classified as core (ships with every project) vs project-specific (OrqaStudio dogfood), including layer renaming, template distribution, and user-facing documentation gaps."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
relationships:
  - target: "EPIC-4a7aeacb"
    type: "guides"
    rationale: "Research findings informed epic design"
---
## Problem Statement

OrqaStudio manages its own governance through `.orqa/` — the same system it provides to other projects. This creates a classification problem: which artifacts are **core** (universal, ship with every new project) and which are **project-specific** (exist only because OrqaStudio is dogfooding itself)?

Currently, the `layer` field uses `canon | project | plugin`, but:
1. "Canon" is unclear — it conflates "ships with the app" with "portable across projects"
2. No mechanism exists to template/inject core artifacts into new projects
3. Many `canon` artifacts are actually OrqaStudio-specific (e.g., [RULE-cb65b5d0](RULE-cb65b5d0) references OrqaStudio's specific shared components)
4. User-facing documentation ("how to use Orqa") doesn't exist — all docs are internal development docs

## Current State

### Layer Field (`layer`)

| Value | Intended Meaning | Actual Usage |
|-------|-----------------|--------------|
| `canon` | Ships with the app, portable | 37 rules, 30 skills, 7 agents — but many contain OrqaStudio-specific content |
| `project` | Project-specific | 3 rules, 16 skills — OrqaStudio dogfood artifacts |
| `plugin` | External ecosystem | 0 artifacts — placeholder for future |

### Scope Field (`scope`) — Rules Only

| Value | Count | Meaning |
|-------|-------|---------|
| `system` | 22 | Universal process rules (delegation, honest reporting, etc.) |
| `project` | 10 | Project-specific standards (coding standards, component rules) |
| `general` | 6 | Recently created rules, general-purpose |
| `artifact` | 1 | [RULE-a764b2ae](RULE-a764b2ae) (schema validation) |

**Problem:** `layer` and `scope` overlap. A rule with `layer: canon, scope: project` is confusing — is it core or project-specific? The two dimensions try to express different things but their values collide (`project` appears in both).

### Skills Layer vs Scope

Skills have both `layer` (canon/project/plugin) and `scope` (general/software-engineering/governance). These are cleaner — `layer` = distribution level, `scope` = domain applicability. But `layer: canon` still conflates "universal" with "ships with the platform."

## Analysis

### What Actually Needs to Ship with Every Project?

**Tier 1 — Universal Process (any project type):**
- Agent delegation (RULE-532100d9)
- Documentation-first (RULE-9daf29c0)
- Honest reporting (RULE-878e5422)
- Artifact lifecycle (RULE-7b770593)
- Structure before work (RULE-8035e176)
- Systems thinking (RULE-d90112d9)
- Error ownership (RULE-57ccb4a3)
- No stubs (RULE-e9c54567)
- No deferred deliverables (RULE-e120bb70)
- No aliases (RULE-9ba80a19)
- Historical preservation (RULE-6d1c8dc7)
- Artifact ID semantics (RULE-22783309)
- Pillar alignment — vision (RULE-1e8a1914) and docs (RULE-39169bcd)
- Plan compliance (RULE-303c1cc8)
- Enforcement before code (RULE-3eccebf3)
- Lessons learned (RULE-551bde31)
- UAT process (RULE-4d4f540d)
- Context window management (RULE-df24948b)
- Tool access restrictions (RULE-f809076f)
- Schema validation (RULE-a764b2ae)
- Artifact link format (RULE-2f7b6a31)
- Skill portability (RULE-11c29c9e)
- User-invocable semantics (RULE-5ee43922)
- Session management (RULE-e352fd0a)
- Skill enforcement (RULE-deab6ea7)
- Required reading (RULE-b2753bad)

**Tier 2 — Software Development (software projects only):**
- Coding standards (RULE-b49142be) — needs to be a template, not hardcoded to Rust/Svelte
- Git workflow (RULE-633e636d) — worktree pattern is universal for software
- End-to-end completeness (RULE-1acb1602) — Tauri-specific, needs generalisation
- Development commands (RULE-c71f1c3f) — OrqaStudio-specific `make` targets
- Testing standards (RULE-f10bb5de) — references OrqaStudio test commands
- Root cleanliness (RULE-1f30904a) — lists OrqaStudio-specific root files
- Architecture decisions (RULE-65973a88) — references OrqaStudio's decisions
- Reusable components (RULE-cb65b5d0) — lists OrqaStudio's specific components
- Code search (RULE-5e03e67b) — references ChunkHound, which is OrqaStudio infra
- Tooltip usage (RULE-89155a7f) — shadcn-specific

**Tier 3 — OrqaStudio-Only:**
- Dogfood mode (RULE-6083347d) — only relevant when editing the app you're inside
- Artifact config integrity (RULE-6c0496e0) — references OrqaStudio's `project.json`

### The Classification Problem

Many "Tier 1" rules reference OrqaStudio-specific examples, paths, and patterns. They would need to be **parameterised** or **templated** before they can truly ship as core. For example:
- [RULE-7b770593](RULE-7b770593) references `.orqa/delivery/epics/` paths — but those paths ARE the core structure
- [RULE-deab6ea7](RULE-deab6ea7) lists OrqaStudio's specific skill injection table — but each project would have its own
- [RULE-cb65b5d0](RULE-cb65b5d0) lists OrqaStudio's shared components — but is the *pattern* of "use shared components" universal

**Key insight:** The *constraint* is often universal, but the *content* (examples, paths, component lists) is project-specific. This maps naturally to rule + skill pairs where the rule is core and the skill is project-specific.

## Proposals

### P1: Rename `canon` to `core`

Replace the `layer` enum:

| Old | New | Meaning |
|-----|-----|---------|
| `canon` | `core` | Ships with OrqaStudio, injected into every new project |
| `project` | `project` | Created by/for this specific project |
| `plugin` | `plugin` | 1st party official extensions |
| — | `community` | Community-contributed extensions |
| — | `user` | User-only workflows (personal preferences, private rules) |

**Why:** "Canon" implies religious authority. "Core" is clearer — it's what comes in the box. "Plugin" is retained for 1st party official extensions. "Community" is added for community-contributed content. "User" supports future personal workflow customisation.

### P2: Separate `scope` from `layer`

Redefine `scope` as "what kind of project does this apply to":

| Value | Meaning | Example |
|-------|---------|---------|
| `universal` | Any project type | [RULE-532100d9](RULE-532100d9) (delegation), [RULE-9daf29c0](RULE-9daf29c0) (docs-first) |
| `software` | Software development projects | [RULE-b49142be](RULE-b49142be) (coding standards), [RULE-633e636d](RULE-633e636d) (git workflow) |
| `governance` | Governance management work | (internal classification) |

Remove `system`, `project`, `general`, `artifact` from scope — these are confusing.

The two dimensions then cleanly separate:
- **`layer`** = distribution mechanism (core / project / community / user)
- **`scope`** = applicability domain (universal / software / governance)

### P3: Template Distribution System

When OrqaStudio initialises a new project (`project-setup` skill), it should:

1. **Copy core artifacts** — all `layer: core` rules, skills, and agents
2. **Apply project type preset** — `project-type-software` adds software-scope artifacts
3. **Parameterise content** — replace OrqaStudio-specific examples with placeholders or project-detected values (using `project-inference` skill)
4. **Mark as core** — copied artifacts retain `layer: core` so future updates can be diffed

This is partially implemented via `project-setup` and `project-type-software` skills, but they describe the *process*, not the *content*. The actual rules/skills they inject are the OrqaStudio dogfood versions, not generalised templates.

### P4: Audit and Reclassify Existing Artifacts

Every current `layer: canon` artifact needs evaluation:

| Question | Classification |
|----------|---------------|
| Is the constraint universal? Is the content universal? | `layer: core` — ships as-is |
| Is the constraint universal but content is project-specific? | Split: core rule (constraint) + project skill (content) |
| Is the entire artifact project-specific? | `layer: project` — stays in OrqaStudio only |
| Is the artifact only useful during dogfooding? | `layer: project` + flag as dogfood-only |

### P5: User-Facing Documentation

Currently all documentation in `.orqa/documentation/` is internal development documentation for OrqaStudio. There is no "how to use OrqaStudio" documentation for end users. Needed:

| Document | Purpose |
|----------|---------|
| Getting Started Guide | How to install and set up OrqaStudio on your project |
| Core Concepts | Pillars, artifacts, layers, governance model explained for users |
| Layer System Guide | What core/project/community/user mean, how to manage them |
| Artifact Types Reference | What each artifact type is for, when to create one |
| Customisation Guide | How to add project-specific rules, skills, and agents |
| Template System | How core artifacts are injected and how to override them |

These should live in a user-facing documentation section, separate from the internal development docs.

### P6: Standardise Layer Across All Schemas

Currently only rules, skills, and agents have `layer`. Decisions and lessons don't. If layer indicates distribution scope, it should be on everything:

| Artifact Type | Has `layer`? | Should Have? |
|--------------|-------------|-------------|
| Rules | Yes | Yes |
| Skills | Yes | Yes |
| Agents | Yes | Yes |
| Decisions | No | Yes — some decisions are core (e.g., "use file-based artifacts"), some are project-specific (e.g., "use SQLite for conversations") |
| Lessons | No | Probably not — lessons are always project-specific until promoted |
| Research | No | No — always project-specific |
| Ideas | No | No — always project-specific |
| Epics | No | No — always project-specific |
| Tasks | No | No — always project-specific |
| Milestones | No | No — always project-specific |
| Pillars | No | Maybe — some pillars might be suggested defaults |

## Recommendations

**Phase 1 — Classification (this task):**
1. Audit every `layer: canon` artifact and classify as truly-core vs OrqaStudio-specific
2. Rename `canon` → `core` across schemas, artifacts, and code
3. Standardise `scope` values to `universal | software | governance`
4. Add `layer` to decision schema

**Phase 2 — Template System:**
1. Create generalised versions of core artifacts (strip OrqaStudio-specific content)
2. Build template injection into `project-setup` skill
3. Support project-type presets that layer on domain-specific artifacts

**Phase 3 — User Documentation:**
1. Create user-facing documentation section
2. Write getting started, core concepts, customisation guides
3. Document the layer system for end users

## Decisions (resolved 2026-03-11)

1. **Layer enum**: `core | project | plugin | community | user`. Both `plugin` (1st party official extensions) and `community` (community-contributed) exist as separate layers. This is NOT a rename of `plugin` — both are needed.
2. **User layer**: Added to schema now, for future support of personal workflows.
3. **User-facing docs**: `.orqa/documentation/guide/` — a new section within the existing documentation tree, separate from internal development docs.
4. **Template storage**: Git repo with a release path. Core artifact templates are maintained in a dedicated repository and distributed via tagged releases.