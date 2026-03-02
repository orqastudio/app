#!/bin/bash
# Reminder hook for commit-related tasks

cat <<'EOF'
PRE-COMMIT CHECKLIST:

□ All checks pass (cargo test, cargo clippy, npm run check)
□ No stub/mock/placeholder patterns in committed code
□ Working in worktree (not main)
□ Changes are within your task scope
□ No TODO comments or stubs added
□ Function size limits respected (≤50 lines)
□ Documentation updated if behaviour changed?

FORBIDDEN:
□ NEVER use --no-verify - fix the errors instead
□ NEVER skip pre-commit hooks

If merging to main:
□ code-reviewer approval received
□ git merge <branch> from main worktree
□ Clean up: git branch -d <branch> && git worktree remove ../forge-<task>

SESSION STATE REMINDER:
Before this session ends, write a session summary to tmp/session-state.md:
- Tasks completed this session
- Tasks in progress (with current state)
- Blockers encountered
- Context needed to resume
- Active worktrees (run: git worktree list)
EOF
