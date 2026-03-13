---
id: EPIC-033
title: Vision Alignment & Schema Simplification
description: "Align all documentation, governance rules, agent definitions, and code with the evolved vision: .orqa/ as sole source of truth, provider-agnostic AI integration, three-layer architecture (Canon/Project/Plugin), and simplified artifact schema where plans are merged into research and tasks trace cleanly to epics to milestones."
status: done
priority: P1
created: 2026-03-08
updated: 2026-03-08
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-031
  - RES-014
  - RES-011
  - RES-017
docs-required: []
docs-produced: []
scoring:
  dogfood-value: 5
  foundation: 5
  user-visible: 4
  scope: 4
  dependency-risk: 3
  score: 4.3
---## Implementation Design

### Phase A: Schema Simplification (DONE)
- Removed Plan type from artifact-framework.md
- Migrated 9 plan files to research, marked surpassed
- Updated artifact-lifecycle.md rules
- Added Research schema with `draft → complete → surpassed` workflow

### Phase B: Reference Migration (IN PROGRESS)
- Convert `plan:` field to `research-refs:` on all epics
- Update all tasks to reference `epic: [EPIC-033](EPIC-033)`
- Remove `plans` from project.json artifacts config
- Update Rust types and frontend types to remove `plan` field
- Verify every task has a valid epic, every epic has a valid milestone

### Phase C: Enforcement
- Create/update rules and skills to enforce the new structure
- Ensure no `plan:` field can be created going forward
- Verify scanning/reading code handles `research-refs:` correctly

### Phase D: Historical Backfill [TASK-011](TASK-011)
- Decision chains, surpassed artifacts, lesson history
- Reference integrity for all existing artifacts

## Tasks

| Task | Title | Status |
|------|-------|--------|
| [TASK-004](TASK-004) | Audit product docs for vision alignment | done |
| [TASK-005](TASK-005) | Audit architecture and process docs | done |
| [TASK-006](TASK-006) | Audit governance rules and agent definitions | done |
| [TASK-007](TASK-007) | Add artifacts config to project.json and Rust types | done |
| [TASK-008](TASK-008) | Update scanner to use config-driven paths | done |
| [TASK-009](TASK-009) | Frontend: config-driven navigation | done |
| [TASK-010](TASK-010) | Update task and artifact-framework schemas | done |
| [TASK-012](TASK-012) | Remove Plan type from artifact-framework.md | done |
| [TASK-013](TASK-013) | Migrate existing plans to research | done |
| [TASK-014](TASK-014) | Update artifact-lifecycle.md rules | done |
| [TASK-011](TASK-011) | Historical backfill | todo |

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
