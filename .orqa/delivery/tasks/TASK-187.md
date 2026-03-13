---
id: TASK-187
title: Document plugin installation and configuration
description: Write documentation for installing, configuring, and using the orqa companion plugin.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-050
depends-on:
  - TASK-185
assignee: AGENT-007
docs: []
skills:
  - SKILL-020
  - SKILL-037
acceptance:
  - Plugin README covers installation steps
  - README covers configuration options
  - README covers available commands (/orqa, /orqa:rules, /orqa:status)
  - README covers enforcement behaviour and how to add enforcement entries to rules
  - OrqaStudio docs updated to reference plugin instead of symlinks
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
