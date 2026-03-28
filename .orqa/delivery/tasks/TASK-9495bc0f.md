---
id: TASK-9495bc0f
type: task
name: "Rename delivery workflow skeleton to agile-methodology"
status: active
description: "Rename the delivery workflow skeleton to agile-methodology in the agile-methodology plugin"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
  - target: TASK-cd118e75
    type: depends-on
    rationale: "Plugin renames must complete before skeleton rename — agile-methodology dir must exist"
acceptance:
  - "plugins/agile-methodology/workflows/agile-methodology.workflow.yaml exists with name: agile-methodology"
  - "plugins/agile-methodology/workflows/delivery.workflow.yaml no longer exists"
  - "All contribution files reference agile-methodology instead of delivery"
  - "The workflow plugin: field is @orqastudio/plugin-agile-methodology"
  - "The skeleton retains all 6 contribution points: discovery-artifacts (optional), planning-methodology (required), documentation-standards (optional), implementation-workflow (required), review-process (required), learning-pipeline (optional)"
  - "npx tsc --noEmit passes for libs/cli"
---

## What

The delivery workflow skeleton stays in `plugins/agile-methodology/` (the workflow-definition plugin) but must be renamed from `delivery` to `agile-methodology`. This is the skeleton that defines contribution point slots that stage-definition plugins fill.

## Knowledge Needed

- Read `plugins/agile-methodology/workflows/delivery.workflow.yaml` for the current skeleton structure
- Read all `*.contribution.workflow.yaml` files across plugins for `contributes_to.workflow: delivery` references
- Read `.orqa/workflows/delivery.resolved.yaml` for the resolved output
- Read `libs/cli/src/commands/install.ts` for workflow resolution logic (install-time only)

## Agent Role

Implementer — file renames, YAML edits, cross-reference updates.

## Steps

1. Rename `plugins/agile-methodology/workflows/delivery.workflow.yaml` to `plugins/agile-methodology/workflows/agile-methodology.workflow.yaml`
2. In the renamed file, change the `name:` field from `delivery` to `agile-methodology`
3. Update the `plugin:` field to `@orqastudio/plugin-agile-methodology`
4. Verify the skeleton retains all 6 contribution points with correct required/optional flags:
   - `discovery-artifacts` (optional)
   - `planning-methodology` (required)
   - `documentation-standards` (optional)
   - `implementation-workflow` (required)
   - `review-process` (required)
   - `learning-pipeline` (optional)
5. Grep for all `contributes_to.workflow: delivery` references in contribution files across all plugins
6. Update each to `contributes_to.workflow: agile-methodology`
7. Update any resolved workflow references in `.orqa/workflows/` if they reference `delivery` by name
8. Run `npx tsc --noEmit` in `libs/cli`

## Verification

- `test -f plugins/agile-methodology/workflows/agile-methodology.workflow.yaml && echo PASS || echo FAIL`
- `test ! -f plugins/agile-methodology/workflows/delivery.workflow.yaml && echo PASS || echo FAIL`
- `grep 'name: agile-methodology' plugins/agile-methodology/workflows/agile-methodology.workflow.yaml | wc -l` should be 1
- `grep 'plugin: @orqastudio/plugin-agile-methodology' plugins/agile-methodology/workflows/agile-methodology.workflow.yaml | wc -l` should be 1
- `grep -r 'contributes_to:' plugins/ --include='*.yaml' -A1 | grep 'workflow: delivery' | wc -l` should be 0
- `cd libs/cli && npx tsc --noEmit`
