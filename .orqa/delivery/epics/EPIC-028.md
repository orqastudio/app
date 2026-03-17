---
id: EPIC-028
title: UX Design
description: "The complete UX specification: design system, wireframes, component inventory, interaction patterns, and responsive behaviour rules."
status: completed
priority: P1
created: 2026-03-02
updated: 2026-03-07
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 4
relationships:
  - target: MS-000
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-108
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-109
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-110
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-111
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-112
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-113
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-114
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-318
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-025
    type: depends-on
  - target: EPIC-027
    type: depends-on
  - target: EPIC-029
    type: depended-on-by
  - target: EPIC-030
    type: depended-on-by
  - target: PILLAR-001
    type: informed-by
  - target: DOC-057
    type: informs
  - target: DOC-058
    type: informs
  - target: DOC-059
    type: informs
  - target: DOC-060
    type: informs
  - target: DOC-061
    type: informs
  - target: PILLAR-001
    type: grounded-by
---
## Why P1

Implementation agents build to UX specifications. Without the UX design, the scaffold (Phase 1) has no spec to follow.

## What Was Done

- Design system — typography, colour palette, spacing scale, iconography conventions
- Wireframes — core layout, conversation view, artifact browser, settings and onboarding, dashboard
- Component inventory — all reusable UI components with their states and variants
- Interaction patterns — how the user navigates, creates, edits, and deletes artifacts
- Responsive behaviour — how the layout adapts across window sizes

## Output

All UX design documentation in `.orqa/documentation/reference/`.

## Notes

Retroactively captured. Work preceded the artifact framework. UX specs govern all subsequent frontend implementation.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-108](TASK-108): Define design system
- [TASK-109](TASK-109): Design core layout wireframes
- [TASK-110](TASK-110): Design conversation view wireframes
- [TASK-111](TASK-111): Design artifact browser wireframes
- [TASK-112](TASK-112): Design settings and onboarding wireframes
- [TASK-113](TASK-113): Define component inventory
- [TASK-114](TASK-114): Define interaction patterns and responsive behaviour
