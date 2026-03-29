---
id: "EPIC-0e8860dd"
type: "epic"
title: "Pillars as First-Class Artifacts"
description: "Make product pillars structured artifacts in .orqa/principles/pillars/ with frontmatter schema, referenced by ID from other artifacts, and injected into AI system prompts. Replaces hardcoded pillar strings across rules and documentation. Implements PD-74a2cb7a."
status: archived
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
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

Product pillars ("Clarity Through Structure" and "Learning Through Reflection")
are currently hardcoded as strings across vision.md, governance.md,
vision-alignment.md, pillar-alignment-docs.md, and scoring dimensions. This
makes the governance framework non-portable — other projects cannot define
their own guiding principles without editing canon rules.

[PD-74a2cb7a](PD-74a2cb7a) establishes that pillars should be first-class artifacts with structured
frontmatter, referenced by ID, and injected into AI system prompts.

## Implementation Scope

### 1. Pillar Artifact Type

Create `.orqa/principles/pillars/` directory with two initial artifacts:

- `[PILLAR-c9e0a695](PILLAR-c9e0a695).md` — Clarity Through Structure
- `[PILLAR-2acd86c1](PILLAR-2acd86c1).md` — Learning Through Reflection

Schema: id, title, description, gate, status, tags.

### 2. Artifact Config Registration

Add pillars path to `project.json` artifacts array under the Planning group.

### 3. System Prompt Injection

Update the system prompt builder (`stream_commands.rs` or governance prompt
assembly) to read active pillars from `.orqa/principles/pillars/` and inject
them as structured context into every AI conversation.

### 4. Rule Genericisation

Update rules that hardcode pillar names to reference pillar artifacts instead:

- `vision-alignment.md` — "serve at least one active pillar" (generic)
- `pillar-alignment-docs.md` — read pillar titles from artifacts, not hardcoded
- `governance.md` — reference pillar artifacts instead of inline definitions

### 5. Artifact Reference Field

Add `pillars: [[PILLAR-c9e0a695](PILLAR-c9e0a695)]` frontmatter field to the epic and idea schemas
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
| ------ | ------- | ------------ |
| [TASK-bf8bf526](TASK-bf8bf526) | Create pillar artifact schema and initial pillars | — |
| [TASK-b34c735a](TASK-b34c735a) | Register pillars in artifact config | [TASK-bf8bf526](TASK-bf8bf526) |
| [TASK-3b07cafa](TASK-3b07cafa) | Update rules to reference pillar artifacts generically | [TASK-bf8bf526](TASK-bf8bf526) |
| [TASK-218bef2c](TASK-218bef2c) | Add pillar reference field to epic/idea schemas | [TASK-bf8bf526](TASK-bf8bf526) |
| [TASK-86596675](TASK-86596675) | Update system prompt assembly to inject pillars | [TASK-bf8bf526](TASK-bf8bf526), [TASK-b34c735a](TASK-b34c735a) |
| [TASK-677a65d4](TASK-677a65d4) | Update product documentation (governance.md, vision.md) | [TASK-3b07cafa](TASK-3b07cafa) |

## Dependency Chain

```text
TASK-bf8bf526 (create pillar artifacts)
  ├── TASK-b34c735a (register in config)
  │     └── TASK-86596675 (system prompt injection)
  ├── TASK-3b07cafa (genericise rules)
  │     └── TASK-677a65d4 (update product docs)
  └── TASK-218bef2c (schema reference field)
```

## Implementation Design

Implementation approach to be defined during planning.
