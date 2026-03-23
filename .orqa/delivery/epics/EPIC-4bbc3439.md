---
id: "EPIC-4bbc3439"
type: epic
title: "Vision Alignment & Schema Simplification"
description: "Align all documentation, governance rules, agent definitions, and code with the evolved vision: .orqa/ as sole source of truth, provider-agnostic AI integration, three-layer architecture (Canon/Project/Plugin), and simplified artifact schema where plans are merged into research and tasks trace cleanly to epics to milestones."
status: "completed"
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
  - target: "RES-c7520d05"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-c7520d05"
  - target: "RES-1c148040"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-1c148040"
  - target: "RES-4057ea03"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-4057ea03"
  - target: "RES-0749aff0"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-0749aff0"
  - target: "TASK-5e893805"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-bb5a702e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ac590adf"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-abf6025d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-dd1484aa"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-614a6a45"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f2872193"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-24c90717"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-1648c4f2"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-88ccf38a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: MS-654badde
    type: fulfils
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
- Update all tasks to reference `epic: [EPIC-4bbc3439](EPIC-4bbc3439)`
- Remove `plans` from project.json artifacts config
- Update Rust types and frontend types to remove `plan` field
- Verify every task has a valid epic, every epic has a valid milestone

### Phase C: Enforcement
- Create/update rules and skills to enforce the new structure
- Ensure no `plan:` field can be created going forward
- Verify scanning/reading code handles `research-refs:` correctly

### Phase D: Historical Backfill [TASK-24c90717](TASK-24c90717)
- Decision chains, surpassed artifacts, lesson history
- Reference integrity for all existing artifacts

## Tasks

| Task | Title | Status |
|------|-------|--------|
| [TASK-5e893805](TASK-5e893805) | Audit product docs for vision alignment | done |
| [TASK-bb5a702e](TASK-bb5a702e) | Audit architecture and process docs | done |
| [TASK-ac590adf](TASK-ac590adf) | Audit governance rules and agent definitions | done |
| [TASK-abf6025d](TASK-abf6025d) | Add artifacts config to project.json and Rust types | done |
| [TASK-dd1484aa](TASK-dd1484aa) | Update scanner to use config-driven paths | done |
| [TASK-614a6a45](TASK-614a6a45) | Frontend: config-driven navigation | done |
| [TASK-f2872193](TASK-f2872193) | Update task and artifact-framework schemas | done |
| [TASK-1648c4f2](TASK-1648c4f2) | Remove Plan type from artifact-framework.md | done |
| [TASK-e1b911d8](TASK-e1b911d8) | Migrate existing plans to research | done |
| [TASK-9f54c3bb](TASK-9f54c3bb) | Update artifact-lifecycle.md rules | done |
| [TASK-24c90717](TASK-24c90717) | Historical backfill | todo |

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