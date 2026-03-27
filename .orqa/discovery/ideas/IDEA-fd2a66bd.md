---
id: IDEA-fd2a66bd
type: discovery-idea
title: "User-customizable navigation overrides"
description: "Allow users to override the default plugin-driven navigation structure with custom layouts, enabling personalized artifact organization beyond what the methodology and stage plugins provide."
status: captured
created: 2026-03-26
updated: 2026-03-26
relationships:
  - target: "PILLAR-c9e0a695"
    type: grounded
    rationale: "Custom navigation makes project structure visible and navigable"
  - target: "PERSONA-477971bf"
    type: benefits
    rationale: "Practitioners can organize navigation to match their workflow"
---

## Context

The default navigation structure is plugin-driven: methodology stages as top-level items, workflow artifacts at second level, with custom views aligned. This works well as a default but power users may want to reorganize navigation to match their mental model or workflow preferences.

## Proposed Approach

A navigation settings page (removed in the current migration) would be reintroduced with the ability to:

- Override the default navigation hierarchy
- Create custom groupings of artifacts across stages
- Pin frequently used views
- Reorder or hide stages that aren't relevant to the user's daily work

The plugin-driven navigation remains the default. Overrides are stored per-user or per-project. Resetting to defaults is always available.
