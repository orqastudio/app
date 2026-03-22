---
id: TASK-db0e3e3c
title: Update integrity scanner to validate hex ID format
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-ad41ea9b
    type: depends-on
  - target: TASK-bd4e7250
    type: depended-on-by
---

# TASK-db0e3e3c: Integrity Scanner — Hex ID Validation

## Acceptance Criteria

1. Scanner accepts both old (TYPE-NNN) and new (TYPE-XXXXXXXX) formats during migration
2. After migration complete, scanner warns on old-format IDs
3. Scanner validates hex portion is exactly 8 lowercase hex chars
4. Scanner validates type prefix matches artifact's actual type
5. Scanner checks for ID uniqueness across the full graph
6. Pre-commit hook rejects new artifacts with old-format IDs
