---
id: "RULE-029db175"
title: "Continuous Operation"
description: "Orchestrating agents keep working until the user says to stop. Agents MUST NOT ask for permission to continue, propose stopping, or wait for confirmation between steps."
status: "active"
created: "2026-03-21"
updated: "2026-03-21"
enforcement: []
relationships: []
---

The user delegates a task to an agent. The agent works until the task is done or a genuine blocker is encountered. **Agents do not ask to stop. They do not propose stopping. They do not request permission to continue.**

## What This Means

- Complete all steps in a plan without pausing to ask "shall I continue?"
- When a step finishes, proceed to the next immediately
- Do not surface "I'm done with step X, want me to do step Y?" — just do step Y
- Do not end a response with "Let me know if you'd like me to continue" — continue

## When It Is Acceptable to Pause

There are only two acceptable reasons to pause mid-task:

1. **Genuine blocker** — a dependency is missing, a required artifact doesn't exist, a decision needs user input that cannot be inferred from context. Describe the blocker clearly and state what you need to proceed.
2. **Risky irreversible action** — force-pushing to main, dropping a database table, deleting files that appear to be user work. Confirm the action and its consequences before proceeding.

Everything else proceeds without asking.

## What Is NOT a Reason to Pause

- Uncertainty about which of two equally valid approaches to use — pick the one that better serves the active pillar and proceed
- A step taking longer than expected — keep going
- The task being large — size is not a reason to stop; break it into subtasks and work through them
- Reaching the end of a delegation step — proceed to the next delegation immediately

## Enforcement

This is a behavioral constraint enforced through two mechanisms:

1. **Agent system prompt** — the orchestrator's `Safety` section (`app/.orqa/process/agents/orchestrator.md`) states: "No deferred deliverables — if a deliverable is in scope, it ships NOW." The `CLAUDE.md` user preferences section states: "Never ask to stop — keep working until the user says to stop."
2. **Reviewer gate** — Reviewers verify acceptance criteria are fully met before a task is declared complete. Partial completion reported as complete is a separate violation (see RULE-honest-reporting).

The constraint is documented in the orchestrator agent definition so it is loaded into context on every session start.

## FORBIDDEN

- "Shall I continue?"
- "Let me know if you'd like me to proceed."
- "I'll pause here and wait for your input." (unless there is a genuine blocker)
- Stopping at the end of a step without beginning the next step

## Related Rules

- [RULE-e9c54567](RULE-e9c54567) (no-stubs) — deferred deliverables violate the same principle from a different angle
- [RULE-6083347d](RULE-6083347d) (dogfood-mode) — session state must be written before risky restarts, not as a reason to stop
