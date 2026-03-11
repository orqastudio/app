#!/usr/bin/env bash
# Session start hook — runs on SessionStart event
# Reads orchestrator context and runs session health checks

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"

# Only run once per session (guard file in tmp/)
GUARD="$PROJECT_DIR/tmp/.session-started"
if [ -f "$GUARD" ]; then
  exit 0
fi
mkdir -p "$PROJECT_DIR/tmp"
touch "$GUARD"

OUTPUT=""

# Check for stashes
STASHES=$(cd "$PROJECT_DIR" && git stash list 2>/dev/null || true)
if [ -n "$STASHES" ]; then
  OUTPUT="${OUTPUT}WARNING: Git stashes found! Investigate and commit before proceeding:\n${STASHES}\n\n"
fi

# Check for stale worktrees
MAIN_DIR=$(cd "$PROJECT_DIR" && git rev-parse --show-toplevel 2>/dev/null || echo "$PROJECT_DIR")
WORKTREES=$(cd "$PROJECT_DIR" && git worktree list 2>/dev/null | grep -v "$MAIN_DIR" || true)
if [ -n "$WORKTREES" ]; then
  OUTPUT="${OUTPUT}WARNING: Non-main worktrees detected! Check if these need cleanup:\n${WORKTREES}\n\n"
fi

# Check for orphaned worktree directories
PARENT_DIR=$(dirname "$MAIN_DIR")
ORPHANS=$(ls -d "$PARENT_DIR"/orqa-* 2>/dev/null || true)
if [ -n "$ORPHANS" ]; then
  OUTPUT="${OUTPUT}WARNING: Orphaned worktree directories found:\n${ORPHANS}\n\n"
fi

# Check for uncommitted changes on main
CURRENT_BRANCH=$(cd "$PROJECT_DIR" && git branch --show-current 2>/dev/null || true)
if [ "$CURRENT_BRANCH" = "main" ]; then
  UNCOMMITTED=$(cd "$PROJECT_DIR" && git status --short 2>/dev/null | wc -l | tr -d ' ')
  if [ "$UNCOMMITTED" -gt 20 ]; then
    OUTPUT="${OUTPUT}WARNING: ${UNCOMMITTED} uncommitted files on main! Commit before starting new work.\n\n"
  elif [ "$UNCOMMITTED" -gt 0 ]; then
    OUTPUT="${OUTPUT}NOTE: ${UNCOMMITTED} uncommitted files on main. Consider committing before starting new work.\n\n"
  fi
fi

# Check for session state from previous session
if [ -f "$PROJECT_DIR/tmp/session-state.md" ]; then
  SESSION_STATE=$(cat "$PROJECT_DIR/tmp/session-state.md")
  OUTPUT="${OUTPUT}PREVIOUS SESSION STATE FOUND:\n${SESSION_STATE}\n\n"
fi

# Dogfood detection
if [ -f "$PROJECT_DIR/.orqa/project.json" ]; then
  if grep -q '"dogfood"[[:space:]]*:[[:space:]]*true' "$PROJECT_DIR/.orqa/project.json" 2>/dev/null; then
    OUTPUT="${OUTPUT}DOGFOOD MODE ACTIVE: You are editing the app from the CLI.\n"
    OUTPUT="${OUTPUT}- CLI context: make restart does NOT end the session\n"
    OUTPUT="${OUTPUT}- Use make restart-tauri after Rust changes\n"
    OUTPUT="${OUTPUT}- See RULE-009 for full dogfood rules\n\n"
  fi
fi

# Delegation reminder
OUTPUT="${OUTPUT}ORCHESTRATOR REMINDERS:\n"
OUTPUT="${OUTPUT}- You coordinate. You do NOT implement. Delegate to specialized agents.\n"
OUTPUT="${OUTPUT}- Universal roles: researcher, planner, implementer, reviewer, writer, designer\n"
OUTPUT="${OUTPUT}- Roles are specialised via skills at runtime\n\n"

OUTPUT="${OUTPUT}SESSION START CHECKLIST:\n"
OUTPUT="${OUTPUT}- Check .orqa/planning/tasks/ for current tasks\n"
OUTPUT="${OUTPUT}- Check .orqa/planning/epics/ for active epics\n"

if [ -n "$OUTPUT" ]; then
  echo -e "$OUTPUT"
fi

exit 0
