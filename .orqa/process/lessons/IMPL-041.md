---
id: IMPL-041
title: "Orchestrator should think critically about user suggestions, not just accept them"
description: "When the user shares an instinct or preference, the orchestrator should evaluate it against the system's principles and present a reasoned assessment — agreeing, disagreeing, or offering alternatives — rather than immediately accepting. The user explicitly asked for critical thinking, indicating the default behaviour is too deferential."
status: promoted
created: "2026-03-13"
updated: "2026-03-13"
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-026
    type: informed-by
    rationale: "Presenting decisions one at a time includes presenting the orchestrator's own analysis, not just options"
  - target: RULE-031
    type: observes
    rationale: "Vision alignment rule says to question misaligned instructions — this extends that to all design decisions, not just principle violations"
  - target: PILLAR-002
    type: observes
    rationale: "Learning through reflection requires honest assessment, not agreement"
  - target: RULE-031
    type: grounded-by
    rationale: "Lesson promoted to RULE-031 — extends questioning requirement to all design suggestions, not just principle violations"
  - target: RULE-031
    type: observed-by
    rationale: "RULE-031 codified the critical thinking requirement for user design suggestions first observed in this lesson"
---

## Pattern

The user proposed extending existing pillars rather than creating a new one. The orchestrator's default response would be to accept and implement. The user explicitly asked "think critically, not just accept my instinct" — revealing that the orchestrator tends toward deference when it should be offering independent analysis. This is especially important during design discussions where the user is looking for a thinking partner, not an executor.

## Fix

Not yet determined. Possible approaches:
1. Design discussion skill (IDEA-072) should include a "devil's advocate" checkpoint for user proposals
2. When presenting options, the orchestrator should always include its own recommendation with rationale
3. When the user states a preference, the orchestrator should evaluate it against system principles before accepting

## Triage

Promoted — extends [RULE-031](RULE-031)'s 'questioning misaligned instructions' requirement. Orchestrator should evaluate all user suggestions against system principles and offer independent analysis, not just accept.
