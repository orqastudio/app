---
id: IDEA-043
title: Rich Popovers on Artifact Links
description: "When hovering over an ArtifactLink chip (e.g. EPIC-001, TASK-045, AGENT-002), show a rich popover with the artifact's title, status, description, and key metadata — without navigating away from the current view."
status: captured
created: 2026-03-10
updated: 2026-03-13
horizon: later
research-needed:
  - Popover component patterns in shadcn-svelte (HoverCard vs Popover)
  - Performance implications of prefetching artifact metadata on hover
  - Debounce/delay strategy to avoid flicker on fast mouse movement
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---
## Motivation

Artifact links are everywhere in frontmatter — epics reference milestones, tasks reference epics, agents reference skills. Currently clicking a link navigates to the artifact, losing your place. A hover popover would let users preview linked artifacts in context, reducing navigation friction and improving the sense of connectedness between artifacts.

## Sketch

- On hover (with ~300ms delay), fetch the linked artifact's frontmatter via the artifact graph
- Display in a HoverCard: title, status badge, description (truncated), and 2-3 key fields
- Click still navigates as today
- Could reuse FrontmatterHeader in a compact/minimal mode
