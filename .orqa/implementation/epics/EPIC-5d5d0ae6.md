---
id: "EPIC-5d5d0ae6"
type: "epic"
title: "UX Design"
description: "The complete UX specification: design system, wireframes, component inventory, interaction patterns, and responsive behaviour rules."
status: archived
priority: "P1"
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-07T00:00:00.000Z
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 4
relationships:
  - target: "MS-063c15b9"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
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

- [TASK-5f8a459a](TASK-5f8a459a): Define design system
- [TASK-a7fc1f4a](TASK-a7fc1f4a): Design core layout wireframes
- [TASK-789686cf](TASK-789686cf): Design conversation view wireframes
- [TASK-6aae6264](TASK-6aae6264): Design artifact browser wireframes
- [TASK-04958677](TASK-04958677): Design settings and onboarding wireframes
- [TASK-f97ce886](TASK-f97ce886): Define component inventory
- [TASK-90c42842](TASK-90c42842): Define interaction patterns and responsive behaviour
