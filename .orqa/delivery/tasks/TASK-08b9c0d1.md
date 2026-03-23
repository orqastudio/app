---
id: TASK-08b9c0d1
type: task
name: "Phase 2 — Stand up Forgejo instance"
status: todo
description: "Docker Compose setup for Forgejo with Caddy reverse proxy, domain + DNS, GitHub OAuth, push mirror to GitHub, branch protection."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 2 — Forgejo instance
  - target: TASK-07a8b9c0
    type: depends-on
    rationale: Monorepo must be verified before migrating hosting
acceptance:
  - "Forgejo running in Docker with auto-TLS via Caddy"
  - "Domain resolves (git.orqastudio.dev or similar)"
  - "GitHub OAuth login working"
  - "Monorepo pushed to Forgejo"
  - "Push mirror to GitHub configured and working"
  - "main branch protected (PRs required)"
---
