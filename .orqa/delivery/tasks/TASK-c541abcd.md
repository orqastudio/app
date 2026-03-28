---
id: "TASK-c541abcd"
type: "task"
title: "Document plugin installation and configuration"
description: "Write documentation for installing, configuring, and using the orqa companion plugin."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
docs: []
acceptance:
  - "Plugin README covers installation steps"
  - "README covers configuration options"
  - "README covers available commands (/orqa, /orqa:rules, /orqa:status)"
  - "README covers enforcement behaviour and how to add enforcement entries to rules"
  - "OrqaStudio docs updated to reference plugin instead of symlinks"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-d38a48c9"
    type: "depends-on"
---

## What

Documentation for the companion plugin so users (and agents) know how to install,
configure, and use it.

## How

1. Write comprehensive README in orqa-plugin repo
2. Update OrqaStudio documentation to reference plugin-based loading
3. Document the enforcement entry format and how to add patterns to rules

## Verification

- README is complete and follows the plugin's actual behaviour
- OrqaStudio docs no longer reference symlink architecture
