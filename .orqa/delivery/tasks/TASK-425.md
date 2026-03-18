---
id: TASK-425
title: "Fix CI workflows for independent builds (no file: references)"
description: "CI workflows fail because packages reference each other via file: paths that don't exist in CI. Update each package's CI to either use npm pack from a checkout or publish tier-0 packages first."
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
  - "types CI builds and tests independently (no file: deps)"
  - eslint-config CI builds independently
  - test-config CI builds independently
  - integrity-validator CI builds and tests with types from GitHub Packages
  - SDK CI builds and tests with types from GitHub Packages
  - All CI workflows pass on GitHub Actions
relationships:
  - target: EPIC-066
    type: delivers
    rationale: CI must work for packages to be publishable
  - target: TASK-426
    type: depended-on-by
---
## Scope

### Tier 0 packages (no orqa deps — CI works now)
- `@orqastudio/types` — zero orqa dependencies, CI should pass as-is
- `@orqastudio/eslint-config` — zero orqa dependencies
- `@orqastudio/test-config` — zero orqa dependencies (types are local interfaces)

### Tier 1 packages (depend on types)
- `@orqastudio/integrity-validator` — depends on `@orqastudio/types`
- `@orqastudio/sdk` — depends on `@orqastudio/types`

For CI: replace `"file:../orqa-types"` with the published version from GitHub Packages. CI needs `.npmrc` with `@orqastudio:registry=https://npm.pkg.github.com` and a `NODE_AUTH_TOKEN`.

### CI .npmrc setup
Each repo needs a CI step that creates `.npmrc` pointing to GitHub Packages:
```yaml
- uses: actions/setup-node@v4
  with:
    registry-url: "https://npm.pkg.github.com"
    scope: "@orqastudio"
```
