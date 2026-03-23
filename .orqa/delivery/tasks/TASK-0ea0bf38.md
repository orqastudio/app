---
id: TASK-0ea0bf38
type: task
title: Product rebrand to Orqa Studio
description: "Full product rebrand from Forge to Orqa Studio across all code, configuration, documentation, agent definitions, and brand assets."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - Product renamed to Orqa Studio throughout
  - src/ renamed to ui/
  - .forge/ config removed
  - Brand assets replaced
  - All 16 agent definitions updated
relationships:
  - target: EPIC-de088896
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-7582d9b2
    type: depended-on-by
---
## What

Full product rebrand from Forge to Orqa Studio across all code, configuration,
documentation, and brand assets.

## Outcome

Complete rebrand across 50+ files. Source directory renamed, brand assets replaced,
WelcomeScreen redesigned. Git commits: `b20f9f8`, `4a1c88f`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.