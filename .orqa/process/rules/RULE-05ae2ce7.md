---
id: RULE-05ae2ce7
type: rule
title: Architecture Decisions
description: OrqaStudio-specific extension of RULE-05ae2ce7 (architecture-decisions). Defines the critical decisions that govern OrqaStudio's Rust/Tauri/Svelte stack and the .orqa/process/decisions/ directory as their source of truth.
status: active
created: 2026-03-07
updated: 2026-03-22
enforcement:
  - mechanism: behavioral
    message: "Orchestrator must read relevant architecture decisions before delegating any implementation; plans must include an Architectural Compliance section verifying all relevant decisions"
summary: "Architecture decisions in .orqa/process/decisions/ are source of truth. Critical decisions: error propagation (Result types, no unwrap), IPC boundary (Tauri invoke only), component purity (display vs container), type safety (strict TS, no any), immutability, UX-first, Svelte 5 only, SQLite for conversations only. Plans must include Architectural Compliance section."
tier: stage-triggered
roles: [implementer, planner, reviewer]
priority: P1
tags: [architecture, decisions, compliance, ipc, type-safety]
relationships:
  - target: AD-859ed163
    type: enforces
---
**Source of Truth:** `.orqa/process/decisions/` — individual `AD-NNN.md` artifacts. Decisions are first-class artifacts browsable in the app's artifact navigation.

This rule is OrqaStudio's specific extension of the generic architecture-decisions methodology (RULE-05ae2ce7). It documents the critical decisions for OrqaStudio's stack and the process for verifying compliance before writing code or plans.

## Critical Decisions (violations = immediate rejection)

| Decision | Rule |
|----------|------|
| Error propagation | All Rust functions return `Result`. No `unwrap()` / `expect()` / `panic!()` in production. `thiserror` for typed errors. |
| IPC boundary | Tauri `invoke()` is the ONLY frontend-backend interface. No side channels, no direct FFI. |
| Component purity | Display components receive props only. Pages/containers fetch data. No `invoke()` in `$lib/components/`. |
| Type safety | Strict TypeScript (no `any`). Rust IPC types derive `Serialize`/`Deserialize`. Types match across the boundary. |
| Immutability | Rust domain types immutable by default. Svelte stores use runes (`$state`, `$derived`). |
| UX-first design | User journeys drive backend requirements, not the reverse. |
| Svelte 5 only | Runes only. No Svelte 4 patterns (`$:`, `export let`, `let:`). |
| SQLite for conversations only | SQLite is scoped to conversation persistence (sessions, messages, metrics). All governance data lives in file-based artifacts with the node graph as the query layer. No localStorage for application state. ([AD-859ed163](AD-859ed163) supersedes [AD-75bb14ae](AD-75bb14ae)) |

## Before Writing Code

1. Check if your change affects any existing decision — browse decisions in the app or search `.orqa/process/decisions/` for the relevant `AD-NNN.md`
2. Read the relevant decision artifact(s) for full context
3. If proposing a new decision, create an `AD-NNN.md` in `.orqa/process/decisions/` following the framework schema (see `.orqa/documentation/about/artifact-framework.md` — Decision schema).

## Before Writing Plans

1. Read [RULE-dccf4226](RULE-dccf4226) (plan-mode-compliance)
2. Start with user journeys and UI design (UX-first)
3. Include architectural compliance section verifying all relevant decisions

## Related Rules

- [RULE-05ae2ce7](RULE-05ae2ce7) (architecture-decisions) — generic methodology this rule extends
- [RULE-dccf4226](RULE-dccf4226) (plan-mode-compliance) — plans must include an architectural compliance section verifying all decisions
- [RULE-ec9462d8](RULE-ec9462d8) (documentation-first) — architecture decisions ARE documentation; this rule defines their source of truth
- [RULE-1b238fc8](RULE-1b238fc8) (vision-alignment) — decisions implement the foundational principles; this rule governs their creation and compliance
- RULE-b03009da (end-to-end-completeness) — decisions define the layer requirements (IPC boundary, component purity, type safety) that every feature must satisfy