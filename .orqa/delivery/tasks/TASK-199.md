---
id: TASK-199
title: Implement skill injection in plugin rule-engine.mjs
description: |
  Extend the plugin rule engine to handle inject actions by reading SKILL.md
  files and returning their content as systemMessage.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - Plugin rule-engine.mjs reads SKILL.md files for inject actions
  - Skill content returned as systemMessage
  - Duplicate skill injection suppressed within same session
  - Missing skill files silently skipped
relationships:
  - target: EPIC-052
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-193
    type: depends-on
  - target: TASK-201
    type: depended-on-by
  - target: TASK-204
    type: depended-on-by
  - target: TASK-207
    type: depended-on-by
  - target: TASK-342
    type: depended-on-by
---

## What

When the rule engine encounters an enforcement entry with `action: inject` and
a `skills` array, it reads the specified SKILL.md files from `.orqa/process/skills/`
and returns their content as `systemMessage`.

## How

1. In `rule-engine.mjs`, add inject action handling alongside block/warn
2. Read SKILL.md files from `$projectDir/.orqa/process/skills/<name>/SKILL.md`
3. Concatenate skill content into a single systemMessage
4. Track injected skills per session via a state file to prevent re-injection
5. Return systemMessage via stdout (exit 0, like warn)

## Verification

- Enforcement entry with `action: inject, skills: [orqa-ipc-patterns]` returns skill content
- Same skill not re-injected in subsequent calls within the same session
- Missing skill files are silently skipped (not a fatal error)
