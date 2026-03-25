---
id: "TASK-687cae0e"
type: "task"
name: "Phase 4 — Bidirectional contribution bridge"
status: "completed"
description: "Custom webhook service syncing PRs and issues between Forgejo (primary) and GitHub (mirror). Contributors use either platform."
relationships:
  - target: "EPIC-2f720d43"
    type: "delivers"
    rationale: "Phase 4 — bidirectional sync"
  - target: "TASK-ab7001d9"
    type: "depends-on"
    rationale: "CI must be on Forgejo before sync bridge"
acceptance:
  - "PR on GitHub creates corresponding PR on Forgejo"
  - "PR merged on Forgejo auto-closes GitHub PR"
  - "Issue sync bidirectional"
  - "CI status visible on both platforms"
---
