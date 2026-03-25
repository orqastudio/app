---
id: TASK-88e72cc1
type: task
title: Audit all rules for enforcement accuracy
description: "Verify every rule in .orqa/process/rules/ has accurate enforcement mechanisms, valid cross-references, current code patterns in FORBIDDEN sections, and no stale content."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - All Related Rules references point to existing rule files
  - No rules describe enforcement of patterns that no longer exist
  - All FORBIDDEN code examples reflect actual anti-patterns
  - Rule statuses accurately reflect enforcement state
relationships:
  - target: EPIC-5aa11e2f
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-3109164e
    type: depended-on-by
  - target: TASK-f2314ba0
    type: depended-on-by
  - target: TASK-dfa29194
    type: depended-on-by
  - target: TASK-698afd4c
    type: depended-on-by
---
## What

Systematic audit of all governance rules to ensure enforcement accuracy and internal consistency.

## How

1. List all rule files in `.orqa/process/rules/`
2. For each rule, verify cross-references, code patterns, and enforcement mechanisms
3. Check FORBIDDEN sections against actual codebase for accuracy
4. Fix stale content

## Verification

- `grep -r "RULE-" .orqa/process/rules/` cross-references all resolve
- No rules reference non-existent files, commands, or patterns