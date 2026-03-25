---
id: TASK-30f5bdc8
type: task
title: "Update core.json schema: skill → knowledge type, SKILL → KNOW idPrefix"
description: Change the artifact type definition in core.json from 'skill' to 'knowledge' and update the idPrefix from 'SKILL' to 'KNOW'.
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - core.json artifact type entry renamed from 'skill' to 'knowledge'
  - idPrefix updated from 'SKILL' to 'KNOW'
  - orqa enforce schema passes after the change
  - No other artifact types or schema fields are affected
relationships:
  - target: EPIC-fdcdb958
    type: delivers
  - target: TASK-9021e959
    type: depended-on-by
  - target: TASK-efc1538d
    type: depended-on-by
  - target: TASK-126e853f
    type: depended-on-by
  - target: TASK-ea03dd06
    type: depended-on-by
  - target: TASK-d8d1fa14
    type: depended-on-by
  - target: TASK-efb42876
    type: depended-on-by
  - target: TASK-f9237a26
    type: depended-on-by
  - target: TASK-7df98f92
    type: depended-on-by
---

## What

Update the `core.json` schema to rename the `skill` artifact type to `knowledge` and change the `idPrefix` from `SKILL` to `KNOW`. This is the foundational schema change that all other rename tasks depend on.

## How

Locate the artifact type definition for `skill` in `core.json` and update:
- `type: "skill"` → `type: "knowledge"`
- `idPrefix: "SKILL"` → `idPrefix: "KNOW"`
- Update any `label`, `plural`, or display name fields accordingly
- Update the directory path reference if hardcoded (`skills/` → `knowledge/`)

## Verification

1. `orqa enforce schema` passes on `core.json`
2. The `knowledge` type appears in `orqa graph` type listings
3. No references to the old `skill` type remain in `core.json`
4. Other tasks (Rust types, TS types, mass rename) can proceed using this as the source of truth