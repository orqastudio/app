---
id: IDEA-ec9f5b71
type: idea
title: Configurable dashboard with plugin-registered widgets
description: "Dashboard widgets should be configurable — users can choose which widgets to display, reorder them, and resize them. Plugins should be able to register custom widgets that appear alongside built-in ones."
status: surpassed
created: 2026-03-13
updated: 2026-03-13
horizon: someday
research-needed:
  - "What widget configuration model works best? Grid-based (like Grafana) or column-based (like Notion)?"
  - "How should plugin widgets register? Manifest declaration with a component entry point?"
  - "Should widget state (position, size, collapsed) persist per-project or globally?"
  - "What data contract do widgets need? Read-only access to artifact graph? Store subscriptions?"
  - "How does this interact with the five-layer architecture? Built-in widgets are core, plugin widgets are plugin/community layer"
relationships:
  - type: realises
    target: EPIC-dcc0bac6
  - target: EPIC-f7055984
    type: realises
    rationale: "UAT revealed dashboard widget UX needs fundamental rethinking — configurable widgets is the long-term direction"
  - target: AD-ee2910b1
    type: crystallises
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

## Motivation

The current dashboard has a fixed layout of widgets. UAT on EPIC-82dd0bd2 revealed that the built-in widgets need significant redesign (orphan detection exclusions, sortable data tables, actionable context). Rather than hardcoding a better fixed layout, the dashboard should be configurable so that:

1. Users can choose which widgets matter to their workflow
2. Plugins can contribute domain-specific widgets (e.g., a test coverage widget from a software project plugin)
3. Widget layout persists across sessions
4. Built-in widgets serve as sensible defaults but can be removed or rearranged

This aligns with the five-layer architecture — built-in widgets are core, official plugins provide recommended widgets, community/user plugins can add custom ones.

> **Surpassed 2026-03-16**: Widget registration and configuration persistence folded into EPIC-dcc0bac6 Phase 1 via AD-ee2910b1. Widget layout model (grid vs column) remains an implementation decision.