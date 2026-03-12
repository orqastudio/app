---
id: TASK-177
title: Create orqa-plugin repository with Claude Code plugin scaffold
description: Set up the separate orqa-plugin repo with plugin.json manifest, directory structure, and README.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-171
assignee: AGENT-002
docs: []
skills:
  - orqa-plugin-development
scope:
  - orqa-plugin/
acceptance:
  - Repository exists with .claude-plugin/plugin.json manifest
  - Directory structure matches EPIC-050 architecture (hooks/, commands/, agents/, skills/, core/)
  - Plugin is loadable by Claude Code (plugin.json validates)
  - README documents the plugin purpose and installation
---

## What

Create the `orqa-plugin` repository with the Claude Code plugin scaffold.
This is the foundation all other [EPIC-050](EPIC-050) tasks build on.

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
