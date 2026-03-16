---
id: EPIC-047
title: Git Workflow Enforcement Review
description: "Review and strengthen git commit discipline enforcement after discovering 237 files uncommitted across multiple sessions. Update session hooks, git-workflow rule, and create a software project-type skill for commit discipline. Addresses IMPL-015."
status: completed
priority: P1
created: 2026-03-09
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-064
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-065
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-066
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-067
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-068
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-337
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
---
## Context

[IMPL-015](IMPL-015) documents a governance gap: 237 files accumulated uncommitted across
multiple sessions because:
- Session hooks don't check for uncommitted changes on main
- Governance-only work is exempt from worktrees, creating no natural commit point
- No commit-frequency enforcement exists
- No maximum uncommitted file threshold triggers a warning

This epic addresses the gap by updating hooks, rules, and creating an
enforceable skill for commit discipline.

## Implementation Scope

### 1. Session Hook Updates

Update `.orqa/process/hooks/session-start-hook.sh` to:
- Run `git status --short | wc -l` and warn if uncommitted changes exceed threshold
- Display a summary of what's uncommitted (by directory group)

Update `.orqa/process/hooks/pre-commit-reminder.sh` (Stop hook) to:
- Check for uncommitted changes and prompt to commit
- Suggest logical commit groupings based on changed file paths

### 2. Git Workflow Rule Update

Update `.orqa/process/rules/git-workflow.md` to address:
- Governance-only work patterns (no worktree, but still requires regular commits)
- Commit-at-boundaries principle: end of task, end of epic, end of session
- Maximum uncommitted file threshold before work is blocked
- Explicit guidance for orchestrator direct-edit sessions

### 3. Commit Discipline Skill

Create a software project-type skill (or update `project-type-software`) that
encodes commit discipline patterns:
- When to commit during governance work
- How to group changes into logical commits
- Commit message conventions for governance vs code changes
- Session-end commit checklist

### 4. Promote [IMPL-015](IMPL-015)

After enforcement artifacts are in place, update [IMPL-015](IMPL-015) with `evolves-into`
referencing the updated rule and new skill.

## Constraints

- **Orchestrator-only work** — All changes are governance artifacts (hooks,
  rules, skills). No code changes needed.
- **Backward compatible** — Existing git workflow still works; this adds
  enforcement for a gap, not a new workflow.

## Tasks

| Task | Title | Depends On |
|------|-------|------------|
| [TASK-064](TASK-064) | Update session-start hook with uncommitted changes check | — |
| [TASK-065](TASK-065) | Update pre-commit-reminder hook with commit prompt | — |
| [TASK-066](TASK-066) | Update git-workflow rule for governance-only work patterns | — |
| [TASK-067](TASK-067) | Create or update commit discipline skill | [TASK-066](TASK-066) |
| [TASK-068](TASK-068) | Promote [IMPL-015](IMPL-015) to enforcement artifacts | [TASK-064](TASK-064), [TASK-065](TASK-065), [TASK-066](TASK-066), [TASK-067](TASK-067) |

## Implementation Design

Implementation approach to be defined during planning.
