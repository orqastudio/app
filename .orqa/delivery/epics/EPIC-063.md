---
id: EPIC-063
title: Dashboard & artifact viewer UAT fixes
description: UAT round covering the full dashboard and artifact viewer experience. Fixes navigation bugs, redesigns dashboard widgets for actionable insight, enhances artifact viewer with relationship consolidation and pipeline position, and improves search UX.
status: active
priority: P1
created: 2026-03-13
updated: 2026-03-14
deadline: null
milestone: MS-001
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on:
  - EPIC-060
horizon: active
scoring:
  user-impact: High — dashboard is the primary entry point; bugs and missing features directly affect daily use
  scope: Medium — 8 task themes spanning frontend components, backend integrity logic, and data backfill
  urgency: High — UAT findings block confidence in EPIC-060 completion
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-060
    type: informs
    rationale: UAT findings from EPIC-060 review drive this epic's scope
  - target: EPIC-065
    type: informs
    rationale: Theme D findings extracted into dedicated artifact viewer epic
  - target: IMPL-053
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-053
  - target: IDEA-089
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IDEA-089
  - target: IDEA-090
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IDEA-090
  - target: IDEA-092
    type: informs
    rationale: Auto-generated inverse of informs relationship from IDEA-092
  - target: EPIC-072
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-072
  - target: EPIC-070
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-070
  - target: EPIC-071
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-071
  - target: EPIC-068
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-068
  - target: EPIC-067
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-067
  - target: EPIC-069
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-069
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-403
    type: contains
    rationale: Epic contains this task
  - target: TASK-404
    type: contains
    rationale: Epic contains this task
  - target: TASK-405
    type: contains
    rationale: Epic contains this task
  - target: TASK-407
    type: contains
    rationale: Epic contains this task
  - target: TASK-408
    type: contains
    rationale: Epic contains this task
  - target: TASK-409
    type: contains
    rationale: Epic contains this task
  - target: TASK-410
    type: contains
    rationale: Epic contains this task
  - target: TASK-403
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-403"
  - target: TASK-404
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-404"
  - target: TASK-408
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-408"
  - target: TASK-410
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-410"
  - target: TASK-409
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-409"
  - target: TASK-405
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-405"
  - target: TASK-407
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-407"
---
## UAT Findings Summary

34 findings collected during UAT of the dashboard and artifact viewer. Grouped into 8 systemic themes after architectural investigation.

### Findings Already Resolved

- **#15** (rules layer classification) — all 45 rules already have `layer` field populated
- **#17** (skills layer/category audit) — all 49 skills already classified
- **#20** (ideas research-needed structure) — current freeform design is appropriate
- **#28** (verification artifact type) — schema exists, awaiting EPIC-059 TASK-J for first instances

### Theme A: Navigation & Layout Bugs

| # | Finding |
|---|---------|
| 10 | Nav sidebar icons missing for Process, Delivery, Docs |
| 22 | Milestones and Verification icons missing |
| 16 | Svelte error: duplicate key `RULE-022relationships` in ReferencesPanel |
| 31 | Secondary nav persists when navigating to Settings |
| 32 | Settings secondary nav may exist but not rendering |
| 33 | Config modal double-renders behind modal AND in content area |

### Theme B: Dashboard Widget Redesign

| # | Finding |
|---|---------|
| 1 | 66 orphans — documentation artifacts should be excluded from orphan detection |
| 2 | Artifact graph status is low-value — combine with governance widget as per-type cards |
| 3 | Pipeline health requires manual scan — should auto-fetch on navigation |
| 4 | 10 errors / 197 warnings — inflated by documentation artifacts |
| 5 | Category dropdowns open by default but data fetches on toggle-to-open |
| 6 | Pipeline health should be a sortable/filterable data table |

### Theme C: Pipeline Visualization

| # | Finding |
|---|---------|
| 7 | Knowledge pipeline should be responsive and fill card width |
| 8 | Stuck/bottleneck labels have no reasoning or suggested actions |
| 9 | Pipeline stage health calculations may not represent project state |

### Theme D: Artifact Viewer Enhancements

| # | Finding |
|---|---------|
| 11 | Pillar grounded-by in metadata box — should be in relationships viewer |
| 12 | Universal: ALL relationships in relationships viewer, not metadata |
| 13 | Actions Needed box — inferred from status, hidden when empty |
| 14 | Pipeline position stepper — show artifact's current stage |
| 23 | Auto-show child artifacts (milestones→epics, epics→tasks) |
| 24 | Ideas horizon field not displayed anywhere |
| 26 | Acceptance criteria should render as checkboxes |
| 27 | Acceptance criteria schema needs structured checklist support |

### Theme E: Search Enhancement

| # | Finding |
|---|---------|
| 29 | Search should be semantic/AI, not just regex |
| 30 | Search results overflow container, input needs more contrast |

### Theme F: Agent Display

| # | Finding |
|---|---------|
| 18 | Subagent mapping should be sidecar-specific (idea) |
| 19 | Capabilities displayed as raw identifiers — need human-readable labels |

### Theme G: Data Integrity Backfill

| # | Finding |
|---|---------|
| 21 | Research documents sparse on relationships — schema update + backfill |
| 25 | Epic horizon field exists in schema but unpopulated |

### Theme H: Framework

| # | Finding |
|---|---------|
| 34 | tao event loop warnings flooding dev controller output |
