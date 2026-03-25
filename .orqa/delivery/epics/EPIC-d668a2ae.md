---
id: EPIC-d668a2ae
type: epic
title: "Schema-driven CLI validator — zero hardcoded types or keys"
description: "Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule."
status: completed
created: 2026-03-18
updated: 2026-03-19
relationships:
  - target: TASK-36edd30e
    type: delivered-by
  - target: TASK-5dd40596
    type: delivered-by
  - target: TASK-68631471
    type: delivered-by
  - target: TASK-0978e3c9
    type: delivered-by
  - target: TASK-8615562a
    type: delivered-by
  - target: TASK-15726960
    type: delivered-by
  - target: TASK-8159418c
    type: delivered-by
  - target: TASK-3801532e
    type: delivered-by
  - target: TASK-87062c4e
    type: delivered-by
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---

# EPIC-d668a2ae: Schema-driven CLI validator — zero hardcoded types or keys

Rewrote the CLI validator to enforce the schema generically. Deleted 6 domain-specific checks, added 3 schema-driven replacements. Both forward and inverse constraint checking. Multi-directory scanning. Plugin relationship loading with constraint extension. Removed dead integrity-validator submodule.