---
id: "TASK-494b2fcc"
type: "task"
title: "Move governance artifacts to process/"
description: "Move lessons, decisions, and rules from .orqa/process/ to .orqa/process/. Remove governance/hooks/ (plugin implementation, not artifacts). Update project.json, pre-commit hook, all path references, .claude/ symlinks."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
assignee: null
docs: []
acceptance:
  - ".orqa/process/lessons/ exists with all lesson files"
  - ".orqa/process/decisions/ exists with all decision files"
  - ".orqa/process/rules/ exists with all rule files"
  - ".orqa/process/ directory no longer exists"
  - "project.json paths updated to process/"
  - ".claude/rules symlink points to .orqa/process/rules/"
  - "Pre-commit hook references updated paths"
rule-overrides:
  - "rule: RULE-63cc16ad"
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-6aa8e1f1"
    type: "depends-on"
---

## What

Move governance artifacts (lessons, decisions, rules) from `.orqa/process/` to `.orqa/process/`. Remove `governance/hooks/` since hooks are plugin implementation.

## How

1. `git mv .orqa/process/lessons/ .orqa/process/lessons/`
2. `git mv .orqa/process/decisions/ .orqa/process/decisions/`
3. `git mv .orqa/process/rules/ .orqa/process/rules/`
4. Remove `.orqa/process/hooks/` (verify hooks are in plugin, not here)
5. Update `project.json` artifact paths
6. Update `.claude/` symlinks
7. Update `.githooks/pre-commit` paths
8. Search and update all references in rules, skills, agents, docs

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- Pre-commit hook runs successfully
- No references to old `.orqa/process/` paths remain
