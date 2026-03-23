---
id: TASK-b8d0e2f4
type: task
name: "Add symlink and aggregation capabilities to plugin manifest schema"
status: done
description: Extend PluginManifest type to support symlink declarations and service aggregation as universal capabilities. Add framework handling in content-lifecycle.
relationships:
  - target: EPIC-d4a8c1e5
    type: delivers
    rationale: Phase 3 — universal plugin capabilities
acceptance:
  - "PluginManifest supports provides.symlinks array for symlink declarations"
  - "PluginManifest supports provides.aggregates for cross-plugin service collection"
  - "PluginManifest supports provides.rootFiles for project root file management"
  - "Framework processes these declarations during orqa plugin refresh"
  - "TypeScript compiles cleanly"
---
