---
id: TASK-155
title: Enforce frontmatter field ordering at creation time via schema validation
description: Add propertyOrder enforcement to the schema validation hook and agent artifact creation process so frontmatter fields are written in the correct order from the start.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on:
  - TASK-152
scope:
  - Add propertyOrder validation to the schema validation hook (TASK-152)
  - Ensure agents creating artifacts write fields in schema-defined order
  - Fix existing rule files that are out of order as a one-time cleanup
  - Do not add frontend reformatting — enforce at creation and commit time
acceptance:
  - Schema validation hook checks field ordering against propertyOrder
  - Out-of-order fields produce a warning or error at commit time
  - Agents and skills that create artifacts document the correct field order
  - Existing rule files fixed as a one-time batch
---
## What

Most rule files don't follow the `propertyOrder` from their `schema.json`. Rather than reformatting on the frontend, enforce correct ordering at the point of creation — in the schema validation hook and in agent/skill instructions for creating artifacts.

## How

1. Extend the schema validation hook (TASK-152) to check `propertyOrder` if defined in the schema
2. When frontmatter fields are out of order, produce a clear warning showing the expected order
3. Update artifact creation skills/instructions to emphasize writing fields in schema order
4. One-time batch fix: reorder existing rule files to match schema propertyOrder
5. The frontend should NOT reorder — what's on disk is what's displayed

## Verification

- [ ] Schema validation hook checks propertyOrder when defined
- [ ] Committing a file with out-of-order fields produces a warning
- [ ] Existing rule files fixed to match propertyOrder
- [ ] No frontend reformatting logic added
