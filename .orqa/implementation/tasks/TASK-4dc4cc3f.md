---
id: "TASK-4dc4cc3f"
type: "task"
title: "Update integrity scanner to validate hex ID format"
status: captured
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-d1d42012"
    type: "delivers"
  - target: "TASK-c8bc9837"
    type: "depends-on"
---

# TASK-4dc4cc3f: Integrity Scanner — Hex ID Validation

## Acceptance Criteria

1. Scanner accepts both old (TYPE-NNN) and new (TYPE-XXXXXXXX) formats during migration
2. After migration complete, scanner warns on old-format IDs
3. Scanner validates hex portion is exactly 8 lowercase hex chars
4. Scanner validates type prefix matches artifact's actual type
5. Scanner checks for ID uniqueness across the full graph
6. Pre-commit hook rejects new artifacts with old-format IDs
