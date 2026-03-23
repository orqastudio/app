---
id: TASK-14b5c6d7
type: task
name: "Status bar daemon connectivity indicator"
status: todo
description: Add periodic health check against the daemon endpoint. Show connected/disconnected/degraded in the status bar based on actual daemon reachability.
relationships:
  - target: IDEA-e3a5b7c9
    type: implements
    rationale: Implements the daemon connectivity idea
acceptance:
  - "Status bar shows daemon connectivity state (connected/disconnected/degraded)"
  - "Periodic health check (every 10s) against daemon /health endpoint"
  - "Shows artifact count when connected"
  - "Shows clear warning when disconnected"
  - "App compiles and runs"
---
