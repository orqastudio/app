---
id: EPIC-8d3a1c5f
type: epic
title: Recurrence Detection & Rule Promotion
description: Implement the recurrence counter that tracks lesson re-occurrence across sessions and triggers the rule promotion workflow when a threshold is crossed. Includes UI for reviewing and confirming rule promotion and injection of promoted rules into generated system prompts.
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
    rationale: Stream 5 — recurrence detection closes the learning loop from lesson to rule to behaviour change
---

## Context

The learning loop requires more than capturing lessons — it requires detecting when the same lesson recurs and automatically proposing its promotion to a rule. Rules change AI behaviour through prompt injection. This epic builds the detection algorithm and the promotion UX that makes the loop visible to the user.

## Acceptance Criteria

- [ ] When a new lesson is captured, the system searches existing lessons for semantic similarity
- [ ] Recurrence counter on each lesson increments when a similar observation is made again
- [ ] At recurrence threshold (configurable, default 3), a rule promotion is proposed to the user
- [ ] UI: the user can review the proposed rule, edit it, and confirm or reject promotion
- [ ] Confirmed rules create a RULE artifact linked back to the source lessons
- [ ] Promoted rules are injected into generated system prompts at session start
- [ ] The user can see "Active Rules" in the UI and trace each rule back to its lessons
