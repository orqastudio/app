---
id: TASK-087
title: Audit hooks for correctness
description: "Verify all hook scripts in .orqa/process/hooks/ exist, are executable, reference correct paths, and function as intended."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - All hook scripts exist and have correct permissions
  - All paths in hook scripts resolve to existing files
  - Hook trigger events match Tauri/Claude Code event names
relationships:
  - target: EPIC-049
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-088
    type: depended-on-by
  - target: TASK-089
    type: depended-on-by
  - target: TASK-339
    type: depended-on-by
---
## What

Verify all governance hooks are functional and reference correct paths.

## How

1. List all files in `.orqa/process/hooks/`
2. Read each hook script
3. Verify file paths and commands referenced
4. Run hooks in test mode if possible

## Verification

- Each hook script runs without path-related errors
- No hooks reference deprecated paths or commands
