---
id: "TASK-05357368"
title: "Document linter-to-standard mapping"
description: "Create a clear mapping between documented coding standards and the linter

  rules/configs that enforce them. Each standard traces to a specific linter

  rule. Each skill describes how to configure the relevant tool.\n"
status: "completed"
created: "2026-03-11"
updated: "2026-03-12"
acceptance:
  - "Every RULE-b49142be standard has a corresponding linter rule or agent discipline note"
  - "lint enforcement entries added and validate against schema"
  - "Skills reference correct linter configs"
relationships:
  - target: "EPIC-4440cdd4"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-2bbc5077"
    type: "depended-on-by"
  - target: "TASK-d6727d2f"
    type: "depended-on-by"
---
## What

Document the full chain from documented standard → linter config → hook trigger
for every coding standard in [RULE-b49142be](RULE-b49142be). This creates traceability and ensures no
standard is undocumented and no linter rule is unexplained.

## How

1. Audit [RULE-b49142be](RULE-b49142be) standards against clippy, ESLint, and svelte-check configs
2. For each standard, document: which linter rule enforces it, how it's configured,
   which skill describes the fix patterns
3. Add `lint` enforcement entries to [RULE-b49142be](RULE-b49142be) for each mapped standard
4. Update `backend-best-practices` and `frontend-best-practices` skills with
   tool configuration guidance

## Verification

- Every [RULE-b49142be](RULE-b49142be) standard has a corresponding linter rule or explicit "agent discipline" note
- `lint` enforcement entries validate against schema
- Skills reference the correct linter configs
