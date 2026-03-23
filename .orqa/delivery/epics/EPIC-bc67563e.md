---
id: EPIC-bc67563e
type: epic
title: "Schema-driven CLI validator — zero hardcoded types or keys"
description: "Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule."
status: completed
created: 2026-03-18
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: fulfils
  - target: TASK-67ba69f7
    type: delivered-by
  - target: TASK-c8543fd5
    type: delivered-by
  - target: TASK-1f14edc1
    type: delivered-by
  - target: TASK-d1020322
    type: delivered-by
  - target: TASK-3cd513c6
    type: delivered-by
  - target: TASK-53852eeb
    type: delivered-by
  - target: TASK-471a4782
    type: delivered-by
  - target: TASK-584179f7
    type: delivered-by
  - target: TASK-682afea5
    type: delivered-by
---

# EPIC-bc67563e: Schema-driven CLI validator — zero hardcoded types or keys

Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule.