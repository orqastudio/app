---
id: "EPIC-962a7bc9"
type: epic
title: "Pillars as First-Class Artifacts"
description: "Make product pillars structured artifacts in .orqa/process/pillars/ with frontmatter schema, referenced by ID from other artifacts, and injected into AI system prompts. Replaces hardcoded pillar strings across rules and documentation. Implements AD-47c41f0c."
status: "completed"
priority: "P1"
created: "2026-03-09"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-ba209f54"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-19a7f11e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a48075b1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-91807ad8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-47df8cbd"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7ea07ed8"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6ea994f2"
    type: "delivered-by"
    rationale: "Epic contains this task"
---
## Context

Product pillars ("Clarity Through Structure" and "Learning Through Reflection")
are currently hardcoded as strings across vision.md, governance.md,
vision-alignment.md, pillar-alignment-docs.md, and scoring dimensions. This
makes the governance framework non-portable — other projects cannot define
their own guiding principles without editing canon rules.

[AD-47c41f0c](AD-47c41f0c) establishes that pillars should be first-class artifacts with structured
frontmatter, referenced by ID, and injected into AI system prompts.

## Implementation Scope

### 1. Pillar Artifact Type

Create `.orqa/process/pillars/` directory with two initial artifacts:

- `[PILLAR-569581e0](PILLAR-569581e0).md` — Clarity Through Structure
- `[PILLAR-cdf756ff](PILLAR-cdf756ff).md` — Learning Through Reflection

Schema: id, title, description, gate, status, tags.

### 2. Artifact Config Registration

Add pillars path to `project.json` artifacts array under the Planning group.

### 3. System Prompt Injection

Update the system prompt builder (`stream_commands.rs` or governance prompt
assembly) to read active pillars from `.orqa/process/pillars/` and inject
them as structured context into every AI conversation.

### 4. Rule Genericisation

Update rules that hardcode pillar names to reference pillar artifacts instead:
- `vision-alignment.md` — "serve at least one active pillar" (generic)
- `pillar-alignment-docs.md` — read pillar titles from artifacts, not hardcoded
- `governance.md` — reference pillar artifacts instead of inline definitions

### 5. Artifact Reference Field

Add `pillars: [[PILLAR-569581e0](PILLAR-569581e0)]` frontmatter field to the epic and idea schemas
in `artifact-framework.md`. Update scoring to reference pillar IDs.

## Constraints

- **Orchestrator-only work** — This affects rules and governance artifacts directly.
  No delegation needed; all changes are governance/docs.
- **No code changes required for MVP** — The pillar artifacts, rule updates, and
  prompt injection text can all be done without Rust/Svelte changes. The system
  prompt is already assembled from governance files. Future: Rust-side pillar
  reading for config-driven injection.
- **Backward compatible** — Existing pillar alignment sections in docs remain
  valid; they just reference artifact IDs instead of hardcoded strings.

## Tasks

| Task | Title | Depends On |
|------|-------|------------|
| [TASK-ba209f54](TASK-ba209f54) | Create pillar artifact schema and initial pillars | — |
| [TASK-19a7f11e](TASK-19a7f11e) | Register pillars in artifact config | [TASK-ba209f54](TASK-ba209f54) |
| [TASK-a48075b1](TASK-a48075b1) | Update rules to reference pillar artifacts generically | [TASK-ba209f54](TASK-ba209f54) |
| [TASK-91807ad8](TASK-91807ad8) | Add pillar reference field to epic/idea schemas | [TASK-ba209f54](TASK-ba209f54) |
| [TASK-47df8cbd](TASK-47df8cbd) | Update system prompt assembly to inject pillars | [TASK-ba209f54](TASK-ba209f54), [TASK-19a7f11e](TASK-19a7f11e) |
| [TASK-7ea07ed8](TASK-7ea07ed8) | Update product documentation (governance.md, vision.md) | [TASK-a48075b1](TASK-a48075b1) |

## Dependency Chain

```
TASK-ba209f54 (create pillar artifacts)
  ├── TASK-19a7f11e (register in config)
  │     └── TASK-47df8cbd (system prompt injection)
  ├── TASK-a48075b1 (genericise rules)
  │     └── TASK-7ea07ed8 (update product docs)
  └── TASK-91807ad8 (schema reference field)
```

## Implementation Design

Implementation approach to be defined during planning.