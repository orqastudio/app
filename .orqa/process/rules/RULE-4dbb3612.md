---
id: RULE-4dbb3612
type: rule
title: Enforcement Gap Priority
description: Any discovered enforcement gap is immediately CRITICAL priority. It is never deferred, scoped out, or addressed in a future epic. This rule applies universally — not only in dogfood mode.
status: active
created: 2026-03-21
updated: 2026-03-21
enforcement:
  - mechanism: behavioral
    message: "Any discovered enforcement gap is immediately CRITICAL priority; it is never deferred, scoped out, or addressed in a future epic"
summary: "Any discovered enforcement gap is immediately CRITICAL priority — never deferred. Gaps exist when rules lack mechanical checks, decisions lack enforcing rules, pillar gates lack tooling, lessons recur without promotion, or stage transitions lack automation. Create CRITICAL task immediately, prioritize above feature work."
tier: always
roles: [orchestrator]
priority: P0
tags: [enforcement-gap, critical, governance-integrity]
relationships: []
---

Governance without enforcement is a fiction. Every rule, architecture decision, and pillar gate question that lacks mechanical enforcement is an enforcement gap. **An enforcement gap is always CRITICAL.**

## What Is an Enforcement Gap

An enforcement gap exists when any of the following are true:

- A rule exists but nothing mechanically checks it
- An architecture decision is accepted but no rule enforces it
- A pillar gate question cannot be answered by current tooling
- A lesson has recurred two or more times but has not been promoted to a rule
- A pipeline stage transition has no automation

## Response Protocol (NON-NEGOTIABLE)

When an enforcement gap is discovered:

1. **Stop what you are doing** — gaps are not tracked for later; they break the credibility of the entire governance system
2. **Capture immediately** — create a CRITICAL task in `.orqa/delivery/tasks/` for the gap
3. **Prioritize above feature work** — enforcement gaps preempt all non-critical work
4. **Do not wait for the next planning cycle** — there is no acceptable delay

This applies regardless of where the gap is found: during feature work, code review, planning, or any other activity.

## Why Universal

RULE-009 scoped this requirement to dogfood mode only. But the reasoning applies universally: any project that claims to use structured governance while tolerating known enforcement gaps is not credibly governed. The dogfood motivation (credibility) is not unique to dogfood projects.

## Enforcement

This is a behavioral constraint on orchestrating agents. Enforcement has two layers:

1. **Agent system prompt** — the orchestrator's `Safety` section includes: "Pipeline integrity first — enforcement gaps are always CRITICAL priority, not backlog." This is loaded on every session start via the orchestrator agent definition (`app/.orqa/process/agents/orchestrator.md`).
2. **Session start hook** — the `SessionStart` hook (`connectors/claude-code/hooks/`) reports enforcement health. Any rule with an empty `enforcement` array AND no `lint` delegation entry is surfaced as a potential gap.

## FORBIDDEN

- Logging an enforcement gap as a non-CRITICAL task
- Deferring an enforcement gap to a future epic without explicit user approval
- Treating "it's hard to enforce mechanically" as a reason not to create the task

## Related Rules

- [RULE-998da8ea](RULE-998da8ea) (dogfood-mode) — enforcement gap priority originated here; this rule makes it universal
- [RULE-1b238fc8](RULE-1b238fc8) (vision-alignment) — enforcement gaps undermine pillar credibility
- [RULE-42d17086](RULE-42d17086) (tooling-ecosystem) — linter delegation is the preferred mechanical enforcement mechanism
