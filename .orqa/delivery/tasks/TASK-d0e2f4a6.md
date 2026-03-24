---
id: TASK-d0e2f4a6
type: task
name: "Bring integrations into plugin lifecycle commands"
status: completed
description: Make orqa plugin list, refresh, and status cover integrations (sidecars). Add integrations to the install pipeline. Report integration health.
relationships:
  - target: EPIC-d4a8c1e5
    type: delivers
    rationale: Phase 4 — integration lifecycle
acceptance:
  - "orqa plugin list shows integrations alongside plugins and connectors"
  - "orqa plugin refresh rebuilds integrations (calls their build command)"
  - "orqa plugin status reports integration presence"
  - "integrations/claude-agent-sdk is included in the install pipeline"
  - "TypeScript compiles cleanly"
---
