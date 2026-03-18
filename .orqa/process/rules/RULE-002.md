---
id: RULE-002
title: Architecture Decisions
description: All code must comply with architecture decisions in .orqa/process/decisions/.
status: active
created: 2026-03-07
updated: 2026-03-10
relationships:
  - target: AD-032
    type: enforces
  - target: DOC-036
    type: documented-by
---
**Source of Truth:** `.orqa/process/decisions/` — individual `AD-NNN.md` artifacts. Decisions are first-class artifacts browsable in the app's artifact navigation.

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
| SQLite for conversations only | SQLite is scoped to conversation persistence (sessions, messages, metrics). All governance data lives in file-based artifacts with the node graph as the query layer. No localStorage for application state. ([AD-032](AD-032) supersedes [AD-005](AD-005)) |

## Before Writing Code

1. Check if your change affects any existing decision — browse decisions in the app or search `.orqa/process/decisions/` for the relevant `AD-NNN.md`
2. Read the relevant decision artifact(s) for full context
3. If proposing a new decision, create an `AD-NNN.md` in `.orqa/process/decisions/` following the framework schema (see `.orqa/documentation/about/artifact-framework.md` — Decision schema).

## Before Writing Plans

1. Read [RULE-022](RULE-022) (plan-mode-compliance)
2. Start with user journeys and UI design (UX-first)
3. Include architectural compliance section verifying all relevant decisions

## Related Rules

- [RULE-022](RULE-022) (plan-mode-compliance) — plans must include an architectural compliance section verifying all decisions
- [RULE-008](RULE-008) (documentation-first) — architecture decisions ARE documentation; this rule defines their source of truth
- [RULE-031](RULE-031) (vision-alignment) — decisions implement the foundational principles; this rule governs their creation and compliance
- [RULE-010](RULE-010) (end-to-end-completeness) — decisions define the layer requirements (IPC boundary, component purity, type safety) that every feature must satisfy

