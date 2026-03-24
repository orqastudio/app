---
id: TASK-11e2f3a4
type: task
name: "Phase 5 — Developer tooling (orqa git)"
status: completed
description: "CLI commands for the monorepo + Forgejo workflow: orqa git status/pr, orqa repo audit/protect, update plugin distribution and registry."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 5 — developer tooling
  - target: TASK-10d1e2f3
    type: depends-on
    rationale: Sync bridge should be in place before tooling wraps it
acceptance:
  - "orqa git status shows monorepo-aware component changes"
  - "orqa git pr creates PR on Forgejo"
  - "orqa repo audit checks branch protection and mirror health"
  - "Plugin distribution works from monorepo subdirectories"
  - "Registry entries updated for monorepo structure"
---
