---
id: "TASK-80c05aa7"
type: task
title: "Implement 7 high-priority integrity checks + codebase-wide clippy compliance"
description: "Implement all 7 graph-implementable integrity checks identified in TASK-8fa37f45 audit. Also fix all clippy too_many_lines violations across the entire backend codebase to achieve zero-warning clippy compliance."
status: archived
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "PlanningPlacement check: flags ideas/epics/tasks with no milestone (direct or indirect) and no horizon"
  - "DependencyViolation check: flags in-progress tasks whose depends-on items are not done"
  - "CircularDependency check: detects cycles in depends-on chains"
  - "SupersessionSymmetry check: flags decisions where supersedes/superseded-by is one-sided"
  - "MilestoneGate check: flags active milestones where P1 epics are not all done"
  - "IdeaPromotionValidity check: flags promoted ideas that were never shaped"
  - "IdeaDeliveryTracking check: flags promoted ideas whose epics are done but idea is not delivered"
  - "All new categories added to IntegrityCategory enum and TypeScript type"
  - "IntegrityWidget labels updated for all new categories"
  - "cargo clippy -- -D warnings passes with zero warnings across entire codebase"
  - "All 610 Rust tests pass"
relationships:
  - target: "EPIC-82dd0bd2"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8fa37f45"
    type: "depends-on"
---

## What

Implement 7 integrity checks identified by the TASK-8fa37f45 audit as high-impact and graph-implementable. Additionally fix all pre-existing clippy too_many_lines violations to achieve zero-warning compliance.

## How

1. Add new variants to `IntegrityCategory` and implement check functions in `artifact_graph.rs`
2. Refactor 16 functions across 8 backend files to comply with 50-line limit
3. Update TypeScript types and IntegrityWidget labels

## Verification

- `cargo clippy -- -D warnings` passes clean
- `cargo test --lib` passes all 610 tests
- Integrity scan fires correctly on known data

## Lessons

- Pre-existing clippy violations accumulate silently when not caught at introduction time. The 12 pre-existing violations blocked this commit even though they were unrelated to the integrity check work.
- The 50-line function limit is genuinely useful — every function that was refactored became more readable after extraction.
- [PD-2dc8ab96](PD-2dc8ab96) created to document exception policy for functions that can't be meaningfully decomposed.
