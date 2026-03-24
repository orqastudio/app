---
id: TASK-10d1e2f3
type: task
name: "Phase 4 — Bidirectional contribution bridge"
status: completed
description: "Custom webhook service syncing PRs and issues between Forgejo (primary) and GitHub (mirror). Contributors use either platform."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 4 — bidirectional sync
  - target: TASK-09c0d1e2
    type: depends-on
    rationale: CI must be on Forgejo before sync bridge
acceptance:
  - "PR on GitHub creates corresponding PR on Forgejo"
  - "PR merged on Forgejo auto-closes GitHub PR"
  - "Issue sync bidirectional"
  - "CI status visible on both platforms"
---
