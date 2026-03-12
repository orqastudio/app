---
id: TASK-181
title: Implement SessionStart hook (orchestrator injection + session checks)
description: Plugin SessionStart hook injects orchestrator context and runs session-start checks.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-177
assignee: AGENT-002
docs: []
skills:
  - orqa-plugin-development
scope:
  - orqa-plugin/hooks/
acceptance:
  - SessionStart hook reads .orqa/team/agents/orchestrator.md
  - Hook injects orchestrator content as additionalContext
  - Hook runs session-start checks (stashes, worktrees, uncommitted files)
  - Hook replaces the current .claude/hooks/session-start-hook.sh functionality
---

## What

The SessionStart hook replaces both the `.claude/CLAUDE.md` symlink (orchestrator
loading) and the `session-start-hook.sh` shell script. It reads the orchestrator
definition and injects it as system context, then runs session health checks.

## How

1. Create `hooks/session-start.md` hook definition
2. On SessionStart, read `.orqa/team/agents/orchestrator.md`
3. Return orchestrator content as `additionalContext`
4. Run session checks: `git stash list`, `git worktree list`, `git status`
5. Include check results in additionalContext

## Verification

- New session loads orchestrator context without `.claude/CLAUDE.md` symlink
- Session-start checks run and report stashes/worktrees/uncommitted files
- Removing `.claude/CLAUDE.md` symlink doesn't break orchestrator loading
