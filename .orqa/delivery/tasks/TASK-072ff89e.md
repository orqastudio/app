---
id: TASK-072ff89e
type: task
title: "Create agile-planning plugin"
status: active
description: "Create plugins/agile-planning/ as a stage-definition plugin filling the planning-methodology contribution point"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
  - target: TASK-cd118e75
    type: depends-on
    rationale: "software-kanban must exist (renamed from software) before moving planning contribution from it"
acceptance:
  - "plugins/agile-planning/ exists with valid orqa-plugin.json"
  - "plugins/agile-planning/workflows/planning.contribution.workflow.yaml exists with contributes_to.workflow: agile-methodology and contributes_to.point: planning-methodology"
  - "planning.contribution.workflow.yaml no longer exists in plugins/software-kanban/"
  - "npx tsc --noEmit passes for libs/cli"
---

## What

Create `plugins/agile-planning/` as a new stage-definition plugin that owns the planning stage. Move the planning contribution workflow from `plugins/software-kanban/` (formerly `plugins/software/`) into this new plugin.

## Knowledge Needed

- Read `plugins/software-kanban/orqa-plugin.json` for manifest structure reference
- Read `plugins/software-kanban/workflows/planning.contribution.workflow.yaml` for the planning contribution to move
- Read `plugins/core/orqa-plugin.json` for manifest format reference (another stage-agnostic plugin)
- Read any existing plugin manifest to understand the `content` mapping structure for `orqa install`

## Agent Role

Implementer — create new plugin directory, manifest, move workflow files.

## Steps

1. Create `plugins/agile-planning/` directory
2. Create `plugins/agile-planning/orqa-plugin.json` manifest:
   - `name`: `@orqastudio/plugin-agile-planning`
   - NO `core:*` exclusive role (stage-definition plugins are not methodology-exclusive)
   - `provides.workflows` array including the contribution
   - `content` mappings for `orqa install` to copy workflow files to `.orqa/`
3. Move `plugins/software-kanban/workflows/planning.contribution.workflow.yaml` to `plugins/agile-planning/workflows/planning.contribution.workflow.yaml`
4. Update the contribution file:
   - `contributes_to.workflow`: change from `delivery` to `agile-methodology`
   - `contributes_to.point`: `planning-methodology`
5. Update `plugins/software-kanban/orqa-plugin.json` to remove the planning contribution from its `provides` arrays and content mappings
6. Move any planning-specific knowledge or rules from software-kanban that exclusively serve the planning stage
7. Run `npx tsc --noEmit` in `libs/cli`

## Verification

- `test -d plugins/agile-planning && echo PASS || echo FAIL`
- `test -f plugins/agile-planning/orqa-plugin.json && echo PASS || echo FAIL`
- `test -f plugins/agile-planning/workflows/planning.contribution.workflow.yaml && echo PASS || echo FAIL`
- `grep 'agile-methodology' plugins/agile-planning/workflows/planning.contribution.workflow.yaml | wc -l` should be >= 1
- `grep 'planning-methodology' plugins/agile-planning/workflows/planning.contribution.workflow.yaml | wc -l` should be >= 1
- `test ! -f plugins/software-kanban/workflows/planning.contribution.workflow.yaml && echo PASS || echo FAIL`
- `cd libs/cli && npx tsc --noEmit`
