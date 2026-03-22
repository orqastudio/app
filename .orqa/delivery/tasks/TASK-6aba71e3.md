---
id: TASK-6aba71e3
title: Add status validation to plugin graph-guardian
description: "Extend graph-guardian.mjs with a PostToolUse hook that validates the status field whenever a .orqa/ artifact is written. If the status value is not in the valid enum for that artifact type, a warning is added to additionalContext listing the valid values."
status: ready
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - graph-guardian.mjs PostToolUse hook validates status field when .orqa/ artifacts are written
  - Invalid statuses produce a warning in additionalContext
  - Warning includes the valid values from the project config
relationships:
  - target: EPIC-9fbc17c0
    type: delivers
  - target: TASK-8024efcb
    type: depends-on
---
## What

Extend the graph-guardian plugin's PostToolUse hook to catch status validation errors at write time — before a commit reaches the pre-commit hook. When an agent writes a `.orqa/` artifact with an invalid status, the hook immediately surfaces a warning in `additionalContext` so the agent can self-correct in the same turn. The valid values are read from the project config (or schema.json) to stay in sync with the Rust-side validation added in TASK-8024efcb.

## How

1. In the `PostToolUse` hook handler in `graph-guardian.mjs`, after any `Write` or `Edit` tool call that targets a `.orqa/**/*.md` file:
   a. Parse the YAML frontmatter from the written content.
   b. Extract the `status` field.
   c. Determine the artifact type from the file path (e.g. `tasks/` → task, `epics/` → epic).
   d. Load the valid status enum for that type from the directory's `schema.json`.
   e. If `status` is not in the valid enum, add a warning to `additionalContext`:
      ```
      ⚠ Invalid status "{actual}" for artifact type {type}.
      Valid values: {comma-separated list}.
      ```
2. Load `schema.json` files lazily (cache after first read per artifact type) to avoid repeated disk reads.
3. If the `status` field is absent or the file cannot be parsed, skip validation silently (the schema pre-commit hook will catch missing required fields separately).
4. No changes to `PreToolUse` — this is a PostToolUse concern.

## Verification

- Hook test: writing a task artifact with status `"in-progress"` produces a warning listing valid task statuses.
- Hook test: writing a task artifact with status `"active"` produces no warning.
- Hook test: writing a non-.orqa/ file does not trigger status validation.
- Hook test: writing an artifact with no status field does not produce an error (fails silently).
- `make check` passes (if applicable to plugin JS) or manual lint of graph-guardian.mjs shows no issues.

## Lessons

(To be filled during/after implementation)
