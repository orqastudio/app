---
id: TASK-c9d0e1f2
type: task
title: "Update TypeScript types: types lib, SDK skill references"
description: "Update the @orqastudio/types library and SDK to rename all skill-related type definitions, interfaces, and constants to knowledge equivalents."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - All TypeScript interfaces/types named Skill* renamed to Knowledge*
  - All 'skill' string literals in type discriminants replaced with 'knowledge'
  - SKILL- prefix constants updated to KNOW-
  - make typecheck passes
  - No 'any' types introduced
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-a1b2c3d4
    type: depends-on
  - target: TASK-1c2d3e4f
    type: depended-on-by
  - target: TASK-9e0fa1b2
    type: depended-on-by
---

## What

Update the TypeScript types library (`libs/types`) and any SDK packages to rename skill-related types to knowledge. This ensures the frontend and any SDK consumers use the new terminology.

## How

Search `libs/types/` and `libs/` for:
- Interface/type names: `SkillArtifact`, `SkillEntry`, etc. → `KnowledgeArtifact`, `KnowledgeEntry`
- String discriminants: `type: "skill"` → `type: "knowledge"`
- Prefix constants: `"SKILL-"` → `"KNOW-"`
- Export names and barrel files

After changes:
- Rebuild the types lib: `npx tsc` in `libs/types/`
- Re-link if needed: `npm link @orqastudio/types` in `app/ui`
- Run `make typecheck` to verify no TypeScript errors

## Verification

1. `make typecheck` passes with zero errors
2. `search_regex "skill"` (case-insensitive) in `libs/types/` returns zero matches outside comments
3. Downstream consumers in `app/ui` resolve the new type names correctly
