---
id: IMPL-012
title: "Encode Improvements in Artifacts"
category: governance
description: >
  If a process improvement exists only in conversation history, it is lost
  in the next session. Every improvement must be recorded as a lesson and
  encoded in the relevant agent, rule, or skill.
status: promoted
recurrence: 1
promoted_to: RULE-030
tags: [learning-loop, governance, enforcement, audit-trail]
---

## What Happened

During UAT Round 1, a better UAT process emerged organically: collect findings, group systemically, investigate architecture, then create tasks. The approach worked well but was only practiced — not encoded in any governance artifact.

## Why It Matters

If a process improvement exists only in conversation history, it is lost the next time a different agent (or the same agent in a new session) runs UAT. The learning loop requires that every process improvement be:

1. **Recorded as a lesson** (this file) — captures what was learned
2. **Encoded in the relevant agent/rule/skill** — ensures future sessions follow it
3. **Auditable** — the change trail from finding → lesson → enforcement artifact is traceable

Without encoding, the system repeats mistakes. With encoding, the system gets smarter. This is the difference between "we learned" and "we learn and enforce."

## The Rule

Every time a process works better than expected or a new pattern emerges:

1. Create a lesson (IMPL-NNN) documenting the pattern
2. Update the relevant governance artifact (agent prompt, rule, or skill)
3. Cross-reference: lesson → artifact, artifact → lesson
4. The lesson's `promoted_to` field tracks whether and where it was promoted

## Applies To

- All process improvements discovered during work
- All UAT findings that reveal process gaps
- All agent behavior corrections that should persist
