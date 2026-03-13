---
id: TASK-287
title: "Move governance artifacts to process/"
description: "Move lessons, decisions, and rules from .orqa/governance/ to .orqa/process/. Remove governance/hooks/ (plugin implementation, not artifacts). Update project.json, pre-commit hook, all path references, .claude/ symlinks."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-286]
assignee: null
docs: []
skills: []
acceptance:
  - ".orqa/process/lessons/ exists with all lesson files"
  - ".orqa/process/decisions/ exists with all decision files"
  - ".orqa/process/rules/ exists with all rule files"
  - ".orqa/governance/ directory no longer exists"
  - "project.json paths updated to process/"
  - ".claude/rules symlink points to .orqa/process/rules/"
  - "Pre-commit hook references updated paths"
rule-overrides:
  - rule: RULE-003
    reason: "Artifact paths are being reorganized — intermediate state will have mismatches"
---

## What

Move governance artifacts (lessons, decisions, rules) from `.orqa/governance/` to `.orqa/process/`. Remove `governance/hooks/` since hooks are plugin implementation.

## How

1. `git mv .orqa/governance/lessons/ .orqa/process/lessons/`
2. `git mv .orqa/governance/decisions/ .orqa/process/decisions/`
3. `git mv .orqa/governance/rules/ .orqa/process/rules/`
4. Remove `.orqa/governance/hooks/` (verify hooks are in plugin, not here)
5. Update `project.json` artifact paths
6. Update `.claude/` symlinks
7. Update `.githooks/pre-commit` paths
8. Search and update all references in rules, skills, agents, docs

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- Pre-commit hook runs successfully
- No references to old `.orqa/governance/` paths remain
