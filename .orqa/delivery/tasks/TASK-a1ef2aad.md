---
id: TASK-a1ef2aad
type: task
name: "Update orqa install pipeline for monorepo"
status: completed
description: "Simplify the orqa install command for monorepo — npm install at root handles workspaces, topological build replaces LIB_ORDER, no npm link."
relationships:
  - target: EPIC-2f720d43
    type: delivers
    rationale: Phase 1 — monorepo consolidation
  - target: TASK-fe45b4f9
    type: depends-on
    rationale: npm workspaces must be set up first
  - target: TASK-70118592
    type: depends-on
    rationale: Cargo workspace must be set up first
  - target: TASK-5fdbf116
    type: depended-on-by
acceptance:
  - "orqa install works from clean clone with no npm link"
  - "LIB_ORDER removed or replaced with workspace-aware build"
  - "Cargo workspace builds via single cargo build command"
  - "Plugin content sync still works"
  - "make install succeeds on clean clone"
---

## Scope

### New install flow

```
1. npm install (at root — workspaces resolve everything)
2. Build TS packages in topological order (tsc per package)
3. cargo fetch (Cargo workspace)
4. orqa plugin refresh (content sync + aggregation)
5. Smoke test
```

### Remove from install.ts

- `LIB_ORDER` constant
- `cmdLink()` and all npm link logic
- `cmdBuildAll()` npm link chain — replace with workspace build
- The fallback `node -e` inline script in `cmdPluginSync()`

### New build approach

npm workspaces + tsc per package, OR use a build orchestrator. Simplest: keep a `BUILD_ORDER` that just runs `npx tsc` in each package directory (workspaces handle resolution, we just need to build in dependency order).
