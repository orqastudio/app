---
id: TASK-82add1d1
title: "Switch tier-1 packages from file: to published deps + publish"
description: "Update integrity-validator and SDK to depend on published @orqastudio/types instead of file: references. Fix CI, verify, then publish both."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 5
  complexity: 2
  dependencies: 4
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "integrity-validator package.json uses @orqastudio/types version (not file:)"
  - "integrity-validator package.json uses @orqastudio/eslint-config version (not file:)"
  - "integrity-validator package.json uses @orqastudio/test-config version (not file:)"
  - "SDK package.json uses @orqastudio/types version (not file:)"
  - Both CI workflows pass on GitHub Actions
  - "@orqastudio/integrity-validator v0.1.0 published to GitHub Packages"
  - "@orqastudio/sdk v0.1.0 published to GitHub Packages"
relationships:
  - target: EPIC-a210c825
    type: delivers
    rationale: All packages published and consuming real versions
  - target: TASK-c3abf5c1
    type: depends-on
  - target: TASK-9593e951
    type: depended-on-by
---

## Scope

### integrity-validator
Replace in package.json:
- `"@orqastudio/types": "file:../orqa-types"` → `"@orqastudio/types": "^0.1.0"`
- `"@orqastudio/eslint-config": "file:../orqa-eslint-config"` → `"@orqastudio/eslint-config": "^0.1.0"`
- `"@orqastudio/test-config": "file:../orqa-test-config"` → `"@orqastudio/test-config": "^0.1.0"`

### SDK
Replace in package.json:
- `"@orqastudio/types": "file:../orqa-types"` → `"@orqastudio/types": "^0.1.0"`
- Same for eslint-config and test-config if present

### For both
- Add `.npmrc` to repo: `@orqastudio:registry=https://npm.pkg.github.com`
- Verify `npm install` works from registry
- Verify CI passes
- Create v0.1.0 release to trigger publish
