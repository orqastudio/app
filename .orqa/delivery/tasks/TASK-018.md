---
id: TASK-018
title: Product rebrand to Orqa Studio
description: "Full product rebrand from Forge to Orqa Studio across all code, configuration, documentation, agent definitions, and brand assets."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - Product renamed to Orqa Studio throughout
  - src/ renamed to ui/
  - .forge/ config removed
  - Brand assets replaced
  - All 16 agent definitions updated
relationships:
  - target: EPIC-036
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-030
    type: informed-by
  - target: SKILL-031
    type: informed-by
  - target: TASK-326
    type: depended-on-by
  - target: SKILL-030
    type: informed-by
  - target: SKILL-031
    type: informed-by
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
