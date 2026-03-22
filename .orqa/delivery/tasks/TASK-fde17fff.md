---
id: TASK-fde17fff
title: Move rule-enforcement skill to core + rename plugin skill
description: |
  Move the rule-enforcement SKILL.md from the plugin to .orqa/process/skills/ as a
  core skill. Rename the plugin's copy to reflect it's specifically about the
  Claude Code companion plugin setup.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - "rule-enforcement SKILL.md exists as real file in .orqa/process/skills/ with layer: core"
  - Plugin skill still exists and is distinct from core skill
relationships:
  - target: EPIC-4440cdd4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-835e2645
    type: depended-on-by
  - target: TASK-d6727d2f
    type: depended-on-by
---

## What

The rule-enforcement skill currently lives in the plugin at
`.orqa/plugins/orqastudio-claude-plugin/skills/rule-enforcement/SKILL.md` and is
symlinked to `.orqa/process/skills/rule-enforcement`. It should be a proper core skill
in `.orqa/process/skills/` with `layer: core`, since understanding enforcement is
universal across all OrqaStudio projects.

The plugin's skill should be renamed to describe what it specifically provides:
the companion plugin setup and configuration.

## How

1. Copy the rule-enforcement SKILL.md content to `.orqa/process/skills/rule-enforcement/SKILL.md` (replace symlink with real file)
2. Set `layer: core` in the skill frontmatter
3. Rename the plugin skill from `rule-enforcement` to a more descriptive name if needed
4. Update any references

## Verification

- `.orqa/process/skills/rule-enforcement/SKILL.md` exists as a real file (not symlink)
- `layer: core` in frontmatter
- Plugin skill still exists and is distinct
