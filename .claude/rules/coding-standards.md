# Coding Standards

**Source of Truth:** `@docs/development/coding-standards.md`

**READ the full document before writing any code.**

## Rust Standards

- **Formatting**: `rustfmt` on all code, no exceptions
- **Linting**: `clippy` with pedantic and nursery lint groups enabled. Zero warnings in CI.
- **Error handling**: `thiserror` for all error types. Every function returns `Result<T, E>`. NO `unwrap()`, `expect()`, or `panic!()` in production code — only in tests.
- **Types**: All IPC types derive `Serialize`, `Deserialize`, `Debug`, `Clone`. Domain types should be immutable by default.
- **Module organization**: One module per domain concept. Public API via `mod.rs` or `lib.rs`. Keep `main.rs` minimal — it wires things together.
- **Functions**: <=50 lines (domain: 20-30, commands: 30-50, utilities: 10-20). Extract helpers when exceeding limits.
- **Dependencies**: Prefer well-maintained crates. No duplicate functionality. Pin versions in `Cargo.toml`.

## TypeScript / Svelte Standards

- **Svelte version**: Svelte 5 runes ONLY (`$state`, `$derived`, `$effect`, `$props`). No Svelte 4 patterns (no `let:`, no `$:` reactive statements, no `export let` for props).
- **Strict TypeScript**: `strict: true` in `tsconfig.json`. No `any` types. No `@ts-ignore`. No `as unknown as T` casts.
- **Components**: shadcn-svelte as the component library. Use variant props (`size`, `spacing`, `layout`) on shadcn components instead of inline Tailwind overrides. If a class appears 3+ times on a component, add it as a variant.
- **Component purity**: Pages and containers fetch data (call `invoke()`). Display components receive props only. No `invoke()` calls in `$lib/components/`.
- **Store pattern**: Runes-based stores in `.svelte.ts` files. Expose reactive state and actions. Stores call `invoke()`, components read stores.
- **NO emoji in UI** — use Lucide icons for all visual indicators. Emoji only for emotional reactions in conversational text.

## Both Languages

- **Coverage**: 80%+ test coverage. No exceptions without documented justification.
- **No TODO comments**: If something isn't done, it's tracked in TODO.md, not scattered across the codebase. TODO comments in committed code are a build failure.
- **No commented-out code**: Delete it. Git history preserves it.
- **No hardcoded fake data**: See `no-stubs.md`.
- **MUST use shared components**: See `reusable-components.md` for the shared component library.

## Enforcement

Run before every commit:

```bash
# Rust
cargo fmt --check && cargo clippy -- -D warnings && cargo test

# Frontend
npm run check && npm run lint && npm run test
```

## Related Rules

- `error-ownership.md` — *when* to verify (always, before every call)
- `reusable-components.md` — *which* components to use (shared library)
- `testing-standards.md` — testing patterns and coverage requirements
- `chunkhound-usage.md` — use semantic search before creating new code
