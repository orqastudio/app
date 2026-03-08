---
id: EPIC-011
title: "Chat-Guided Onboarding"
status: draft
priority: P1
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 4
  impact: 5
  dependency: 3
  effort: 3
score: 10.3
roadmap-ref: "M1"
docs-required:
  - docs/product/personas.md
  - docs/product/journeys.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (onboarding plan)
  - docs/architecture/project-configuration.md (update with project state awareness)
description: >
  Make the chat conversation itself guide new users through setup,
  with project state awareness and entry mode detection.
tags: [onboarding, chat, entry-modes]
---

## Why P1

The conversation IS the onboarding. Without this, new users don't know what to do.

## Tasks

- [ ] System prompt includes project state awareness (what artifacts exist, what's configured, what's missing)
- [ ] AI suggests next steps when project is new or incomplete ("I notice you don't have any rules defined yet...")
- [ ] First-conversation guidance — when no sessions exist, AI introduces OrqaStudio's capabilities naturally
- [ ] Entry mode detection — AI identifies which mode (Problem, Idea, Goal, Chaos) fits the user's opening message
