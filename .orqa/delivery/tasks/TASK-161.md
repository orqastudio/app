---
id: TASK-161
title: Unify scrollbar styling across all panels
description: Make scrollbar appearance consistent across all panels — use the viewer panel scrollbar as the reference style.
status: ready
created: 2026-03-11
updated: 2026-03-11
docs:
  - DOC-021
acceptance:
  - All panels use the same scrollbar styling as the viewer panel
  - "Scrollbar width, track colour, thumb colour, and border radius are consistent"
  - No panel-specific scrollbar overrides remain unless intentionally different
relationships:
  - target: EPIC-010
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-030
    type: informed-by
  - target: SKILL-031
    type: informed-by
  - target: SKILL-030
    type: informed-by
  - target: SKILL-031
    type: informed-by
---
## What

The scrollbar styling varies across panels — the viewer panel has the correct appearance but other panels (conversation, artifact list, settings, etc.) use different or default scrollbar styles. This is a visual inconsistency bugfix.

## How

1. Inspect the viewer panel's scrollbar CSS (likely in a component CSS or global styles)
2. Extract the scrollbar styles into a shared CSS class or Tailwind utility
3. Apply to all scrollable panels: conversation panel, artifact list, settings, any other scrollable containers
4. Remove any panel-specific scrollbar overrides that conflict

## Verification

- [ ] All panels have visually identical scrollbars
- [ ] Viewer panel scrollbar unchanged (it's the reference)
- [ ] Scrollbar works correctly on all panel content (smooth scrolling, correct thumb size)
