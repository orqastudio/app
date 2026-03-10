---
id: TASK-007
title: Add artifacts config to project.json and Rust types
description: Define the artifacts config schema in project.json and add corresponding Rust types. This is the foundation — the scanner and frontend will read from this config.
status: done
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-033
assignee: AGENT-002
skills:
  - SKILL-003
  - SKILL-012
  - SKILL-009
  - SKILL-027
scope:
  - .orqa/project.json
  - src-tauri/src/domain/project_settings.rs
acceptance:
  - project.json has artifacts array with all current groups and types
  - ArtifactEntry enum (Group | Type) added to project_settings.rs
  - ArtifactTypeConfig struct with key
  - label
  - icon
  - path fields
  - artifacts field on project config struct with
  - Group variant listed before Type in untagged enum (serde ordering)
  - cargo build passes
  - cargo clippy -- -D warnings passes
---
## What

1. Add `artifacts` array to `.orqa/project.json` with all current groups/types
2. Add Rust types to `project_settings.rs`:
   - `ArtifactTypeConfig { key, label, icon: Option, path }`
   - `ArtifactEntry` untagged enum: `Group { key, label, icon, children }` | `Type(ArtifactTypeConfig)`
3. Add `artifacts: Vec<ArtifactEntry>` to the project config struct

## Config Values

See plan for the full artifacts array. Groups: docs, planning, team, governance.
Types within each group per current `.orqa/` directory structure.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
