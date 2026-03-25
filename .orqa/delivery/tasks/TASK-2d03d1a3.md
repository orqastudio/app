---
id: TASK-2d03d1a3
type: task
name: "Add symlink and aggregation capabilities to plugin manifest schema"
status: completed
description: Extend PluginManifest type to support symlink declarations and service aggregation as universal capabilities. Add framework handling in content-lifecycle.
relationships:
  - target: EPIC-8b01ee51
    type: delivers
    rationale: Phase 3 — universal plugin capabilities
  - target: TASK-d39e416a
    type: depended-on-by
acceptance:
  - "PluginManifest supports provides.symlinks array for symlink declarations"
  - "PluginManifest supports provides.aggregates for cross-plugin service collection"
  - "PluginManifest supports provides.rootFiles for project root file management"
  - "Framework processes these declarations during orqa plugin refresh"
  - "TypeScript compiles cleanly"
---
