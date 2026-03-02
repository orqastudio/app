#!/usr/bin/env bash
# Session start hook — runs on first UserPromptSubmit of a new session

# Only run once per session
GUARD="tmp/.session-started"
if [ -f "$GUARD" ]; then
    exit 0
fi
mkdir -p tmp
touch "$GUARD"

# Check for stashes
STASHES=$(git stash list 2>/dev/null)
if [ -n "$STASHES" ]; then
    echo "WARNING: Git stashes found! Investigate and commit before proceeding:"
    echo "$STASHES"
    echo ""
fi

# Check for stale worktrees (git-linked)
WORKTREES=$(git worktree list 2>/dev/null | grep -v "$(pwd)")
if [ -n "$WORKTREES" ]; then
    echo "WARNING: Non-main worktrees detected! Check if these need cleanup:"
    echo "$WORKTREES"
    echo ""
fi

# Check for orphaned worktree directories
PARENT_DIR=$(dirname "$(pwd)")
ORPHANS=$(find "$PARENT_DIR" -maxdepth 1 -name "forge-*" -type d 2>/dev/null)
if [ -n "$ORPHANS" ]; then
    echo "WARNING: Orphaned worktree directories found (no .git link, likely from prior sessions):"
    echo "$ORPHANS"
    echo ""
    echo "Check for stale processes: ps -ef | grep forge-"
    echo "Then: kill <pids> && rm -rf <directory>"
    echo ""
fi

# Check for session state from previous session
if [ -f "tmp/session-state.md" ]; then
    echo "PREVIOUS SESSION STATE FOUND (tmp/session-state.md):"
    cat "tmp/session-state.md"
    echo ""
    echo "Review the above and continue where the previous session left off."
    echo ""
fi

echo "SESSION START CHECKLIST:"
echo "- Read TODO.md for current tasks"
echo "- Check BLOCKERS.md for known issues"
