---
id: "TASK-d38a48c9"
type: "task"
title: "Test plugin against OrqaStudio governance artifacts"
description: "Integration testing of the plugin against the real .orqa/ directory in orqa-studio."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-11T00:00:00.000Z
docs: []
acceptance:
  - "Plugin loads all 39+ rules from .orqa/process/rules/"
  - "Plugin loads all 7 agents from .orqa/process/agents/"
  - "Plugin loads all 44+ skills from .orqa/process/skills/"
  - "PreToolUse enforcement blocks known violation patterns"
  - "SessionStart injects orchestrator context"
  - "Slash commands return accurate governance data"
  - "No errors or crashes during normal Claude Code usage"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-7565b47d"
    type: "depends-on"
  - target: "TASK-821ab799"
    type: "depends-on"
  - target: "TASK-7c2638f0"
    type: "depends-on"
  - target: "TASK-563e0955"
    type: "depends-on"
  - target: "TASK-243f8acc"
    type: "depends-on"
  - target: "TASK-055c10f7"
    type: "depends-on"
---

## What

End-to-end testing of the companion plugin against OrqaStudio's real governance
artifacts. This validates that the plugin works with the actual `.orqa/` directory
structure, not just test fixtures.

## How

1. Install plugin in orqa-studio project
2. Start a Claude Code session — verify SessionStart hook loads orchestrator
3. Run `/orqa:rules` — verify all active rules are listed
4. Run `/orqa:status` — verify governance health metrics
5. Attempt a blocked action (e.g., file with `unwrap()`) — verify PreToolUse blocks it
6. Attempt a normal action — verify it's allowed through
7. Spawn a subagent — verify capabilities are resolved to CLI tools

## Verification

- Test report with evidence for each acceptance criterion
- No false positives (legitimate actions blocked)
- No false negatives (violations allowed through)
