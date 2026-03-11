---
id: TASK-087
title: Audit hooks for correctness
description: Verify all hook scripts in .orqa/governance/hooks/ exist, are executable, reference correct paths, and function as intended.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Read every hook script in .orqa/governance/hooks/
  - Verify paths referenced in scripts are correct
  - Verify trigger events are still valid
  - Test hook execution
acceptance:
  - All hook scripts exist and have correct permissions
  - All paths in hook scripts resolve to existing files
  - Hook trigger events match Tauri/Claude Code event names
---
## What

Verify all governance hooks are functional and reference correct paths.

## How

1. List all files in `.orqa/governance/hooks/`
2. Read each hook script
3. Verify file paths and commands referenced
4. Run hooks in test mode if possible

## Verification

- Each hook script runs without path-related errors
- No hooks reference deprecated paths or commands
