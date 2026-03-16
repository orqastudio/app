---
id: TASK-288
title: Move team artifacts to process/
description: "Move skills and agents from .orqa/process/ to .orqa/process/. Update project.json, all path references, .claude/ symlinks."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - .orqa/process/skills/ exists with all skill directories
  - .orqa/process/agents/ exists with all agent files
  - .orqa/process/ directory no longer exists
  - project.json paths updated
  - .claude/agents and .claude/skills symlinks point to new paths
rule-overrides:
  - "rule: RULE-003"
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-286
    type: depends-on
  - target: TASK-291
    type: depended-on-by
  - target: TASK-292
    type: depended-on-by
  - target: TASK-349
    type: depended-on-by
---

## What

Move team artifacts (skills, agents) from `.orqa/process/` to `.orqa/process/`.

## How

1. `git mv .orqa/process/skills/ .orqa/process/skills/`
2. `git mv .orqa/process/agents/ .orqa/process/agents/`
3. Update `project.json` artifact paths
4. Update `.claude/` symlinks
5. Search and update all references in rules, skills, agents, docs

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- `.claude/` symlinks work
- No references to old `.orqa/process/` paths remain
