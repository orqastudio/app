---
id: IMPL-035
title: Design discussions need a structured mode with built-in checkpoints
description: "Extended design discussions produce decisions, observations, ideas, and scope changes without structured checkpoints. The orchestrator should have a 'design discussion mode' that enforces: one decision at a time, periodic zoom-outs, automatic observation logging, and a wrap-up protocol that captures all outcomes before transitioning to implementation."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships: []
---
## Pattern

This session's design discussion produced 18 observations, 3 ideas, a research document, design decisions across 4 themes, and scope changes. The orchestrator had no structured protocol for managing this. Problems that emerged:

1. Decisions were batched instead of presented one at a time (IMPL-026)
2. No periodic zoom-out until user prompted (IMPL-034)
3. Observations accumulated without automatic logging until user prompted (IMPL-023)
4. No wrap-up protocol to verify all outcomes are captured before moving to implementation
5. Scope crept without explicit checkpoints to assess coherence

A "design discussion mode" would provide structure:
- Enter mode when a conversation shifts from implementation to design
- Present decisions sequentially
- Auto-zoom-out every N decisions or observations
- Track all outcomes (decisions, observations, ideas, scope changes)
- Wrap-up protocol before exiting: summary of decisions, observations logged, scope confirmed, tasks created

## Fix

Not yet determined. Could be:
1. A skill that defines the design discussion protocol
2. A plan mode variant optimised for design discussions
3. An orchestrator behaviour triggered by detecting design-level conversation
