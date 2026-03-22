---
id: TASK-8024efcb
title: Add status validation to artifact graph integrity checks
description: "Extend the artifact graph integrity scan to validate every artifact's status field against the valid enum values defined in project.json. Invalid statuses are reported as integrity errors with the artifact ID and the offending value. An auto-fix suggestion derives the closest valid status from a migration map."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - "Artifact graph integrity scan checks every artifact's status against the valid enum from project.json statuses config"
  - Invalid statuses reported as integrity errors with the artifact ID and current invalid status
  - "Auto-fix available: suggest the closest valid status based on the migration map"
  - Results surfaced in IntegrityWidget
relationships:
  - target: EPIC-9fbc17c0
    type: delivers
  - target: TASK-a2384d29
    type: depended-on-by
  - target: TASK-6aba71e3
    type: depended-on-by
---

## What

Extend the artifact graph integrity scan to include status validation. Every artifact has a `status` field; the valid values for each artifact type are defined in `project.json` (or the per-directory `schema.json`). When a scan finds a status value that is not in the valid enum, it is recorded as an integrity error. A migration map provides auto-fix suggestions — for each invalid value, the map declares the canonical replacement so the user can apply it in one click from IntegrityWidget.

## How

1. Read the valid status enums from `project.json` (or `schema.json` per artifact type) in the integrity scanner.
2. For each artifact loaded into the graph, compare `status` against the valid set for that type.
3. If invalid, emit an `IntegrityError` entry with:
   - `artifactId`: the artifact's ID
   - `field`: `"status"`
   - `actual`: the current invalid value
   - `suggestion`: the closest valid value from the migration map (or `null` if no mapping exists)
4. Add the migration map as a small lookup table in the scanner module (e.g., `"in-progress" → "active"`, `"done" → "completed"`).
5. Surface the new error category in `IntegrityWidget` alongside existing integrity errors — reuse the existing error row component.
6. The auto-fix action calls the existing artifact-update IPC command with the suggested status value.

## Verification

- Unit test: artifact with status `"in-progress"` (invalid) is returned as an integrity error with suggestion `"active"`.
- Unit test: artifact with status `"active"` (valid) produces no integrity error.
- Unit test: migration map returns `null` suggestion for a completely unknown status string.
- IntegrityWidget renders the new status error category with auto-fix button when errors exist.
- IntegrityWidget shows no status errors when all artifacts have valid statuses.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
