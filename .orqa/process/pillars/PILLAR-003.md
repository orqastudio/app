---
id: PILLAR-003
title: Purpose Through Continuity
description: The system actively maintains coherence between intention and action. It prevents drift between what the user set out to do and what is actually being done, ensuring purpose is never lost during implementation.
status: active
created: 2026-03-13
updated: 2026-03-13
gate:
  - Does this feature help users stay oriented toward their original purpose during extended work?
  - Does this prevent knowledge, decisions, or context from being silently lost?
  - Does this make scope drift visible and require explicit approval rather than happening implicitly?
  - Does this reduce the user's cognitive burden rather than adding to it?
  - Is the gap between declared intent and actual enforcement visible and shrinking?
relationships:
  - type: informed-by
    target: PILLAR-001
    rationale: Structure is the foundation continuity builds on — without visible, organised artifacts, there is nothing to maintain coherence across
  - type: informed-by
    target: PILLAR-002
    rationale: The learning loop feeds back into ongoing purpose maintenance — lessons and retrospectives help the system detect when execution has drifted from intention
  - target: RULE-046
    type: grounded-by
    rationale: "Auto-generated inverse of grounded-by relationship from RULE-046"
  - target: IMPL-050
    type: observed-by
    rationale: "Auto-generated inverse of observed-by relationship from IMPL-050"
  - target: IDEA-065
    type: informs
    rationale: "Friction-as-feature question resolved — absorbed into this pillar rather than becoming a separate one"
  - target: IMPL-065
    type: observed-by
    rationale: "Auto-generated inverse of observed-by relationship from IMPL-065"
  - target: AGENT-003
    type: grounded
    rationale: Orchestrator is grounded by this pillar — Purpose Through Continuity is injected as foundational context
  - target: DOC-064
    type: informs
    rationale: Purpose Through Continuity pillar is distilled into the product-purpose grounding document — inverse of informed-by on DOC-064
  - target: IDEA-096
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IDEA-096"
  - target: IMPL-066
    type: observed-by
    rationale: "Auto-generated inverse of observed-by relationship from IMPL-066"
  - target: RES-064
    type: informs
    rationale: "Auto-generated inverse of informs relationship from RES-064"
  - target: IDEA-095
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IDEA-095"
  - target: DOC-021
    type: informs
    rationale: Consistent standards allow agents across sessions to work without re-learning customs
  - target: DOC-072
    type: informs
    rationale: Plugin hooks ensure enforcement is continuous across every tool call
  - target: DOC-074
    type: informs
    rationale: Resetting store state in beforeEach ensures tests remain independent across sessions
  - target: DOC-073
    type: informs
    rationale: 80%+ coverage ensures the test suite remains meaningful as agents add features
  - target: DOC-069
    type: informs
    rationale: Correct delegation keeps the orchestrator focused on coordination across sessions
---
## What This Pillar Means

Purpose Through Continuity is the principle that the system actively maintains coherence between what the user intended and what is actually happening — across time, across interruptions, and across the complexity that accumulates during extended work.

Where Pillar 1 covers the present (make the current state clear) and Pillar 2 covers past-to-future (learn from experience), Pillar 3 covers through time: maintaining the thread of purpose from when work began to when it completes.

This pillar governs features that:

- **Mid-cycle orientation** — Periodic re-grounding in the original goal during extended work, so users never lose sight of why they started
- **Decision persistence** — Pending decisions survive interruptions and context changes, preventing silent knowledge loss between sessions or across long workflows
- **Scope coherence** — Changes to scope are explicit, tracked, and user-approved rather than drifting implicitly as implementation details accumulate
- **Therapeutic framing** — The system acts as a thinking partner that reduces cognitive burden, not one that adds to it through process overhead or information overload
- **Implementation awareness** — The system knows when execution has drifted from intention and surfaces that drift before it compounds

## Examples of Work That Serves This Pillar

- Session state that captures in-progress decisions, blockers, and resumption context so nothing is lost between sessions
- Scope tracking that surfaces when deliverables have been silently added, removed, or deferred without user approval
- Mid-task orientation prompts that reconnect implementation work to the original epic purpose
- Decision queues that persist unanswered questions across interruptions rather than dropping them
- Cognitive load indicators that help users recognise when a session has accumulated too much complexity
- Progress summaries that show not just what was done, but how it connects to the stated goal

## Anti-Patterns

- Features that add process steps without reducing the user's cognitive burden
- Systems that lose pending decisions or unanswered questions when context changes
- Scope changes that happen implicitly through implementation choices without surfacing them to the user
- Workflows that require the user to hold the thread of purpose in their own memory across sessions
- Tools that generate more information than the user can absorb, adding noise rather than clarity

## Conflict Resolution

Pillars are equal in importance. When this pillar appears to conflict with Pillar 1 (Clarity Through Structure) or Pillar 2 (Learning Through Reflection), the conflict should be flagged to the user for resolution rather than one pillar automatically winning. Agents do not prioritise one pillar over another unilaterally.

In practice, this pillar often reinforces both others: maintaining purpose requires the structured artifacts that Pillar 1 provides, and benefits from the accumulated lessons that Pillar 2 captures. Tension is most likely when continuity-preserving features risk adding cognitive overhead (conflicting with this pillar's own therapeutic framing principle) or when extensive reflection slows momentum toward the user's goal.
