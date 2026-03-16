---
id: TASK-431
title: Reusable pipeline stepper with configurable stages and visual refresh
description: Refactor PipelineStepper to accept stages as a prop and update styling to match the app look and feel.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Single PipelineStepper component used across all artifact types
  - Stages configurable via props
  - Visual styling reviewed and updated to match app look and feel
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Reusable pipeline stepper enables consistent status visualization across artifact types
  - target: TASK-430
    type: depended-on-by
---

## Scope

Refactor existing PipelineStepper.svelte to accept stages as a prop instead of hardcoding them. Update visual styling for consistency with the app design system. Ensure the component works for all artifact types with different stage configurations.
