---
id: IMPL-025
title: Lessons must flow forward — a review task should exist whenever observations accumulate
description: "Observations logged during an epic must not sit idle. A lesson review task should be added to the epic to triage each observation: implement now (if needed to complete the epic), promote to rule/skill, or defer to a future idea. Lessons without a forward path are dead weight in the system."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-023
    type: informed-by
    rationale: "Automated logging produces observations — this lesson addresses what happens AFTER they're logged"
  - target: IMPL-024
    type: informed-by
    rationale: "Recording lessons on tasks makes them visible — this lesson addresses ensuring they are acted on"
  - target: RULE-017
    type: informs
    rationale: "Lessons-learned rule defines the promotion pipeline but has no mechanism for ensuring observations are triaged during the epic that created them"
  - target: RULE-004
    type: informed-by
    rationale: "Lesson promoted to RULE-004 — observation triage task auto-creation added to artifact lifecycle"
  - target: RULE-004
    type: informed-by
    rationale: "RULE-004 codified the observation triage task requirement first observed in this lesson"
  - target: IMPL-038
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-038"
---
## Pattern

[EPIC-059](EPIC-059) has accumulated 8 observations (IMPL-018 through IMPL-025) during implementation. None of them have been triaged. Some may need implementation within this epic to complete the work (e.g., [IMPL-021](IMPL-021) about tracking open items — that process gap is actively causing problems right now). Others could be deferred to future ideas (e.g., [IMPL-018](IMPL-018) about configurable paths is a non-trivial refactor).

Without a triage step, observations accumulate indefinitely at `maturity: observation` with `recurrence: 1`. The promotion pipeline never fires. The learning loop captures input but produces no output.

The missing piece: every epic that produces observations during implementation should automatically get a "review lessons" task. That task triages each observation into one of:

1. **Implement now** — the observation reveals a gap that blocks or undermines the epic's goals. Create a task within this epic.
2. **Promote** — the observation has reached understanding and should become a rule, skill update, or AD. Do it in this epic or create a task.
3. **Defer to idea** — the observation is valid but out of scope. Create an IDEA-NNN so it enters the planning pipeline and doesn't just sit in lessons.

"Leave it as an observation" is not a valid triage outcome for an epic that's trying to close. Every observation must have a forward path.

## Fix

Auto-created triage task (user-approved via RES-052). When the first observation is logged under an epic, a triage task is automatically created. Subsequent observations accumulate under the same task. At epic close, each observation must have a forward path:
1. **Implement now** — gap blocks or undermines epic goals, create task within epic
2. **Promote** — mature enough to become rule, skill, or AD
3. **Defer to idea** — valid but out of scope, create IDEA-NNN with relationship edge

"Leave it sitting" is not a valid triage outcome.

## Triage

Promoted — [RULE-004](RULE-004) observation triage protocol ensures a triage task exists when observations accumulate during an epic.
