---
id: IDEA-fca8089b
type: discovery-idea
title: "Sections are graph filters — Principles, Discovery, Learning, Delivery"
description: "Nav sections don't categorise artifacts — they filter the graph by relationship patterns. An artifact appears in every section its relationships qualify it for. Decisions appear in multiple sections with contextual labels explaining why."
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: next
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

## Motivation

Pillars are currently under Process alongside rules, lessons, and decisions. But pillars aren't process artifacts — they're foundational principles that determine what the project IS. They belong with the product vision and grounding documents.

More fundamentally, sections shouldn't categorise artifacts at all. They should be graph queries — filters over the relationship graph that show different perspectives on the same data.

## The Four Sections

| Section | Shows | Label for Decisions |
|---|---|---|
| **Principles** | Pillars, Vision, Personas, Grounding | — |
| **Discovery** | Ideas, Decisions, project discovery types | "Decisions" (all) |
| **Learning** | Rules, Lessons, Skills, Agents, + decisions with `governs` edges | "Governing Decisions" |
| **Delivery** | Project delivery types + decisions with `drives` edges | "Driving Decisions" |

**Decisions** appear in Discovery (where they're made), Learning (when they govern rules), and Delivery (when they drive work). The contextual label explains WHY the decision appears in each section.

An artifact can appear in multiple sections. The sections are views, not containers.

## Relationship to Three-Layer Artifact Model

- **Principles** shows Layer 2 artifacts (app-required, project-authored)
- **Discovery** shows Layer 3 core instances (ideas, decisions) + project discovery types
- **Learning** shows Layer 3 core instances (rules, lessons, skills, agents) + governing decisions
- **Delivery** shows Layer 3 project-defined types + driving decisions