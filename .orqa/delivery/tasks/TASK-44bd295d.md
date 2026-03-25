---
id: TASK-44bd295d
type: task
title: Fix connector basics — path bugs, intent mappings, license, dual manifests
status: completed
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-9b58fdcb
    type: delivers
  - target: TASK-0c32785c
    type: depended-on-by
  - target: TASK-cc8bf843
    type: depended-on-by
---

# TASK-44bd295d: Fix Connector Basics

## Acceptance Criteria

1. Path bugs fixed: `.orqa/team/skills/` → `.orqa/process/skills/` in prompt-injector.ts and rule-engine.mjs
2. Intent mappings updated to match actual skill directory names
3. README license badge matches package.json (BSL-1.1)
4. `orqa-plugin.json` manifest created for OrqaStudio plugin registration
5. `.claude-plugin/plugin.json` updated to current Claude Code plugin spec
6. package.json `files` array includes new directories (agents)