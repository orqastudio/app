---
id: KNOW-c9d3a7f4
type: knowledge
status: active
title: "Plugin Manifest Format and Category Constraints"
description: "orqa-plugin.json structure, category declarations, required config blocks per category, and enforcement plugin manifest contracts"
tier: on-demand
created: 2026-03-29
roles: [implementer, reviewer, planner]
paths: [plugins/, engine/plugin/]
tags: [architecture, plugins, manifest, enforcement, categories]
relationships:
  - type: synchronised-with
    target: DOC-41ccf7c4
---

# Plugin Manifest Format and Category Constraints

## `orqa-plugin.json` Top-Level Structure

```json
{
  "name": "@orqastudio/plugin-<name>",
  "description": "...",
  "version": "...",
  "categories": ["<category>", ...],
  "dependencies": ["@orqastudio/plugin-<name>"],
  "provides": { ... }
}
```

## Valid Category Values

`categories` is a plural array — a plugin participates in every category it declares. Valid values:

- `methodology` — defines overarching workflow skeleton with stage slots
- `workflow` — defines complete sub-workflow for one methodology stage
- `domain-knowledge` — provides domain expertise (KNOW/DOC artifacts)
- `enforcement-generator` — owns an enforcement engine and its config output
- `enforcement-contributor` — provides rule files for a generator to consume
- `connector` — generates a tool-native plugin and watches for changes

## Category-to-Config-Block Constraints (Schema-Validated)

The JSON schema enforces structural consistency: declaring a category requires the matching config block. These are validated by `orqa install` BEFORE any installation action.

| Category declared | Required block |
| ----------------- | -------------- |
| `"enforcement-generator"` | `enforcement` block with `role: generator`, `engine`, `generator`, `actions`, `watch`, `file_types` |
| `"enforcement-contributor"` | `enforcement` block with `role: contributor`, `contributes_to`, `rules_path`; AND `dependencies` including the generator |
| `"domain-knowledge"` | `knowledge_declarations` block |
| `"workflow"` | `workflows` block with `stage_slot` |
| `"methodology"` | `methodology` block |
| `"connector"` | `connector` block |

## Authoritative Enforcement Generator Manifest

```json
{
  "name": "@orqastudio/plugin-eslint",
  "categories": ["enforcement-generator", "enforcement-contributor"],
  "enforcement": {
    "role": "generator",
    "engine": "eslint",
    "config_output": ".orqa/configs/eslint.config.js",
    "generator": "scripts/generate-config.rs",
    "actions": {
      "check": { "command": "eslint", "args": ["--config", ".orqa/configs/eslint.config.js"] },
      "fix": { "command": "eslint", "args": ["--config", ".orqa/configs/eslint.config.js", "--fix"] }
    },
    "watch": {
      "paths": [".orqa/learning/rules/*.md"],
      "filter": "enforcement_type: mechanical AND engine: eslint",
      "on_change": "regenerate"
    },
    "file_types": ["*.ts", "*.svelte", "*.js"],
    "rules_path": "rules/"
  }
}
```

**Key fields:**

- `engine` → becomes the `orqa enforce --eslint` dynamic CLI flag
- `config_output` → always under `.orqa/configs/` — never in project root
- `generator` → script composing config from rule files in `.orqa/learning/rules/`
- `actions.check` / `actions.fix` → dispatched by `orqa enforce`
- `watch.paths` → daemon registers these at startup
- `watch.filter` → which rule files apply to this engine
- `watch.on_change: "regenerate"` → daemon re-runs generator on matched change
- `file_types` → used for `--staged` filtering

## Authoritative Enforcement Contributor Manifest

```json
{
  "name": "@orqastudio/plugin-typescript",
  "categories": ["domain-knowledge", "enforcement-contributor"],
  "dependencies": ["@orqastudio/plugin-eslint"],
  "enforcement": {
    "role": "contributor",
    "contributes_to": "@orqastudio/plugin-eslint",
    "rules_path": "rules/"
  }
}
```

**Critical:** `dependencies` MUST include the generator plugin. `orqa install` warns/fails if the declared generator is not installed.

## Complete Enforcement Contract

**For enforcement-generator:** must provide ALL of:

1. Generator script
2. `enforcement.engine` declaration (becomes CLI flag)
3. `check`/`fix` command declarations
4. File watcher declarations
5. KNOW artifacts (agent-optimized standards knowledge)
6. DOC artifacts (human-readable reference)

**For enforcement-contributor:** must provide ALL of:

1. Rule data installed to `.orqa/learning/rules/<domain>/`
2. `contributes_to` field (full generator plugin name)
3. `dependencies` entry on the generator
4. KNOW artifacts for the contributed domain
5. DOC artifacts for the contributed domain

**A plugin with mechanical checks but no KNOW artifacts is incomplete.**
