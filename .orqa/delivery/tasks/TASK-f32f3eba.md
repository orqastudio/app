---
id: TASK-f32f3eba
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
  - target: EPIC-4a7aeacb
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-1ac4d16f
    type: depended-on-by
  - target: TASK-77b6e5bd
    type: depended-on-by
  - target: TASK-4a8c3c6a
    type: depended-on-by
  - target: TASK-ec136ce9
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
