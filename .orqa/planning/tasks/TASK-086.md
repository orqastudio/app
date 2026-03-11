---
id: TASK-086
title: Audit all rules for enforcement accuracy
description: Verify every rule in .orqa/governance/rules/ has accurate enforcement mechanisms, valid cross-references, current code patterns in FORBIDDEN sections, and no stale content.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Read every rule in .orqa/governance/rules/
  - Verify Related Rules cross-references point to existing rules
  - Verify code patterns in FORBIDDEN sections match reality
  - Check enforcement mechanisms are still applicable
  - Identify rules that need updating for codebase evolution
  - Check rule status field (active/inactive) is accurate
acceptance:
  - All Related Rules references point to existing rule files
  - No rules describe enforcement of patterns that no longer exist
  - All FORBIDDEN code examples reflect actual anti-patterns
  - Rule statuses accurately reflect enforcement state
---
## What

Systematic audit of all governance rules to ensure enforcement accuracy and internal consistency.

## How

1. List all rule files in `.orqa/governance/rules/`
2. For each rule, verify cross-references, code patterns, and enforcement mechanisms
3. Check FORBIDDEN sections against actual codebase for accuracy
4. Fix stale content

## Verification

- `grep -r "RULE-" .orqa/governance/rules/` cross-references all resolve
- No rules reference non-existent files, commands, or patterns
