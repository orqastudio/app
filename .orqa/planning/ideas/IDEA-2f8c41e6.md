---
id: IDEA-2f8c41e6
type: planning-idea
title: "Evaluate bunqueue.dev as a workflow execution queue"
description: "Research bunqueue.dev as a candidate runtime for the OrqaStudio workflow execution layer. Compare it to what the daemon currently does (in-process async workflow state), and decide whether the workflow-runner plugin model should lean on a dedicated queue primitive."
status: captured
priority: P3
created: 2026-04-11
updated: 2026-04-11
horizon: someday
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Workflow execution reliability is a core system capability — a proper queue primitive would make durable workflow state visible and recoverable instead of hidden in memory."
---

## Source

User research lead, captured 2026-04-11 in conversation: "look into https://bunqueue.dev/ as part of workflow execution".

## What

[bunqueue.dev](https://bunqueue.dev/) is a job queue library for the Bun runtime. The project claims to be a lightweight alternative to BullMQ / Sidekiq with first-class Bun support. The lead is: evaluate whether OrqaStudio's workflow-runner should adopt a dedicated queue primitive instead of the current in-process async state machine approach.

## Why it's relevant

OrqaStudio workflows are state machines that transition artifacts through lifecycle stages (discovery → planning → implementation → review → completion). Today the daemon holds workflow state in memory and rehydrates it on startup. A dedicated queue would buy:

- Durable retries for failed stage transitions (e.g. a hook that failed because the LLM API was down)
- Back-pressure on long-running stages without blocking other transitions
- A visible queue depth the devtools could expose as a governance-health signal
- Natural scheduling primitives (delayed jobs, recurring jobs) for reminders, session timeouts, dogfood checks
- Observability — job logs and failures become a first-class artifact

## What to investigate

- Does bunqueue work from Node as well as Bun, or is it Bun-only? OrqaStudio CLI is Node-first; the daemon is Rust. Which process would own the queue worker?
- How does bunqueue persist state? (SQLite? Redis? In-memory?) The daemon already owns a SQLite database — an embedded queue that piggybacks on `.state/orqa.db` would be ideal.
- What's the Rust story? Can the Rust daemon enqueue jobs that a Node/Bun worker consumes, or vice versa? Cross-language queue needs a stable wire format.
- Compare against `apalis` (Rust-native), `faktory`, `neoq` (Go, Postgres-backed), and embedded options like `sqlx` + a single `jobs` table.
- How does it compose with the event bus? Jobs that emit events → events that schedule jobs → both should flow through the bus.

## Decision criteria

- Must not require a standalone process beyond what the daemon already runs (no external Redis).
- Must support Rust producers and consumers (even if via an HTTP bridge).
- Must persist across daemon restarts — in-memory is a non-starter for workflow state.
- Prefer embedded-SQLite backing so the queue lives alongside the rest of `.state/orqa.db`.

## Relationship to existing work

- The workflow runner currently lives inside `engine/workflow/`. A queue primitive would slot behind the runner as an execution backend.
- The event bus (`daemon/src/event_bus.rs`) is already the fan-out channel for runtime events; queue job lifecycle events should publish there.
- The issue-group consumer (`daemon/src/issue_group_consumer.rs`) is a bespoke single-subscriber queue today. A proper queue library could subsume that pattern.

## Not in scope

- Replacing the current in-process workflow runner before the research concludes — this is a "look into" lead, not a committed migration.
- Adopting Bun as a runtime anywhere in the stack. If bunqueue is Bun-only, the research result is "not a fit" and we move on.
