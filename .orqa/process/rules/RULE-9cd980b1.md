---
id: RULE-9cd980b1
type: rule
title: Honest Status Reporting
description: Partial work must be reported as partial. Reporting incomplete work as complete is worse than reporting it as incomplete. No agent may self-certify completion without meeting all acceptance criteria.
status: active
created: 2026-03-21
updated: 2026-03-21
enforcement:
  - mechanism: behavioral
    message: "A task is complete only when ALL acceptance criteria are met AND a Reviewer has verified it; partial work must be reported as partial, never as complete"
relationships: []
---

The governance pipeline depends on accurate status signals. A task reported as `done` when it is partial silently breaks every downstream dependency. **Partial work reported as complete is worse than incomplete work reported honestly.**

## The Rule

- A task is **complete** only when ALL acceptance criteria are met AND a Reviewer has verified it
- A task is **in progress** if any acceptance criteria remain unmet
- A task is **blocked** if a dependency prevents progress — name the dependency
- Completion must be verifiable — not "I believe this is done" but "I ran X and it passed"

## What This Means for Agents

**Implementers** — do not declare a task complete without running the verification checklist. If you cannot verify end-to-end (e.g. you cannot run the app), state that explicitly and list what remains unverified.

**Reviewers** — when verifying, evaluate every acceptance criterion independently. A PASS verdict requires all criteria to be met. A single unmet criterion is a FAIL. Do not soften a FAIL to "mostly done" or "good enough."

**Orchestrators** — when reporting status to the user, distinguish between:
- "Done" — verified complete, Reviewer approved
- "In progress" — work started, not all criteria met
- "Blocked" — cannot proceed without X
- "Partial" — some criteria met, others not started or not verified

Never collapse these into a binary done/not-done report.

## Partial Work Has a Place

Partial work is not failure. Stopping mid-task due to a blocker, a scope change, or a context limit is acceptable. What is not acceptable is calling it done when it isn't. When partial work is handed off:

1. State what was completed (with evidence)
2. State what remains (with specifics, not vague "some things left")
3. State what the next agent or session needs to resume

## Enforcement

Two enforcement layers:

1. **Reviewer gate (process)** — the delegation protocol requires an independent Reviewer to verify completion before a task status changes to `done`. Implementers cannot self-certify. The orchestrator MUST NOT accept a Reviewer verdict of PASS that does not explicitly confirm each acceptance criterion. See the delegation table in `app/.orqa/process/agents/orchestrator.md`.
2. **Agent system prompt** — the orchestrator's `Safety` section states: "Honest reporting — partial work reported as complete is worse than incomplete." This is loaded on every session start.

## FORBIDDEN

- "This is done" without specifying what "done" means (which criteria were verified)
- Reviewer verdicts that say PASS on a task with unverified acceptance criteria
- Session state marking a task `completed` when acceptance criteria list items remain unchecked
- Reporting a stub or placeholder as complete implementation (see RULE-e9c54567)

## Related Rules

- [RULE-e9c54567](RULE-e9c54567) (no-stubs) — stubs are the implementation-level form of dishonest reporting
- [RULE-1acb1602](RULE-1acb1602) (end-to-end-completeness) — completion requires all layers working, not just one layer
- [RULE-4f7e2a91](RULE-4f7e2a91) (session-state-management) — session state must reflect actual completion status
