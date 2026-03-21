---
id: "TASK-354c88f2"
title: "Implement rule-overrides in pre-commit hook"
description: "Pre-commit hook checks if the commit references a task with rule-overrides that suspend a failing validation rule. If so, warn but don't block."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: null
docs: []
acceptance:
  - "Pre-commit hook reads rule-overrides from referenced task/epic"
  - "Suspended validation rules produce warnings instead of blocking"
  - "Non-suspended validation rules still block on failure"
  - "Clear output distinguishing warnings (suspended) from errors (blocking)"
rule-overrides: []
relationships:
  - target: "EPIC-ca7b398b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-1ec1a07c"
    type: "depended-on-by"
  - target: "TASK-508cf6cd"
    type: "depended-on-by"
---
## What

Extend the pre-commit hook to be aware of scoped rule suspensions. When a validation fails for a rule that is suspended on the active task/epic, emit a warning instead of blocking the commit.

## How

1. Determine active task from commit message or `tmp/session-state.md`
2. Read task's `rule-overrides` (fall back to epic's)
3. When schema validation or other rule-based checks fail, check if the failing rule is suspended
4. Suspended: print warning with reason, continue
5. Not suspended: block as normal

## Verification

- Stage an artifact that would fail [RULE-a764b2ae](RULE-a764b2ae) validation
- Commit with a message referencing a task that suspends [RULE-a764b2ae](RULE-a764b2ae) — commit succeeds with warning
- Commit without task reference — commit blocked as normal
