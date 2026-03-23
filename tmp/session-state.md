---
updated: 2026-03-23
scope: EPIC-d4a8c1e5 — Plugin Framework: Universal Capability Model
---

## Status: All 5 Phases COMPLETE — Pending Review

### Completed Tasks (10)

| Task | Phase | Description |
|------|-------|-------------|
| TASK-a1c3e5f7 | 1 | Three-way diff hash tracking in manifest.json |
| TASK-b2d4f6a8 | 1 | Three-way diff comparison + smart refresh |
| TASK-d4f6b8ca | 1 | Content sync fixes (.md filter, injector config, blind overwrite) |
| TASK-c3e5a7b9 | 1 | orqa plugin status command |
| TASK-f6b8d0ec | 2 | Extends strategy implementation |
| TASK-a7c9e1fd | 2 | Refactor plugins/typescript to use extends |
| TASK-b8d0e2f4 | 3 | Universal symlink + aggregation capabilities |
| TASK-c9e1f3a5 | 3 | Migrate connector to declarative capabilities |
| TASK-d0e2f4a6 | 4 | Integrations in plugin lifecycle |
| TASK-e1f3a5b7 | 5 | Template validation command |

### Files Modified

- libs/cli/src/lib/content-lifecycle.ts — three-way diff, hash tracking, extends, symlinks, aggregation
- libs/cli/src/commands/plugin.ts — status command, callers updated, aggregation wiring, template validate
- libs/cli/src/commands/install.ts — updated fallback script, LIB_ORDER deps
- libs/cli/src/lib/installer.ts — integrations/ in scan dirs
- libs/cli/src/lib/injector-config.ts — integrations/ in scan dirs
- libs/cli/src/lib/version-sync.ts — integrations/ in version sync
- libs/cli/src/lib/license.ts — integrations/ in license checks
- libs/cli/src/lib/readme.ts — integrations/ in readme checks
- libs/cli/src/commands/validate-schema.ts — integrations/ in schema validation
- libs/cli/src/commands/id.ts — integrations/ in ID scan
- libs/types/src/plugin.ts — strategy/mechanism on content mapping, symlink/aggregation types
- libs/types/src/index.ts — new type exports
- connectors/claude-code/src/connector-setup.ts — simplified, bespoke wiring removed
- connectors/claude-code/orqa-plugin.json — declarative symlinks + aggregatedFiles
- plugins/typescript/orqa-plugin.json — config section with extends presets
- integrations/claude-agent-sdk/orqa-plugin.json — build command added
- libs/cli/tsconfig.json — extends from relative path
- libs/sdk/tsconfig.json — extends from relative path
- libs/cli/package.json — removed plugin-typescript dependency
- libs/sdk/package.json — removed plugin-typescript dependency
- templates/registry.json — new template registry

### What's Next

- EPIC-d4a8c1e5 needs review before marking complete
- EPIC-f2b9e7d3 (Git Infrastructure) is unblocked once this epic is done
- All changes need to be committed
