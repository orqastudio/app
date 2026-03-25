---
id: "TASK-dbbb669b"
type: "task"
title: "Schema validation compliance audit"
description: "Verify all .orqa/ artifacts have valid YAML frontmatter per their directory schema.json. Fix any validation failures."
status: "ready"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "Every .orqa/**/*.md artifact is validated against its directory schema.json"
  - "All validation errors are listed with artifact ID, field, and error description"
  - "All fixable validation errors are corrected"
  - "graph_validate returns clean after fixes"
  - "Pre-commit hook schema validation passes on all staged files"
relationships:
  - target: "EPIC-e24086ed"
    type: "delivers"
---

## What

Audit all governance and delivery artifacts for schema compliance per RULE-23699df2 (schema-validation). The artifact graph depends on valid frontmatter — invalid artifacts silently break queries, relationships, and integrity checks.

## How

1. Run `graph_validate()` to get the current integrity report
2. Run the pre-commit schema validator (`.githooks/validate-schema.mjs`) against ALL `.orqa/**/*.md` files, not just staged ones
3. Categorise failures:
   - Missing required fields
   - Invalid enum values (wrong status, wrong type)
   - Type mismatches (string where array expected, etc.)
   - Extra fields not in schema
4. Fix each failure, verifying the fix doesn't break relationships
5. Re-run validation to confirm clean

## Verification

1. `graph_validate()` returns no errors
2. `.githooks/validate-schema.mjs` passes on all `.orqa/**/*.md` files
3. No artifacts silently dropped from graph queries due to invalid frontmatter
