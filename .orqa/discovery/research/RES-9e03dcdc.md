---
id: "RES-9e03dcdc"
type: discovery-research
title: "Vision Alignment Audit & Config-Driven Artifacts"
description: "Comprehensive audit aligning documentation, governance rules, and agent definitions with the three-layer architecture vision. Produced config-driven artifact scanning, simplified schema (plans merged into research), and clean traceability chain from Task to Epic to Milestone."
status: surpassed
created: "2026-03-08"
updated: "2026-03-08"
relationships:
  - target: "EPIC-57dd7d4c"
    type: "guides"
    rationale: "Research findings informed the design of Vision Alignment & Schema Simplification"
  - target: "RES-aefc9e44"
    type: "merged-into"
---

## Problem Statement

The product vision has evolved to explicitly articulate a three-layer enforcement
architecture (Canon + Project + Plugin), provider-agnostic AI integration, and
`.orqa/` as the sole source of truth. Multiple documentation pages, governance
rules, and agent definitions still reference outdated concepts. Additionally, the
artifact schema has unnecessary complexity — plans and research overlap, and the
traceability chain (Task → ??? → Milestone) is broken.

## Guiding Principles

1. **`.orqa/` is the source of truth** — `.claude/` is an optional symlink layer
2. **AI provider is an implementation option** — the app is a framework for

   systems thinking, not a Claude companion

3. **Config drives structure** — `project.json` `artifacts` array defines what

   the app shows and scans

4. **Canon vs Project vs Plugin** — three layers, clearly separated
5. **Tasks are traceable** — Task → Epic → Milestone, always navigable
6. **Research is thinking, epics are commitment** — no separate "plan" type

---

## Phase 1: Documentation Audit (DONE)

Audited all 57 documentation files + 20 rules + 15 agents for vision alignment.

### Completed Tasks

- [TASK-c79077be](TASK-c79077be): Product docs audit (8 files)
- [TASK-43c190d2](TASK-43c190d2): Architecture/process docs audit (15 files)
- [TASK-8db2e1c3](TASK-8db2e1c3): Rules and agents audit (35 files)
- Additional: Remaining architecture docs (11 files), process/dev docs (8 files), UI/wireframes (15 files)

---

## Phase 2: Config-Driven Artifacts (DONE)

Replaced hardcoded navigation and scanner with config-driven logic.

### Completed Tasks

- [TASK-25e35dfc](TASK-25e35dfc): Rust types for ArtifactEntry/ArtifactTypeConfig
- [TASK-601a75ca](TASK-601a75ca): Config-driven scanner in artifact_reader.rs (recursive file explorer pattern)
- [TASK-0c48a446](TASK-0c48a446): Frontend config-driven navigation (deleted hardcoded constants)
- [TASK-36a4b6c8](TASK-36a4b6c8): Task schema enhancement (assignee + skills fields)

### Also Completed (unplanned but necessary)

- Fixed `$derived(() => ...)` reactivity bugs across 3 components
- Fixed `humanize_name` to preserve artifact IDs with digits
- Fixed config paths in project.json to match actual disk structure
- Set up `.claude/` → `.orqa/` symlinks (rules, agents, skills, hooks, CLAUDE.md)
- Created enforcement rules: `artifact-config-integrity.md`, `enforcement-before-code.md`, `historical-artifacts.md`

---

## Phase 3: Artifact Schema Simplification

Merge the "plan" artifact type into research. Establish a clean traceability chain
where tasks always reference epics, and research documents are referenced wherever
relevant via `research-refs`.

### The Change

**Before:**

```text
Research ←→ Ideas
Plan ← Tasks (no link to epics or milestones)
Epic → Milestone (separate from plans)
```

**After:**

```text
Research ←→ Ideas (exploration phase)
Epic → Milestone (committed work, design in body)
Task → Epic (always, no exceptions)
research-refs on any artifact (traceability to thinking)
```

- **Plans become research docs** — design explorations, architecture spikes, investigation documents
- **Existing plans** move to `.orqa/discovery/research/` and are marked `status: surpassed`
- **Tasks always reference an epic** via `epic:` field (not `plan:`)
- **Epics contain their own design** in the markdown body
- **`research-refs`** field available on epics, tasks, and decisions for linking back to research
- **Remove `plans` from the `artifacts` config** in project.json
- **Remove `.orqa/delivery/plans/` directory** after migration

### Tasks

- [TASK-edeea471](TASK-edeea471): Update artifact-framework.md — remove Plan type, update schemas
- [TASK-e3c4da9f](TASK-e3c4da9f): Migrate existing plans to research, update references
- [TASK-252828c9](TASK-252828c9): Update artifact-lifecycle.md — remove plan status transitions, add `epic:` to task schema
- [TASK-c197caf7](TASK-c197caf7): Update project.json artifacts config — remove plans entry
- [TASK-e79a1581](TASK-e79a1581): Update all existing tasks to reference epics instead of plans

### Verification

- No `plan:` field in any task frontmatter (replaced by `epic:`)
- No files in `.orqa/delivery/plans/` (moved to research, marked surpassed)
- artifact-framework.md has no Plan type definition
- artifact-lifecycle.md has no plan status transitions
- Every task has an `epic:` field that references an existing epic
- `research-refs` field documented and used on at least one epic

---

## Phase 4: Historical Backfill

Backfill decision chains, surpassed artifacts, and lesson history so the
artifact lifecycle has real data to test navigation and visualization against.

### Tasks

- [TASK-bf4b1013](TASK-bf4b1013): Historical backfill (decision chains, surpassed research, lessons, surpassed tasks)

### Verification

- At least 3 decisions have surpassed predecessors
- At least 2 research docs marked surpassed
- At least 3 additional lessons from real session history
- Decision chains are navigable (AD-NNN surpassed-by AD-MMM)

---

## Overall Verification

- `make check` passes
- App starts, all artifact categories show correct content
- Clean chain: Task → Epic → Milestone (no orphaned tasks)
- No `.claude/`-as-source-of-truth anywhere in docs
- Provider-agnostic language throughout
- Historical artifacts preserved with `status: surpassed`
