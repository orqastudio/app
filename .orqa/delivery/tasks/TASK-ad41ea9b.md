---
id: TASK-ad41ea9b
title: Bulk ID migration script — sequential to hex
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: EPIC-4cedf7bc
    type: delivers
  - target: TASK-0b703fd2
    type: depends-on
  - target: TASK-db0e3e3c
    type: depended-on-by
---

# TASK-ad41ea9b: Bulk ID Migration Script

## Acceptance Criteria

1. Script reads all artifacts, generates hex IDs for each
2. Updates `id:` field in each artifact's frontmatter
3. Updates all `target:` references in relationship arrays across the entire graph
4. Updates all body text references (prose, links, tables)
5. Updates plugin manifests (`orqa-plugin.json` skill ID entries)
6. Produces a migration manifest mapping old → new IDs for audit
7. `orqa enforce` passes with 0 errors after migration
