---
id: IMPL-023
title: "Observation logging and recurrence tracking should be automated, not manual"
description: "Agents and orchestrators encounter 'why did that happen?' moments during implementation but don't automatically log observations or increment recurrence on existing lessons. The learning loop depends on manual discipline which breaks under cognitive load. Automation would make the loop self-sustaining."
status: promoted
created: "2026-03-13"
updated: "2026-03-13"
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-021
    type: informed-by
    rationale: "The open-item tracking gap is a specific instance of the broader problem — agents don't self-log learning moments"
  - target: RULE-017
    type: observes
    rationale: "Lessons-learned rule requires agents to check and update lessons, but enforcement is purely procedural — agents forget under task pressure"
  - target: RULE-017
    type: grounded-by
    rationale: "Lesson promoted to RULE-017 — three-tier observation logging discipline encoded in lessons-learned rule"
  - target: RULE-017
    type: observed-by
    rationale: "RULE-017 codified the three-tier logging discipline (blocking/non-blocking/borderline) first observed in this lesson"
---

## Pattern

The Knowledge Maturity Pipeline has an automated observation-to-enforcement path on paper, but in practice:

1. **Observation creation is manual.** When an agent hits a surprising failure (e.g., stale paths after a directory move), it fixes the problem but doesn't create an IMPL entry unless specifically prompted by the user or a review agent.

2. **Recurrence tracking is manual.** Even when a lesson exists (e.g., [IMPL-017](IMPL-017) about stale paths), the next agent that hits the same pattern doesn't search lessons and increment the count. The recurrence counter stays at 1 forever.

3. **The promotion threshold is never reached.** Because recurrence isn't tracked, lessons never hit the >= 2 threshold that triggers promotion to rules or skills. The pipeline stalls at the observation stage.

The root cause: creating/updating lessons is a context switch that competes with the agent's primary task. Under cognitive load (debugging, implementing, fixing tests), the learning step gets dropped.

## Fix

Three-tier logging discipline (user-approved via RES-052):
1. **Clearly blocking** (affects other in-flight tasks) — log immediately, surface to orchestrator, pause affected tasks
2. **Clearly non-blocking** (self-contained) — log at task completion in Lessons section
3. **Borderline** — orchestrator asks user preference (block vs continue with caveat). Decision and rationale recorded on the task. Context-dependent: overnight autonomous work favours continue, supervised work favours block.

Learning checkpoint at task completion: orchestrator asks "what observations were logged?" before accepting done. If task involved debugging/workarounds/user corrections and answer is "none", orchestrator prompts for lesson review.

## Triage

Promoted — three-tier observation logging discipline (blocking/non-blocking/borderline) promoted to [RULE-017](RULE-017) update. Learning checkpoint at task completion encoded as process requirement.
