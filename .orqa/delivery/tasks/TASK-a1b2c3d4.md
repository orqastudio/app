---
id: TASK-a1b2c3d4
title: "Update core.json schema: skill → knowledge type, SKILL → KNOW idPrefix"
description: Change the artifact type definition in core.json from 'skill' to 'knowledge' and update the idPrefix from 'SKILL' to 'KNOW'.
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - core.json artifact type entry renamed from 'skill' to 'knowledge'
  - idPrefix updated from 'SKILL' to 'KNOW'
  - orqa validate schema passes after the change
  - No other artifact types or schema fields are affected
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-e5f6a7b8
    type: depended-on-by
  - target: TASK-c9d0e1f2
    type: depended-on-by
  - target: TASK-3a4b5c6d
    type: depended-on-by
  - target: TASK-7e8f9a0b
    type: depended-on-by
  - target: TASK-1c2d3e4f
    type: depended-on-by
  - target: TASK-5a6b7c8d
    type: depended-on-by
  - target: TASK-9e0fa1b2
    type: depended-on-by
  - target: TASK-c5d6e7f8
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

1. `orqa validate schema` passes on `core.json`
2. The `knowledge` type appears in `orqa graph` type listings
3. No references to the old `skill` type remain in `core.json`
4. Other tasks (Rust types, TS types, mass rename) can proceed using this as the source of truth
