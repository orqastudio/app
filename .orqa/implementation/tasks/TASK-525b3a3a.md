---
id: "TASK-525b3a3a"
type: "task"
title: "Initial publish of tier-0 packages to GitHub Packages"
description: "Create GitHub releases for types, eslint-config, and test-config to trigger the publish workflows. These have no orqa dependencies so they can be published first."
status: archived
priority: "P1"
scoring:
  impact: 4
  urgency: 5
  complexity: 1
  dependencies: 4
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
assignee: null
acceptance:
  - "@orqastudio/types v0.1.0 published to GitHub Packages"
  - "@orqastudio/eslint-config v0.1.0 published to GitHub Packages"
  - "@orqastudio/test-config v0.1.0 published to GitHub Packages"
  - "All three installable via npm install @orqastudio/types (with .npmrc configured)"
relationships:
  - target: "EPIC-90cb7349"
    type: "delivers"
    rationale: "First packages published — unblocks tier-1 publishing"
  - target: "TASK-543509da"
    type: "depends-on"
---

## Scope

For each tier-0 package:

1. Verify CI passes on GitHub Actions
2. Create a GitHub release tagged `v0.1.0`
3. Verify the publish workflow runs and succeeds
4. Test installation: `npm install @orqastudio/types` with `.npmrc` configured
