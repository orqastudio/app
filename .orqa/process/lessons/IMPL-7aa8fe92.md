---
id: IMPL-7aa8fe92
type: lesson
title: Dogfooding proves the methodology before the app
description: "A CLI-only session building the Claude Code connector demonstrated full systems thinking and agile governance without the app running. The session itself was proof that the process works — CLI, artifact graph, hooks, and plugins are sufficient infrastructure for structured thinking."
status: captured
created: 2026-03-21
updated: 2026-03-21
maturity: observation
recurrence: 1
relationships: []
---

## Pattern

During a CLI-only session building the Claude Code connector, the orchestrator and user naturally demonstrated systems thinking and agile governance without the app running at all. Every change was traced for second-order effects (removing prompt content → where does it land?). Lessons were captured in real time. Patterns that recurred were promoted to rules immediately rather than deferred. Enforcement was built alongside the rule, not after.

The session itself was proof that the process works. No app window was open. No UI was involved. Yet the methodology — understand, plan, document, implement, review, learn — ran cleanly end to end.

## Root Cause

Not a failure — an observation. The product's value isn't the app UI. It's the structured thinking infrastructure:

- **CLI** (`orqa`) — validates artifacts, manages versions, queries the graph
- **Artifact graph** (`.orqa/`) — the single source of truth for governance, decisions, and delivery state
- **Hooks** (connector hooks) — inject context, enforce gates, surface health at the right moment
- **Plugins** — provide domain-specific knowledge and skills without hardcoding

These four are sufficient to demonstrate the full methodology. The app adds a visual layer on top of an already-working process — it makes the methodology more accessible but is not required for it to function.

## Significance

This validates the architectural decision to separate infrastructure from presentation. The CLI and artifact graph can be used standalone — by Jordan (solo developer) or in any environment where the app can't run (CI, headless servers, pairing sessions where the partner doesn't have the app).

It also sets a bar for what "the methodology works" means: if a session can run structured governance without the app, the infrastructure is healthy. If governance collapses without the app, that's a signal the tooling has drifted toward UI-dependency.

## Suggested Promotion

If this pattern recurs — sessions that demonstrate the methodology work at the CLI level and produce high-quality outcomes — promote to a knowledge artifact documenting CLI-first governance as a first-class operating mode. Consider adding a pillar gate question: "Can this methodology be demonstrated without the app UI?"