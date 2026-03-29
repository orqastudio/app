---
id: KNOW-dff413a0
type: knowledge
status: active
title: Migration Plan
domain: architecture
description: Phase-by-phase migration overview — current phase status, sequencing rules, and zero tech debt enforcement during migration
tier: always
relationships:
  synchronised-with: DOC-dff413a0
---

# Migration Plan

## Core Principle: Target States First

Hand-write target outputs as test fixtures BEFORE building generation pipelines. Targets are only replaced by generated output once validated.

**`ORQA_DRY_RUN=true`** — all generation pipelines must support this. Writes to `.state/dry-run/` instead of overwriting live files.

## Phase Sequence

| Phase | What | Status |
| ------- | ------ | -------- |
| 1 | Establish target states + migration enforcement | Complete |
| 2 | Engine extraction (Rust library crates) | Complete |
| 3 | Daemon (persistent Rust process) | Complete |
| 4 | Connector cleanup (pure generation + watching) | Complete |
| 5 | Plugin manifest standardization | Complete |
| 6 | Content cleanup (zero dead weight) | Complete |
| 7 | Governance artifact migration (restructure `.orqa/`) | Complete |
| 8 | Codebase restructure (directory layout) | Complete |
| 9 | Frontend alignment | Complete |
| 10 | Validate against targets | In Progress |
| 11 | Post-migration documentation | In Progress |

## Zero Tech Debt Enforcement

Every phase must leave zero legacy:

- Delete deprecated code — do not comment out
- No backwards-compatibility shims
- No "follow-up" cleanup tasks
- Legacy code WILL influence future agent behavior in the wrong direction

## Completion Test (Post-Migration)

- Every target from Phase 1 produced by a generation pipeline
- Same methodology applies via app AND via Claude Code
- All enforcement is mechanical (generated hooks, linting, validation, permissions)
- `.orqa/` looks like something the finished app would have created
- Agents work without bypass permissions, scoped to their role
- Architecture documentation exists as proper governance artifacts
