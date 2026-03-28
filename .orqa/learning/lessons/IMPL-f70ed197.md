---
id: "IMPL-f70ed197"
type: lesson
title: "Recording observations is not scope creep — scope decisions happen at triage"
description: "The orchestrator sometimes hesitates to record observations because it perceives them as scope creep. But observations are just capture — whether to include them in the current epic's scope is a triage decision made at task completion or epic closure, per the three-tier observation logging discipline. Capture should never be gatekept."
status: completed
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---

## Pattern

The orchestrator sometimes avoids recording observations because it perceives the act of capture as scope creep. This conflates two distinct activities: (1) capturing that something was noticed, and (2) deciding whether to act on it within the current scope. Recording an observation commits to nothing — it simply ensures the learning isn't lost. Whether to implement, promote, or defer is a triage decision made later per the observation triage protocol.

## Fix

Observation: capture should be automatic and ungatekept. Scope decisions are made at triage (task completion or epic closure). The three-tier discipline ([IMPL-a1373533](IMPL-a1373533)) and observation triage ([RULE-b10fe6d1](RULE-b10fe6d1)) already define this separation — this lesson reinforces it.

## Triage

Promoted — encoded in [RULE-b10fe6d1](RULE-b10fe6d1) observation triage protocol. Recording is never scope creep. Triage determines disposition.
