---
id: TASK-09c0d1e2
type: task
name: "Phase 3 — Migrate CI to Forgejo Actions"
status: todo
description: "Set up Forgejo runner, migrate publish and check workflows, post CI status to GitHub mirror."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 3 — CI migration
  - target: TASK-08b9c0d1
    type: depends-on
    rationale: Forgejo must be running
acceptance:
  - "Forgejo runner registered and operational"
  - "PR check workflow runs make check on every PR"
  - "Publish workflow triggers on tags"
  - "CI status posted to GitHub mirror PRs"
---
