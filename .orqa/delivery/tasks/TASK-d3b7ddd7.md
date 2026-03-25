---
id: TASK-d3b7ddd7
type: task
name: "Status bar daemon connectivity indicator"
status: captured
description: Add periodic health check against the daemon endpoint. Show connected/disconnected/degraded in the status bar based on actual daemon reachability.
relationships:
  - target: EPIC-9e3d320b
    type: delivers
    rationale: Delivers daemon connectivity monitoring for the port allocation epic
acceptance:
  - "Status bar shows daemon connectivity state (connected/disconnected/degraded)"
  - "Periodic health check (every 10s) against daemon /health endpoint"
  - "Shows artifact count when connected"
  - "Shows clear warning when disconnected"
  - "App compiles and runs"
---
