---
id: TASK-0be99e74
type: task
title: "Enforce push-after-commit rule mechanically"
description: "RULE-633e636d requires every commit to be followed by git push, but there is no mechanical enforcement. This is an enforcement gap per RULE-12e74734 and must be addressed as CRITICAL priority."
status: captured
priority: P1
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - A post-commit hook or pre-commit hook extension verifies that the previous commit was pushed before allowing a new commit
  - OR a session-end hook checks for unpushed commits and blocks session end until they are pushed
  - The enforcement mechanism is documented in RULE-633e636d enforcement section
  - make check or git commit workflow catches unpushed commits mechanically
  - No false positives on the first commit of a new branch (no remote tracking yet)
relationships:
  - target: EPIC-2362adfc
    type: delivers
    rationale: "Push-after-commit enforcement is part of schema-driven enforcement migration — mechanical hooks replacing behavioral-only compliance"
---

## Context

RULE-633e636d states: "Every commit MUST be followed by `git push` to the remote." and "Committing without pushing — every commit must be pushed to the remote immediately." However, this requirement has no mechanical enforcement — it relies entirely on agent behavioral compliance via prompt injection.

Per RULE-12e74734, any rule without mechanical enforcement is an enforcement gap, and enforcement gaps are always CRITICAL priority.

## Problem

An agent can commit changes but forget to push. The pre-commit hook validates code quality but does not check whether the previous commit was pushed. No post-commit hook exists to remind or enforce pushing. The stop hook checks for uncommitted changes but not for unpushed commits.

## Proposed Solutions

### Option A: Post-commit hook
Add a `.githooks/post-commit` hook that:
1. Checks if the current branch has a remote tracking branch
2. If yes, checks if HEAD is ahead of the remote (`git rev-list @{u}..HEAD`)
3. If ahead by more than 1 commit (the one just made), warns that previous commits were not pushed
4. Optionally auto-pushes or prompts

### Option B: Pre-commit hook extension
Extend `.githooks/pre-commit` to check before allowing a new commit:
1. If the branch has a remote, check if there are unpushed commits older than the staged changes
2. If unpushed commits exist, warn or block

### Option C: Stop hook enhancement
Extend `connectors/claude-code/hooks/scripts/stop-checklist.sh` to:
1. Check `git rev-list @{u}..HEAD` for unpushed commits
2. If any exist, warn with CRITICAL priority in the systemMessage

### Recommendation

Option A (post-commit hook) is the most natural fit — it fires immediately after each commit, which is exactly when the push should happen. Option C is a good safety net but only catches the gap at session end, not at commit time. Both A and C could be implemented together for defense in depth.

## Verification

1. Make a commit without pushing — the enforcement mechanism fires
2. Make a commit and push — no warning
3. First commit on a new branch with no upstream — no false positive
4. The hook handles detached HEAD state gracefully
