---
id: "TASK-1d45fac7"
type: "task"
title: "Remove .claude/ symlinks and update RULE-63cc16ad"
description: "Remove the .claude/ symlink architecture and update RULE-63cc16ad to describe plugin-based loading."
status: "surpassed"
created: "2026-03-11"
updated: "2026-03-11"
assignee: "AGENT-4c94fe14"
docs: []
acceptance:
  - ".claude/rules/ symlink removed"
  - ".claude/agents/ symlink removed"
  - ".claude/skills/ symlink removed"
  - ".claude/hooks/ symlink removed"
  - ".claude/CLAUDE.md symlink removed"
  - ".claude/ contains only settings.json and settings.local.json"
  - "RULE-63cc16ad symlink section replaced with plugin loading description"
  - "Plugin fully replaces all symlink functionality"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-d38a48c9"
    type: "depends-on"
---
## What

Once the plugin is tested and working (TASK-d38a48c9), the symlink architecture is no
longer needed. Remove all symlinks and update [RULE-63cc16ad](RULE-63cc16ad) to describe the new
plugin-based loading model.

## How

1. Verify plugin handles all functionality the symlinks provided
2. Remove symlinks: rules/, agents/, skills/, hooks/, CLAUDE.md
3. Update [RULE-63cc16ad](RULE-63cc16ad): remove ".claude/ Symlink Architecture" section
4. Add new section describing plugin-based loading
5. Update orchestrator.md if it references symlinks
6. Update MEMORY.md symlink map

## Verification

- `.claude/` contains only `settings.json` and `settings.local.json`
- Claude Code sessions still load orchestrator, rules, agents, skills
- No broken references to `.claude/` symlink paths