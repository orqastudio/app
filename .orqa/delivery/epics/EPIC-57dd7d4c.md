---
id: "EPIC-57dd7d4c"
type: "epic"
title: "Vision Alignment & Schema Simplification"
description: "Align all documentation, governance rules, agent definitions, and code with the evolved vision: .orqa/ as sole source of truth, provider-agnostic AI integration, three-layer architecture (Canon/Project/Plugin), and simplified artifact schema where plans are merged into research and tasks trace cleanly to epics to milestones."
status: archived
priority: "P1"
created: "2026-03-08"
updated: "2026-03-08"
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Implementation Design

### Phase A: Schema Simplification (DONE)

- Removed Plan type from artifact-framework.md
- Migrated 9 plan files to research, marked surpassed
- Updated artifact-lifecycle.md rules
- Added Research schema with `draft → complete → surpassed` workflow

### Phase B: Reference Migration (IN PROGRESS)

- Convert `plan:` field to `research-refs:` on all epics
- Update all tasks to reference `epic: [EPIC-57dd7d4c](EPIC-57dd7d4c)`
- Remove `plans` from project.json artifacts config
- Update Rust types and frontend types to remove `plan` field
- Verify every task has a valid epic, every epic has a valid milestone

### Phase C: Enforcement

- Create/update rules and skills to enforce the new structure
- Ensure no `plan:` field can be created going forward
- Verify scanning/reading code handles `research-refs:` correctly

### Phase D: Historical Backfill [TASK-bf4b1013](TASK-bf4b1013)

- Decision chains, surpassed artifacts, lesson history
- Reference integrity for all existing artifacts

## Tasks

| Task | Title | Status |
| ------ | ------- | -------- |
| [TASK-c79077be](TASK-c79077be) | Audit product docs for vision alignment | done |
| [TASK-43c190d2](TASK-43c190d2) | Audit architecture and process docs | done |
| [TASK-8db2e1c3](TASK-8db2e1c3) | Audit governance rules and agent definitions | done |
| [TASK-25e35dfc](TASK-25e35dfc) | Add artifacts config to project.json and Rust types | done |
| [TASK-601a75ca](TASK-601a75ca) | Update scanner to use config-driven paths | done |
| [TASK-0c48a446](TASK-0c48a446) | Frontend: config-driven navigation | done |
| [TASK-36a4b6c8](TASK-36a4b6c8) | Update task and artifact-framework schemas | done |
| [TASK-edeea471](TASK-edeea471) | Remove Plan type from artifact-framework.md | done |
| [TASK-e3c4da9f](TASK-e3c4da9f) | Migrate existing plans to research | done |
| [TASK-252828c9](TASK-252828c9) | Update artifact-lifecycle.md rules | done |
| [TASK-bf4b1013](TASK-bf4b1013) | Historical backfill | todo |

## Acceptance Criteria

- No `plan:` field in any artifact frontmatter (replaced by `research-refs:` on epics, `epic:` on tasks)
- No Plan type in artifact-framework.md or artifact-lifecycle.md
- Every task has an `epic:` field referencing an existing epic
- Every epic has a `milestone:` field referencing an existing milestone
- Rust types and frontend types have no `plan` field
- `research-refs:` field documented and in use
- All audit results recorded as research documents

## Context

This epic addresses a need identified during project development.
