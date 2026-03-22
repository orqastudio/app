---
id: TASK-367f0026
title: Fix connector basics — path bugs, intent mappings, license, dual manifests
status: completed
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-b5f9240b
    type: depended-on-by
  - target: TASK-c9880303
    type: depended-on-by
---

# TASK-367f0026: Fix Connector Basics

## Acceptance Criteria

1. Path bugs fixed: `.orqa/team/skills/` → `.orqa/process/skills/` in prompt-injector.mjs and rule-engine.mjs
2. Intent mappings updated to match actual skill directory names
3. README license badge matches package.json (BSL-1.1)
4. `orqa-plugin.json` manifest created for OrqaStudio plugin registration
5. `.claude-plugin/plugin.json` updated to current Claude Code plugin spec
6. package.json `files` array includes new directories (agents)
