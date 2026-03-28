---
id: TASK-cd118e75
type: task
name: "Rename plugins"
status: active
description: "Rename agile-governance to agile-methodology and software to software-kanban, updating all cross-references"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
acceptance:
  - "plugins/software-kanban/ exists with manifest name @orqastudio/plugin-software-kanban"
  - "plugins/agile-methodology/ exists with manifest name @orqastudio/plugin-agile-methodology and role core:workflow"
  - "plugins/software/ and plugins/agile-governance/ no longer exist"
  - "grep for old plugin names returns 0 matches across plugins/ libs/ connectors/ .orqa/"
  - "npx tsc --noEmit passes for libs/cli"
  - "cargo clippy -- -D warnings passes"
---

## What

Two plugin renames to align with the architecture defined in RES-d6e8ab11:

1. `plugins/agile-methodology/` (DONE): The workflow-definition plugin. Owns the agile-methodology skeleton with contribution point slots.
   - `name`: `@orqastudio/plugin-agile-methodology`
   - `role`: `core:workflow` (owns the agile-methodology skeleton)

2. `plugins/software-kanban/` (DONE): The software plugin provides implementation-workflow artifacts (epic, task, milestone).
   - `name`: `@orqastudio/plugin-software-kanban`
   - `core:delivery` role removed (stage-definition plugins are not methodology-exclusive)

## Knowledge Needed

- Read `plugins/agile-methodology/orqa-plugin.json` for current manifest structure
- Read `plugins/software-kanban/orqa-plugin.json` for current manifest structure
- Read `libs/cli/src/commands/install.ts` for BUILD_ORDER or plugin name references
- Read `.orqa/manifest.json` for installed plugin references
- Read `.orqa/prompt-registry.json` for prompt-registry plugin name entries
- Grep across `plugins/`, `libs/`, `connectors/`, `.orqa/` for old plugin names

## Agent Role

Implementer — this involves file moves, manifest edits, and cross-reference updates across the codebase.

## Steps

1. Rename plugin directories (DONE)
2. Update plugin manifests (DONE)
3. Grep for all references to old names across the entire repo (DONE)
4. Update every match (DONE)
5. Update every match: manifests, workflow files, knowledge declarations, prompt-registry, content mappings, BUILD_ORDER in install.ts
6. Run `npx tsc --noEmit` in `libs/cli` to verify TypeScript compilation
7. Run `cargo clippy -- -D warnings` to verify Rust compilation

## Verification

- `test -d plugins/software-kanban && echo PASS || echo FAIL`
- `test -d plugins/agile-methodology && echo PASS || echo FAIL`
- `test ! -d plugins/software && echo PASS || echo FAIL`
- `test ! -d plugins/agile-governance && echo PASS || echo FAIL`
- grep for old plugin names across plugins/ libs/ connectors/ .orqa/ | wc -l should be 0
- `cd libs/cli && npx tsc --noEmit`
- `cargo clippy -- -D warnings`
