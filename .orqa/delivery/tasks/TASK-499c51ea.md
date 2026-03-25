---
id: TASK-499c51ea
type: task
name: "Phase 2 — Stand up Forgejo instance"
status: completed
description: "Docker Compose setup for local Forgejo instance. Server deployment deferred — Docker for now."
relationships:
  - target: EPIC-2f720d43
    type: delivers
    rationale: Phase 2 — Forgejo instance
  - target: TASK-5fdbf116
    type: depends-on
    rationale: Monorepo must be verified before migrating hosting
  - target: TASK-ab7001d9
    type: depended-on-by
acceptance:
  - "Forgejo running locally via Docker Compose"
  - "Monorepo pushed to Forgejo"
  - "Push mirror to GitHub configured"
  - "main branch protected"
  - "docker compose up starts Forgejo"
---
