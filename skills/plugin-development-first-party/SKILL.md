---
name: plugin-development-first-party
description: "First-party plugin workflow for the OrqaStudio dev environment. Use when creating or modifying plugins within the orqastudio-dev monorepo. Plugins are submodules, managed by the dev environment's orqa CLI, and published via GitHub Actions."
user-invocable: false
---

# First-Party Plugin Development

## Detection

This skill is loaded when the base plugin development skill (SKILL-b453410f) detects the dev environment. Detection: the current working directory is inside a repository that contains `orqastudio-dev` in its path or has a `.orqa/` directory AND a `plugins/` directory at root.

## Workflow

### 1. Scaffold from Template

```bash
# Choose a template
orqa plugin create --template <cli-tool|frontend|full|sidecar> --name <plugin-name>
```

This:
- Copies the template into `plugins/<plugin-name>/`
- Creates a GitHub repo under the `orqastudio` org: `orqastudio-plugin-<name>`
- Initialises git, sets remote, pushes initial commit
- Registers as a git submodule in the dev environment
- Activates workflow templates (renames `.template` → `.yml`)
- Generates LICENSE (BSL-1.1) and CONTRIBUTING.md pointing to dev environment

### 2. Plugin Manifest

Every plugin must have `orqa-plugin.json` at root. The template provides a skeleton — fill in:
- `name` — `@orqastudio/plugin-<name>` for first-party
- `displayName` — human-readable name
- `description` — one-line summary
- `category` — `coding-standards`, `delivery`, `integration`, `custom`
- `provides` — what the plugin contributes (skills, views, tools, schemas, relationships)
- `extends` — optional, list of plugins this one extends

### 3. Development

First-party plugins live as submodules in the dev environment. The `orqa dev` command watches them automatically if they have a `dev` or `build` script.

- Edit source in `plugins/<name>/src/`
- Watchers auto-rebuild to `dist/`
- Vite picks up changes via HMR
- No separate project.json needed — the dev environment manages the project

### 4. Skills, Documentation, Agents

Every plugin that defines artifact types or relationships MUST ship:
- A **skill** teaching agents how to use the plugin's artifacts
- A **documentation** artifact teaching humans the same
- Connected via `synchronised-with`

### 5. Publishing

Push to `main` triggers the `publish-dev` workflow which publishes `0.1.0-dev.<SHA>` to GitHub Packages.

### 6. Validation

Run `orqa validate` in the plugin directory. The validator checks:
- Manifest schema compliance
- Skill/doc frontmatter validity
- Relationship target resolution
- Template compatibility (if templates exist)
