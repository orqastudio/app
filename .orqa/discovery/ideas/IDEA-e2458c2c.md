---
id: IDEA-e2458c2c
type: discovery-idea
title: Design discussion mode with structured checkpoints
description: "A structured protocol for design discussions that enforces: sequential decision presentation, periodic zoom-outs, automatic observation logging, outcome tracking, and a wrap-up protocol before transitioning to implementation. Could be a skill, plan mode variant, or orchestrator behaviour."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: active
research-needed:
  - "Should this be a skill (protocol the orchestrator loads), a mode (like plan mode), or an automatic behaviour?"
  - "What are the right triggers for entering design discussion mode?"
  - "What checkpoint intervals work — every N decisions, N observations, or time-based?"
  - "How does the wrap-up protocol interact with the human gate on epic completion?"
  - "How does this work in the app vs CLI context?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

Extended design discussions produce decisions, observations, ideas, and scope changes. Without structure, problems emerge: decisions get batched, zoom-outs are forgotten, observations accumulate without logging, scope creeps without checkpoints, and outcomes aren't systematically captured before transitioning to implementation.

This session (EPIC-88f359b0 Phase 4 design) demonstrated all of these problems — each caught by the user, not the orchestrator. A structured mode would make the orchestrator self-correcting during design discussions, the same way plan mode makes it self-correcting during implementation planning.

The mode would define:

- **Entry**: detect when conversation shifts from implementation to design
- **During**: one decision at a time, immediate observation logging, zoom-out every 5 decisions
- **Wrap-up**: summarise all decisions, verify all observations logged with forward paths, confirm scope, create/update tasks
- **Exit**: transition to implementation only after wrap-up is complete
