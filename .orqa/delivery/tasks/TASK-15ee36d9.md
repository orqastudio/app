---
id: TASK-15ee36d9
type: task
title: "Switch orqa-studio from file: to published package versions"
description: "Update the main app to consume published @orqastudio packages from GitHub Packages instead of file: references to sibling directories."
status: archived
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "ui/package.json uses versioned @orqastudio/* dependencies (no file: references)"
  - "ui/.npmrc configures @orqastudio scope to GitHub Packages registry"
  - npm install succeeds from registry
  - All 218 existing tests pass
  - svelte-check passes (same error count as before)
  - App builds and runs correctly
  - make verify-integrity works with published integrity-validator
relationships:
  - target: EPIC-90cb7349
    type: delivers
    rationale: Main app fully consuming published packages — the loop is closed
  - target: TASK-f336fa1d
    type: depends-on
---

## Scope

Replace in ui/package.json:

- `"@orqastudio/types": "file:../../orqa-types"` → `"@orqastudio/types": "^0.1.0"`
- `"@orqastudio/sdk": "file:../../orqa-sdk"` → `"@orqastudio/sdk": "^0.1.0"`
- Same for eslint-config, test-config, integrity-validator

Add ui/.npmrc:

```text
@orqastudio:registry=https://npm.pkg.github.com
```

Verify everything still works end-to-end.
