---
id: TASK-fe45b4f9
type: task
name: "Set up npm workspaces"
status: completed
description: Replace the entire npm link chain with npm workspaces. Root package.json declares all TS packages as workspace members. Internal @orqastudio/* deps resolve via workspace protocol.
relationships:
  - target: EPIC-2f720d43
    type: delivers
    rationale: Phase 1 — monorepo consolidation
  - target: TASK-7b011351
    type: depends-on
    rationale: Repos must be imported first
  - target: TASK-a1ef2aad
    type: depended-on-by
acceptance:
  - "Root package.json has workspaces field listing all TS packages"
  - "npm install at root resolves all @orqastudio/* packages locally"
  - "No npm link commands needed"
  - "All TS packages build successfully"
  - "orqa install simplified — no LIB_ORDER npm link chain"
---

## Scope

### Root package.json workspaces

```json
{
  "name": "orqastudio-monorepo",
  "private": true,
  "workspaces": [
    "libs/types",
    "libs/logger",
    "libs/cli",
    "libs/sdk",
    "libs/svelte-components",
    "libs/graph-visualiser",
    "libs/brand",
    "plugins/typescript",
    "plugins/software",
    "plugins/cli",
    "connectors/claude-code",
    "integrations/claude-agent-sdk",
    "app/ui"
  ]
}
```

### Update internal deps

Each package.json that references `@orqastudio/*` packages should use workspace protocol:
```json
"@orqastudio/types": "workspace:*"
```

Or just `"*"` — npm workspaces resolve local packages automatically by name.

### Update orqa install

Remove the LIB_ORDER npm link chain from `libs/cli/src/commands/install.ts`. Replace with:
1. `npm install` at repo root (resolves all workspaces)
2. Build each package in dependency order (or use a topological build)
3. Plugin content sync (already framework-managed)
