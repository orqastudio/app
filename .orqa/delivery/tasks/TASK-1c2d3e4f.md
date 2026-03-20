---
id: TASK-1c2d3e4f
type: task
title: "Update CLI: orqa validate, orqa graph, ID generation"
description: "Update the orqa CLI tool to recognise 'knowledge' as an artifact type, scan knowledge/ directories, generate KNOW- prefixed IDs, and validate KNOW- references."
status: ready
created: 2026-03-20
updated: 2026-03-20
acceptance:
  - orqa validate recognises knowledge/ directory and KNOW- prefix
  - orqa graph lists knowledge artifacts under 'knowledge' type
  - ID generation produces KNOW-XXXXXXXX for new knowledge artifacts
  - orqa validate reports errors for any remaining SKILL- references
  - All existing make targets still work after CLI update
relationships:
  - target: EPIC-663d52ac
    type: delivers
  - target: TASK-a1b2c3d4
    type: depends-on
  - target: TASK-c9d0e1f2
    type: depends-on
---

## What

Update the `libs/cli` package and the `orqa` CLI binary to:
1. Scan `knowledge/` directories instead of (or in addition to, during transition) `skills/`
2. Generate `KNOW-` prefixed IDs when creating new knowledge artifacts
3. Validate that `KNOW-` prefixed IDs match the `knowledge` type
4. Report `SKILL-` references as validation errors (post-migration)
5. Update `orqa graph` output to display `knowledge` type nodes correctly

## How

Search `libs/cli/` for:
- Directory scanner constants: `"skills"` glob patterns → `"knowledge"`
- ID prefix registry: `"SKILL-"` → `"KNOW-"`
- Type-to-prefix mapping: `skill → knowledge`
- Help text and error messages referencing "skill"

Update `plugins/cli/` if it provides additional CLI commands related to skills.

Rebuild CLI after changes and run `orqa verify` to confirm the installation.

## Verification

1. `orqa graph --type knowledge` lists all knowledge artifacts
2. Creating a new knowledge artifact generates a `KNOW-` prefixed ID
3. `orqa validate` flags `SKILL-` references as errors
4. `make check` passes (no regressions in CLI tests)
