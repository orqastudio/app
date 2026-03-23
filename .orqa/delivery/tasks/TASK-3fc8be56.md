---
id: "TASK-3fc8be56"
type: task
title: "Remove .claude/ symlinks and update RULE-6c0496e0"
description: "Remove the .claude/ symlink architecture and update RULE-6c0496e0 to describe plugin-based loading."
status: "surpassed"
created: "2026-03-11"
updated: "2026-03-11"
assignee: "AGENT-1dab5ebe"
docs: []
acceptance:
  - ".claude/rules/ symlink removed"
  - ".claude/agents/ symlink removed"
  - ".claude/skills/ symlink removed"
  - ".claude/hooks/ symlink removed"
  - ".claude/CLAUDE.md symlink removed"
  - ".claude/ contains only settings.json and settings.local.json"
  - "RULE-6c0496e0 symlink section replaced with plugin loading description"
  - "Plugin fully replaces all symlink functionality"
relationships:
  - target: "EPIC-3a8ad459"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-11cf4c1d"
    type: "depends-on"
  - target: "TASK-fa39671d"
    type: "depended-on-by"
---
## What

Once the plugin is tested and working (TASK-11cf4c1d), the symlink architecture is no
longer needed. Remove all symlinks and update [RULE-6c0496e0](RULE-6c0496e0) to describe the new
plugin-based loading model.

## How

1. Verify plugin handles all functionality the symlinks provided
2. Remove symlinks: rules/, agents/, skills/, hooks/, CLAUDE.md
3. Update [RULE-6c0496e0](RULE-6c0496e0): remove ".claude/ Symlink Architecture" section
4. Add new section describing plugin-based loading
5. Update orchestrator.md if it references symlinks
6. Update MEMORY.md symlink map

## Verification

- `.claude/` contains only `settings.json` and `settings.local.json`
- Claude Code sessions still load orchestrator, rules, agents, skills
- No broken references to `.claude/` symlink paths