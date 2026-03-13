---
id: IMPL-038
title: "Recording observations is not scope creep — scope decisions happen at triage"
description: "The orchestrator sometimes hesitates to record observations because it perceives them as scope creep. But observations are just capture — whether to include them in the current epic's scope is a triage decision made at task completion or epic closure, per the three-tier observation logging discipline. Capture should never be gatekept."
status: promoted
created: "2026-03-13"
updated: "2026-03-13"
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-023
    type: informed-by
    rationale: "Automated observation logging discipline defines when to log — this observation addresses the hesitation barrier that prevents logging"
  - target: IMPL-025
    type: informed-by
    rationale: "Lessons must flow forward — the triage task handles scope decisions, not the capture moment"
  - target: RULE-004
    type: observes
    rationale: "Observation triage section in RULE-004 separates capture from scoping — this lesson reinforces that separation"
  - target: RULE-004
    type: grounded-by
    rationale: "Lesson promoted to RULE-004 — observation capture is mandatory, scope decisions at triage"
  - target: RULE-004
    type: observed-by
    rationale: "RULE-004 codified the capture-vs-scope separation (observations always captured, scoped at triage) first observed in this lesson"
---

## Pattern

The orchestrator sometimes avoids recording observations because it perceives the act of capture as scope creep. This conflates two distinct activities: (1) capturing that something was noticed, and (2) deciding whether to act on it within the current scope. Recording an observation commits to nothing — it simply ensures the learning isn't lost. Whether to implement, promote, or defer is a triage decision made later per the observation triage protocol.

## Fix

Observation: capture should be automatic and ungatekept. Scope decisions are made at triage (task completion or epic closure). The three-tier discipline ([IMPL-023](IMPL-023)) and observation triage ([RULE-004](RULE-004)) already define this separation — this lesson reinforces it.

## Triage

Promoted — encoded in RULE-004 observation triage protocol. Recording is never scope creep. Triage determines disposition.
