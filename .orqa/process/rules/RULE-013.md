---
id: RULE-013
title: Git Workflow
description: Worktree-based workflow with mandatory cleanup, stash policy, and data loss prevention rules.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: core
enforcement:
  - event: bash
    pattern: git commit.*--no-verify
    action: block
    message: Never bypass pre-commit hooks with --no-verify (RULE-013).
  - event: bash
    pattern: git push.*--no-verify
    action: block
    message: Never bypass push hooks with --no-verify (RULE-013).
  - event: bash
    pattern: git reset --hard
    action: warn
    message: Destructive git operation. Verify with git status and git diff first (RULE-013).
  - event: bash
    pattern: git checkout \.
    action: warn
    message: Destructive git operation. Verify with git status and git diff first (RULE-013).
  - event: bash
    pattern: git clean -f
    action: warn
    message: Destructive git operation. Verify what will be removed first (RULE-013).
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Git workflow creates structured, traceable version control practices
  - target: RULE-007
    type: informs
    rationale: Git commands remain raw — make does not wrap version control operations
  - target: RULE-012
    type: informs
    rationale: Pre-commit hooks enforce error ownership — bypassing with --no-verify is forbidden
  - target: RULE-006
    type: informs
    rationale: Pre-commit hook runs coding standard checks that must pass before every commit
  - target: RULE-009
    type: informs
    rationale: Restart protocol and session-ending commit discipline intersect in dogfood mode
  - target: IMPL-015
    type: observes
    rationale: Rule promoted from lesson IMPL-015 (worktree cleanup discipline)
  - type: informed-by
    target: RULE-007
    rationale: Make targets define the dev server commands that git workflow interacts with
  - type: informed-by
    target: RULE-025
    rationale: Root directory cleanliness governs tmp/ directory usage during git operations
  - type: informed-by
    target: RULE-039
    rationale: Session management requires clean git state at session boundaries
  - type: grounded
    target: IMPL-015
    rationale: Worktree cleanup lesson grounds the mandatory cleanup constraint
  - target: RULE-045
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RULE-045
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
  - target: DOC-002
    type: documented-by
    rationale: Referenced in documentation page Enforcement Architecture
  - target: DOC-035
    type: documented-by
    rationale: workflow.md is the source-of-truth document for the git workflow this rule enforces
---
**Source of Truth:** `.orqa/documentation/process/workflow.md`

## Worktree Workflow (MANDATORY)

```bash
# 1. Create worktree at task start
git worktree add ../orqa-<task> -b <agent>/<task>
cd ../orqa-<task>

# 2. Work in isolation, commit regularly

# 3. Request code-reviewer approval BEFORE merging

# 4. Merge only after approval
cd ../orqa-studio && git merge <branch>

# 5. MANDATORY CLEANUP — no stale worktrees
git branch -d <branch> && git worktree remove ../orqa-<task>
```

## Worktree Cleanup (NON-NEGOTIABLE)

**Every completed task MUST have its worktree merged and cleaned up immediately.**

- **Kill all processes in the worktree BEFORE removal** — `ps -ef | grep "orqa-<task>"` then `kill` each PID. Background dev servers, test runs, and log tailers hold file locks that prevent directory deletion on Windows.
- Merge conflicts MUST be resolved aligned with the task goals:
  - Task changes take priority in task-modified files
  - Preserve unrelated changes from main in shared files
  - NEVER discard task work to resolve conflicts
  - NEVER discard unrelated main work
- After merge: kill processes -> delete branch -> remove worktree -> verify directory is gone
- No worktree should survive past its task completion
- Orchestrator verifies no stale worktrees accumulate after each task batch
- **If directory persists after removal**, there are still processes holding locks — find child processes too (e.g., `node.exe`, `cargo.exe` spawned by a killed bash shell)

## Background Process Discipline (NON-NEGOTIABLE)

- **NEVER** launch perpetual commands (`npm run dev`, `cargo tauri dev`, `cargo watch`) as background tasks in worktrees
- **NEVER** leave background task processes running after a task completes
- Context compaction loses background task IDs, making orphaned processes unmanageable
- If a background task was launched, its ID must be tracked and the process stopped before worktree cleanup

## Untracked Files Policy (NON-NEGOTIABLE)

**Before starting ANY task, ALL untracked and modified files MUST be committed.** Untracked files are invisible to worktree operations and `git merge`. Starting work with uncommitted files risks silent data loss — the same class of problem as stashes.

**Session Start — Untracked File Check (MANDATORY):**

Run `git status --short`. If untracked (`??`) or modified (`M`) files exist: commit them to main with a descriptive message before creating any worktree or starting any task. The working tree must be completely clean before proceeding.

## Git Safety

**NEVER without verification:**

- `git checkout .`
- `git reset --hard`
- `git clean -fd`

**Before ANY revert:** `git status` then `git diff` to see what will be lost.

## Data Loss Prevention (CRITICAL)

**Before deleting or moving ANY directory:**

1. Verify replacement content exists and is committed: `git ls-tree HEAD -- <destination>`
2. Check for stashed work: `git stash list`
3. Never delete source-of-truth directories (.orqa/, backend/src-tauri/, ui/, sidecar/, tests/) without explicit user approval
4. Never `git rm -r` on directories with >5 files without user confirmation
5. If migrating content between directories, confirm destination is committed BEFORE deleting source

**Before bulk cleanup commits:** Run `git diff --cached --stat` and verify no documentation or source code is being accidentally deleted.

## Commit Discipline (NON-NEGOTIABLE)

Commit at natural boundaries. Never accumulate large numbers of uncommitted files.

### When to Commit

| Boundary | Action |
|----------|--------|
| Task completion | Commit all task deliverables |
| Epic completion | Commit + update epic/task statuses |
| Session end | Commit ALL uncommitted changes before ending |
| Governance-only work (rules, docs, artifacts) | Commit at least every 20 files or at each logical milestone |

### Governance-Only Work

Work that only modifies `.orqa/` files (rules, docs, agents, skills, planning artifacts) is often done on main without a worktree. This is acceptable, but commit discipline still applies:

- Commit at each logical milestone (e.g., "all rules updated", "epic planned", "docs migrated")
- The session-start hook warns when uncommitted files exceed 20
- The session-end hook reminds to commit before closing
- **Never end a session with uncommitted changes on main**

### Threshold

If `git status --short | wc -l` exceeds 20 on main, stop current work and commit. This threshold exists because large uncommitted batches increase the risk of context loss and make git history harder to understand.

## Git Stash Policy (NON-NEGOTIABLE)

**`git stash` is ONLY for temporary use within a single procedure** (e.g., stash -> rebase -> pop). Must be popped/applied in the same command sequence. NEVER leave stashes at session end.

**Session Start:** Always run `git stash list`. If stashes exist, investigate (`git stash show -p`), commit the work, inform the user, then drop the stash.

**Why:** Stashes are invisible to worktree merges and cause silent data loss.

## FORBIDDEN

**NEVER use `--no-verify` on commits.** Fix the errors instead.

- `git commit --no-verify` — FORBIDDEN
- `git push --no-verify` — FORBIDDEN
- `git stash` as a way to park work between sessions — FORBIDDEN (commit to a branch instead)

If pre-commit hooks fail, it's YOUR responsibility to fix the issues.

## Related Rules

- [RULE-007](RULE-007) (development-commands) — `make dev` and `make restart-tauri` commands that interact with the worktree lifecycle
- [RULE-012](RULE-012) (error-ownership) — pre-commit hook enforcement; `--no-verify` is forbidden under both rules
- [RULE-006](RULE-006) (coding-standards) — defines the quality checks the pre-commit hook enforces; commits must be clean
- [RULE-009](RULE-009) (dogfood-mode) — restart protocol and session-ending behavior that intersects with commit discipline

