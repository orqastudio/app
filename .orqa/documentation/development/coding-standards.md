---
id: DOC-021
title: Coding Standards
description: Coding standards for Rust and TypeScript/Svelte covering formatting, linting, error handling, and component patterns.
created: 2026-03-02
updated: 2026-03-08
relationships:
  - target: AD-004
    type: documents
    rationale: Documentation page references AD-004
  - target: RULE-020
    type: documents
    rationale: Documentation page references RULE-020
---

**Date:** 2026-03-02

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

### Svelte 5 Runes [AD-004](AD-004)

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
- **No hardcoded fake data:** See [RULE-020](RULE-020)
- **Documentation-first:** Read governing docs before implementing

## Enforcement

```bash
make check
```

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Coding standards make the expected form of all code explicit and checkable — removing ambiguity about what "correct" looks like and making compliance visible through automated linting. |
| Learning Through Reflection | N/A |
