---
id: TASK-185
title: Test plugin against OrqaStudio governance artifacts
description: Integration testing of the plugin against the real .orqa/ directory in orqa-studio.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-178
  - TASK-179
  - TASK-180
  - TASK-181
  - TASK-182
  - TASK-184
assignee: AGENT-006
docs: []
skills:
  - SKILL-020
  - SKILL-018
acceptance:
  - Plugin loads all 39+ rules from .orqa/governance/rules/
  - Plugin loads all 7 agents from .orqa/team/agents/
  - Plugin loads all 44+ skills from .orqa/team/skills/
  - PreToolUse enforcement blocks known violation patterns
  - SessionStart injects orchestrator context
  - Slash commands return accurate governance data
  - No errors or crashes during normal Claude Code usage
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
