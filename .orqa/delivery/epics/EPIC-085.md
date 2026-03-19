---
id: EPIC-085
title: "Schema-driven CLI validator — zero hardcoded types or keys"
description: "Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule."
status: completed
created: 2026-03-18
updated: 2026-03-19
relationships:
  - target: MS-001
    type: fulfils
  - target: TASK-528
    type: delivered-by
  - target: TASK-529
    type: delivered-by
  - target: TASK-530
    type: delivered-by
  - target: TASK-531
    type: delivered-by
  - target: TASK-532
    type: delivered-by
  - target: TASK-533
    type: delivered-by
  - target: TASK-534
    type: delivered-by
  - target: TASK-535
    type: delivered-by
  - target: TASK-536
    type: delivered-by
---

# EPIC-085: Schema-driven CLI validator — zero hardcoded types or keys

Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule.
