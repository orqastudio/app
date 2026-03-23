---
id: TASK-08b9c0d1
type: task
name: "Phase 2 — Stand up Forgejo instance"
status: done
description: "Docker Compose setup for local Forgejo instance. Server deployment deferred — Docker for now."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 2 — Forgejo instance
  - target: TASK-07a8b9c0
    type: depends-on
    rationale: Monorepo must be verified before migrating hosting
acceptance:
  - "Forgejo running locally via Docker Compose"
  - "Monorepo pushed to Forgejo"
  - "Push mirror to GitHub configured"
  - "main branch protected"
  - "docker compose up starts Forgejo"
---
