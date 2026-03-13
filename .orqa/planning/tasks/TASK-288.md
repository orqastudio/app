---
id: TASK-288
title: "Move team artifacts to process/"
description: "Move skills and agents from .orqa/team/ to .orqa/process/. Update project.json, all path references, .claude/ symlinks."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-286]
assignee: null
docs: []
skills: []
acceptance:
  - ".orqa/process/skills/ exists with all skill directories"
  - ".orqa/process/agents/ exists with all agent files"
  - ".orqa/team/ directory no longer exists"
  - "project.json paths updated"
  - ".claude/agents and .claude/skills symlinks point to new paths"
rule-overrides:
  - rule: RULE-003
    reason: "Artifact paths are being reorganized — intermediate state will have mismatches"
---

## What

Move team artifacts (skills, agents) from `.orqa/team/` to `.orqa/process/`.

## How

1. `git mv .orqa/team/skills/ .orqa/process/skills/`
2. `git mv .orqa/team/agents/ .orqa/process/agents/`
3. Update `project.json` artifact paths
4. Update `.claude/` symlinks
5. Search and update all references in rules, skills, agents, docs

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- `.claude/` symlinks work
- No references to old `.orqa/team/` paths remain
