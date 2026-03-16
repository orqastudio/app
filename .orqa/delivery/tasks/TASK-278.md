---
id: TASK-278
title: Make relationship fields required in schemas
description: "Move relationships, maturity, and category from optional to required in all governance schemas. Update orchestrator prompt with relationship guidance."
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: null
docs: []
acceptance:
  - "relationships is in required array for lessons, decisions, rules, skills schemas"
  - maturity is in required array for lesson schema
  - category is in required array for skill schema
  - Pre-commit hook enforces on all new/modified artifacts
  - Orchestrator prompt updated with guidance for including relationships when creating artifacts
  - All existing artifacts pass validation against updated schemas
rule-overrides: []
relationships:
  - target: EPIC-058
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-277
    type: depends-on
  - target: TASK-279
    type: depended-on-by
  - target: TASK-348
    type: depended-on-by
---

## What

Now that all ~150 artifacts have been backfilled, make the new fields required. From this point forward, no artifact can be created without relationships.

## How

1. Update all four governance schema.json files — move new fields into `required`
2. Run full validation against all existing artifacts to confirm none fail
3. Update orchestrator prompt (CLAUDE.md / orchestrator.md) with relationship guidance
4. Update relevant skills (orqa-schema-compliance, governance-maintenance)

## Verification

- `make check` passes (pre-commit hook validates all staged artifacts)
- Create a test artifact without relationships — validation rejects it
- Create a test artifact with relationships — validation passes
