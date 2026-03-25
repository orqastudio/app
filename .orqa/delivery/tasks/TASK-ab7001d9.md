---
id: TASK-ab7001d9
type: task
name: "Phase 3 — Migrate CI to Forgejo Actions"
status: completed
description: "Set up Forgejo runner, migrate publish and check workflows, post CI status to GitHub mirror."
relationships:
  - target: EPIC-2f720d43
    type: delivers
    rationale: Phase 3 — CI migration
  - target: TASK-499c51ea
    type: depends-on
    rationale: Forgejo must be running
  - target: TASK-687cae0e
    type: depended-on-by
acceptance:
  - "Forgejo runner registered and operational"
  - "PR check workflow runs make check on every PR"
  - "Publish workflow triggers on tags"
  - "CI status posted to GitHub mirror PRs"
---
