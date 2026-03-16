---
id: IDEA-105
title: "Delivery pipeline as a plugin — roadmap view, dashboard widgets, and type config packaged as an installable plugin"
description: "The current delivery pipeline (milestones/epics/tasks), roadmap view, dashboard widgets (milestone context, what's next, epics tab), and status transition rules should ultimately be packaged as an official plugin. This separates the universal thinking framework from the delivery-specific UI and type configuration. Requires plugin architecture work to support custom views, widgets, and type registration."
status: surpassed
created: 2026-03-15
updated: 2026-03-15
horizon: later
research-needed:
  - What plugin architecture changes are needed to support custom views and dashboard widgets
  - How does a plugin register new artifact types and their hierarchy
  - How do plugins contribute to the roadmap/kanban view
relationships:
  - type: merged-into
    target: EPIC-080
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
  - target: AD-055
    type: merged-into
---

## Motivation

The delivery pipeline (milestones, epics, tasks) is project-configurable per AD-051. The natural next step is packaging it as a plugin so the core app ships without any delivery opinion, and projects install the pipeline that fits their domain. A software project installs the milestones/epics/tasks plugin. A research project installs a hypotheses/experiments plugin. The core app provides the thinking framework; plugins provide the doing framework.

## Sketch

Requires plugin architecture to support:
- Custom artifact type registration (not just views)
- Custom views (roadmap kanban is a plugin-contributed view)
- Dashboard widget contribution (milestone context card, what's next)
- Status transition rules contributed by plugins

> **Surpassed 2026-03-16**: Delivery plugin extraction is EPIC-080 Phase 3. Status transition rules and domain-specific pipeline examples added to scope.
