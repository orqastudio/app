---
id: TASK-c6dd7927
type: task
title: Clear stale .claude/ symlinks and session state
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-5edafb59
    type: delivers
  - target: TASK-60598b3d
    type: depended-on-by
---

# TASK-c6dd7927: Clear Stale .claude/ Symlinks and Session State

## Acceptance Criteria

1. All old .claude/ symlinks removed from every repo in the dev environment (app, libs, plugins, tools)
2. Stale session state files cleaned (tmp/, .claude/session-*, any leftover context dumps)
3. Old CLAUDE.md files that were symlinked are identified and archived or removed
4. Verify no process is still reading from old .claude/ paths
5. Document what was removed for rollback if needed
