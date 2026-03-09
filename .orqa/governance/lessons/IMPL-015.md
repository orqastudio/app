---
id: IMPL-015
title: "237 files uncommitted across multiple sessions"
category: process
recurrence: 1
promoted_to: [git-workflow.md (Commit Discipline section), project-type-software/SKILL.md (Commit Discipline section), session-start-hook.sh (uncommitted check), pre-commit-reminder.sh (uncommitted check)]
tags: [git, workflow, commit-discipline, governance-gap]
created: 2026-03-09
---

## What Happened

Over multiple sessions, 237 files of changes accumulated on main without being
committed. This included app code changes (artifact scanning system), governance
restructuring (EPIC-045), documentation migration, planning artifacts, and
architecture decisions. The user discovered this when reviewing priorities.

## Why It Was Unexpected

The git-workflow.md rule requires worktree-based development and regular commits.
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
4. The git-workflow rule should explicitly address governance-only work patterns
   where worktrees aren't used
