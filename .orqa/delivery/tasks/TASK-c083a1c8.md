---
id: TASK-c083a1c8
title: Document plugin installation and configuration
description: "Write documentation for installing, configuring, and using the orqa companion plugin."
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-ec1b3785
docs: []
acceptance:
  - Plugin README covers installation steps
  - README covers configuration options
  - "README covers available commands (/orqa, /orqa:rules, /orqa:status)"
  - README covers enforcement behaviour and how to add enforcement entries to rules
  - OrqaStudio docs updated to reference plugin instead of symlinks
relationships:
  - target: EPIC-3a8ad459
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-11cf4c1d
    type: depends-on
  - target: TASK-fa39671d
    type: depended-on-by
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
