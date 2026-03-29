---
id: KNOW-f2a5e8b1
type: knowledge
status: active
title: "Plugin Composition Pipeline"
description: "How plugin contributions are merged into resolved workflows and composed schemas — the 8-step pipeline that runs on every definition plugin install"
tier: on-demand
created: 2026-03-29
roles: [implementer, reviewer, planner]
paths: [engine/plugin/, .orqa/workflows/]
tags: [architecture, plugins, composition, workflows, schema]
relationships:
  - type: synchronised-with
    target: DOC-41ccf7c4
---

# Plugin Composition Pipeline

## When It Runs

The composition pipeline runs whenever a **definition plugin** is installed (via `orqa plugin install` or as part of the dev environment `orqa install`). Definition plugins are: methodology plugins and workflow plugins. Non-definition plugins (knowledge, views, sidecars, infrastructure) do NOT trigger recomposition — they only install their assets.

## The 8-Step Pipeline

1. **Read methodology skeleton** — read the installed methodology plugin's workflow skeleton with named stage slots
2. **Read workflow contributions** — for each installed workflow plugin, read its contribution manifest. Source is the plugin directory, NOT copies in `.orqa/`.
3. **Merge contributions** — merge each workflow plugin's contribution into the methodology's stage slot it claims
4. **Compose full JSON schema** — collect all artifact type definitions and state machines from all plugins; compile into one schema
5. **Validate** — validate the composed schema for consistency (no conflicting types, valid state machine references, etc.)
6. **Write resolved workflows** — write one resolved YAML file per stage to `.orqa/workflows/<stage>.resolved.yaml`
7. **Write composed schema** — write `schema.composed.json` for LSP/MCP validation
8. **Write prompt registry** — write `prompt-registry.json` for the prompt generation pipeline

## Key Invariant: Source Stays in Plugin Dirs

Source workflow definitions stay in plugin directories (`plugins/<name>/`). Only resolved output goes to `.orqa/workflows/`. This means:

- The resolved workflows in `.orqa/` are always generated, never hand-edited
- Recomposition overwrites `.orqa/workflows/` — any manual edits there are lost
- To change a workflow, change the plugin source and recompose

## Resolved Workflow File Format

Each resolved workflow file (`<stage>.resolved.yaml`) contains:

- All artifact types for that stage (with full schema definitions)
- Complete state machine per artifact type (states, transitions, guards, actions)
- State categories mapped (planning, active, review, completed, terminal)
- Human gate definitions where applicable
- Relationship types relevant to the stage
- Contribution point metadata (which plugin contributed this content)

The runtime reads only resolved files. Recomposition happens only at plugin install/update.

## Installation Constraints Enforced by `orqa install`

| Constraint | Error |
| ----------- | ------- |
| More than one methodology plugin | `Error: second methodology plugin` |
| Two workflow plugins claiming the same stage | `Error: stage already claimed` |
| Workflow plugin targeting nonexistent stage | `Error: stage does not exist in methodology` |
| Manifest declares category without matching config block | `Error: schema validation failure` |
| Enforcement-contributor without dependency on generator | `Error: missing dependency` |

These constraints are enforced before any installation action runs. Schema validation of the manifest happens first.

## Three-Way Diff for Content Updates

When `orqa install` runs for a plugin update, it compares three versions:

1. **Plugin source** — the new version from the plugin
2. **Installed baseline** — what was installed last time (recorded in `manifest.json` source hashes)
3. **Project copy** — what exists in `.orqa/` right now (may have local edits)

This detects both plugin updates and local user edits, enabling intelligent merge rather than blind overwrite.
