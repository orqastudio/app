---
id: TASK-ccb0269c
type: task
title: "Plugin packaging — dual-manifest, new commands, end-to-end testing"
status: active
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-9b58fdcb
    type: delivers
  - target: TASK-90a0f752
    type: depends-on
---

# TASK-ccb0269c: Plugin Packaging

## Acceptance Criteria

1. Dual-manifest structure works (orqa-plugin.json + .claude-plugin/plugin.json)
2. `/orqa-validate` command created — runs full integrity check
3. `/orqa-create` command created — guided artifact creation with frontmatter
4. Installation tested via `orqa plugin install`
5. Claude Code plugin discovery works
6. All hooks fire correctly on session events
