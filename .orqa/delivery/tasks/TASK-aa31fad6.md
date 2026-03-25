---
id: TASK-aa31fad6
type: task
title: "Dashboard integrity widget — health score, issue list, scan action"
description: "Add an integrity health widget to the ProjectDashboard that calls run_integrity_scan, displays categorised findings with severity badges, and provides a Scan button to refresh. This is the first user-visible output of the pipeline health system."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - IntegrityCheck TypeScript type mirrors the Rust struct
  - IntegrityWidget component shows health indicator (green/amber/red) based on error count
  - Issue list grouped by category with severity badges and artifact links
  - Scan button triggers run_integrity_scan and refreshes the display
  - "Empty state shows 'All clear' with green indicator"
  - Widget appears on ProjectDashboard below existing GraphStats
  - make typecheck passes
relationships:
  - target: EPIC-82dd0bd2
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-6ec25d4b
    type: depends-on
  - target: TASK-1648055e
    type: depended-on-by
  - target: TASK-c15165f5
    type: depended-on-by
---

## What

Surface the native integrity checks on the project dashboard so users can see pipeline health at a glance and run scans on demand.

## How

1. Add IntegrityCheck types to `ui/src/lib/types/artifact-graph.ts`
2. Add `runIntegrityScan()` method to `artifactGraphSDK`
3. Create `IntegrityWidget.svelte` component in `ui/src/lib/components/dashboard/`
4. Integrate into `ProjectDashboard.svelte`

## Verification

- `make typecheck` passes
- Widget renders on dashboard with scan results
- Clicking Scan refreshes the findings

## Lessons

(none yet)