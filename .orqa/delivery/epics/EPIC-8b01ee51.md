---
id: EPIC-8b01ee51
type: epic
name: "Plugin Framework: Universal Capability Model"
status: active
description: Implement the universal plugin capability model (AD-9ab3e0a4) — three-way diff tracking, extends strategy for config plugins, universal capabilities replacing connector-specific wiring, integration lifecycle, and template schema tracking.
priority: P1
relationships:
  - target: AD-9ab3e0a4
    type: implements
    rationale: Implements the universal plugin capability model decision
  - target: MS-b1ac0a20
    type: fulfils
    rationale: Plugin framework is foundational to dogfooding — the dev environment runs on plugins
  - target: TASK-d39e416a
    type: delivered-by
  - target: TASK-a3774911
    type: delivered-by
  - target: TASK-a282af1c
    type: delivered-by
  - target: TASK-bbda414d
    type: delivered-by
  - target: TASK-9f4a331e
    type: delivered-by
  - target: TASK-2d03d1a3
    type: delivered-by
  - target: RES-999def94
    type: guided-by
  - target: TASK-cfcd78e4
    type: delivered-by
  - target: TASK-64b12b40
    type: delivered-by
  - target: TASK-924dcfac
    type: delivered-by
  - target: TASK-009aa585
    type: delivered-by
  - target: EPIC-2f720d43
    type: depended-on-by
  - target: TASK-ccf6efae
    type: delivered-by
---

## Problem

The plugin framework has gaps that must be resolved before the git infrastructure overhaul:

1. **No three-way diff tracking** — `orqa install` copies files but doesn't record baseline hashes. Can't distinguish "plugin updated" from "user edited their copy."
2. **No extends strategy** — config plugins (`plugins/typescript`) are consumed as direct npm dependencies rather than through the plugin content model.
3. **Connector-specific wiring** — `runConnectorSetup()` contains bespoke logic (symlinks, MCP aggregation, cache sync) that should be universal plugin capabilities.
4. **Integrations outside lifecycle** — sidecar integrations aren't covered by `plugin list`, `plugin refresh`, or the install pipeline.
5. **Template schema tracking** — no mechanism to ensure plugin templates stay in sync with schema changes.

## Phases

### Phase 1: Three-Way Diff Infrastructure

Add baseline hash tracking to the existing copy-based content sync.

- Extend `.orqa/manifest.json` entries to include `sourceHash` and `installedHash` per file
- `orqa install` / `orqa plugin refresh` records hashes when copying
- `orqa plugin diff` compares all three states (plugin source, baseline, project copy)
- `orqa plugin status` reports: clean, plugin-updated, locally-modified, conflict
- Existing artifact content sync continues working — this is additive

### Phase 2: Config Extends Strategy

Add a new content strategy for config files that support extension chains.

- Add `strategy` field to content entries: `"copy"` (default, existing) or `"extends"`
- `orqa install` for extends strategy: writes/updates the extends reference in the target file
- Path resolution works in both dev environment and installed project
- Refactor `plugins/typescript` to use extends strategy for tsconfig and eslint configs
- Remove direct npm dependency on `@orqastudio/plugin-typescript` from `libs/cli` and `libs/sdk`

### Phase 3: Universal Plugin Capabilities

Decompose connector-specific wiring into framework capabilities.

- **Symlink declarations** — plugins declare source→target symlinks in manifest; framework creates/maintains them
- **Service aggregation** — framework collects `provides.mcpServers` and `provides.lspServers` from all plugins, writes merged config
- **Root file management** — plugins declare files needed at project root; framework manages them
- **External cache sync** — plugins declare external tool cache locations; framework syncs on refresh
- **Lifecycle callbacks** — `onInstall`, `onRefresh`, `onEnable`, `onDisable` replace `runConnectorSetup()`
- Migrate `connectors/claude-code` from `runConnectorSetup()` to declarative capabilities
- Verify all existing connector functionality works through the new model

### Phase 4: Integration Lifecycle

Bring sidecar integrations into the plugin lifecycle.

- `orqa plugin list` shows integrations
- `orqa plugin refresh` rebuilds integrations (calls their build command)
- `orqa plugin status` reports integration health (sidecar process running, version match)
- Add `integrations/claude-agent-sdk` to the install pipeline
- Manifest `provides.sidecar` drives lifecycle management

### Phase 5: Template Schema Tracking

Ensure plugin templates stay in sync with schema changes.

- Templates declare which schemas they depend on
- `orqa plugin create` validates template against current schemas
- Schema changes trigger a check: "these templates need updating"
- `orqa template validate` verifies all templates produce valid artifacts against current schemas
- Add template validation to CI

## Tasks

| Task | Phase | Status |
|------|-------|--------|
| Design manifest schema extensions for capabilities | 1-3 | todo |
| Implement three-way diff hash tracking in manifest.json | 1 | todo |
| Update `orqa plugin diff` to use three-way comparison | 1 | todo |
| Add `orqa plugin status` command | 1 | todo |
| Implement extends strategy in content sync | 2 | todo |
| Refactor plugins/typescript to use extends | 2 | todo |
| Implement symlink capability in framework | 3 | todo |
| Implement service aggregation in framework | 3 | todo |
| Implement root file management capability | 3 | todo |
| Implement lifecycle callbacks | 3 | todo |
| Migrate connector to declarative capabilities | 3 | todo |
| Add integrations to plugin lifecycle commands | 4 | todo |
| Add integration build to install pipeline | 4 | todo |
| Template schema dependency tracking | 5 | todo |
| `orqa template validate` command | 5 | todo |

## Out of Scope

(Requires user approval to exclude anything)

## Acceptance Criteria

- [ ] `orqa plugin status` shows three-way diff state for all installed plugin content
- [ ] `plugins/typescript` config consumed via extends, not direct npm dependency
- [ ] `connectors/claude-code` has zero bespoke setup logic — all through manifest capabilities
- [ ] `orqa plugin list` and `orqa plugin refresh` cover integrations
- [ ] `orqa template validate` passes for all templates against current schemas
- [ ] Existing `orqa install` workflow continues to work throughout migration
