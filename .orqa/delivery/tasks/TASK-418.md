---
id: TASK-418
title: "Create @orqastudio/eslint-config package"
description: "Extract shared ESLint and TypeScript config from orqa-studio into a standalone @orqastudio/eslint-config package. All orqastudio repos and plugins use this for consistent code standards."
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - New repo orqastudio/orqastudio-eslint-config exists with CI + publish workflow
  - Exports flat ESLint config compatible with eslint.config.js
  - "Includes TypeScript strict rules, no-any, ban-ts-comment"
  - tsconfig base exported for extends
  - Published to GitHub Packages
  - orqa-studio main repo updated to use the package
relationships:
  - target: EPIC-066
    type: delivers
    rationale: Shared code standards for all repos and plugins
  - target: TASK-419
    type: depended-on-by
---

## Scope

Extract from orqa-studio:
- `eslint.config.js` rules → shareable flat config
- `tsconfig.json` strict settings → base tsconfig for extends
- Any shared prettier config if applicable
