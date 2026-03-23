---
id: TASK-13a4b5c6
type: task
name: "Remove All Projects dropdown — crashes app, org model deprecated"
status: in-progress
description: The All Projects dropdown in the toolbar crashes the app. The multi-project organisation model from the submodule era is no longer appropriate. Organisations will be driven by the git hosting platform in the future. Remove the current implementation.
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Monorepo cleanup — removing stale multi-project infrastructure
acceptance:
  - "All Projects dropdown removed from toolbar"
  - "No crash when interacting with the toolbar"
  - "App compiles and runs"
---
