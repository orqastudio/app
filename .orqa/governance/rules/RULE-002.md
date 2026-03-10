---
id: RULE-002
title: Architecture Decisions
description: All code must comply with architecture decisions in docs/architecture/decisions.md.
status: active
layer: canon
scope: project
created: "2026-03-07"
updated: "2026-03-07"
---
**Source of Truth:** `.orqa/documentation/architecture/decisions.md` (index) and `.orqa/decisions/` (individual `AD-NNN.md` artifacts)

**READ the index document to understand all architecture decisions. For detail on any specific decision, read its individual `AD-NNN.md` artifact.**

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
| SQLite for conversations only | SQLite is scoped to conversation persistence (sessions, messages, metrics). All governance data lives in file-based artifacts with the node graph as the query layer. No localStorage for application state. (AD-032 supersedes AD-005) |

## Before Writing Code

1. Read `.orqa/documentation/architecture/decisions.md` for the decision index
2. Check if your change affects any existing decision; read the relevant `AD-NNN.md` artifact for full context
3. If proposing a new decision, create an `AD-NNN.md` in `.orqa/decisions/` following the framework schema (see `.orqa/documentation/product/artifact-framework.md` — Decision schema). Adding a decision only to the index without an individual artifact is FORBIDDEN.

## Before Writing Plans

1. Read RULE-022 (plan-mode-compliance)
2. Start with user journeys and UI design (UX-first)
3. Include architectural compliance section verifying all relevant decisions

## Related Rules


