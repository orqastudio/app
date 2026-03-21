---
id: RULE-e352fd0a
type: rule
title: Session Management
description: "Sessions must be managed with state persistence, clean handoffs, and no unsaved work at session boundaries."
status: active
created: 2026-03-11
updated: 2026-03-11
relationships:
---

Every session that performs work must leave the codebase in a clean, resumable state. Session state bridges the gap between context windows.

## Session State File

When ending a session or stepping away, write a session state file to `tmp/session-state.md` covering:

- Tasks completed and their status
- Tasks in progress and current state
- Blockers and decisions needed from the user
- Context needed to resume (branch, worktree, key files)

## Overnight Mode Protocol

When the user signals they are stepping away:

1. Write detailed session state to `tmp/session-state.md`
2. Commit all work-in-progress to the current branch (NOT to main unless working on main)
3. Do NOT continue implementing without user oversight
4. Verify no stale processes are running (dev servers, background tasks)

## Session Resume Protocol

When resuming from a previous session:

1. Read `tmp/session-state.md` if it exists
2. Run `git status` and `git stash list` to verify clean state
3. Check for stale worktrees (`git worktree list`)
4. Resume from where the previous session left off

## FORBIDDEN

- Ending a session with uncommitted changes on any branch
- Leaving stale `tmp/session-state.md` from a prior session without reading it on resume
- Continuing implementation after the user signals they are stepping away
- Leaving background processes running at session end without documenting them in session state
- Leaving stashes at session end (commit to a branch instead)

## Related Rules

- [RULE-633e636d](RULE-633e636d) (git-workflow) — git stash policy and commit discipline
- [RULE-c71f1c3f](RULE-c71f1c3f) (development-commands) — dev server lifecycle management
