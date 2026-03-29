---
id: RULE-30a223ca
type: rule
title: Session Management
description: "Sessions must be managed with state persistence, clean handoffs, and no unsaved work at session boundaries."
status: active
enforcement_type: mechanical
created: 2026-03-11
updated: 2026-03-11
enforcement:

  - mechanism: behavioral

    message: "Sessions must be managed with state persistence, clean handoffs, and no unsaved work at session boundaries"

  - mechanism: hook

    type: Stop
    action: check
    description: "Stop hook verifies session state is written to .state/session-state.md and changes are committed"

  - mechanism: hook

    type: SessionStart
    action: surface
    description: "SessionStart hook reads .state/session-state.md and surfaces unfinished work"
relationships:

  - target: PD-4ea9a290

    type: enforces
---

Every session that performs work must leave the codebase in a clean, resumable state. Session state bridges the gap between context windows.

## Session State File

When ending a session or stepping away, write a session state file to `.state/session-state.md` covering:

- Tasks completed and their status
- Tasks in progress and current state
- Blockers and decisions needed from the user
- Context needed to resume (branch, worktree, key files)

## Overnight Mode Protocol

When the user signals they are stepping away:

1. Write detailed session state to `.state/session-state.md`
2. Commit all work-in-progress to the current branch (NOT to main unless working on main)
3. Do NOT continue implementing without user oversight
4. Verify no stale processes are running (dev servers, background tasks)

## Session Resume Protocol

When resuming from a previous session:

1. Read `.state/session-state.md` if it exists
2. Run `git status` and `git stash list` to verify clean state
3. Check for stale worktrees (`git worktree list`)
4. Resume from where the previous session left off

## FORBIDDEN

- Ending a session with uncommitted changes on any branch
- Leaving stale `.state/session-state.md` from a prior session without reading it on resume
- Continuing implementation after the user signals they are stepping away
- Leaving background processes running at session end without documenting them in session state
- Leaving stashes at session end (commit to a branch instead)

## Related Rules

- [RULE-f609242f](RULE-f609242f) (git-workflow) — git stash policy and commit discipline
- [RULE-d2c2063a](RULE-d2c2063a) (development-commands) — dev server lifecycle management
