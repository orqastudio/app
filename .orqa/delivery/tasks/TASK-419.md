---
id: TASK-419
title: Wire types + eslint-config into integrity validator + add tests
description: "Update @orqastudio/integrity-validator to import types from @orqastudio/types, use @orqastudio/eslint-config, and add comprehensive unit tests for all 10 checks."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "integrity-validator imports types from @orqastudio/types (not local)"
  - "eslint-config from @orqastudio/eslint-config is wired in"
  - Unit tests exist for all 10 check functions
  - "Tests cover error cases, edge cases, and empty graphs"
  - CI passes with tests + lint
  - Can run against orqa-studio and produce identical results to app scanner
relationships:
  - target: EPIC-066
    type: delivers
    rationale: Complete the integrity validator as a production-ready package
  - target: RES-057
    type: informed-by
    rationale: Audit identified which checks need parity with Rust scanner
  - target: TASK-417
    type: depends-on
  - target: TASK-418
    type: depends-on
  - target: SKILL-033
    type: grounded-by
  - target: SKILL-034
    type: grounded-by
  - target: TASK-423
    type: depended-on-by
---

## Scope

- Replace local `types.ts` with imports from `@orqastudio/types`
- Add `@orqastudio/eslint-config` as dev dependency, wire into eslint config
- Write tests using vitest with fixture artifacts (small .orqa/ trees)
- Test each check function independently with constructed graph inputs
- Integration test: run against orqa-studio, compare with app results
