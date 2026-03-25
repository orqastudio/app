---
id: DOC-e42efeaf
type: doc
name: Repository and Package Naming Conventions
category: reference
status: active
relationships:
  - target: KNOW-85a449e7
    type: synchronised-with
---

# Repository and Package Naming Conventions

OrqaStudio uses consistent naming conventions across all repositories, local folders, and package names. These conventions encode what each component IS so that its purpose is clear from the name alone.

## Categories

### App

The core desktop application.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-app` | `orqastudio-app` |
| Dev folder | `app/` | `app/` |

### Libraries

Shared code packages published to GitHub Packages. Libraries are our own code — they don't bridge to external systems.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-lib-{name}` | `orqastudio-lib-types` |
| Dev folder | `libs/{name}/` | `libs/types/` |
| npm package | `@orqastudio/{name}` | `@orqastudio/types` |

### Plugins

Extend OrqaStudio by registering artifact schemas, views, widgets, relationships, CLI tools, or hooks. Plugins run INSIDE OrqaStudio.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-plugin-{name}` | `orqastudio-plugin-software` |
| Dev folder | `plugins/{name}/` | `plugins/software/` |
| npm package | `@orqastudio/plugin-{name}` | `@orqastudio/plugin-software` |
| Manifest | `orqa-plugin.json` | |

A plugin can install a connector as part of its setup. For example, the Claude plugin (`orqastudio-plugin-claude`) brings the Claude AI provider into OrqaStudio AND can install the Claude Code connector to set up the reverse connection.

### Connectors

Bridge OrqaStudio's governance into a third-party system. Connectors run INSIDE the third-party, not inside OrqaStudio.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-connector-{name}` | `orqastudio-connector-claude-code` |
| Dev folder | `connectors/{name}/` | `connectors/claude-code/` |
| npm package | `@orqastudio/connector-{name}` | `@orqastudio/connector-claude-code` |

The distinction between plugins and connectors is **direction**:
- **Plugin**: brings external capability INTO OrqaStudio (runs in the app)
- **Connector**: brings OrqaStudio governance INTO an external tool (runs in the external tool)

A connector may have its own plugin manifest for the external system. For example, the Claude Code connector has a `.claude-plugin/plugin.json` for Claude Code's plugin system, plus hooks and skills that run inside Claude Code sessions.

### Plugin + Connector Pairs

Some integrations need both directions. The Claude integration is a good example:

```
orqastudio-plugin-claude          (Plugin — runs in OrqaStudio)
├── Agent SDK sidecar              Brings Claude AI into the app
├── On install: sets up connector  Installs the Claude Code connector
└── orqa-plugin.json               Registers sidecar, hooks

orqastudio-connector-claude-code  (Connector — runs in Claude Code)
├── .claude-plugin/plugin.json     Claude Code plugin manifest
├── hooks/                         Rule engine, prompt injector
├── skills/                        Agent-facing governance skills
├── commands/                      /orqa slash command
└── Artifact bridge                Maps .claude/ ↔ .orqa/
```

The plugin is installed through OrqaStudio's plugin browser. The connector is installed into the external tool (e.g. `claude plugin add @orqastudio/connector-claude-code`). The plugin CAN automate connector installation as part of its setup flow.

### Registries

Catalogs of available plugins and connectors.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-registry-{name}` | `orqastudio-registry-official` |
| Dev folder | `registry/{name}/` | `registry/official/` |

### Templates

Scaffolds for creating new plugins and connectors.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-templates` | `orqastudio-templates` |
| Dev folder | `templates/` | `templates/` |

### Tools

Development utilities that aren't published packages.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `orqastudio-tool-{name}` | `orqastudio-tool-debug` |
| Dev folder | `tools/{name}/` | `tools/debug/` |

### Org Metadata

GitHub organisation profile, shared workflows, etc.

| | Pattern | Example |
|---|---|---|
| GitHub repo | `.github` | `.github` |
| Dev folder | `.github-org/` | `.github-org/` |

## Folder Derivation Rule

The dev environment folder is always the repo name with the category prefix stripped:

```
orqastudio-lib-types          → libs/types/
orqastudio-plugin-software    → plugins/software/
orqastudio-connector-claude-code → connectors/claude-code/
orqastudio-registry-official  → registry/official/
orqastudio-tool-debug         → tools/debug/
orqastudio-templates          → templates/
```

## npm Package Derivation Rule

```
orqastudio-lib-types          → @orqastudio/types
orqastudio-plugin-software    → @orqastudio/plugin-software
orqastudio-connector-claude-code → @orqastudio/connector-claude-code
```

Libraries drop the `lib-` infix in the package name (since `@orqastudio/types` is cleaner than `@orqastudio/lib-types`). Plugins and connectors keep their category prefix in the package name because it communicates what they are.
