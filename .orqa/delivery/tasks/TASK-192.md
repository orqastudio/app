---
id: TASK-192
title: Update plugin README with installation docs
description: |
  Update the companion plugin README.md with installation instructions,
  what the plugin provides, and how it integrates with OrqaStudio projects.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - Plugin README.md has installation instructions
  - "All plugin components (hooks, skills) are listed in README"
relationships:
  - target: EPIC-052
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-191
    type: depends-on
  - target: TASK-342
    type: depended-on-by
---

## What

The plugin README should clearly explain what the OrqaStudio Claude Code companion
plugin provides and how to install it.

## How

1. Update `.orqa/plugins/orqastudio-claude-plugin/README.md`
2. Document: what hooks are provided, what skills are included, installation steps
3. Keep it concise — reference governance docs for detailed rule/skill documentation

## Verification

- README exists with installation section
- All plugin components (hooks, skills) are listed
