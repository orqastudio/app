---
id: KNOW-dff413a0
type: knowledge
status: active
title: "Migration Plan and Phase Status"
domain: architecture
description: "Phase-by-phase migration overview — current phase status, sequencing rules, and zero tech debt enforcement during migration"
tier: on-demand
created: 2026-03-28
roles: [orchestrator, planner, reviewer]
paths: [.orqa/, targets/]
tags: [migration, phases, zero-tech-debt, target-first]
relationships:
  - type: synchronised-with
    target: DOC-dff413a0
---

# Migration Plan and Phase Status

## Core Principle: Target States First

Hand-write target outputs as test fixtures BEFORE building generation pipelines. Targets are only replaced by generated output once validated.

**`ORQA_DRY_RUN=true`** — all generation pipelines must support this. Writes to `.state/dry-run/` instead of overwriting live files.

## Phase Sequence

| Phase | What | Status |
| ------- | ------ | -------- |
| 1 | Establish target states + migration enforcement | (COMPLETE) |
| 2 | Engine extraction (Rust library crates) | (COMPLETE) |
| 3 | Daemon (persistent Rust process) | (COMPLETE) |
| 4 | Connector cleanup (pure generation + watching) | (COMPLETE) |
| 5 | Plugin manifest standardization | (COMPLETE) |
| 6 | Content cleanup (zero dead weight) | (COMPLETE) |
| 7 | Governance artifact migration (restructure `.orqa/`) | (COMPLETE) |
| 8 | Codebase restructure (directory layout) | (COMPLETE) |
| 9 | Frontend alignment | (COMPLETE) |
| 10 | Validate against targets | (IN PROGRESS) |
| 11 | Post-migration documentation | (IN PROGRESS) |

## Phase Gating Rules

- Do NOT start Phase N+1 until Phase N has PASS verdicts for ALL tasks from an independent Reviewer
- Each task requires: Implementer does work, Reviewer verifies, orchestrator reads verdict
- No silent deferrals — failed criteria must be fixed before moving on

## Zero Tech Debt Enforcement

Every phase must leave zero legacy:

- Delete deprecated code — do not comment out, never leave removal comments
- No backwards-compatibility shims
- No "follow-up" cleanup tasks deferred to later phases
- Legacy code WILL influence future agent behavior in the wrong direction
- File migration is never a blind copy — review against target architecture before moving

## Phase 10: Validate Against Targets

For each target from Phase 1:

1. Run the generation pipeline
2. Compare generated output against hand-written target
3. If match: replace target with generated version
4. If gap: fix the generation pipeline — do NOT modify the target
5. Remove `targets/` directory once all generation is validated

## Phase 11: Post-Migration Documentation

1. Convert architecture split files into proper `.orqa/` DOC and KNOW artifacts
2. Ensure every architectural concept has both a human-readable DOC and agent-consumable KNOW
3. Remove `targets/` directory
4. Remove `file-audit/` directory

## Completion Test (Post-Migration)

- Every target from Phase 1 produced by a generation pipeline
- Same methodology applies via app AND via Claude Code
- All enforcement is mechanical (generated hooks, linting, validation, permissions)
- `.orqa/` looks like something the finished app would have created
- Agents work without bypass permissions, scoped to their role
- Architecture documentation exists as proper governance artifacts
