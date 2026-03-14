---
id: TASK-428
title: "Switch orqa-studio from file: to published package versions"
description: "Update the main app to consume published @orqastudio packages from GitHub Packages instead of file: references to sibling directories."
status: done
priority: P1
created: "2026-03-14"
updated: "2026-03-14"
epic: EPIC-066
depends-on:
  - TASK-427
assignee: null
skills:
  - SKILL-030
  - SKILL-034
acceptance:
  - "ui/package.json uses versioned @orqastudio/* dependencies (no file: references)"
  - "ui/.npmrc configures @orqastudio scope to GitHub Packages registry"
  - "npm install succeeds from registry"
  - "All 218 existing tests pass"
  - "svelte-check passes (same error count as before)"
  - "App builds and runs correctly"
  - "make verify-integrity works with published integrity-validator"
relationships:
  - target: EPIC-066
    type: delivers
    rationale: "Main app fully consuming published packages — the loop is closed"
---

## Scope

Replace in ui/package.json:
- `"@orqastudio/types": "file:../../orqa-types"` → `"@orqastudio/types": "^0.1.0"`
- `"@orqastudio/sdk": "file:../../orqa-sdk"` → `"@orqastudio/sdk": "^0.1.0"`
- Same for eslint-config, test-config, integrity-validator

Add ui/.npmrc:
```
@orqastudio:registry=https://npm.pkg.github.com
```

Verify everything still works end-to-end.
