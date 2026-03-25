---
id: "TASK-c8bc9837"
type: "task"
title: "Bulk ID migration script — sequential to hex"
status: "captured"
created: 2026-03-19T00:00:00.000Z
updated: 2026-03-19T00:00:00.000Z
relationships:
  - target: "EPIC-d1d42012"
    type: "delivers"
  - target: "TASK-83ba8cae"
    type: "depends-on"
---

# TASK-c8bc9837: Bulk ID Migration Script

## Acceptance Criteria

1. Script reads all artifacts, generates hex IDs for each
2. Updates `id:` field in each artifact's frontmatter
3. Updates all `target:` references in relationship arrays across the entire graph
4. Updates all body text references (prose, links, tables)
5. Updates plugin manifests (`orqa-plugin.json` skill ID entries)
6. Produces a migration manifest mapping old → new IDs for audit
7. `orqa enforce` passes with 0 errors after migration