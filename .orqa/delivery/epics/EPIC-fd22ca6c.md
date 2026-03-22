---
id: EPIC-fd22ca6c
title: "Dashboard & artifact viewer UAT fixes"
description: "UAT round covering the full dashboard and artifact viewer experience. Fixes navigation bugs, redesigns dashboard widgets for actionable insight, enhances artifact viewer with relationship consolidation and pipeline position, and improves search UX."
status: active
priority: P1
created: 2026-03-13
updated: 2026-03-14
deadline: null
horizon: active
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-d622e989
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-b1688502
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-a0914d02
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-6757a72e
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-9637b896
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-8f0ef13d
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-40445eb7
    type: delivered-by
    rationale: Epic contains this task
  - target: IDEA-f9bb2c66
    type: realised-by
  - target: IDEA-e0aa14a3
    type: realised-by
  - target: IMPL-65c3ca6c
    type: cautioned-by
---
## UAT Findings Summary

34 findings collected during UAT of the dashboard and artifact viewer. Grouped into 8 systemic themes after architectural investigation.

### Findings Already Resolved

- **#15** (rules layer classification) — all 45 rules already have `layer` field populated
- **#17** (skills layer/category audit) — all 49 skills already classified
- **#20** (ideas research-needed structure) — current freeform design is appropriate
- **#28** (verification artifact type) — schema exists, awaiting EPIC-942c7678 TASK-J for first instances

### Theme A: Navigation & Layout Bugs

| # | Finding |
|---|---------|
| 10 | Nav sidebar icons missing for Process, Delivery, Docs |
| 22 | Milestones and Verification icons missing |
| 16 | Svelte error: duplicate key `RULE-303c1cc8relationships` in ReferencesPanel |
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
