---
id: "IMPL-53fc59b5"
type: lesson
title: "Commit at natural boundaries to prevent file accumulation"
description: "Commit governance changes at natural boundaries (end of epic, task batch, or session) to prevent large numbers of uncommitted files from accumulating on main."
status: "completed"
created: "2026-03-09"
updated: "2026-03-09"
maturity: "understanding"
recurrence: 1
relationships: []
---
## What Happened

Over multiple sessions, 237 files of changes accumulated on main without being
committed. This included app code changes (artifact scanning system), governance
restructuring [EPIC-7394ba2a](EPIC-7394ba2a), documentation migration, planning artifacts, and
architecture decisions. The user discovered this when reviewing priorities.

## Why It Was Unexpected

[RULE-f609242f](RULE-f609242f) (`git-workflow`) requires worktree-based development and regular commits.
The session-start hook checks for stashes and worktrees but does NOT check for
uncommitted changes on main. Multiple sessions of governance-only work (rules,
agents, skills, documentation) bypassed the worktree requirement because the
orchestrator is permitted to edit governance artifacts directly.

## Root Causes

1. **No commit-frequency enforcement** — No rule requires committing within a
   session or at session end. The pre-commit-reminder hook fires on Stop but
   doesn't check if there are uncommitted changes.
2. **Governance work exempt from worktrees** — The agent-delegation rule allows
   the orchestrator to edit .orqa/ directly, which means no branch → no merge
   → no natural commit point.
3. **Session-start hook blind spot** — Checks stashes and worktrees but not
   `git status` for uncommitted changes on main.
4. **No maximum uncommitted file count** — No threshold that triggers a warning
   or blocks further work until changes are committed.

## Correct Approach

1. Session-start hook should run `git status --short` and warn if uncommitted
   changes exceed a threshold (e.g., 20 files)
2. Session-end (Stop hook) should prompt to commit if uncommitted changes exist
3. A rule should require committing governance changes at natural boundaries
   (end of epic, end of task batch, end of session)
4. [RULE-f609242f](RULE-f609242f) (`git-workflow`) should explicitly address governance-only work patterns
   where worktrees aren't used

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.