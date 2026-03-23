---
id: KNOW-63cc1a00
type: knowledge
title: Third-Party Plugin Development
description: |
  Third-party plugin workflow for community and external developers. Plugins are
  standalone projects with their own project configuration and the software plugin
  pre-installed for independent lifecycle management.
status: active
created: 2026-03-19
updated: 2026-03-23
category: domain
version: 0.2.0
user-invocable: false
relationships:
  - target: DOC-c65f07b7
    type: synchronised-with
  - target: KNOW-b453410f
    type: synchronised-with
  - target: DOC-99a1b71a
    type: synchronised-with
  - target: DOC-a1b2c3d4
    type: synchronised-with
---

# Third-Party Plugin Development

## Detection

This skill is loaded when the base plugin development skill detects that the working directory is NOT the platform dev environment. Any standalone project creating a plugin uses this workflow.

## Workflow

### 1. Scaffold from Template

```bash
orqa plugin create --template <cli-tool|frontend|full|sidecar> --name <plugin-name>
```

This:
- Creates a new directory `<plugin-name>/`
- Copies the chosen template
- Initialises a git repo
- Creates project configuration with the software plugin pre-installed for lifecycle management
- Activates workflow templates (renames `.template` → `.yml`)
- Generates LICENSE (user chooses) and CONTRIBUTING.md

### 2. Project Structure

Third-party plugins are standalone projects:

```
my-plugin/
├── <governance-dir>/             # Project governance artifacts
│   ├── project configuration     # Software plugin pre-installed
│   └── delivery/                 # Milestones, epics, delivery items
├── orqa-plugin.json              # Plugin manifest
├── package.json
├── src/
├── knowledge/                    # Knowledge artifacts (copied to .orqa/ on install)
├── rules/                        # Rule artifacts (copied to .orqa/ on install)
├── .github/workflows/
│   ├── ci.yml
│   └── publish-dev.yml
├── LICENSE
├── CONTRIBUTING.md
└── README.md
```

### 3. Plugin Manifest

```json
{
  "name": "@yourorg/plugin-name",
  "version": "0.1.0-dev",
  "displayName": "My Plugin",
  "description": "One-line description.",
  "category": "coding-standards|delivery|integration|custom",
  "provides": {
    "schemas": [],
    "knowledge": [],
    "enforcement_mechanisms": []
  },
  "content": {
    "knowledge": { "source": "knowledge", "target": ".orqa/process/knowledge" },
    "rules": { "source": "rules", "target": ".orqa/process/rules" }
  },
  "dependencies": {
    "npm": []
  }
}
```

- `name` — `@yourorg/plugin-<name>` (your package scope, not the platform's)
- `provides` — at least one capability (schemas, views, hooks, agents, knowledge, enforcement_mechanisms, etc.) or a `content` mapping
- `content` — source-to-target mappings; files are copied to the project's `.orqa/` at install and tracked in `.orqa/manifest.json`

### 4. Content Ownership

Plugin-owned files in `.orqa/` are protected — users cannot edit them directly. The engine enforces this using `manifest.json`. To update content:

1. Edit in the plugin source directory
2. Run `orqa plugin refresh` in the consuming project

This applies to your own plugin too when testing locally.

### 5. Development

Third-party plugins develop independently:
- Create governance seed data for testing
- Run `orqa dev` within the plugin project
- Use `orqa check` for coding standards enforcement
- Run `orqa enforce` for manifest and integrity validation

### 6. Testing Locally

Install in a test project via file path:

```bash
orqa plugin install --path /path/to/my-plugin
```

After making content changes during development:

```bash
orqa plugin refresh my-plugin-name
```

To inspect what has drifted between source and `.orqa/`:

```bash
orqa plugin diff my-plugin-name
```

### 7. Community Registry Submission

To submit to the community plugin registry:
1. Ensure all enforcement passes (`orqa enforce`)
2. Submit a PR to the community registry repository
3. Maintainers review for quality, security, and compatibility
4. Verified plugins show a verified badge in the app

### 8. Lifecycle Reference

```bash
orqa plugin install <name-or-path>   # Install and copy content to .orqa/
orqa plugin uninstall <name>          # Remove plugin and its owned files from .orqa/
orqa plugin enable <name>             # Re-copy content for a disabled plugin
orqa plugin disable <name>            # Remove content without uninstalling
orqa plugin refresh [name]            # Rebuild and re-sync content (one or all)
orqa plugin diff [name]               # Show content drift between source and .orqa/
```

### 9. Licensing

Third-party plugins choose their own license. The plugin creation workflow asks:
- Apache-2.0 (permissive, attribution required)
- MIT (permissive, minimal requirements)
- Other (manual LICENSE file)