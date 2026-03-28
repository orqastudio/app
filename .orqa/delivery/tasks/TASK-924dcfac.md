---
id: TASK-924dcfac
type: task
name: "Refactor plugins/typescript to use extends strategy"
status: archived
description: Convert plugins/typescript from being consumed as a direct npm dependency to using the extends content strategy. Remove @orqastudio/plugin-typescript from libs/cli and libs/sdk direct dependencies.
relationships:
  - target: EPIC-8b01ee51
    type: delivers
    rationale: Phase 2 — proving ground for extends strategy
  - target: TASK-ccf6efae
    type: depends-on
    rationale: Needs extends strategy implemented first
acceptance:
  - "plugins/typescript orqa-plugin.json declares config entries with strategy: extends"
  - "libs/cli does not have @orqastudio/plugin-typescript as a direct npm dependency"
  - "libs/sdk does not have @orqastudio/plugin-typescript as a direct npm dependency"
  - "tsconfig.json in consuming packages extends the plugin's base config"
  - "orqa install sets up the extends references correctly"
  - "make check passes"
---

## Scope

### Current state

- `libs/cli/package.json` has `"@orqastudio/plugin-typescript": "..."` as a dependency
- `libs/sdk/package.json` has `"@orqastudio/plugin-typescript": "..."` as a dependency
- These resolve via npm link in dev, GitHub Packages in CI

### Target state

- `plugins/typescript/orqa-plugin.json` declares extends config entries
- `orqa install` sets up `"extends": "<plugin-path>/tsconfig.base.json"` in consuming tsconfigs
- No direct npm dependency needed — the plugin install handles it

### Key files

- `plugins/typescript/orqa-plugin.json` — add config entries
- `libs/cli/package.json` — remove direct dep
- `libs/sdk/package.json` — remove direct dep
- `libs/cli/tsconfig.json` — update to extends from plugin
- `libs/sdk/tsconfig.json` — update to extends from plugin
