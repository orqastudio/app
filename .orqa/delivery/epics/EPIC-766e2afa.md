---
id: "EPIC-766e2afa"
type: epic
title: "Artifact System Migration"
description: "Make the artifact system self-sustaining: correct default creation, historical content linkage, and framework coverage for all 8 types."
status: "completed"
priority: "P1"
created: "2026-03-08"
updated: "2026-03-08"
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "RES-7ad6368f"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-7ad6368f"
  - target: "RES-8354435e"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-8354435e"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-e6418f4a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-953f7ac3"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b13e6846"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-da84a27e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "AD-6ce44025"
    type: "driven-by"
  - target: "AD-0c56aa90"
    type: "driven-by"
  - target: "AD-c3700062"
    type: "driven-by"
---
## Workstreams

### WS-1: Framework & Rules (DONE)
- [x] Add Decision (AD-NNN) type to `artifact-framework.md`
- [x] Add Decision creation section to `artifact-workflow.md`
- [x] Add Decision enforcement to `artifact-lifecycle.md`
- [x] Add `.orqa/decisions/` to CLAUDE.md resources table
- [x] Update `architecture-decisions.md` to reference individual artifacts

### WS-2: Monolithic Doc Transition (DONE)
- [x] Convert `docs/architecture/decisions.md` from full content to index table
- [x] 20 individual `AD-NNN.md` artifacts created in `.orqa/decisions/`
- [x] Index links to all individual artifacts

### WS-3: Roadmap & Cross-Reference Integrity (DONE)
- [x] Roadmap completed work section references [MS-85b9269b](MS-85b9269b) and [EPIC-a8a7e205](EPIC-a8a7e205)-031
- [x] All research ↔ decision cross-references validated and fixed
- [x] [MS-654badde](MS-654badde) completed-epics count updated (0 → 1)

### WS-4: Migration Tracking (DONE)
- [x] This epic [EPIC-766e2afa](EPIC-766e2afa) created to track the migration

### WS-5: Viewer Infrastructure (DEFERRED → [EPIC-6787bb93](EPIC-6787bb93))
- [ ] Backend readers for milestones, epics, tasks, ideas, decisions
- [ ] Tauri commands for artifact scanning and reading
- [ ] Store bindings for new artifact types
- [ ] Viewer components for each type
- [ ] Sidebar navigation entries

## Acceptance Criteria

- [x] `artifact-framework.md` defines all 8 artifact types
- [x] `artifact-lifecycle.md` enforces all 8 types
- [x] `artifact-workflow.md` describes creation paths for all types
- [x] CLAUDE.md lists `.orqa/decisions/` in resources table
- [x] Monolithic `decisions.md` is an index only
- [x] All cross-references resolve (research ↔ decisions)
- [x] Roadmap references [MS-85b9269b](MS-85b9269b) with epic breakdown
- [x] Migration tracked as this epic
- [ ] Viewer infrastructure built (WS-5 → [EPIC-6787bb93](EPIC-6787bb93))

## Notes

WS-1 through WS-4 are documentation/rules changes completed in a single session. WS-5 is code work deferred to [EPIC-6787bb93](EPIC-6787bb93) (Artifact Browser) which was already planned as a P1 dogfooding epic.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

Task breakdown to be defined.