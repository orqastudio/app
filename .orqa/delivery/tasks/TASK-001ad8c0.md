---
id: TASK-001ad8c0
type: task
name: "Phase 5 — Developer tooling (orqa git)"
status: archived
description: "CLI commands for the monorepo + Forgejo workflow: orqa git status/pr, orqa repo audit/protect, update plugin distribution and registry."
relationships:
  - target: EPIC-2f720d43
    type: delivers
    rationale: Phase 5 — developer tooling
  - target: TASK-687cae0e
    type: depends-on
    rationale: Sync bridge should be in place before tooling wraps it
acceptance:
  - "orqa git status shows monorepo-aware component changes"
  - "orqa git pr creates PR on Forgejo"
  - "orqa repo audit checks branch protection and mirror health"
  - "Plugin distribution works from monorepo subdirectories"
  - "Registry entries updated for monorepo structure"
---
