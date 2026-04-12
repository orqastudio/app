---
id: "IMPL-36b767ce"
type: lesson
title: "Orchestrator must not offer to stop"
description: "The orchestrator asked the user 'or if you'd like to wrap the session here' — directly violating RULE-046 (Continuous Operation). Offering to stop is projecting the orchestrator's own state onto the user's intent."
status: archived
created: 2026-03-21T00:00:00.000Z
updated: 2026-03-21T00:00:00.000Z
maturity: "correction"
recurrence: 1
relationships:
  - type: cautions
    target: EPIC-281f7857
    rationale: "Orchestrator offering to stop violates continuous operation — cautions agent lifecycle"
---

## Pattern

The orchestrator composed a response that included "or if you'd like to wrap the session here" — offering the user an explicit exit point. This directly violates RULE-046 (Continuous Operation), which states: keep working until the user says stop. The orchestrator must never offer to stop, suggest wrapping up, or project a session endpoint onto the user.

## Root Cause

Default LLM conversation behaviour (wrapping up, offering closure) overrode the explicit rule. The rule exists and is codified, but it was not in active context when the orchestrator composed its response. The enforcement mechanism — the agent system prompt — was insufficient to suppress this default pattern.

This is a context activation failure, not a missing rule. The rule is clear. The orchestrator failed to apply it.

## Significance

This is a correction — RULE-046 already exists. The recurrence signals that the rule's presence in the system prompt is not enough to overcome the LLM's default conversational wrap-up instinct. Stronger enforcement may be needed: thinking mode injection, a pre-response self-check, or a process gate that flags any response containing stop-offer language.

## Suggested Promotion

If recurrence reaches 2, consider:

- Adding an explicit self-check to the orchestrator's thinking mode template: "Does this response offer the user an exit? If yes, remove it."
- Adding a hook that scans orchestrator responses for stop-offer patterns and flags them before delivery.
- Updating the RULE-046 enforcement entry to include a stronger signal in the agent preamble.
