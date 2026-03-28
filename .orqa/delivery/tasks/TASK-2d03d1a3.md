---
id: "TASK-2d03d1a3"
type: "task"
title: "Add symlink and aggregation capabilities to plugin manifest schema"
status: archived
description: "Extend PluginManifest type to support symlink declarations and service aggregation as universal capabilities. Add framework handling in content-lifecycle."
relationships:
  - target: "EPIC-8b01ee51"
    type: "delivers"
    rationale: "Phase 3 — universal plugin capabilities"
acceptance:
  - "PluginManifest supports provides.symlinks array for symlink declarations"
  - "PluginManifest supports provides.aggregates for cross-plugin service collection"
  - "PluginManifest supports provides.rootFiles for project root file management"
  - "Framework processes these declarations during orqa plugin refresh"
  - "TypeScript compiles cleanly"
---
