---
id: KNOW-63cc1a00
title: Third-Party Plugin Development
description: |
  Third-party plugin workflow for community and external developers. Plugins are
  standalone projects with their own project configuration and the software plugin
  pre-installed for independent lifecycle management.
status: active
created: 2026-03-19
updated: 2026-03-19
category: domain
version: 0.1.0
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
# Choose a template
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
├── .github/workflows/
│   ├── ci.yml
│   └── publish-dev.yml
├── LICENSE
├── CONTRIBUTING.md
└── README.md
```

### 3. Plugin Manifest

- `name` — `@yourorg/plugin-<name>` (your package scope)
- `displayName`, `description`, `category`, `provides` — same as first-party
- Use your own package scope, not the platform's scope

### 4. Development

Third-party plugins develop independently:
- Create governance seed data for testing
- Run `orqa dev` within the plugin project
- Use `orqa check` for coding standards enforcement
- Use `orqa verify` for integrity validation

### 5. Testing Locally

Install in a test project via file path:

```bash
orqa plugin install --path /path/to/my-plugin
```

### 6. Community Registry Submission

To submit to the community plugin registry:
1. Ensure all validation passes (`orqa enforce`)
2. Submit a PR to the community registry repository
3. Maintainers review for quality, security, and compatibility
4. Verified plugins show a verified badge in the app

### 7. Licensing

Third-party plugins choose their own license. The plugin creation workflow asks:
- Apache-2.0 (permissive, attribution required)
- MIT (permissive, minimal requirements)
- Other (manual LICENSE file)
