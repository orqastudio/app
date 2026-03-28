---
id: "TASK-15cb18ee"
type: "task"
title: "Create orqa-plugin repository with Claude Code plugin scaffold"
description: "Set up the separate orqa-plugin repo with plugin.json manifest, directory structure, and README."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
assignee: "AGENT-e5dd38e4"
docs: []
acceptance:
  - "Repository exists with .claude-plugin/plugin.json manifest"
  - "Directory structure matches EPIC-9a1eba3f architecture (hooks/, commands/, agents/, skills/, core/)"
  - "Plugin is loadable by Claude Code (plugin.json validates)"
  - "README documents the plugin purpose and installation"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-9726f126"
    type: "depends-on"
---

## What

Create the `orqa-plugin` repository with the Claude Code plugin scaffold.
This is the foundation all other [EPIC-9a1eba3f](EPIC-9a1eba3f) tasks build on.

## How

1. Create new repository `orqa-plugin`
2. Add `.claude-plugin/plugin.json` manifest
3. Create directory structure: hooks/, commands/, agents/, skills/, core/
4. Add README with plugin purpose and installation instructions
5. Verify Claude Code can discover and load the plugin

## Verification

- `plugin.json` is valid and Claude Code recognises the plugin
- All required directories exist
- README explains installation
