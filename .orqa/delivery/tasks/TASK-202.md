---
id: TASK-202
title: Document linter-to-standard mapping
description: |
  Create a clear mapping between documented coding standards and the linter
  rules/configs that enforce them. Each standard traces to a specific linter
  rule. Each skill describes how to configure the relevant tool.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on: []
---

## What

Document the full chain from documented standard → linter config → hook trigger
for every coding standard in [RULE-006](RULE-006). This creates traceability and ensures no
standard is undocumented and no linter rule is unexplained.

## How

1. Audit [RULE-006](RULE-006) standards against clippy, ESLint, and svelte-check configs
2. For each standard, document: which linter rule enforces it, how it's configured,
   which skill describes the fix patterns
3. Add `lint` enforcement entries to [RULE-006](RULE-006) for each mapped standard
4. Update `backend-best-practices` and `frontend-best-practices` skills with
   tool configuration guidance

## Verification

- Every [RULE-006](RULE-006) standard has a corresponding linter rule or explicit "agent discipline" note
- `lint` enforcement entries validate against schema
- Skills reference the correct linter configs
