---
id: EPIC-a2d6e4b9
type: epic
title: Thinking Mode Routing
description: Implement the 7-mode dispatch layer that routes user intent signals to the correct thinking mode workflow, with per-mode system prompt generation, quality gates, and mode-specific agent spawning. The orchestrator routes; agents execute; the user never needs to know the mode names.
status: captured
priority: P1
created: 2026-04-13
updated: 2026-04-13
horizon: next
scoring:
  impact: 5
  urgency: 3
  complexity: 4
  dependencies: 3
relationships:
  - target: MS-21d5096a
    type: fulfils
    rationale: Stream 3 — thinking mode routing is the AI integration layer for the methodology
---

## Context

OrqaStudio has 7 thinking modes (Research, Planning, Implementation, Review, Documentation, Debugging, Learning Loop). The orchestrator needs to detect which mode a user request belongs to and route it accordingly. The mode determines the system prompt, the agent roles spawned, the quality gates applied, and the artifact types produced.

## Acceptance Criteria

- [ ] Intent detection: user message classified into one of 7 modes (or "unknown" for re-prompt)
- [ ] Each mode has a defined system prompt template assembled from the prompt generation pipeline
- [ ] Mode routing is declarative — the mode definitions come from the core methodology plugin, not hardcoded
- [ ] Quality gates per mode: each mode defines its own completion criteria
- [ ] The user is never exposed to mode names unless they explicitly ask
- [ ] Mode detection accuracy measurable via DevTools session inspection
