---
id: TASK-c4718ed7
type: task
title: "Fix pre-commit hook extension and integer expression bug"
description: "Fix two bugs in the plugin pre-commit hook: (1) plugins/githooks/hooks/pre-commit.sh has .sh extension preventing Git discovery, (2) line 57 has integer expression expected error when stdin buffer contains multi-line input."
status: captured
priority: P1
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "plugins/githooks/hooks/pre-commit has no file extension (Git hooks via core.hooksPath require extensionless names)"
  - "The integer comparison on line 57 (or equivalent) handles multi-line stdin correctly"
  - "Pre-commit hook fires correctly in projects that use the githooks plugin"
  - "Pre-commit hook correctly validates staged artifacts"
relationships:
  - target: EPIC-5ab0265a
    type: delivers
    rationale: "Task delivers work to the deduplication epic — pre-commit hook infrastructure"
---

## What

Two bugs discovered during the SoT audit:

1. **Extension bug**: `plugins/githooks/hooks/pre-commit.sh` has a `.sh` extension. Git hooks discovered via `core.hooksPath` must not have extensions. This means the pre-commit hook is likely NOT firing in projects that install the githooks plugin.

2. **Integer expression bug**: Pre-commit hook output shows `line 57: [: ... : integer expression expected`. The `[` test expression is receiving multi-line input instead of a single integer, likely from piped stdin or a command substitution that captures more than expected.

## How

1. Rename `plugins/githooks/hooks/pre-commit.sh` to `plugins/githooks/hooks/pre-commit`
2. Fix the integer comparison: buffer stdin before parsing, use `wc -l` or similar to get a clean integer
3. Test in a fresh project that uses the githooks plugin
4. Update any references to the old filename

## Files

- `plugins/githooks/hooks/pre-commit.sh` — rename and fix
- `plugins/githooks/orqa-plugin.json` — update if it references the hook by name