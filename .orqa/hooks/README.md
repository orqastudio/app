# Hooks

Hooks are automated actions that run in response to lifecycle events. They enforce process compliance at key moments — session start, pre-commit, and task completion.

## Active Hooks

- **Session start**: Checks for stale stashes, orphaned worktrees, and unresolved session state
- **Pre-commit reminder**: Verifies checklist compliance and prompts for session state updates

## How Hooks Work

Hooks are configured in `.claude/settings.json` and triggered by specific events. They run shell scripts that perform checks and return feedback. If a hook fails, it blocks the action and reports what needs to be fixed.
