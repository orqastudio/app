---
id: TASK-9519bf0e
type: task
title: "Create agile-review plugin"
status: active
description: "Create plugins/agile-review/ as a stage-definition plugin filling the review-process contribution point"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
  - target: TASK-cd118e75
    type: depends-on
    rationale: "software-kanban must exist (renamed from software) before moving review contribution from it"
acceptance:
  - "plugins/agile-review/ exists with valid orqa-plugin.json"
  - "plugins/agile-review/workflows/review.contribution.workflow.yaml exists with contributes_to.workflow: agile-methodology and contributes_to.point: review-process"
  - "review.contribution.workflow.yaml no longer exists in plugins/software-kanban/"
  - "npx tsc --noEmit passes for libs/cli"
---

## What

Create `plugins/agile-review/` as a new stage-definition plugin that owns the review stage. Move the review contribution workflow from `plugins/software-kanban/` (formerly `plugins/software/`) into this new plugin.

## Knowledge Needed

- Read `plugins/software-kanban/orqa-plugin.json` for manifest structure reference
- Read `plugins/software-kanban/workflows/review.contribution.workflow.yaml` for the review contribution to move
- Read `plugins/agile-planning/orqa-plugin.json` (created in TASK-3) for sibling plugin format reference
- Read any existing plugin manifest to understand the `content` mapping structure for `orqa install`

## Agent Role

Implementer — create new plugin directory, manifest, move workflow files.

## Steps

1. Create `plugins/agile-review/` directory
2. Create `plugins/agile-review/orqa-plugin.json` manifest:
   - `name`: `@orqastudio/plugin-agile-review`
   - NO `core:*` exclusive role (stage-definition plugins are not methodology-exclusive)
   - `provides.workflows` array including the contribution
   - `content` mappings for `orqa install` to copy workflow files to `.orqa/`
3. Move `plugins/software-kanban/workflows/review.contribution.workflow.yaml` to `plugins/agile-review/workflows/review.contribution.workflow.yaml`
4. Update the contribution file:
   - `contributes_to.workflow`: change from `delivery` to `agile-methodology`
   - `contributes_to.point`: `review-process`
5. Update `plugins/software-kanban/orqa-plugin.json` to remove the review contribution from its `provides` arrays and content mappings
6. Move any review-specific knowledge or rules from software-kanban that exclusively serve the review stage
7. Run `npx tsc --noEmit` in `libs/cli`

## Verification

- `test -d plugins/agile-review && echo PASS || echo FAIL`
- `test -f plugins/agile-review/orqa-plugin.json && echo PASS || echo FAIL`
- `test -f plugins/agile-review/workflows/review.contribution.workflow.yaml && echo PASS || echo FAIL`
- `grep 'agile-methodology' plugins/agile-review/workflows/review.contribution.workflow.yaml | wc -l` should be >= 1
- `grep 'review-process' plugins/agile-review/workflows/review.contribution.workflow.yaml | wc -l` should be >= 1
- `test ! -f plugins/software-kanban/workflows/review.contribution.workflow.yaml && echo PASS || echo FAIL`
- `cd libs/cli && npx tsc --noEmit`
