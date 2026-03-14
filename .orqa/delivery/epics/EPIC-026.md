---
id: EPIC-026
title: Architecture Decisions
description: Formal architecture decision records (AD-007 through AD-017) capturing every significant technical choice made before implementation.
status: done
priority: P1
created: 2026-03-02
updated: 2026-03-07
milestone: MS-000
horizon: null
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on:
  - EPIC-025
blocks:
  - EPIC-029
  - EPIC-030
docs-required: []
docs-produced:
  - AD-007
  - AD-008
  - AD-009
  - AD-010
  - AD-011
  - AD-012
  - AD-013
  - AD-014
  - AD-015
  - AD-016
  - AD-017
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 1
  score: 34
relationships:
  - target: MS-000
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-099
    type: contains
    rationale: Epic contains this task
  - target: TASK-100
    type: contains
    rationale: Epic contains this task
  - target: TASK-101
    type: contains
    rationale: Epic contains this task
  - target: TASK-102
    type: contains
    rationale: Epic contains this task
  - target: TASK-316
    type: contains
    rationale: Epic contains this task
  - target: DOC-044
    type: documented-by
    rationale: Referenced in documentation page Roadmap
---
## Why P1

Architecture decisions are the governing law of the codebase. Every implementation agent must comply with them. Without these decisions, implementation is ungoverned.

## What Was Done

- [AD-007](AD-007) through [AD-017](AD-017) recorded in `.orqa/documentation/development/decisions.md`
- Decisions cover: sidecar integration pattern, streaming pipeline design, security model, MCP host approach, persistence strategy, governance format, composability principle
- Each decision includes context, the decision made, consequences, and status

## Output

`.orqa/documentation/development/decisions.md` — the authoritative record of all architecture decisions.

## Notes

Retroactively captured. Work preceded the artifact framework. These decisions remain active and govern all subsequent implementation.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-099](TASK-099): Record core architecture decisions (AD-007 through AD-010)
- [TASK-100](TASK-100): Record persistence and governance decisions (AD-011 through AD-014)
- [TASK-101](TASK-101): Record composability and integration decisions (AD-015 through AD-017)
- [TASK-102](TASK-102): Create architecture decisions index
