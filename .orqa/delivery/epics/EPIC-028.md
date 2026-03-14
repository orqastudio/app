---
id: EPIC-028
title: UX Design
description: "The complete UX specification: design system, wireframes, component inventory, interaction patterns, and responsive behaviour rules."
status: done
priority: P1
created: "2026-03-02"
updated: "2026-03-07"
milestone: MS-000
horizon: null
pillars:
  - PILLAR-001
depends-on:
  - EPIC-025
  - EPIC-027
blocks:
  - EPIC-029
  - EPIC-030
docs-required: []
docs-produced: []
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
  score: 28
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

All UX design documentation in `.orqa/documentation/ui/`.

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
