---
id: TASK-417
title: "Create @orqastudio/types package"
description: "Extract all 13 type files from ui/src/lib/types/ into a standalone @orqastudio/types package. Include shared constants (INVERSE_MAP, SINGLE_REF_FIELDS, ARRAY_REF_FIELDS). Set up GitHub repo, CI, and GitHub Packages publishing."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 5
  complexity: 3
  dependencies: 5
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - New repo orqastudio/orqastudio-types exists with CI + publish workflow
  - All 13 type files from ui/src/lib/types/ are in the package
  - "INVERSE_MAP, SINGLE_REF_FIELDS, ARRAY_REF_FIELDS constants exported"
  - Package builds with zero errors
  - "Published to GitHub Packages as @orqastudio/types"
relationships:
  - target: EPIC-066
    type: delivers
    rationale: Foundation package — everything depends on this
  - target: RES-058
    type: informed-by
    rationale: Research confirmed all type files are fully portable
  - target: SKILL-034
    type: informed-by
  - target: TASK-419
    type: depended-on-by
  - target: TASK-420
    type: depended-on-by
  - target: SKILL-034
    type: informed-by
---

## Scope

Extract from `ui/src/lib/types/`:
- `artifact-graph.ts`, `session.ts`, `message.ts`, `project.ts`, `artifact.ts`
- `nav-tree.ts`, `streaming.ts`, `enforcement.ts`, `lessons.ts`
- `settings.ts`, `setup.ts`, `errors.ts`, `index.ts`

Also move shared constants from integrity-validator `types.ts` into this package.
