---
id: IDEA-7e193b5c
type: planning-idea
title: "Evaluate HyperFlow for the learning-loop lessons pipeline"
description: "Research HyperFlow (hyperflow.lablnet.com) as a candidate framework for the OrqaStudio learning loop — the pipeline that turns captured lessons into enforced rules over time."
status: captured
priority: P3
created: 2026-04-11
updated: 2026-04-11
horizon: someday
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "The learning loop is what makes OrqaStudio harden itself over time. Any external framework that formalises lesson → rule escalation is worth comparing against the current bespoke pipeline."
---

## Source

User research lead, captured 2026-04-11 in conversation: "look into https://hyperflow.lablnet.com/ for learning loop lessons".

## What

[HyperFlow](https://hyperflow.lablnet.com/) is described as a workflow / learning-loop framework. The lead is: evaluate whether it offers primitives OrqaStudio could adopt for its own lessons-to-rules escalation pipeline, or whether the bespoke escalation model in `.orqa/learning/` is the right shape.

## Why it's relevant

OrqaStudio's learning loop is one of the system's distinguishing properties — captured lessons with repeated recurrence are supposed to automatically graduate into enforcement rules (see TASK-343851a8 and the `orqa audit escalation` command idea). The loop today is mostly declarative: rule files are hand-authored, lesson files record observations, and a future CLI command is supposed to bridge them. An external learning-loop framework might give us:

- A formalised state machine for lesson maturity (observation → correction → escalation → enforcement → surpassed).
- Metric primitives for recurrence, drift, and effectiveness.
- A reference implementation of the "close the loop" pattern that we can adopt or compare against.

## What to investigate

- What is HyperFlow's actual domain? The URL suggests a workflow orchestration tool, but the "lessons" framing could mean anything from MLOps to process mining. Skim the homepage and docs first.
- Does it have a concept analogous to OrqaStudio lessons (observational records that accumulate over time and escalate into automated rules)?
- What's the persistence model? SQLite / Postgres / document store / event log? Does it compose with OrqaStudio's existing `.state/orqa.db` + `.orqa/learning/` dual-store?
- Is there a runtime embedding path, or is it a hosted SaaS? OrqaStudio runs locally — a SaaS dependency is a non-starter.
- Licensing: is it open-source? What are the redistribution constraints?

## Decision criteria

- Must run entirely local / self-hosted — no external SaaS dependency.
- Must expose a primitive that maps cleanly to OrqaStudio's `mechanism: behavioral` → `mechanism: hook` escalation, or else offer a genuinely better model we can adopt wholesale.
- Must integrate with file-backed artifact storage (the lessons and rules live as markdown files in `.orqa/learning/`).

## Relationship to existing work

- Complements the `orqa audit escalation` idea (`TASK-343851a8`) — that work designs a CLI pass that scans lessons and creates escalation tasks. HyperFlow might provide the metric primitives that decide when escalation triggers.
- Related to the `lessons-as-memory-source` research (`research_lessons_as_memory_source.md` user memory) — that work explores the reverse direction (auto-memory → lessons). HyperFlow's learning loop might unify both directions.
- Feeds the graph health model (`project_graph_health_model.md` user memory) — one of the two pipelines there is "lessons → rules", so any framework we adopt must map onto that health view.

## Not in scope

- Wholesale replacement of the `.orqa/learning/` directory structure. Lessons and rules remain file-backed markdown artifacts.
- Adopting a new runtime language or framework just to use HyperFlow. If it requires us to rewrite the learning pipeline outside Rust/TypeScript, the answer is "inspiration only, not adoption".
- SaaS integration. This is a research lead, not a vendor evaluation.
