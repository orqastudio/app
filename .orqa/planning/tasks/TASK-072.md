---
id: TASK-072
title: Backfill existing artifacts to match body templates
description: "Audit all existing planning and governance artifacts and add missing required body sections so they pass the new body template linting."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-071
assignee: orchestrator
skills:
  - orqa-governance
scope:
  - .orqa/planning/
  - .orqa/governance/
acceptance:
  - "All artifacts pass body template linting via pre-commit hook"
  - "No empty placeholder sections — each section has meaningful content or a brief note"
  - "Existing content preserved and reorganised into correct sections where needed"
---
## What

Existing artifacts were created before body templates existed. Many are missing required sections or have content that doesn't follow the template structure.

## How

Run the body template linter against all artifacts. For each failure, add the missing section with appropriate content. Where existing content covers the topic but under a different heading, reorganise it.

## Verification

- `git add . && node .githooks/validate-schema.mjs` passes with zero body template errors
- Spot-check 5 artifacts per type to verify sections have real content
