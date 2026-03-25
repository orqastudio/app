---
id: TASK-cd118e75
type: task
name: "Rename plugins"
status: active
description: "Rename agile-governance to agile-workflow and software to software-kanban, updating all cross-references"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 1 — Plugin Decomposition"
acceptance:
  - "plugins/software-kanban/ exists with manifest name @orqastudio/plugin-software-kanban"
  - "plugins/agile-workflow/ exists with manifest name @orqastudio/plugin-agile-workflow and role core:workflow"
  - "plugins/software/ and plugins/agile-governance/ no longer exist"
  - "grep -r '@orqastudio/plugin-software-project\\|@orqastudio/plugin-agile-governance' plugins/ libs/ connectors/ .orqa/ returns 0 matches"
  - "npx tsc --noEmit passes for libs/cli"
  - "cargo clippy -- -D warnings passes"
---

## What

Two plugin renames to align with the architecture defined in RES-d6e8ab11:

1. `plugins/agile-governance/` -> `plugins/agile-workflow/`: The agile-governance plugin is the workflow-definition plugin. It owns the agile-methodology skeleton with contribution point slots. Rename the directory and update the manifest:
   - `name`: `@orqastudio/plugin-agile-governance` -> `@orqastudio/plugin-agile-workflow`
   - `role`: set to `core:workflow` (owns the agile-methodology skeleton)

2. `plugins/software/` -> `plugins/software-kanban/`: The software plugin provides implementation-workflow artifacts (epic, task, milestone). Rename the directory and update the manifest:
   - `name`: `@orqastudio/plugin-software-project` -> `@orqastudio/plugin-software-kanban`
   - Remove `core:delivery` role (stage-definition plugins are not methodology-exclusive)

## Knowledge Needed

- Read `plugins/agile-governance/orqa-plugin.json` for current manifest structure
- Read `plugins/software/orqa-plugin.json` for current manifest structure
- Read `libs/cli/src/commands/install.ts` for BUILD_ORDER or plugin name references
- Read `.orqa/manifest.json` for installed plugin references
- Read `.orqa/prompt-registry.json` for prompt-registry plugin name entries
- Grep across `plugins/`, `libs/`, `connectors/`, `.orqa/` for old plugin names

## Agent Role

Implementer — this involves file moves, manifest edits, and cross-reference updates across the codebase.

## Steps

1. Rename `plugins/agile-governance/` directory to `plugins/agile-workflow/`
2. Update `plugins/agile-workflow/orqa-plugin.json`: change `name` to `@orqastudio/plugin-agile-workflow`, set `role` to `core:workflow`
3. Rename `plugins/software/` directory to `plugins/software-kanban/`
4. Update `plugins/software-kanban/orqa-plugin.json`: change `name` to `@orqastudio/plugin-software-kanban`, remove `core:delivery` role
5. Grep for all references to old names (`@orqastudio/plugin-software-project`, `@orqastudio/plugin-agile-governance`, `plugins/software/`, `plugins/agile-governance/`) across the entire repo
6. Update every match: manifests, workflow files, knowledge declarations, prompt-registry, content mappings, BUILD_ORDER in install.ts
7. Run `npx tsc --noEmit` in `libs/cli` to verify TypeScript compilation
8. Run `cargo clippy -- -D warnings` to verify Rust compilation

## Verification

- `test -d plugins/software-kanban && echo PASS || echo FAIL`
- `test -d plugins/agile-workflow && echo PASS || echo FAIL`
- `test ! -d plugins/software && echo PASS || echo FAIL`
- `test ! -d plugins/agile-governance && echo PASS || echo FAIL`
- `grep -r '@orqastudio/plugin-software-project\|@orqastudio/plugin-agile-governance' plugins/ libs/ connectors/ .orqa/ | wc -l` should be 0
- `cd libs/cli && npx tsc --noEmit`
- `cargo clippy -- -D warnings`
