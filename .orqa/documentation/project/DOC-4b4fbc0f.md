---
id: DOC-4b4fbc0f
title: Coding Standards
category: reference
description: Coding standards for Rust and TypeScript/Svelte covering formatting, linting, error handling, and component patterns.
created: 2026-03-02
updated: 2026-03-08
sort: 3
relationships:
  - target: AD-8d552e96
    type: documents
    rationale: Documentation page references AD-8d552e96
  - target: RULE-e9c54567
    type: documents
    rationale: Documentation page references RULE-e9c54567
  - target: RULE-b49142be
    type: documents
    rationale: This document is the source-of-truth referenced by RULE-b49142be — it defines the specific standards that rule enforces
  - target: KNOW-01a64d58
    type: synchronised-with
  - target: KNOW-025fc31d
    type: synchronised-with
  - target: KNOW-5ad0bf1b
    type: synchronised-with
  - target: KNOW-58611337
    type: synchronised-with
  - target: KNOW-dac84f00
    type: synchronised-with
  - target: KNOW-8a821622
    type: synchronised-with
  - target: KNOW-282c0305
    type: synchronised-with
  - target: KNOW-49f495ff
    type: synchronised-with
  - target: KNOW-2b6147c9
    type: synchronised-with
  - target: KNOW-1b990160
    type: synchronised-with
  - target: KNOW-65f5aa67
    type: synchronised-with
  - target: KNOW-3f34e682
    type: synchronised-with
  - target: KNOW-7a96b952
    type: synchronised-with
  - target: KNOW-c60144c1
    type: synchronised-with
  - target: KNOW-12ed4953
    type: synchronised-with
---
## Purpose

Good code in this project is code that a future agent or contributor can read, trust, and extend without fear. That requires three things: explicit structure, honest error paths, and testable units.

The standards below exist not to impose bureaucracy but to enforce the principles that make code trustworthy at speed. When code is small, typed, and returns explicit results, it is safe to compose. When it silently swallows errors or grows past 50 lines, it becomes opaque — the next agent can't reason about it, and neither can you.

These standards serve **Clarity Through Structure** directly: a codebase that conforms to them is one where the expected form of every construct is checkable by machine, not negotiable by opinion. They serve **Purpose Through Continuity** because consistent standards allow agents across many sessions to work in the same codebase without re-learning local customs.

The standards are enforced by `make check`. The principles behind them are governed by [RULE-b49142be](RULE-b49142be). This document is the reference for what those standards are — read [RULE-b49142be](RULE-b49142be) for why each one exists.

---

## Rust Standards

### Formatting and Linting

- **`rustfmt`** — All code must be formatted. Run `cargo fmt --check` before commits.
- **`clippy`** — Pedantic and nursery lint groups enabled. Zero warnings. Run `cargo clippy --all-targets -- -D warnings`.

### Error Handling

- Use `thiserror` for all custom error types
- Every function returns `Result<T, E>` — no `unwrap()`, `expect()`, or `panic!()` in production code
- Error types are enums with descriptive variants
- IPC commands return `Result<T, String>` for Tauri serialization

### Type Design

- IPC types derive `Serialize`, `Deserialize`, `Debug`, `Clone`
- Domain types should be immutable by default
- Use newtypes for domain identifiers (`SessionId(String)`, `ArtifactId(String)`)

### Module Organization

- One module per domain concept
- Public API via `mod.rs` or `lib.rs`
- Keep `main.rs` minimal — it wires things together
- Commands in `commands/`, domain logic in `domain/`, data access in `repo/`

### Function Size

- Domain functions: 20-30 lines
- Command handlers: 30-50 lines
- Utilities: 10-20 lines
- If a function exceeds 50 lines, extract helpers

### Dependencies

- Prefer well-maintained crates with recent releases
- No duplicate functionality between crates
- Pin versions in `Cargo.toml`

## TypeScript / Svelte Standards

### Svelte 5 Runes [AD-8d552e96](AD-8d552e96)

- `$state()` for reactive state — never `let x = value` for reactive vars
- `$derived()` for computed values — never `$:` reactive declarations
- `$effect()` for side effects — never `$:` for side effects
- `$props()` for component inputs — never `export let`
- `{#snippet}` and `{@render}` — never `<slot>`

### TypeScript

- `strict: true` in `tsconfig.json`
- No `any` types — use proper types or `unknown`
- No `@ts-ignore` or `@ts-expect-error`
- No `as unknown as T` casts
- All function parameters and return types must be typed

### Components

- shadcn-svelte as the component library
- Use variant props on components, not inline Tailwind overrides
- Components under 150 lines — extract sub-components if larger
- All components handle loading, empty, and error states
- No emoji in UI — use Lucide icons for visual indicators

### Stores

- Runes-based stores in `.svelte.ts` files
- Stores call `invoke()` — components read from stores
- Expose reactive state and action methods
- One store per domain concept

## Both Languages

- **Coverage:** 80%+ test coverage, no exceptions without documented justification
- **No TODO comments:** Track as task artifacts in `.orqa/delivery/tasks/`, not scattered across the codebase
- **No commented-out code:** Delete it. Git history preserves it.
- **No hardcoded fake data:** See [RULE-e9c54567](RULE-e9c54567)
- **Documentation-first:** Read governing docs before implementing

## Enforcement

```bash
make check
```
  - target: KNOW-01a64d58
    type: synchronised-with
  - target: KNOW-63cc1a00
    type: synchronised-with
  - target: KNOW-e1333874
    type: synchronised-with
  - target: KNOW-025fc31d
    type: synchronised-with
  - target: KNOW-5ad0bf1b
    type: synchronised-with
  - target: KNOW-13ec986c
    type: synchronised-with
  - target: KNOW-58611337
    type: synchronised-with
  - target: KNOW-dac84f00
    type: synchronised-with
  - target: KNOW-8a821622
    type: synchronised-with
  - target: KNOW-282c0305
    type: synchronised-with
  - target: KNOW-49f495ff
    type: synchronised-with
  - target: KNOW-40addb7a
    type: synchronised-with
  - target: KNOW-2b6147c9
    type: synchronised-with
  - target: KNOW-a4b6310b
    type: synchronised-with
  - target: KNOW-1b990160
    type: synchronised-with
  - target: KNOW-65f5aa67
    type: synchronised-with
  - target: KNOW-3f34e682
    type: synchronised-with
  - target: KNOW-7a96b952
    type: synchronised-with

---
