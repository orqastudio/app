---
id: KNOW-e6fee7a0
type: knowledge
title: First-Party Plugin Development
description: |
  First-party plugin workflow for the platform dev environment. Use when creating
  or modifying plugins within the platform monorepo. Plugins are submodules,
  managed by the dev environment's CLI, and published via CI workflows.
status: active
created: 2026-03-19
updated: 2026-03-23
category: domain
version: 0.2.0
user-invocable: false
relationships:
  - target: DOC-05f59d04
    type: synchronised-with
  - target: KNOW-2f38309a
    type: synchronised-with
  - target: DOC-266182d2
    type: synchronised-with
  - target: DOC-bad8e26f
    type: synchronised-with
  - target: AGENT-ce86fb50
    type: employed-by
    rationale: "Auto-generated inverse of employed-by relationship from AGENT-ce86fb50"
---
# First-Party Plugin Development

## Detection

This skill is loaded when the base plugin development skill detects the dev environment: the current working directory is inside a repository that has a project governance directory AND a `plugins/` directory at root.

## Workflow

### 1. Scaffold from Template

```bash
orqa plugin create --template <cli-tool|frontend|full|sidecar> --name <plugin-name>
```

This:
- Copies the template into `plugins/<plugin-name>/`
- Creates a repository under the platform org
- Initialises git, sets remote, pushes initial commit
- Registers as a git submodule in the dev environment
- Activates workflow templates (renames `.template` → `.yml`)
- Generates LICENSE and CONTRIBUTING.md

### 2. Plugin Manifest

Every plugin must have `orqa-plugin.json` at root. The template provides a skeleton — fill in:
- `name` — `@platform/plugin-<name>` for first-party
- `displayName` — human-readable name
- `description` — one-line summary
- `category` — `coding-standards`, `delivery`, `integration`, `custom`
- `provides` — what the plugin contributes (schemas, views, hooks, agents, knowledge, enforcement_mechanisms, etc.)
- `content` — source-to-target mappings for files copied to `.orqa/` at install time
- `dependencies` — npm packages to install into the project
- `build` — optional build command run before content sync
- `extends` — optional, list of plugins this one extends

### 3. Content Ownership

Plugin content is copied from plugin source directories to `.orqa/` at install time, tracked in `.orqa/manifest.json`. The engine scans `.orqa/`, not plugin source directories.

**Do NOT edit plugin-owned files directly in `.orqa/`.** The engine enforces edit protection on manifest-tracked files.

When editing plugin content:
1. Edit source in `plugins/<name>/knowledge/`, `plugins/<name>/rules/`, etc.
2. Run `orqa plugin refresh <name>` to rebuild and re-sync to `.orqa/`

### 4. Development

First-party plugins live as submodules in the dev environment. The `orqa dev` command watches them automatically if they have a `dev` or `build` script.

- Edit source in `plugins/<name>/src/` (frontend/sidecar code)
- Edit governance content in `plugins/<name>/knowledge/`, `plugins/<name>/rules/`, etc.
- Watchers auto-rebuild frontend source to `dist/`
- After governance content changes: run `orqa plugin refresh <name>`
- No separate project configuration needed — the dev environment manages the project

### 5. Knowledge, Documentation, Agents

Every plugin that defines artifact types or relationships MUST ship:
- A **knowledge artifact** teaching agents how to use the plugin's artifacts
- A **documentation** artifact teaching humans the same
- Connected via `synchronised-with`

### 6. Publishing

Push to `main` triggers the publish workflow which publishes a dev-tagged version to the package registry.

### 7. Enforcement

Run `orqa enforce` in the plugin directory. The validator checks:
- Manifest schema compliance (`content`, `provides`, `dependencies`, `build` fields)
- Knowledge/doc frontmatter validity
- Relationship target resolution
- Template compatibility (if templates exist)

### 8. Lifecycle Reference

```bash
orqa plugin install <name>     # Install and copy content to .orqa/
orqa plugin uninstall <name>   # Remove plugin and its owned files from .orqa/
orqa plugin enable <name>      # Re-copy content for a disabled plugin
orqa plugin disable <name>     # Remove content without uninstalling
orqa plugin refresh <name>     # Rebuild and re-sync content to .orqa/
orqa plugin diff <name>        # Show content drift between source and .orqa/
```
