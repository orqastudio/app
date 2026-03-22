---
id: TASK-bc24af0b
title: Define delivery type schema in project.json and Rust/TS types
description: "Introduce a delivery.types array in project.json that describes the hierarchy of delivery artifact types (milestone, epic, task), including their key, label, path, and parent config. Add corresponding Rust and TypeScript types so the rest of the system can read the hierarchy from config rather than hardcoding it."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 5
  complexity: 3
  dependencies: 5
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - "project.json has a delivery.types array defining milestone, epic, task with key, label, path, parent config"
  - "Rust: DeliveryTypeConfig struct in project_settings.rs with serde deserialization"
  - "TypeScript: DeliveryTypeConfig interface in project.ts"
  - Existing artifact scanning still works (paths are already in artifacts config — delivery.types adds hierarchy metadata)
relationships:
  - target: EPIC-ed09464b
    type: delivers
  - target: TASK-fcb7ddc4
    type: depended-on-by
  - target: TASK-c2db174d
    type: depended-on-by
  - target: TASK-26acd796
    type: depended-on-by
---
## What

Add a `delivery.types` array to `project.json` that captures the hierarchy of delivery artifact types. Each entry records the type's key, human-readable label, path on disk, and which type (if any) is its parent. This gives the system a single config-driven source of truth for the milestone → epic → task relationship instead of having it hardcoded in Rust and TypeScript.

## How

1. Add a `delivery` object to `project.json` with a `types` array. Example shape:
   ```json
   "delivery": {
     "types": [
       { "key": "milestone", "label": "Milestone", "path": ".orqa/delivery/milestones", "parent": null },
       { "key": "epic",      "label": "Epic",      "path": ".orqa/delivery/epics",      "parent": "milestone" },
       { "key": "task",      "label": "Task",      "path": ".orqa/delivery/tasks",      "parent": "epic" }
     ]
   }
   ```
2. In `backend/src-tauri/src/project_settings.rs`, add:
   - `DeliveryTypeConfig` struct with fields `key: String`, `label: String`, `path: String`, `parent: Option<String>` — derive `Serialize`, `Deserialize`, `Debug`, `Clone`
   - `DeliveryConfig` struct with field `types: Vec<DeliveryTypeConfig>`
   - Add `delivery: Option<DeliveryConfig>` to the root `ProjectSettings` struct
3. In `ui/src/lib/types/project.ts`, add matching `DeliveryTypeConfig` and `DeliveryConfig` interfaces and include `delivery?: DeliveryConfig` on `ProjectSettings`.
4. Verify existing artifact scanning is unaffected: the new `delivery` key is additive and does not overlap with the existing `artifacts` array.
5. Run `make check` — zero warnings, zero type errors.

## Verification

- `project.json` parses correctly with the new `delivery.types` array present.
- Rust unit test: deserialise a fixture `project.json` containing `delivery.types` and assert all three type entries are present with correct field values.
- TypeScript type test: `DeliveryTypeConfig` compiles without `any` under strict mode.
- Existing integration tests for artifact scanning still pass.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
