---
id: TASK-152
title: Create schema validation pre-commit hook
description: Pre-commit hook that validates YAML frontmatter of staged .orqa/ markdown files against their directory's schema.json before allowing the commit.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Create or update hook that validates staged .orqa/**/*.md files against schema.json
  - Hook discovers schema via directory (each artifact type dir has a schema.json)
  - Validates required fields, field types, enum values, pattern constraints
  - Validates propertyOrder — frontmatter fields must appear in the order defined by schema
  - Integrates with existing pre-commit pipeline
acceptance:
  - Staged .orqa/ markdown files are validated against their schema.json
  - Missing required fields block the commit with clear error messages
  - Invalid enum values (e.g., wrong status) block the commit
  - Pattern violations (e.g., malformed ID) block the commit
  - Out-of-order frontmatter fields flagged when schema defines propertyOrder
  - Files outside .orqa/ are not affected
  - Hook runs as part of the existing .githooks/pre-commit chain
---
## What

RULE-032 requires all artifacts to validate against their directory's `schema.json`. The existing `.githooks/validate-schema.mjs` script does this but may not be properly integrated into the pre-commit pipeline. Ensure schema validation runs automatically on every commit that touches `.orqa/` files.

## How

1. Review current `.githooks/pre-commit` and `.githooks/validate-schema.mjs` to understand existing validation
2. Ensure the pre-commit hook calls schema validation for all staged `.orqa/**/*.md` files
3. Validation should: find the nearest `schema.json` in the file's directory, parse YAML frontmatter, validate against the schema using ajv
4. Output clear error messages: which file, which field, what's wrong
5. Validate `propertyOrder` if defined in the schema — frontmatter fields must appear in the specified order
6. Also validate body template sections (required headings) if defined in the schema's `bodyTemplate`
6. Update RULE-032 to reference the hook as active enforcement

## Verification

- [ ] Committing a task with missing `description` field is blocked
- [ ] Committing an epic with invalid `status` value is blocked
- [ ] Committing a well-formed artifact passes validation
- [ ] Error messages clearly identify the file, field, and violation
- [ ] Frontmatter field ordering validated against schema propertyOrder
- [ ] Body template sections validated (missing required headings flagged)
- [ ] Non-.orqa files are unaffected
