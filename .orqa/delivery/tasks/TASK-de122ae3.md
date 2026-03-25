---
id: TASK-de122ae3
type: task
title: Wire orqa-studio to import from packages + update pre-commit and make verify
description: "Update the main orqa-studio app to import types, SDK, and stores from the extracted packages instead of local files. Replace verify-links.mjs and verify-pipeline-integrity.mjs with @orqastudio/integrity-validator. Update pre-commit hook and make targets."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 3
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "ui/src/lib/types/ replaced with imports from @orqastudio/types"
  - "ui/src/lib/sdk/ replaced with imports from @orqastudio/sdk"
  - "ui/src/lib/stores/ (except navigation) replaced with imports from @orqastudio/sdk"
  - "ui/src/lib/ipc/ replaced with imports from @orqastudio/sdk"
  - "Pre-commit hook uses @orqastudio/integrity-validator instead of verify-links.mjs"
  - "make verify uses @orqastudio/integrity-validator"
  - verify-links.mjs and verify-pipeline-integrity.mjs removed or deprecated
  - All existing tests pass
  - App builds and runs correctly with package imports
relationships:
  - target: EPIC-90cb7349
    type: delivers
    rationale: Integration — wires the main app to use the extracted packages
  - target: TASK-aa1cc420
    type: depends-on
  - target: TASK-b1039dca
    type: depends-on
  - target: TASK-ddb2414f
    type: depends-on
---

## Scope

### Import rewiring
- All `$lib/types/*` imports → `@orqastudio/types`
- All `$lib/sdk/*` imports → `@orqastudio/sdk`
- All `$lib/stores/*` imports (except navigation) → `@orqastudio/sdk`
- All `$lib/ipc/*` imports → `@orqastudio/sdk`
- `$lib/utils/frontmatter` → `@orqastudio/sdk`

### Pre-commit update
- Replace `node tools/verify-links.mjs --staged --check-bidirectional` with `npx @orqastudio/integrity-validator .`
- Replace `node tools/verify-pipeline-integrity.mjs --staged` with same (single tool)

### Make targets
- `make verify` → `npx @orqastudio/integrity-validator .`
- Individual targets (`verify-links`, `verify-integrity`) → delegate to package

### navigationStore
Stays in `ui/src/lib/stores/navigation.svelte.ts` — not extracted.