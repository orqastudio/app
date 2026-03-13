---
id: TASK-304
title: "Remove ArtifactType::Hook dead code"
description: "ArtifactType::Hook is dead code — hooks are plugin implementation, not governance artifacts. Remove the enum variant and all handling code (parse_artifact_type, derive_rel_path, infer_artifact_type_from_path, governance_dir)."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: []
acceptance:
  - "ArtifactType::Hook variant removed from enum"
  - "All match arms and path handling for Hook removed"
  - "make lint-backend passes with zero warnings"
  - "make test-rust passes"
---

## What

Remove the `Hook` variant from `ArtifactType` enum in `artifact.rs` and all code that handles it — `parse_artifact_type`, `derive_rel_path`, `infer_artifact_type_from_path`, `governance_dir`. Hooks live in `.orqa/plugins/` and are handled by the plugin system, not the artifact scanner.

## How

1. Remove `Hook` from `ArtifactType` enum
2. Remove Hook arms from all match statements
3. Remove "hook" from `parse_artifact_type` valid values
4. Update any tests that reference Hook
5. Run `make lint-backend && make test-rust`

## Verification

- `grep -r "ArtifactType::Hook" backend/` returns zero results
- `make check` passes
