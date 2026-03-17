---
id: EPIC-011
title: Chat-Guided Onboarding
description: "Make the chat conversation itself guide new users through setup, with project state awareness and entry mode detection."
status: captured
priority: P1
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
relationships:
  - target: MS-002
    type: delivers
    rationale: Epic belongs to this milestone
  - target: PILLAR-001
    type: grounded-by
  - target: EPIC-023
    type: depended-on-by
---
## Why P1

The conversation IS the onboarding. Without this, new users don't know what to do.

## Tasks

- [ ] System prompt includes project state awareness (what artifacts exist, what's configured, what's missing)
- [ ] AI suggests next steps when project is new or incomplete ("I notice you don't have any rules defined yet...")
- [ ] First-conversation guidance — when no sessions exist, AI introduces OrqaStudio's capabilities naturally
- [ ] Entry mode detection — AI identifies which mode (Problem, Idea, Goal, Chaos) fits the user's opening message

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
