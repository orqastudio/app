---
id: TASK-405
title: Improve pipeline visualization responsiveness and actionability
description: Make knowledge pipeline responsive to fill card width. Add reasoning and suggested actions to stuck/bottleneck labels. Audit pipeline stage health calculations for accuracy.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 2
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - Pipeline visualization fills surrounding card width — stages expand responsively
  - "Stuck and bottleneck labels include reasoning (e.g., '5 of 12 lessons have no downstream research link')"
  - Each stuck/bottleneck stage shows suggested actions to resolve
  - Pipeline stage health calculations audited and documented — thresholds adjusted if needed
  - Relationship types used for flow calculation match actual project usage
relationships:
  - target: EPIC-063
    type: delivers
    rationale: Theme C — pipeline visualization improvements from UAT
---

## Scope

### Responsive Layout (Finding #7)
- **File**: `PipelineWidget.svelte` (lines 249-309)
- **Current**: `flex` with `min-w-[100px]` fixed stage boxes, `overflow-x-auto`
- **Fix**: Remove fixed min-width, use `flex-1` or CSS grid so stages expand to fill container

### Reasoning & Actions (Finding #8)
- **File**: `PipelineWidget.svelte` (lines 156-163, 264-272)
- **Current**: Just shows "stuck" or "bottleneck" label based on flow rate thresholds
- **Fix**: Calculate which artifacts lack downstream connections and surface that list. Add suggested action (e.g., "Review 7 unlinked lessons")

### Health Calculation Accuracy (Finding #9)
- **File**: `PipelineWidget.svelte` (lines 27-70, 141-151)
- **Current**: Hardcoded relationship types and thresholds (0% = stuck, <30% = bottleneck)
- **Audit**: Verify the relationship types (`observes`, `grounded`, `grounded-by`, `enforces`, `enforces`) match what's actually used in the project. Adjust thresholds if warranted.
