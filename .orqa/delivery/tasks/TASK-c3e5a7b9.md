---
id: TASK-c3e5a7b9
type: task
name: "orqa plugin status command"
status: completed
description: Add orqa plugin status command that reports three-way diff state across all installed plugins. Shows clean, updated, modified, and conflict counts per plugin with file-level detail.
relationships:
  - target: EPIC-d4a8c1e5
    type: delivers
    rationale: Phase 1 — three-way diff infrastructure
  - target: TASK-b2d4f6a8
    type: depends-on
    rationale: Needs three-way diff comparison working
acceptance:
  - "orqa plugin status lists all installed plugins with per-file three-way state"
  - "Summary shows counts: N clean, N plugin-updated, N user-modified, N conflict"
  - "Per-plugin detail shows individual file states"
  - "Exit code is non-zero if conflicts exist"
  - "make check passes"
---

## Scope

### Output format

```
Plugin Status:
  @orqastudio/plugin-software (0.1.4-dev)
    15 files: 14 clean, 1 user-modified
  @orqastudio/plugin-core-framework (0.1.4-dev)
    37 files: 35 clean, 2 plugin-updated
  @orqastudio/claude-code-connector (0.1.0-dev)
    2 files: 2 clean

Summary: 130 files across 11 plugins — 127 clean, 2 plugin-updated, 1 user-modified, 0 conflicts
```

### Key files

- `libs/cli/src/commands/plugin.ts` — new `cmdStatus()` function
- `libs/cli/src/lib/content-lifecycle.ts` — utility to compute three-way state per file
