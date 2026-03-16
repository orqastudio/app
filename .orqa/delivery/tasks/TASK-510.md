---
id: TASK-510
title: Replace hardcoded parent-child field references in integrity checks
description: "Refactor the integrity check functions so they read the delivery type hierarchy from the project config instead of using hardcoded artifact type names such as 'epic' and 'milestone'. The checks must produce identical results for the current project while being driven entirely by config."
status: completed
priority: P1
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - check_parent_child_consistency reads hierarchy from delivery type config, not hardcoded "epic"/"milestone"
  - check_milestone_gate reads the gate type from config
  - No hardcoded artifact type names in integrity check functions
  - All existing integrity checks still produce the same results
relationships:
  - target: EPIC-078
    type: delivers
  - target: TASK-509
    type: depends-on
  - target: TASK-512
    type: depended-on-by
---
## What

The integrity check functions currently contain hardcoded strings like `"epic"` and `"milestone"` when resolving parent-child relationships and gate logic. This task replaces those hardcoded references with config-driven lookups so the same code works for any delivery type hierarchy defined in `project.json`.

## How

1. Thread `&[DeliveryTypeConfig]` (from `ProjectSettings.delivery.types`) into the integrity check module — either as a parameter to each check function or as part of a context struct passed to the checker.
2. In `check_parent_child_consistency`:
   - Replace any hardcoded `"epic"` / `"milestone"` / `"task"` strings with a lookup against the `DeliveryTypeConfig` slice.
   - Use `config.iter().find(|t| t.key == child_type).and_then(|t| t.parent.as_deref())` to resolve the expected parent type for each artifact.
3. In `check_milestone_gate`:
   - Locate the root type (the one with `parent: null`) from config instead of assuming it is `"milestone"`.
   - Locate the second-level type (the one whose `parent` equals the root key) instead of assuming it is `"epic"`.
4. Remove all remaining hardcoded artifact type name literals from integrity check functions.
5. Add a unit test that constructs a two-level hierarchy config (`"sprint"` → `"story"`) and verifies the checks run without compilation errors or panics against a minimal fixture dataset.
6. Run `make check` — zero warnings, zero type errors.

## Verification

- Grep for literal strings `"epic"`, `"milestone"`, `"task"` in the integrity check module — none should remain outside comments or test fixtures.
- Existing integrity check integration tests pass with no changes to expected output.
- New unit test with a non-standard hierarchy config (`sprint`/`story`) passes.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
