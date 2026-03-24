---
id: TASK-a1c2d3e4
type: task
title: "Lint rule coverage audit"
description: "Verify all documented coding standards have corresponding linter rules. Identify any standards without mechanical enforcement and any linter rules without documented standards."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "Every standard in .orqa/documentation/development/coding-standards.md is mapped to a linter rule or has a documented exception"
  - "Every active ESLint rule in eslint.config.js is traceable to a documented standard"
  - "Every active clippy lint is traceable to a documented Rust standard"
  - "Gaps are logged as findings with recommended fixes"
  - "Any new linter rules added fix ALL existing violations in the same commit"
relationships:
  - target: EPIC-b2f0399e
    type: delivers
---

## What

Audit the bidirectional mapping between documented coding standards and automated linting rules per RULE-d4b8e3f2 (lint-rule-alignment) and RULE-7f416d7d (tooling-ecosystem).

## How

1. Read `.orqa/documentation/development/coding-standards.md` and extract every stated standard
2. For each standard, find the corresponding linter rule:
   - Rust: check clippy configuration and `Cargo.toml` lint settings
   - TypeScript/Svelte: check `eslint.config.js` and `tsconfig.json`
3. For each active linter rule, verify it maps to a documented standard
4. Produce a gap report: standards without enforcement, rules without documentation

## Verification

1. Gap report exists with zero unresolved gaps (all gaps either fixed or documented as exceptions)
2. `make check` passes after any linter config changes
3. No `eslint-disable` or `#[allow(...)]` without documented justification
