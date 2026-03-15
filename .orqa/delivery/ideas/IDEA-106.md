---
id: IDEA-106
title: "Principles section — pillars, vision, and grounding documents as a dedicated nav section"
description: "Pillars are guiding principles that determine project purpose, not process artifacts. They should live alongside the product vision and agent grounding documents in a dedicated 'Principles' section. The current 'Process' section should be renamed to 'Learning' (rules, lessons, decisions, skills, agents) as it represents the learning and enforcement loop."
status: captured
created: 2026-03-15
updated: 2026-03-15
pillars:
  - PILLAR-001
  - PILLAR-003
milestone: null
horizon: next
research-needed:
  - "What belongs in Principles vs Learning vs Discovery"
  - "Should grounding docs be browsable in Principles or stay in Documentation"
  - "What icon represents Principles"
  - "How should the knowledge maturity state machine be configured alongside delivery statuses"
  - "What stages does the knowledge pipeline have and how do they transition"
promoted-to: null
spun-off-from: null
relationships:
  - type: informed-by
    target: PILLAR-001
    rationale: Principles section makes the project's guiding purpose visible
---

## Motivation

Pillars are currently under Process alongside rules, lessons, and decisions. But pillars aren't process artifacts — they're foundational principles that determine what the project IS, what ideas fit, and what work matters. They belong with the product vision and grounding documents, not with enforcement rules and implementation lessons.

The split:
- **Principles**: Pillars, vision, personas (first-class artifact), grounding docs (first-class artifact) — what the project believes, who it serves, and why
- **Learning** (renamed from Process): Rules, lessons, decisions, skills, agents — how the project improves
- **Discovery**: Ideas, research, wireframes (first-class artifact) — what the project is exploring

Wireframes belong in Discovery because that's when they're created — during exploration and shaping, before work is committed to the delivery pipeline. They're a discovery tool, not a reference document.

Grounding documents should be a first-class artifact type with their own schema, not just documentation files. They anchor agent behavior and project principles — that's structural, not prose.

## Knowledge/Learning State Machine

The governance artifacts (rules, lessons, decisions, skills, agents) should also have a configured state machine — separate from the delivery status machine. The knowledge maturity pipeline (observation → understanding → principle → practice → enforcement → verification) is a state machine that governs how knowledge evolves, just as the delivery statuses govern how work evolves.

This should be:
- Defined in project.json alongside the delivery and status configurations
- The single source of truth for knowledge lifecycle stages
- Documented in a dedicated guide
- Enforced by the same transition engine that handles delivery statuses

The current `maturity` field on lessons (observation/understanding) is a start, but it's not connected to the transition engine or configured as a formal state machine. The knowledge pipeline shown in the PipelineWidget on the dashboard should read its stages from this configuration rather than being hardcoded.
