---
id: TASK-06f7a8b9
type: task
name: "License per directory and version management"
status: completed
description: "Set up per-directory LICENSE files for mixed BSL-1.1/Apache-2.0 licensing. Update orqa version sync for monorepo (single VERSION file, workspace-aware propagation)."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 1 — monorepo consolidation
  - target: TASK-02b3c4d5
    type: depends-on
    rationale: Repos must be imported first
  - target: TASK-07a8b9c0
    type: depended-on-by
acceptance:
  - "Each component directory has a LICENSE file matching its intended license"
  - "Root LICENSE is BSL-1.1 (project default)"
  - "Apache-2.0 libraries have their own LICENSE in their directory"
  - "orqa version sync works across all workspace packages"
  - "VERSION file at root remains the canonical source"
---

## Scope

### License mapping

| Component | License | Reason |
|-----------|---------|--------|
| Root, app, libs/cli | BSL-1.1 | Core product |
| libs/types, libs/sdk, libs/logger, libs/svelte-components, libs/graph-visualiser, libs/brand | Apache-2.0 | Open libraries |
| libs/search, libs/validation, libs/mcp-server, libs/lsp-server | Apache-2.0 | Open Rust crates |
| plugins/* | Apache-2.0 | Open plugins |
| connectors/claude-code | Apache-2.0 | Open connector |
| integrations/claude-agent-sdk | Apache-2.0 | Open integration |
| templates | Apache-2.0 | Open scaffolds |

### Version sync

The `VERSION` file at root already drives `orqa version sync`. In the monorepo, this propagates to all `package.json` and `Cargo.toml` files in the workspace. The `version-sync.ts` module already scans workspace packages — verify it works without submodule git operations.
