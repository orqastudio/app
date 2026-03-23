---
id: AD-b7e3f1a2
type: decision
title: Universal plugin capability model
status: active
description: All plugin functionality is expressed through a universal capability vocabulary declared in the manifest. No bespoke wiring code in individual plugins.
created: 2026-03-23
updated: 2026-03-23
relationships:
  - target: AD-c6abc8e6
    type: revises
    rationale: Extends the organisation-mode architecture with a concrete plugin capability model
  - target: IDEA-09979c9d
    type: informs
    rationale: orqa git depends on how plugins declare and consume content
  - target: IDEA-7c3d9f2e
    type: informs
    rationale: Forgejo hosting depends on how plugins are distributed
---

## Decision

All plugin functionality — content sync, config extension, service registration, hook execution, symlink management, cache sync, and lifecycle callbacks — is expressed through a **universal capability vocabulary** declared in `orqa-plugin.json`. The plugin framework handles all mechanics. Individual plugins contain no bespoke wiring code.

## Context

The Claude Code connector (`connectors/claude-code`) currently contains custom setup logic (`runConnectorSetup()`) that:
- Creates symlinks for `.claude/agents/`
- Aggregates MCP/LSP server declarations from all plugins
- Syncs to Claude Code's plugin cache
- Writes `.mcp.json` to project root

This logic is connector-specific but the capabilities it implements are universal — any plugin could need symlinks, service aggregation, root file management, or external cache sync. As new connectors emerge (VS Code, Cursor, Windsurf), duplicating this bespoke wiring is unsustainable.

Additionally, the plugin content model has gaps:
- No three-way diff tracking (plugin source vs installed baseline vs user-modified copy)
- No "extends" strategy for config files that support extension chains (tsconfig, eslint)
- Sidecar integrations are outside the plugin lifecycle (`plugin list`, `plugin refresh` don't cover them)
- Plugin templates must track schema changes but have no mechanism to do so

## Capability Vocabulary

| Capability | Manifest Key | What It Does |
|-----------|-------------|-------------|
| **Artifact content** | `content` | Copy files to `.orqa/`, three-way diff tracking |
| **Extendable config** | `config` | Set up extends references, base stays in plugin |
| **Service providers** | `provides.mcpServers`, `provides.lspServers`, `provides.sidecar` | Register services; framework aggregates across plugins |
| **Symlink declarations** | `provides.symlinks` | Declare source→target symlinks; framework creates/maintains them |
| **Service aggregation** | `provides.aggregates` | Collect cross-plugin service declarations into merged config files |
| **Root file management** | `provides.rootFiles` | Declare files that must exist at project root |
| **Event hooks** | `hooks` | Scripts triggered on events (PreToolUse, SessionStart, etc.) |
| **Lifecycle callbacks** | `lifecycle` | Plugin lifecycle: onInstall, onRefresh, onEnable, onDisable |
| **Schemas** | `provides.schemas` | Artifact type definitions, frontmatter validation, status transitions |
| **Relationships** | `provides.relationships` | Typed edges between artifact types |
| **Views** | `provides.views` | UI components loaded as ES modules |
| **Widgets** | `provides.widgets` | Dashboard widget declarations |
| **CLI tools** | `provides.cliTools` | CLI command declarations |
| **Navigation** | `provides.defaultNavigation` | Sidebar navigation structure |
| **Templates** | `provides.templates` | Scaffold templates for `orqa plugin create` |

## Content Sync Model

### Pattern 1: Copy + Three-Way Diff (artifacts, non-extendable files)

Plugin is the upstream source. `orqa install` copies to managed location and records the baseline hash. On subsequent install/refresh:
- **Plugin source hash** compared to baseline → plugin updated?
- **Baseline hash** compared to current file on disk → user modified?
- Both changed → conflict surfaced to user (merge, keep theirs, take plugin's)

### Pattern 2: Extends (config files with extension infrastructure)

Plugin install sets up an extends reference in the project's config. Base config stays in the plugin source. Project config extends it with local overrides. Plugin updates propagate automatically through the extends chain.

Declared per content entry:
```json
"config": {
  "tsconfig": { "source": "tsconfig.base.json", "target": "tsconfig.json", "strategy": "extends" },
  "eslint": { "source": "eslint.base.config.js", "target": "eslint.config.js", "strategy": "extends" }
}
```

## Alternatives Considered

### Keep connector-specific wiring (REJECTED)

Each connector maintains its own setup logic. Rejected because:
- Duplicates mechanics across connectors
- Each new connector reinvents symlink management, service aggregation, cache sync
- No consistency in how capabilities are declared or discovered

### Framework handles everything, no lifecycle hooks (REJECTED)

All setup is purely declarative with zero escape hatches. Rejected because:
- Some plugins will need custom setup logic that cannot be anticipated
- Lifecycle callbacks (`onInstall`, `onRefresh`) provide the escape hatch while keeping the default path declarative

## Consequences

### Positive
- New connectors (VS Code, Cursor) are thin manifests, not codebases
- Any plugin can aggregate services, manage symlinks, sync caches
- Three-way diff gives users visibility into both plugin updates and their own local edits
- Integrations (sidecars) become first-class in the plugin lifecycle
- Plugin templates can declare schema tracking dependencies

### Negative
- Migration effort: connector's `runConnectorSetup()` must be decomposed into capabilities
- The `orqa-plugin.json` manifest grows in complexity
- Framework code must handle all capability mechanics (more framework code)

### Constraints
- All plugin interaction goes through the manifest — no side-channel wiring
- The `orqa install` / `orqa plugin refresh` pipeline must handle all capability types
- Baseline hash tracking requires a lockfile or manifest extension
