---
id: TASK-cd24193f
type: task
title: Hook improvements — validate-artifact, save-context, subagent-review
status: completed
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-b5f9240b
    type: depends-on
  - target: TASK-0751c0ff
    type: depended-on-by
  - target: TASK-e9a4f8f3
    type: depended-on-by
---

# TASK-cd24193f: Hook Improvements

## Acceptance Criteria

1. PostToolUse `validate-artifact.mjs` hook — validates .orqa/ files after Write/Edit
2. PreCompact `save-context.mjs` hook — preserves governance context before compaction
3. SubagentStop `subagent-review.mjs` hook — validates subagent output against governance scope
4. prompt-injector updated with config-driven intent mapping (reads from skill frontmatter)
5. rule-engine updated to use CLI validator library where available
6. hooks.json updated with all new hook entries