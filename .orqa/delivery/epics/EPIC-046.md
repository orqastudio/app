---
id: EPIC-046
title: Pillars as First-Class Artifacts
description: "Make product pillars structured artifacts in .orqa/process/pillars/ with frontmatter schema, referenced by ID from other artifacts, and injected into AI system prompts. Replaces hardcoded pillar strings across rules and documentation. Implements AD-031."
status: completed
priority: P1
created: 2026-03-09
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-058
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-059
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-060
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-061
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-062
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-063
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-336
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: AD-031
    type: informs
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: informed-by
---
## Context

Product pillars ("Clarity Through Structure" and "Learning Through Reflection")
are currently hardcoded as strings across vision.md, governance.md,
vision-alignment.md, pillar-alignment-docs.md, and scoring dimensions. This
makes the governance framework non-portable — other projects cannot define
their own guiding principles without editing canon rules.

[AD-031](AD-031) establishes that pillars should be first-class artifacts with structured
frontmatter, referenced by ID, and injected into AI system prompts.

## Implementation Scope

### 1. Pillar Artifact Type

Create `.orqa/process/pillars/` directory with two initial artifacts:

- `[PILLAR-001](PILLAR-001).md` — Clarity Through Structure
- `[PILLAR-002](PILLAR-002).md` — Learning Through Reflection

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

Add `pillars: [[PILLAR-001](PILLAR-001)]` frontmatter field to the epic and idea schemas
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
| [TASK-058](TASK-058) | Create pillar artifact schema and initial pillars | — |
| [TASK-059](TASK-059) | Register pillars in artifact config | [TASK-058](TASK-058) |
| [TASK-060](TASK-060) | Update rules to reference pillar artifacts generically | [TASK-058](TASK-058) |
| [TASK-061](TASK-061) | Add pillar reference field to epic/idea schemas | [TASK-058](TASK-058) |
| [TASK-062](TASK-062) | Update system prompt assembly to inject pillars | [TASK-058](TASK-058), [TASK-059](TASK-059) |
| [TASK-063](TASK-063) | Update product documentation (governance.md, vision.md) | [TASK-060](TASK-060) |

## Dependency Chain

```
TASK-058 (create pillar artifacts)
  ├── TASK-059 (register in config)
  │     └── TASK-062 (system prompt injection)
  ├── TASK-060 (genericise rules)
  │     └── TASK-063 (update product docs)
  └── TASK-061 (schema reference field)
```

## Implementation Design

Implementation approach to be defined during planning.
