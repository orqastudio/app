---
id: "IMPL-336adfc0"
type: "lesson"
title: "Orchestrator should think critically about user suggestions, not just accept them"
description: "When the user shares an instinct or preference, the orchestrator should evaluate it against the system's principles and present a reasoned assessment — agreeing, disagreeing, or offering alternatives — rather than immediately accepting. The user explicitly asked for critical thinking, indicating the default behaviour is too deferential."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

The user proposed extending existing pillars rather than creating a new one. The orchestrator's default response would be to accept and implement. The user explicitly asked "think critically, not just accept my instinct" — revealing that the orchestrator tends toward deference when it should be offering independent analysis. This is especially important during design discussions where the user is looking for a thinking partner, not an executor.

## Fix

Not yet determined. Possible approaches:
1. Design discussion skill (IDEA-5e90b487) should include a "devil's advocate" checkpoint for user proposals
2. When presenting options, the orchestrator should always include its own recommendation with rationale
3. When the user states a preference, the orchestrator should evaluate it against system principles before accepting

## Triage

Promoted — extends [RULE-1e8a1914](RULE-1e8a1914)'s 'questioning misaligned instructions' requirement. Orchestrator should evaluate all user suggestions against system principles and offer independent analysis, not just accept.
