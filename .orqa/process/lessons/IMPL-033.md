---
id: IMPL-033
title: Blocking vs non-blocking observation handling should be a user preference
description: "When an agent encounters a borderline observation during implementation — one that might affect other tasks but isn't clearly blocking — the orchestrator should ask the user whether to block or continue. Context matters: overnight autonomous work favours 'continue with caveat', supervised work favours 'block and reassess'. The choice and its rationale should be recorded on the task."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships:
  - target: IMPL-023
    type: informed-by
    rationale: "Automated observation logging needs a decision framework for when to interrupt vs continue"
---

## Pattern

Three categories of observation during implementation:

1. **Clearly blocking** — affects how other in-flight tasks must be done. Log immediately, surface to orchestrator, pause affected tasks.
2. **Clearly non-blocking** — self-contained learning. Log at task completion in Lessons section.
3. **Borderline** — might affect other tasks, might not. Rework may or may not be needed.

For borderline cases, the right answer depends on context the agent doesn't have:
- User going to bed → prefer continue, accept potential rework, record the caveat
- User actively supervising → prefer block and reassess
- Deadline pressure → prefer continue
- High-risk area → prefer block

The orchestrator should ask the user for their preference in borderline cases. The decision and rationale get recorded on the task so that if rework IS needed later, the context for why it wasn't blocked is preserved.

## Fix

Add to the observation logging discipline:
- Clearly blocking: immediate, no question
- Clearly non-blocking: task completion
- Borderline: orchestrator asks user, records the preference and rationale on the task
