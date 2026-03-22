---
id: RULE-c4fe67a2
title: Governance Priority Over Delivery
description: Lesson promotions (recurrence >= 2) and rule violations are ALWAYS CRITICAL priority — they take precedence over all delivery tasks. The learning loop breaks if governance debt accumulates.
status: active
created: 2026-03-21
updated: 2026-03-21
enforcement: stop hook escalation check + BEHAVIORAL_RULES injection + orqa audit escalation CLI (pending)
relationships: []
---

The learning loop is the mechanism by which observed problems become enforced constraints. If lesson promotions are deferred in favour of delivery work, the loop breaks silently — lessons recur indefinitely, governance debt accumulates, and the system loses credibility.

**Governance debt is always more expensive than delivery debt.** A lesson promoted today prevents ten future violations. A rule violation ignored today compounds.

## The Rule

When any of the following conditions are true, they are CRITICAL priority and MUST be addressed before continuing with any delivery task:

1. **A lesson has recurred two or more times** (`recurrence >= 2`) — it is due for promotion to a rule or knowledge artifact
2. **A rule is being actively violated** — the violation must be corrected and the enforcement gap that allowed it must be captured as a task

These conditions preempt all feature work, refactoring, and planning tasks. They do not go on the backlog. They do not wait for the next sprint or planning cycle.

## Promotion Protocol

When a lesson reaches `recurrence >= 2`:

1. **Stop current work** — log the current task state in session state before switching
2. **Promote the lesson** — create the appropriate artifact (rule, knowledge, or decision) in `.orqa/process/`
3. **Trace the promotion** — apply RULE-67b91c13 (trace-to-usage): update all affected milestones, pillars, rules, and epics
4. **Resume delivery work** — only after promotion and tracing are complete

## Violation Protocol

When a rule violation is discovered:

1. **Correct the violation** — fix the violating code, artifact, or behavior
2. **Capture the enforcement gap** — if a mechanical check would have caught this, create a CRITICAL task per RULE-12e74734
3. **Resume delivery work** — only after the violation is corrected and any gap is captured

## Why This Cannot Wait

Every session that passes without promoting an eligible lesson is a session where:
- The lesson can recur again
- The knowledge is not encoded where agents can find it
- The governance system claims to learn but doesn't

Delivery velocity without governance integrity is not velocity — it is accumulated risk.

## Enforcement

Three layers:

1. **Stop hook escalation check** — the `SessionStop` hook (`connectors/claude-code/hooks/`) checks for escalation candidates (lessons with `recurrence >= 2`, recent rule violations) and includes them in its output with CRITICAL priority. The orchestrator sees these at session end and must address them before the next session begins.
2. **BEHAVIORAL_RULES injection** — the `BEHAVIORAL_RULES` constant in `connectors/claude-code/hooks/prompt-injector.mjs` states this rule explicitly so orchestrating agents cannot claim ignorance.
3. **`orqa audit escalation` CLI** — a planned CLI command that auto-creates CRITICAL tasks in `.orqa/delivery/tasks/` for any unaddressed escalation candidates. This is to be implemented as part of the CLI maintenance tooling work.

## FORBIDDEN

- Logging a lesson promotion as a low or medium priority task
- Deferring a lesson promotion to a future sprint or epic without explicit user approval
- Continuing feature work when a lesson with `recurrence >= 2` is in scope
- Dismissing a rule violation with "I'll address this later"

## Related Rules

- [RULE-12e74734](RULE-12e74734) (enforcement-gap-priority) — both rules protect governance pipeline integrity; enforcement gaps and governance debt are closely related failure modes
- [RULE-67b91c13](RULE-67b91c13) (trace-to-usage) — lesson promotions trigger the trace-to-usage obligation; this rule ensures promotions happen promptly
