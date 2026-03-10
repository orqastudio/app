---
id: EPIC-028
title: UX Design
status: done
priority: P1
milestone: MS-000
pillars: [PILLAR-001]
description: "The complete UX specification: design system, wireframes, component inventory, interaction patterns, and responsive behaviour rules."
created: 2026-03-02
updated: 2026-03-07
docs-required: [docs/product/]
docs-produced: [docs/ui/]
depends-on: [EPIC-025, EPIC-027]
blocks: [EPIC-029, EPIC-030]
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

All UX design documentation in `docs/ui/`.

## Notes

Retroactively captured. Work preceded the artifact framework. UX specs govern all subsequent frontend implementation.
