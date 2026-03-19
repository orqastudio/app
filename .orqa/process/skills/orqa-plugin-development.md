---
id: SKILL-b453410f
title: Plugin Development
description: |
  OrqaStudio plugin development base skill. Detects whether this is a first-party
  plugin (dev environment) or third-party plugin (standalone), then loads the
  appropriate sub-skill. Use when: creating new plugins, scaffolding from templates,
  or extending OrqaStudio with custom views, tools, or artifact types.
status: active
created: 2026-03-01
updated: 2026-03-19
category: domain
version: 2.0.0
user-invocable: true
relationships:
  - target: DOC-99a1b71a
    type: synchronised-with
  - target: SKILL-12ed4953
    type: informs
  - target: SKILL-c60144c1
    type: informs  - target: DOC-a1b2c3d4
    type: synchronised-with

---

# Plugin Development

## Environment Detection

Before starting any plugin work, determine the context automatically:

**First-party** (SKILL-12ed4953): The working directory is inside `orqastudio-dev/` or any repository with `plugins/` and `.orqa/` at root AND a `.gitmodules` file referencing `orqastudio` org repos. First-party plugins are submodules in the dev environment.

**Third-party** (SKILL-c60144c1): Everything else. The plugin will be a standalone project with its own `.orqa/` governance and the software plugin pre-installed.

Load the appropriate sub-skill based on detection. Do NOT ask the user — infer from the environment.

## Common Knowledge (Both Paths)

### Plugin Manifest (`orqa-plugin.json`)

Every plugin requires `orqa-plugin.json` at root:

```json
{
  "name": "@scope/plugin-name",
  "displayName": "Human Name",
  "version": "0.1.0-dev",
  "description": "One-line description.",
  "category": "coding-standards|delivery|integration|custom",
  "provides": {
    "skills": [],
    "schemas": [],
    "views": [],
    "relationships": [],
    "tools": {},
    "cliTools": [],
    "hooks": []
  }
}
```

See DOC-99a1b71a (Plugin Manifest Schema Reference) for the full field reference.

### Extension Pattern

Plugins can extend other plugins via the `extends` field:

```json
{
  "extends": ["@orqastudio/plugin-rust"],
  "configExtensions": {
    "tsconfig": { "compilerOptions": { ... } },
    "eslint": { "import": "@scope/plugin/eslint", "export": "config" }
  }
}
```

### Templates

Four scaffolding templates are available:

| Template | Use When |
|----------|----------|
| `cli-tool` | One-shot CLI commands (build tools, generators, analysers) |
| `frontend` | Views and dashboards rendered in the OrqaStudio explorer |
| `sidecar` | Long-running provider processes (AI services, language servers) |
| `full` | All of the above — views, sidecar, CLI tools, and hooks |

### Artifact Graph SDK

Plugins consume the SDK for graph queries:

```typescript
import { getStores } from "@orqastudio/sdk";
const { artifactGraphSDK } = getStores();

// Resolution
artifactGraphSDK.resolve("EPIC-e045ab6d");
artifactGraphSDK.resolveByPath("path/to/file.md");

// Traversal
artifactGraphSDK.referencesFrom("EPIC-e045ab6d");
artifactGraphSDK.referencesTo("TASK-a80d3862");

// Bulk queries
artifactGraphSDK.byType("task");
artifactGraphSDK.byStatus("active");

// Content
await artifactGraphSDK.readContent("path/to/file.md");

// Subscriptions (reactive updates)
artifactGraphSDK.subscribe("EPIC-e045ab6d", callback);
```

### Skill + Doc Requirement

Every plugin that defines artifact types or relationships MUST ship:
- A **skill** teaching agents how to use the artifacts
- A **documentation** artifact teaching humans the same
- Connected via `synchronised-with` relationship

### Validation

Run `orqa validate` in the plugin directory to check:
- Manifest schema compliance
- Skill/doc frontmatter validity
- Relationship target resolution
- ID uniqueness
- Template compatibility (for template repos)

## Anti-Patterns

| Anti-Pattern | Correct Approach |
|-------------|-----------------|
| Developing inside production `.orqa/` | Always scaffold from a template |
| Hardcoding artifact paths | Use SDK resolution methods |
| Polling for changes | Use SDK subscriptions |
| Skipping validation | Run `orqa validate` before every commit |
| Missing skill+doc pair | Every artifact-providing plugin needs both |
