---
id: TASK-177
title: Create orqa-plugin repository with Claude Code plugin scaffold
description: "Set up the separate orqa-plugin repo with plugin.json manifest, directory structure, and README."
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-002
docs: []
acceptance:
  - Repository exists with .claude-plugin/plugin.json manifest
  - "Directory structure matches EPIC-050 architecture (hooks/, commands/, agents/, skills/, core/)"
  - Plugin is loadable by Claude Code (plugin.json validates)
  - README documents the plugin purpose and installation
relationships:
  - target: EPIC-050
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-171
    type: depends-on
  - target: SKILL-020
    type: grounded-by
  - target: TASK-178
    type: depended-on-by
  - target: TASK-179
    type: depended-on-by
  - target: TASK-181
    type: depended-on-by
  - target: TASK-182
    type: depended-on-by
  - target: TASK-340
    type: depended-on-by
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
