---
id: EPIC-032
title: Artifact System Migration
description: "Make the artifact system self-sustaining: correct default creation, historical content linkage, and framework coverage for all 8 types."
status: in-progress
priority: P1
created: "2026-03-08"
updated: "2026-03-08"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-028
  - RES-025
docs-required:
  - RES-028
  - RES-025
docs-produced:
  - DOC-036
  - DOC-025
  - DOC-001
scoring:
  dogfood-value: 5
  foundation: 5
  user-visible: 3
  scope: 3
  dependency-risk: 2
  score: 4.1
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
- [x] Roadmap completed work section references MS-000 and EPIC-025-031
- [x] All research ↔ decision cross-references validated and fixed
- [x] MS-001 completed-epics count updated (0 → 1)

### WS-4: Migration Tracking (DONE)
- [x] This epic (EPIC-032) created to track the migration

### WS-5: Viewer Infrastructure (DEFERRED → EPIC-005)
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
- [x] Roadmap references MS-000 with epic breakdown
- [x] Migration tracked as this epic
- [ ] Viewer infrastructure built (WS-5 → EPIC-005)

## Notes

WS-1 through WS-4 are documentation/rules changes completed in a single session. WS-5 is code work deferred to EPIC-005 (Artifact Browser) which was already planned as a P1 dogfooding epic.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

Task breakdown to be defined.
