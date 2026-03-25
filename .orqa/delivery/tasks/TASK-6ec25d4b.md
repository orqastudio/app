---
id: "TASK-6ec25d4b"
type: "task"
title: "Native integrity checks in artifact_graph.rs"
description: "Add integrity check methods to the artifact graph that detect broken links, missing relationship inverses, and pipeline gaps. Returns Vec<IntegrityCheck> with categorised findings, severity levels, and auto-fix indicators."
status: "completed"
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "IntegrityCheck struct defined with category, severity, artifact_id, message, auto_fixable fields"
  - "check_integrity() method returns all findings from the graph"
  - "Detects: broken refs, missing bidirectional inverses, null relationship targets"
  - "Severity levels: Error for broken refs, Warning for missing inverses"
  - "auto_fixable flag set for deterministic fixes (missing inverses)"
  - "Tauri command run_integrity_scan returns Vec<IntegrityCheck>"
  - "make check passes"
relationships:
  - target: "EPIC-82dd0bd2"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-25a42498"
    type: "depends-on"
---

## What

Port the logic from `verify-links.mjs` and `verify-pipeline-integrity.mjs` into native Rust methods on the artifact graph, so the app can run integrity checks without shelling out to Node.

## How

1. Define `IntegrityCheck` and `IntegrityCategory` types in `artifact_graph.rs`
2. Add `check_integrity(&self) -> Vec<IntegrityCheck>` method to `ArtifactGraph`
3. Add `run_integrity_scan` Tauri command in `graph_commands.rs`
4. Implement checks: broken refs, missing inverses, null targets

## Verification

- `make check` passes
- Run integrity scan on the live project, compare results with `node tools/verify-links.mjs`

## Lessons

(none yet)